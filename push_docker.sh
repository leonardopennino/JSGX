#!/bin/bash
set -e
docker build . -t jsgx
docker image save jsgx > jsgx.tar
scp jsgx.tar azurevm:~/jsgx.tar
ssh azurevm 'docker load < jsgx.tar'
