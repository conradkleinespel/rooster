#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf 'y\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list || exit 1

# change password to abcd
printf 'xxxx\nabcd\nabcd\n' | docker run --rm -i -v rooster:/home/rooster rooster set-master-password || exit 1

# check that password works
printf 'abcd\n' | docker run --rm -i -v rooster:/home/rooster rooster list || exit 1
