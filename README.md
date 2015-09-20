[![Rooster Banner](http://conradk.com/rooster/images/rooster-banner.png)](http://conradk.com/rooster/)

## Basic commands

```shell
# Generate a strong random password
rooster generate youtube john@doe.com

# Retrieve a password and copy it to the clipboard (needs xsel installed)
rooster get youtube | xsel -ib

# Delete a password
rooster delete youtube

# Need more ? RTFM :-)
rooster --help
```

## Installation

```shell
curl -sSf https://raw.githubusercontent.com/conradkleinespel/rooster/master/install | bash
```

## Contributors

- [@conradkleinespel](https://github.com/conradkleinespel)
- [@jaezun](https://github.com/jaezun)
