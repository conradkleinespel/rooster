#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf '\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster init || exit 1

# create a password
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate -s YouTube yt@example.com || exit 1

# check the username
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list | grep YouTube | grep yt@example.com || exit 1

# transfer the password
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster transfer YouTube vids@example.com || exit 1

# check transfer worked
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list | grep YouTube | grep vids@example.com || exit 1
