#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf '\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster init || exit 1

# add passwords - 2 strong ones and 2 weak ones
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate -s YouTube1 yt@example.com || exit 1
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate -s YouTube2 yt@example.com || exit 1
printf 'xxxx\nabcd\n' | docker run --rm -i -v rooster:/home/rooster rooster add -s YouTube3 yt@example.com || exit 1
printf 'xxxx\nabcd\n' | docker run --rm -i -v rooster:/home/rooster rooster add -s YouTube4 yt@example.com || exit 1

# check for weak passwords
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster weak | grep YouTube3 || exit 1
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster weak | grep YouTube4 || exit 1
