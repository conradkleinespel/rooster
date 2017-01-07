![Rooster Banner](rooster-banner.png)

## Installation

Rooster currently works on OSX, Linux, BSD and [other platforms](https://forge.rust-lang.org/platform-support.html).

To install, open a terminal and run this command:

```shell
curl -sSL 'https://raw.githubusercontent.com/conradkleinespel/rooster/master/install.sh' | sh
```

The script above has been tested on Ubuntu (14.04, 16.04, 16.10), Debian 8, CentOS 7, Fedora 25 and FreeBSD 11. If you use another operating system, you might need to install `unzip`, `pkg-config`, `libx11-dev` and `libxmu-dev` packages first.

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
