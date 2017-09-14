![Rooster Banner](rooster-banner.png)

## Why another password manager

There are a lot of password managers out there. Rooster has some unique goals:

- it is easy to maintain so that it never becomes unmaintained
- it works completely offline with optional support for online sync
- it stores simple username/password tuples, nothing more, nothing less

Rooster protects your passwords with state-of-the-art cryptography algorithms:

- scrypt for key derivation
- aes256-cbc for encryption
- hmac-sha256 for authentication

To top it off, it works Linux, BSD and OSX.

## Installation

On **Arch Linux**, you install [Rooster from AUR](https://aur.archlinux.org/packages/rooster).

On **Fedora/CentOS/Ubuntu/OSX**:

```shell
curl -sSL 'https://raw.githubusercontent.com/conradkdotcom/rooster/master/install.sh' | sh
```

For **BSD and other Linux distributions**:

- make sure you have `pkg-config`, `python3`, `libxmu-dev`, `libx11-dev` and one of `xsel`/`xclip`
- install Rust and Cargo with:
    ```bash
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    ```
- install Rooster with:
    ```bash
    cargo install --root /usr rooster
    ```

Once you have installed Rooster (see instructions below), you can view documentation with:

```shell
rooster --help
```

## Automated tests

Rooster has 2 sets of tests:

- code level tests which you can run with `cargo test`
- integration tests which you can run with `./integration-tests.sh`

## Contributors

We welcome contribution from everyone. Feel free to open an issue or a pull request at any time.

Check out the [unassigned issues](https://github.com/conradkdotcom/rooster/issues?q=is%3Aissue+is%3Aopen+label%3Aunassigned) to get started. If you have any questions, just let us know and we'll jump in to help.

Here's a list of existing Rooster contributors:

- [@conradkleinespel](https://github.com/conradkleinespel)
- [@Eternity-Yarr](https://github.com/Eternity-Yarr)
- [@jaezun](https://github.com/jaezun)
- [@maxjacobson](https://github.com/maxjacobson)
- [@qmx](https://github.com/qmx)
- [@yamnikov-oleg](https://github.com/yamnikov-oleg)
- Awesome Rustaceans from the [Rust Paris meetup](http://www.meetup.com/Rust-Paris/)

Thank you very much for your help!  :smiley:  :heart:
