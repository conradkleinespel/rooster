#!/bin/sh

curl https://sh.rustup.rs -sSf | sh -s -- -y
if [ "$?" != "0" ]; then
    echo 'aborting: could not install rust' 1>&2
    exit 1
fi
if [ "$CARGO_HOME" != "" ]; then
    export PATH="$CARGO_HOME/bin:$PATH"
else
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# ubuntu/debian
distro="`lsb_release -is`"
if [ "$?" = "0" ]; then
    if [ "$distro" = "Ubuntu" ]; then
        version="`lsb_release -rs`"
        if [ "$?" = "0" ]; then
            if [ "$version" = "16.04" -o "$version" = "16.10" ]; then
                sudo apt update -y && sudo apt install -y unzip pkg-config libx11-dev libxmu-dev
                if [ "$?" != "0" ]; then
                    echo 'aborting: could not install rooster dependencies' 1>&2
                    exit 1
                fi
            elif [ "$version" = "14.04" ]; then
                sudo apt-get update -y && sudo apt-get install -y unzip pkg-config libx11-dev libxmu-dev
                if [ "$?" != "0" ]; then
                    echo 'aborting: could not install rooster dependencies' 1>&2
                    exit 1
                fi
            fi
        fi
    elif [ "$distro" = "Debian" ]; then
        sudo apt-get install -y gcc unzip pkg-config libx11-dev libxmu-dev
        if [ "$?" != "0" ]; then
            echo 'aborting: could not install rooster dependencies' 1>&2
            exit 1
        fi
    fi
fi

# fedora/centos with dnf/yum
dnf -h > /dev/null
dnfstatus="$?"
yum -h > /dev/null
yumstatus="$?"
if [ "$dnfstatus" = "0" ]; then
    sudo dnf install -y gcc unzip pkgconfig libX11-devel libXmu-devel
    if [ "$?" != "0" ]; then
        echo 'aborting: could not install rooster dependencies' 1>&2
        exit 1
    fi
elif [ "$yumstatus" = "0" ]; then
    sudo yum install -y gcc unzip pkgconfig libX11-devel libXmu-devel
    if [ "$?" != "0" ]; then
        echo 'aborting: could not install rooster dependencies' 1>&2
        exit 1
    fi
fi

rm -rf /tmp/rooster-master /tmp/rooster-master.zip

curl -sSL https://github.com/conradkleinespel/rooster/archive/master.zip -o /tmp/rooster-master.zip
if [ "$?" != "0" ]; then
    echo 'aborting: could not download rooster' 1>&2
    exit 1
fi

unzip /tmp/rooster-master.zip -d /tmp
if [ "$?" != "0" ]; then
    echo 'aborting: could not unzip rooster' 1>&2
    exit 1
fi

cd /tmp/rooster-master
cargo build --release && cargo build
buildstatus="$?"
cd -
if [ "$buildstatus" != "0" ]; then
    echo 'aborting: could not build rooster' 1>&2
    exit 1
fi

# there is currently a bug in the clipboard library that prevents it from working
# when built in "release" mode, so we need to build it in "debug" mode
sudo cp /tmp/rooster-master/target/debug/rooster-clipboard /usr/bin/rooster-clipboard
if [ "$?" != "0" ]; then
    echo 'aborting: could not copy rooster-clipboard' 1>&2
    exit 1
fi

# but, we build rooster in "release" mode for better performance
sudo cp /tmp/rooster-master/target/release/rooster /usr/bin/rooster
if [ "$?" != "0" ]; then
    echo 'aborting: could not copy rooster' 1>&2
    exit 1
fi
