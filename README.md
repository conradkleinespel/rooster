# Peevee, the password manager

A simple and secure, command-line based, offline password manager.

Here are a few examples of how to use Peevee:

```
# Tries to find the entry for the app you're most likely looking for.
# The app name is case insensitive.
p get <app-name>

# Adds an entry with this username. The password is retrieved via STDIN.
p add <app-name> <username>

# Removes an entry.
p delete <app-name>

# Adds a new entry with an automatically generated password.
# The app name is case insensitive.
p generate <app-name> <username>

# List all passwords.
p list
```

Please open an issue or a pull request for bug reports, ideas, etc.

## FAQ

### How securely does Peevee store my passwords ?

Peevee stores all of your passwords in an encrypted file. The encryption algorithm used is
the [Advanced Encryption Standard](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard).
This algorithm is used by government agencies all around the world to protect classified
information. Your passwords are never saved to disk unencrypted.

### Can Peevee sync my passwords in the cloud ?

Not at the moment. If you would like that, please notify us by [opening an issue](https://github.com/conradkleinespel/peevee-cli/issues/new) so we are aware.
We may allow online syncing if there is enough demand for it.

### Can I use Peevee on Windows / OSX / Linux ?

I use exclusively UNIX systems. At the moment, I'm testing Peevee on Linux and
OSX 10.9. However, I understand that you may be interested in using Peevee on
Windows. Please [open an issue](https://github.com/conradkleinespel/peevee-cli/issues/new)
if you would like Windows support, so we know there is demand for it.

If you are a developer, you can submit a pull request to fix Windows bugs. I am thankful
for any contributions and will review pull requests in a timely manner.

### How can I install Peevee ?

**You can install Peevee in 3 simple steps. Let's get started! Don't worry, we'll guide you through.**

Peevee is built using [the Rust programming language](http://www.rust-lang.org/)
which we'll need to install so we can compile the source code:

```sh
curl static.rust-lang.org/rustup.sh | sudo sh
```

When you run this command, you will need to enter your administrator password.
This is because Rust needs access to special folders to install correctly.

When this is done, you can check that Rust was installed correctly:

```sh
rustc -V
cargo version
```

You should see two lines similar to (but not necessarily the same):

```
rustc 1.0.0-nightly (a954663db 2015-02-10 22:08:30 +0000)
cargo 0.0.1-pre-nightly (9404539 2015-02-09 20:54:26 +0000)
```

**Awesome ! We're almost there. Just two more steps :-)**

We'll now download the source code for Peevee:
```sh
git clone https://github.com/conradkleinespel/peevee-cli.git
```

**Alright, good! One last step.**

Now, we'll compile Peevee so we can use it in our terminal:
```sh
cd peevee-cli
cargo build
```

You may see some yellow or green text appearing with all kinds of weird
technical messages. As long as there is no red text, you're good to go.

**That's it! You did it!**

Peevee is ready to go. Here's how you can use it:
```
./target/peevee-cli list
```

#### Pro tip

You'll probably want to use Peevee with a simple command like `peevee list`
instead of `~/peevee-cli/target/peevee-cli list`. To do that, you'll need to
make what's called a symbolic link. Here's how that might work:

```sh
sudo ln -s $(pwd)/target/peevee-cli /usr/local/bin/peevee
```

Now you can use Peevee like this, from any directory:

```sh
peevee list
```
