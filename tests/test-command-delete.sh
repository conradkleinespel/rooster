#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf 'y\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster generate YouTube yt@example.com || exit 1

# check that password is listed
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list | grep YouTube || exit 1

# delete it
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster delete YouTube || exit 1

# check that password is no longer there
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list | grep YouTube
if [ "$?" = 0 ]; then
    exit 1
fi
