#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# don't create the file
printf 'n\n' | docker run --rm -i -v rooster:/home/rooster \
    rooster list >& /dev/null
# rooster should return an error code
test "$?" = 1 || exit 1

# test that the file is not there
docker run --rm -v rooster:/data --entrypoint /bin/sh \
    busybox -c 'test "`ls -a1 /data | grep .passwords.rooster | wc -l`" = 0' || exit 2
