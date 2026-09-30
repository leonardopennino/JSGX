# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

################################################################################
# Pick a base image to serve as the foundation for the other build stages in
# this file.
#
# For illustrative purposes, the following FROM command
# is using the alpine image (see https://hub.docker.com/_/alpine).
# By specifying the "latest" tag, it will also use whatever happens to be the
# most recent version of that image when you build your Dockerfile.
# If reproducability is important, consider using a versioned tag
# (e.g., alpine:3.17.2) or SHA (e.g., alpine@sha256:c41ab5c992deb4fe7e5da09f67a8804a46bd0592bfdf0b1847dde0e0889d2bff).
#FROM alpine:latest as base

FROM rust AS teebuilder
COPY rust-ta /media/rust-ta/
COPY ra-tls /media/ra-tls/
WORKDIR /media/rust-ta/
RUN ls -R
RUN cargo test
RUN cargo build --bin server --release --target-dir .

FROM maven:3.8.3-openjdk-17 AS tabuilder
COPY java-ta /media/java-ta
WORKDIR /media/java-ta
RUN mvn compile install && \
		mv ./target/java-ta-1.0-SNAPSHOT-jar-with-dependencies.jar ./java-tee.jar


################################################################################
# Create a final stage for running your application.
#
# The following commands copy the output from the "build" stage above and tell
# the container runtime to execute it when the image is run. Ideally this stage
# contains the minimal runtime dependencies for the application as to produce
# the smallest image possible. This often means using a different and smaller
# image than the one used for building the application, but for illustrative
# purposes the "base" image is used here.
#FROM base AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/

FROM ubuntu:24.04

# ARGs cannot be grouped since each FROM in a Dockerfile initiates a new build
# stage, resulting in the loss of ARG values from earlier stages.
ARG UBUNTU_CODENAME=noble

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y curl gnupg2 binutils

RUN curl -fsSLo /usr/share/keyrings/gramine-keyring.gpg https://packages.gramineproject.io/gramine-keyring-${UBUNTU_CODENAME}.gpg && \
    echo 'deb [arch=amd64 signed-by=/usr/share/keyrings/gramine-keyring.gpg] https://packages.gramineproject.io/ '${UBUNTU_CODENAME}' main' > /etc/apt/sources.list.d/gramine.list

RUN curl -fsSLo /usr/share/keyrings/intel-sgx-deb.key https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key && \
    echo 'deb [arch=amd64 signed-by=/usr/share/keyrings/intel-sgx-deb.key] https://download.01.org/intel-sgx/sgx_repo/ubuntu '${UBUNTU_CODENAME}' main' > /etc/apt/sources.list.d/intel-sgx.list

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y gramine \
    sgx-aesm-service \
    libsgx-aesm-launch-plugin \
    libsgx-aesm-epid-plugin \
    libsgx-aesm-quote-ex-plugin \
    libsgx-aesm-ecdsa-plugin \
    libsgx-dcap-quote-verify \
	libsgx-dcap-default-qpl \
	make \
	build-essential\
	findutils \
	openjdk-17-jre \
    psmisc && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /var/run/aesmd/

COPY restart_aesm.sh /restart_aesm.sh
WORKDIR /media/jsgx/
COPY --from=teebuilder /media/rust-ta/release/server .
COPY sgx_default_qcnl.conf /etc/
COPY rust.manifest.template Makefile ./
COPY --from=tabuilder /media/java-ta/java-tee.jar .

RUN gramine-manifest \
		-Dlog_level=error \
		-Darch_libdir=/lib/$(cc -dumpmachine) \
		-Dexec_dir=$(pwd) \
		-Djava_path=$(readlink -f $(which java)) \
		-Dentrypoint=./server \
		rust.manifest.template > rust.manifest

RUN --mount=type=secret,id=sgxkey \
	 gramine-sgx-sign -m rust.manifest -o rust.manifest.sgx -k /run/secrets/sgxkey

RUN chmod +x /restart_aesm.sh
ENTRYPOINT ["/bin/sh", "-c"]
CMD ["/restart_aesm.sh ; exec gramine-sgx rust"]
