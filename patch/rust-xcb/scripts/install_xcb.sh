#! /bin/bash

set -x

wget --no-check-certificate https://xcb.freedesktop.org/dist/xcb-proto-1.11.tar.bz2
wget --no-check-certificate https://xcb.freedesktop.org/dist/libxcb-1.11.1.tar.bz2

tar -xjf xcb-proto-1.11.tar.bz2
cd xcb-proto-1.11
./configure
make
sudo make install
cd ..

tar -xjf libxcb-1.11.1.tar.bz2
cd libxcb-1.11.1
./configure --enable-xkb --enable-xinput --enable-selinux
make
sudo make install
cd ..
