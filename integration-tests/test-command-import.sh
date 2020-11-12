#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# create the file
printf '\nxxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster init || exit 1

# add a password
printf 'xxxx\nabcd\n' | docker run --rm -i -v rooster:/home/rooster rooster add -s YouTube test@example.com || exit 1

# export in rooster format
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster export json > /home/rooster/export.json' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'jq ".[0].password" /home/rooster/export.json' | grep '"abcd"' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'jq ".[0].username" /home/rooster/export.json' | grep '"test@example.com"' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'jq ".[0].name" /home/rooster/export.json' | grep '"YouTube"' || exit 1

# export in 1password format
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster export json > /home/rooster/export.1password' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'cat /home/rooster/export.1password | grep "YouTube,test@example.com,abcd"' || exit 1
