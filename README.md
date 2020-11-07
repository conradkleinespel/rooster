# Rooster

Rooster is a simple password manager for geeks (it works in the terminal).

[![Build Status](https://travis-ci.org/conradkleinespel/rooster.svg?branch=master)](https://travis-ci.org/conradkleinespel/rooster)
[![Chat on Discord](https://img.shields.io/badge/chat-on%20discord-7289DA)](https://discord.gg/9zXtYqQ)

[![asciicast](https://asciinema.org/a/9opp6uXiI2XFURj8yNHYV3xfb.svg)](https://asciinema.org/a/9opp6uXiI2XFURj8yNHYV3xfb)

Rooster is made available free of charge. You can support its development through [Liberapay](https://liberapay.com/conradkleinespel/) 💪

## Features

Rooster has some unique goals:

- it is easy to maintain so that it **never becomes unmaintained**
- it **works completely offline** by saving your password in a single local file
- it stores **username/password combinations, nothing more, nothing less**

Rooster protects your passwords with state-of-the-art cryptography algorithms:

- scrypt for key derivation (`n = 2^12, r = 8, p = 1` by default, customizable)
- aes-256-cbc for encryption
- hmac-sha512 for authentication

Supported operating systems include Linux, BSD and OSX. Windows is not supported at this time.

## Installation

To install Rooster, run the following commands as `root`.

On **Arch Linux**, install [Rooster from AUR](https://aur.archlinux.org/packages/rooster).

On **Void Linux**, install [Rooster from XBPS](https://github.com/void-linux/void-packages/blob/master/srcpkgs/rooster/template).

On **Fedora**:

```shell
dnf update -y
dnf install -y curl gcc unzip pkgconfig libX11-devel libXmu-devel python3 openssl-devel libsodium-devel
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
cargo install --root /usr rooster
```

On **CentOS**: instructions should be similar to Fedora, but it seems like `libsodium` is not available on CentOS and I
haven't been able to figure out how to install it. If you know, please let me know.

On **Debian**:

```shell
apt-get update -y
apt-get install -y curl gcc unzip pkg-config libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libx11-dev libxmu-dev python3 libssl-dev libsodium-dev xsel
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
cargo install --root /usr rooster
```

On **Ubuntu 16.04/18.04**:

```shell
apt update -y
apt install -y curl unzip pkg-config libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libx11-dev libxmu-dev python3 libssl-dev libsodium-dev xsel
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
cargo install --root /usr rooster
```

On **OSX**:

```shell
brew install curl libsodium openssl
curl https://sh.rustup.rs -sSf | sh -s -- -y
cargo install --root /usr rooster
```

For other distributions, the various Docker files can help you find which dependencies you need.

Once you have installed Rooster (see instructions below), you can view documentation with:

```shell
rooster --help
```

## Trustless security

For added trustless security, you can restrict the operating system capabilities that Rooster has access to.

For instance, to run Rooster without network access on Linux, you might do this:

```shell
# make unshare usable without being root
sudo chmod u+s "`which unshare`"

# run rooster without network
unshare -n rooster
```

Other operating systems have similar protections.

## Automated tests

Rooster has 3 sets of tests:

- code level tests which you can run with `cargo test`
- integration tests which you can run with `./tests-integration.sh`
- build tests for various Linux distributions which you can run with `./tests-build.sh`

You'll need to install [Docker](https://www.docker.com/) to run build and integration tests.

## Contributors

We welcome contribution from everyone. Feel free to open an issue or a pull request at any time.

Here's a list of existing Rooster contributors:

- [@conradkleinespel](https://github.com/conradkleinespel)
- [@cr6git](https://github.com/cr6git)
- [@dwrensha](https://github.com/dwrensha)
- [@Eternity-Yarr](https://github.com/Eternity-Yarr)
- [@jaezun](https://github.com/jaezun)
- [@maxjacobson](https://github.com/maxjacobson)
- [@qmx](https://github.com/qmx)
- [@yamnikov-oleg](https://github.com/yamnikov-oleg)
- Awesome Rustaceans from the [Rust Paris meetup](http://www.meetup.com/Rust-Paris/)

Thank you very much for your help!  :smiley:  :heart:

## License

The source code is released under the Apache 2.0 license.
