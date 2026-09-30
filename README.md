# JSGX — Java Trusted Applications inside Intel SGX
## Written as my Master Thesis at SICPA SA

JSGX runs an ordinary Java class as a **Trusted Application (TA)** inside an
Intel SGX enclave, using [Gramine](https://gramineproject.io/) as the library OS.
You annotate the methods that should be callable from outside. JSGX then exposes
each one as an HTTP endpoint and checks who is calling. When the `sgx` feature
is enabled, it also serves the endpoints over **RA-TLS**, a TLS certificate
that carries an SGX attestation quote.

This repository contains the prototype from my master thesis.

---

## How it works

```
             HTTP(S) :3000                     Unix socket /tmp/socket
  client  ───────────────────►  Rust server  ─────────────────────────►  JVM (java-tee.jar)
 (curl /                        (axum, rust-ta)     JSON request          IPCListener
  tee-client)                   • routing            JSON response         • reflection over
                  ◄───────────  • RSA login → JWT  ◄─────────────────────    @SecureFunction /
                                • RA-TLS cert (sgx)                           @AuthenticatedSecureFunction
                                                                           • your @TrustedApplication class
  └──────────────────────── all of this runs in one Gramine enclave (rust.manifest) ────────────────────┘
```

1. **Gramine** starts the Rust `server` binary as the enclave entrypoint
   (see `rust.manifest.template`). The manifest marks the JVM, its libraries
   and `java-tee.jar` as *trusted files*, so their hashes count toward
   `MRENCLAVE`.
2. On startup the server **spawns the JVM** with
   `java -jar -Xmx1G java-tee.jar` from the current working directory
   (`rust-ta/src/uds.rs`). It then retries a connection to `/tmp/socket` for
   up to about 10 s.
3. The jar's `Main` class is **generated at compile time** by the annotation
   processor. It creates an instance of your `@TrustedApplication` class and
   calls `new IPCListener(ta).run()`, which binds the Unix domain socket and
   serves requests on a 12-thread pool.
4. For each HTTP request the Rust server builds a JSON message and writes it
   to the socket. `IPCListener` looks the method up by name and converts the
   JSON parameters to the Java parameter types with Jackson. It then calls the
   method by reflection and writes back `{"data": …}` or `{"error": …}`.
5. **Build with `--features sgx`** to serve the API over RA-TLS. At startup
   `ra-tls` generates a P-384 key pair and writes a hash of the public key to
   `/dev/attestation/user_report_data`. It reads a DCAP quote from
   `/dev/attestation/quote` and embeds it, CBOR-encoded, in a self-signed
   X.509 certificate under the TCG DICE tagged-evidence OID `2.23.133.5.4.9`.
   The server also publishes the certificate in PEM form on
   `http://<host>:3333/certificate`.
   **Build without the feature** (the default) to get plain HTTP, which is
   what you want for local development.

### HTTP API (port 3000)

| Route                     | Auth             | Dispatches to                                                           |
|---------------------------|------------------|-------------------------------------------------------------------------|
| `POST /api/authenticate`  | signed body      | Returns a JWT-like session token                                        |
| `POST /<method>`          | none             | `@SecureFunction` method `<method>`                                     |
| `POST /ra/<method>`       | `Bearer <token>` | `@AuthenticatedSecureFunction` method `<method>`; gets the caller `User` |

The request body is a **JSON array of positional arguments**, for example
`[10]`, `[[1,2,3]]` or `[1, "2", 3.0]`. Leave the body empty or send `[]` for
methods that take no arguments. The response body is the method's return
value serialized with Jackson.

### Authentication protocol

Users are declared in a TOML config with an RSA-2048 public key and a list of
free-form **capabilities**:

```toml
mr_enclave = "TSTMRENCLAVE"

[[users]]
pubkey = "MIIBCgKCAQEA…"          # base64 of the PKCS#1 RSAPublicKey DER
capabilities = ["ADD", "READ"]
```

At startup each user gets a random UUID and the SHA-256 hash of their public
key.

To log in, the client sends:

```json
POST /api/authenticate
{ "body": b64url(json({ "mr_enclave", "timestamp", "pub_key_hash_encoded" })),
  "signature": b64url(RSA-PKCS1-SHA256(body)) }
```

The server then:

1. looks the user up by the public-key hash;
2. verifies the RSA signature;
3. checks that `mr_enclave` equals the configured value;
4. checks that `timestamp` is no more than **5 seconds** old.

If all checks pass, the server returns the token
`b64url({uuid, iat}).b64url(HMAC-SHA256(...))`. The HMAC key is generated
fresh on every server start, so tokens stop working after a restart.

For calls to `/ra/<method>`, the Rust server adds the resolved user to the IPC
message. The Java side receives the user as a `com.sicpa.javata.User(uuid,
capabilities)` record, passed as the method's **first parameter**. Each
method checks capabilities itself.

You can generate a user key pair with `rust-ta/gen_key.sh priv.der pub.der`.
It prints the base64 public key to paste into the config.

---

## Writing a Trusted Application

Add the two local Maven artifacts as dependencies:
`com.sicpa.java-ta:AnnotationProcessor` and `com.sicpa.jsgx:java-ta`.
Register the annotation processor, and build a `jar-with-dependencies` whose
main class is `com.sicpa.javata.Main`. Use
`use_cases/blacklist/java/pom.xml` as the template.

```java
@TrustedApplication                      // processor generates com.sicpa.javata.Main for this class
public class TA {
    @SecureFunction                      // POST /isHealthy
    public boolean isHealthy() { return true; }

    @AuthenticatedSecureFunction         // POST /ra/AddGovernmentBlacklist  (Bearer token)
    public boolean AddGovernmentBlacklist(User government, String[] blackList) {
        if (!government.capabilities().contains("ADD")) return false;
        ...
    }
}
```

Rules:

- Methods must be `public`.
- They are looked up by **simple name**, so overloads collide.
- Use exactly one `@TrustedApplication` class per jar.
- A thrown exception becomes an HTTP error response carrying the exception
  message.

### Included use cases

- **`use_cases/blacklist`** is the working reference. Two "governments" each
  upload a secret blacklist through `AddGovernmentBlacklist`, which requires
  the `ADD` capability. Once both lists are in, `getOutput` returns **only the
  intersection** and requires `READ`. Neither party ever sees the other's
  full list.
- **`use_cases/compute-and-anonymize`** is an earlier draft that is not
  maintained. It imports annotations from an old package, and its main class
  is hand-written instead of generated. It does not build against the current
  `java-ta`.

---

## Repository layout

```
.
├── rust-ta/                 Rust host: HTTP server, auth, IPC to the JVM (crate "lscp", lib "rust_ta")
│   ├── src/server.rs          axum router, TLS/RA-TLS serving
│   ├── src/auth.rs            signed login + HMAC session tokens
│   ├── src/uds.rs             JVM spawn + Unix-socket request/response
│   ├── src/config.rs          user/capability config loading
│   ├── src/client.rs          reusable HTTP client (incl. authenticated calls)
│   ├── src/bin/server/        server entrypoint
│   ├── src/bin/client/        `client` CLI
│   ├── resources/tests/       Config_test.toml + test RSA keys (govA, govB, key, unused)
│   └── gen_key.sh             RSA key pair helper for users
├── java-ta/
│   ├── processor/             annotations + processor that generates `Main`
│   └── ta/                    runtime: IPCListener, method dispatch, Result type, User record
├── ra-tls/                  RA-TLS library: quote → X.509 (attest.rs) and verification via
│                            Microsoft Azure Attestation (verify.rs, maa_client.rs)
├── use_cases/               example Trusted Applications
├── rust.manifest.template   Gramine manifest (4 GB enclave, 64 threads, DCAP, debug enclave)
├── Makefile                 local build / Gramine targets
├── Dockerfile, build.sh     containerized build that signs the enclave
├── run_docker.sh            run container with /dev/sgx_enclave + /dev/sgx_provision
├── restart_aesm.sh          starts the AESM service inside the container
├── sgx_default_qcnl.conf    DCAP quote provider config pointing at Azure's PCCS
├── push_docker.sh, push_to_azure.sh   helpers to ship builds to an Azure SGX VM (`azurevm` SSH alias)
└── tests.sh                 example curl calls against a running server
```

---

## Building and running

### Prerequisites

- Rust (stable, edition 2021)
- JDK 21 and Maven for building. The POMs target Java 21 and use pattern
  matching on sealed types.
- For enclave runs: Linux on SGX2/FLC hardware (for example an Azure DCsv3
  VM), plus Gramine, the Intel SGX DCAP/AESM packages, and a JRE at
  `/usr/lib/jvm/java-17-openjdk-amd64` as the manifest expects. Adjust the
  manifest if your JRE is somewhere else.

### 1. Install the Java runtime libraries into your local Maven repo

`java-ta/` has no aggregator POM, so build the two modules one at a time:

```bash
cd java-ta/processor && mvn install
```

```bash
cd java-ta/ta && mvn install
```

### 2. Run locally without SGX (plain HTTP)

```bash
make blacklist
```

This builds the blacklist jar, copies it to `rust-ta/java-tee.jar` and starts
the debug server from `rust-ta/`. The server loads
`./resources/tests/Config_test.toml` (the path is hard-coded in
`src/bin/server/main.rs`) and `./java-tee.jar` relative to its working
directory, so always start it from `rust-ta/`. In debug builds the JVM is
found on `PATH` and its output goes to your terminal.

Try it:

```bash
curl -X POST http://localhost:3000/isHealthy -H 'Content-Type: application/json' -d '[]'
```

To try a different TA, build its fat jar, copy it to `rust-ta/java-tee.jar`,
then run `cargo run --bin server` from `rust-ta/`.

### 3. Run under Gramine

```bash
make gramine-dev          # gramine-direct: no SGX hardware, useful to validate the manifest
make gramine-sgx          # real enclave; builds rust.manifest(.sgx) and signs it
```

`gramine-sgx-sign` needs an SGX signing key: RSA-3072 with public exponent 3.

```bash
openssl genpkey -algorithm RSA -out key.pem -pkeyopt rsa_keygen_bits:3072 -pkeyopt rsa_keygen_pubexp:3
```

Set `DEBUG=1` for Gramine debug logs and `RELEASE=1` for a Cargo release build.
Release builds launch the JVM from its absolute path and capture its
stdout/stderr.

### 4. Docker

```bash
./build.sh                # docker build, passing key.pem as the `sgxkey` build secret
```

```bash
./run_docker.sh           # needs /dev/sgx_enclave and /dev/sgx_provision on the host
```

The image builds the Rust server, then installs Gramine and the SGX packages on
Ubuntu 24.04. It then generates and signs `rust.manifest`, and its entrypoint
starts AESM followed by `gramine-sgx rust`. Use `push_docker.sh` to copy the
image to an Azure VM over SSH.

---

## Tests

```bash
cd rust-ta && cargo test
```

The tests cover:

- config parsing;
- the signed-login and token round trip;
- rejection of a wrong `MRENCLAVE`, an unknown key, or a stale timestamp;
- the command parser.

The server start-up tests (`server::tests`, `tests/auth.rs`) spawn the JVM, so
they need `rust-ta/java-tee.jar` to exist.

`cd ra-tls && cargo test` includes a test that calls the public Azure
Attestation endpoint, so it needs network access.


