![Rooster Banner](rooster-banner.png)

## Installation

[![Build Status](https://drone.conradk.com/api/badges/conradk/rooster/status.svg)](https://drone.conradk.com/conradk/rooster)

Rooster depends on [Rust & Cargo][0] being installed on your system.

On Ubuntu Linux, you need to install some packages before installing Rooster:
```shell
sudo apt-get install pkg-config libx11-dev libxmu-dev
```

Once you have those installed, you can run the following command to install Rooster:
```shell
cargo install rooster
```

Once you have installed Rooster, you can view documentation with:
```shell
rooster --help
```

We welcome contribution from everyone. Head over to [CONTRIBUTING.md][2] to learn
more about how to contribute to the project.

## Ideas

- Import from other password managers
- Avoid retyping master password every time (see https://github.com/conradkleinespel/rooster/issues/2)
- Easy to install packages (see https://github.com/conradkleinespel/rooster/issues/6)

## Contributors

- [@conradkleinespel](https://github.com/conradkleinespel)
- [@jaezun](https://github.com/jaezun)
- [@maxjacobson](https://github.com/maxjacobson)
- [@qmx](https://github.com/qmx)
- Awesome Rustaceans from the [Rust Paris meetup](http://www.meetup.com/Rust-Paris/)

[0]: https://www.rust-lang.org/downloads.html "How to install Rust & Cargo"
[1]: https://github.com/conradkleinespel/rooster/issues/new "Open an issue"
[2]: CONTRIBUTING.md "Contribution guidelines"
