#!/usr/bin/env bash
docker build -t jsgx --secret id=sgxkey,src=key.pem .
