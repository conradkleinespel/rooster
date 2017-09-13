#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf 'y\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list || exit 1 

# test that the file is there
docker run --rm -v rooster:/data --entrypoint /bin/sh \
    busybox -c 'test "`ls -a1 /data | grep .passwords.rooster | wc -l`" = 1' || exit 2

# test that file need not be recreated on a new run
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list || exit 3

