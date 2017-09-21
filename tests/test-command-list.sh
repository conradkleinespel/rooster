#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf '\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster init || exit 1

# add passwords
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate YouTube yt@example.com || exit 1
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate Google google@example.com || exit 1

# check that password is listed
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list | grep YouTube | grep yt@example.com || exit 1
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list | grep Google | grep google@example.com || exit 1
