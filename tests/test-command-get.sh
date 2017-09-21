#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf '\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster init || exit 1

# add passwords
printf 'xxxx\nabcd\n' | docker run --rm -i -v rooster:/home/rooster rooster add -s YouTube1 yt@example.com || exit 1
printf 'xxxx\nefgh\n' | docker run --rm -i -v rooster:/home/rooster rooster add -s YouTube2 yt@example.com || exit 1

# exact match
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s youtube1 2>&1 | grep abcd
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s youtube2 2>&1 | grep efgh

# fuzzy search
printf 'xxxx\n1\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s yt 2>&1 | grep abcd
printf 'xxxx\n2\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s yt 2>&1 | grep efgh
