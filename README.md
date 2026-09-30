# Building docker image
To build the docker image run `./build.sh`
To complete the build a secret key file must be provided, the key has to be SGX-compatible e.g. RSA 3072 keys with public exponent equal to 3.

## Key Generation
You can run the following openssl command to generate a signing key for sgx:
`openssl genpkey -outform PEM -out sgx_key.pem -algorithm RSA -pkeyopt rsa_keygen_bits:3072 -pkeyopt rsa_keygen_pubexp:0x3`

# Running on SGX
First, push the docker image to azure.
```
docker image save > image.tar
scp image.tar azurevm:/home/azureuser/image.tar
scp run_docker.sh /home/azureuser/
```
In VM :  `docker load < image.tar && ./run_docker.sh`

