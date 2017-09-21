#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf '\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster init || exit 1

# add a password
printf 'xxxx\nabcd\n' | docker run --rm -i -v rooster:/home/rooster rooster add -s YouTube test@example.com || exit 1

# check the password
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s youtube 2>&1 | grep abcd || exit 1
