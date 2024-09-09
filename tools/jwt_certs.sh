#!/bin/sh
openssl genrsa -out jwt.private.pem 3072
openssl rsa -in jwt.private.pem -pubout -out jwt.public.pem
