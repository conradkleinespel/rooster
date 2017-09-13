#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf 'y\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate -s YouTube yt@example.com || exit 1
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list | grep YouTube | grep yt@example.com || exit 1

# renaming worked
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster rename YouTube Videos || exit 1
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list | grep Videos | grep yt@example.com || exit 1
