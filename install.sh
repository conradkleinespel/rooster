#!/bin/sh

pkgname=rooster
pkgver=2.4.1
sha256=69a6893c9a98dab650e6234d559922b09a84e7aa03aa25da3a8220ff31812509

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

rm -rf /tmp/$pkgname-$pkgver /tmp/$pkgname-$pkgver.tar.gz

curl -sSL https://crates.io/api/v1/crates/$pkgname/$pkgver/download -o /tmp/$pkgname-$pkgver.tar.gz
if [ "$?" != "0" ]; then
    echo 'aborting: could not download rooster' 1>&2
    exit 1
fi

actual_sha256="`sha256sum /tmp/$pkgname-$pkgver.tar.gz | cut -d' ' -f1`"
if [ "$actual_sha256" != "$sha256" ]; then
    echo 'aborting: could not verify file signature' 1>&2
    exit 1
fi

tar -C /tmp -zxvf /tmp/$pkgname-$pkgver.tar.gz
if [ "$?" != "0" ]; then
    echo 'aborting: could not unzip rooster' 1>&2
    exit 1
fi

cd /tmp/$pkgname-$pkgver
cargo build --release && cargo build
buildstatus="$?"
cd -
if [ "$buildstatus" != "0" ]; then
    echo 'aborting: could not build rooster' 1>&2
    exit 1
fi

# there is currently a bug in the clipboard library that prevents it from working
# when built in "release" mode, so we need to build it in "debug" mode
sudo cp /tmp/$pkgname-$pkgver/target/debug/rooster-clipboard /usr/bin/rooster-clipboard
if [ "$?" != "0" ]; then
    echo 'aborting: could not copy rooster-clipboard' 1>&2
    exit 1
fi

# but, we build rooster in "release" mode for better performance
sudo cp /tmp/$pkgname-$pkgver/target/release/rooster /usr/bin/rooster
if [ "$?" != "0" ]; then
    echo 'aborting: could not copy rooster' 1>&2
    exit 1
fi
