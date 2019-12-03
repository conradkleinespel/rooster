#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf '\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster init || exit 1

# try repeating password, OK on 3rd try
printf 'fail\nfail\nfail\n' | docker run --rm -i -v rooster:/home/rooster rooster list
test "$?" != 0 || exit 1
