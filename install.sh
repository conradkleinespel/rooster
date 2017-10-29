#!/bin/sh

pkgname=rooster
pkgver=2.7.1
sha256=b1413d220f240e9f9fc99e3be705c029a10f82e67df2858ee15ba09c3f5c1b51
os=`uname`

# Arch Linux gets its own package on the AUR
cat /etc/*-release | grep -i 'Arch Linux' > /dev/null
if [ "$?" = "0" ]; then
    echo 'Looks like you are using Arch Linux. You can find Rooster on the AUR:'
    echo 'https://aur.archlinux.org/packages/rooster'
    exit
fi

# install Rust/Cargo so we can compile the sources
rustc 2>/dev/null >/dev/null && cargo 2>/dev/null >/dev/null
if [ "$?" != 0 ]; then
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    if [ "$?" != "0" ]; then
        echo 'aborting: could not install rust' 1>&2
        exit 1
    fi
    # if rust was not previously installed, we'll also set the environment
    # so cargo works without reloading the shell
    if [ "$CARGO_HOME" != "" ]; then
        export PATH="$CARGO_HOME/bin:$PATH"
    else
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
fi

# ubuntu/debian
distro="`lsb_release -is`"
if [ "$?" = "0" ]; then
    if [ "$distro" = "Ubuntu" ]; then
        version="`lsb_release -rs`"
        if [ "$?" = "0" ]; then
            if [ "$version" = "16.04" -o "$version" = "16.10" ]; then
                sudo apt update -y && sudo apt install -y unzip pkg-config libx11-dev libxmu-dev python3
                if [ "$?" != "0" ]; then
                    echo 'aborting: could not install rooster dependencies' 1>&2
                    exit 1
                fi
            elif [ "$version" = "14.04" ]; then
                sudo apt-get update -y && sudo apt-get install -y unzip pkg-config libx11-dev libxmu-dev python3
                if [ "$?" != "0" ]; then
                    echo 'aborting: could not install rooster dependencies' 1>&2
                    exit 1
                fi
            fi
        fi
    elif [ "$distro" = "Debian" ]; then
        sudo apt-get install -y gcc unzip pkg-config libx11-dev libxmu-dev python3
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
    sudo dnf install -y gcc unzip pkgconfig libX11-devel libXmu-devel python3
    if [ "$?" != "0" ]; then
        echo 'aborting: could not install rooster dependencies' 1>&2
        exit 1
    fi
elif [ "$yumstatus" = "0" ]; then
    sudo yum install -y gcc unzip pkgconfig libX11-devel libXmu-devel python3
    if [ "$?" != "0" ]; then
        echo 'aborting: could not install rooster dependencies' 1>&2
        exit 1
    fi
fi


rm -rf /tmp/$pkgname-$pkgver /tmp/$pkgname-$pkgver.tar.gz

curl -sSL https://github.com/conradkdotcom/$pkgname/archive/v$pkgver.tar.gz -o /tmp/$pkgname-$pkgver.tar.gz
if [ "$?" != "0" ]; then
    echo 'aborting: could not download rooster' 1>&2
    exit 1
fi

# check that we downloaded the correct file (sha256sum on Linux, shasum on OSX)
if [ "$os" = "Darwin" ];then
    actual_sha256="`shasum -a 256 /tmp/$pkgname-$pkgver.tar.gz | cut -d' ' -f1`"
else
    actual_sha256="`sha256sum /tmp/$pkgname-$pkgver.tar.gz | cut -d' ' -f1`"
fi
if [ "$actual_sha256" != "$sha256" ]; then
    echo 'aborting: could not verify file signature' 1>&2
    exit 1
fi

tar -C /tmp -zxvf /tmp/$pkgname-$pkgver.tar.gz
if [ "$?" != "0" ]; then
    echo 'aborting: could not untar rooster' 1>&2
    exit 1
fi

cd /tmp/$pkgname-$pkgver
cargo build --release
buildstatus="$?"
cd -
if [ "$buildstatus" != "0" ]; then
    echo 'aborting: could not build rooster' 1>&2
    exit 1
fi

# copy binaries to /usr/bin on Linux and /usr/local/bin on OSX
if [ "$os" = "Darwin" ];then
    sudo cp /tmp/$pkgname-$pkgver/target/release/rooster /usr/local/bin/rooster
else
    sudo cp /tmp/$pkgname-$pkgver/target/release/rooster /usr/bin/rooster
fi
if [ "$?" != "0" ]; then
    echo 'aborting: could not copy rooster' 1>&2
    exit 1
fi
