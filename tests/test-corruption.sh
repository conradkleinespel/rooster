#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# https://stackoverflow.com/a/246128/1127635
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# import a corrupted rooster file
# NOTE: corrupting a file can be done with:
#     bbe -e 'r 77 X'
# where X is a different char than the one that was at index 77 (the signature start)
docker run --rm -v rooster:/data -v $DIR/corrupted.rooster:/corrupted.rooster --entrypoint sh \
    busybox -c 'cp /corrupted.rooster /data/.passwords.rooster && chmod 777 /data/.passwords.rooster' || exit 1

# try reading the file, rooster should warn about corruption
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster list
test $? != 0 || exit 1
