#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create a file
printf 'y\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list || exit 1

# try repeating password, OK on 3rd try
printf 'fail\nfail\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list || exit 3
