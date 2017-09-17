#!/bin/bash

# create a new empty volume for tests
docker volume rm rooster >& /dev/null
docker volume create rooster >& /dev/null

# export a password
printf 'y\nxxxx\nabcd\n' | docker run --rm -i -v rooster:/home/rooster rooster add -s YouTube test@example.com || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster export > /home/rooster/export.json' || exit 1

# check the exported JSON
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'jq ".[0].password" /home/rooster/export.json' | grep '"abcd"' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'jq ".[0].username" /home/rooster/export.json' | grep '"test@example.com"' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'jq ".[0].name" /home/rooster/export.json' | grep '"YouTube"' || exit 1
