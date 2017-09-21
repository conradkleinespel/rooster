#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf '\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster init || exit 1

# add a password
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate YouTube yt@example.com || exit 1

# get initial password
out1="`printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s youtube 2>&1`"

# password exists
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate YouTube test@example.com
if [ $? = 0 ]; then
    exit 1
fi

# check that it has changed
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s youtube
