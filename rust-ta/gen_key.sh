#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: ./gen_key.sh private_key.der public_key.der"
	exit
fi

openssl genpkey -algorithm rsa -pkeyopt rsa_keygen_bits:2048 -out $1 -outform der > /dev/null
echo "Create private key at $1"
openssl rsa -in $1  -inform der  -RSAPublicKey_out  -outform der -out $2 > /dev/null
echo "Create public key at $2"
data=$(cat $2 | base64)
echo "---Key data---"
echo $data
