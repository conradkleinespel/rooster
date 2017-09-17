#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf 'y\nxxxx\nabcd\n' | docker run --rm -i -v rooster:/home/rooster rooster add -s YouTube test@example.com || exit 1

# change password
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster get -s youtube 2>&1 | grep abcd || exit 1
