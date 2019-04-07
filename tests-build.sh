#!/bin/bash

set -e

docker build --pull --no-cache -t rooster -f Dockerfile.fedora .
docker build --pull --no-cache -t rooster -f Dockerfile.debian .
docker build --pull --no-cache -t rooster -f Dockerfile.ubuntu1604 .
docker build --pull --no-cache -t rooster -f Dockerfile.ubuntu1804 .
docker build --pull --no-cache -t rooster -f Dockerfile.alpine .
