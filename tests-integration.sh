#!/bin/bash

set -e

# https://stackoverflow.com/a/246128/1127635
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $DIR

docker build --pull --no-cache -t rooster -f Dockerfile.alpine .

chmod +x tests/*.sh

for testfile in `ls tests/*.sh`; do
    echo $testfile
    ./$testfile || exit 1
done

cd -
