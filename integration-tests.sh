#!/bin/bash

set -e

# https://stackoverflow.com/a/246128/1127635
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $DIR

docker build -t rooster .

chmod +x tests

for testfile in `ls tests/*.sh`; do
    echo $testfile
    ./$testfile
done

cd -
