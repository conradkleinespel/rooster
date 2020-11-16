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
# export in 1password format
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster export 1password > /home/rooster/export.1password' || exit 1

# import and check json import
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster delete YouTube || exit 1
json_export='{"passwords":[{"name":"YouTube","username":"test@example.com","password":"abcd","created_at":1605554169,"updated_at":1605554169}]}' || exit 1
echo "$json_export" | docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'tee /home/rooster/export.json' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster import json /home/rooster/export.json' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster export json' | jq ".passwords[0].password" | grep '"abcd"' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster export json' | jq ".passwords[0].username" | grep '"test@example.com"' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster export json' | jq ".passwords[0].name" | grep '"YouTube"' || exit 1

# import and check 1password import
printf 'xxxx\n' | docker run --rm -i -v rooster:/home/rooster rooster delete YouTube || exit 1
one_password_export='Note,abcd,YouTube,Login,youtube.com,test@example.com' || exit 1
echo "$one_password_export" | docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'tee /home/rooster/export.1password' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster import 1password /home/rooster/export.1password' || exit 1
docker run --rm -i -v rooster:/home/rooster --entrypoint sh rooster -c 'printf "xxxx\n" | rooster export 1password | grep "YouTube,test@example.com,abcd"' || exit 1
