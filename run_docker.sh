#!/bin/env bash
docker run --device /dev/sgx_enclave --device /dev/sgx_provision -it jsgx
