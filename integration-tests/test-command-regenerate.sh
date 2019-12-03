#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf '\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster init || exit 1

# generate a password
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate YouTube yt@example.com || exit 1

# get initial password
out1="`printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s youtube 2>&1`"

# regenerate it
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster regenerate YouTube || exit 1

# check that it has changed
out2="`printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s youtube 2>&1`"

if [ "$out1" = "$out2" ]; then
    exit 1
fi
