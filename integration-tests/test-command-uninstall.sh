#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

docker run --rm -i -v rooster:/home/rooster rooster uninstall | grep 'sudo rm /usr/bin/rooster' || exit 1
