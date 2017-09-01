#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# adds Dropbox folder
docker run --rm -v rooster:/data --entrypoint sh \
    busybox -c 'mkdir /data/Dropbox && chmod 777 /data/Dropbox' || exit 1

# create the file in Dropbox folder
printf 'y\nxxxx\nn\n' | docker run --rm -i -v rooster:/home/rooster rooster list || exit 2

# test that the file is there, not in the Dropbox folder
docker run --rm -v rooster:/data --entrypoint /bin/sh \
    busybox -c 'test "`ls -a1 /data | grep .passwords.rooster | wc -l`" = 1' || exit 4

# test that file need not be recreated on a new run
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list || exit 5
