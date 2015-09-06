# Peevee, the password manager

A simple and secure password manager for the command line.

Is it currently available on Linux and OSX. There is no official Windows support at the moment.
See the FAQ below for more information.

Here are a few examples of how to use Peevee:

```
# Tries to find the entry for the app you're most likely looking for.
# The app name is case insensitive.
peevee get <app-name>

# Adds an entry with this username. The password is retrieved via STDIN.
peevee add <app-name> <username>

# Removes an entry.
peevee delete <app-name>

# Adds a new entry with an automatically generated password.
# The app name is case insensitive.
peevee generate <app-name> <username>

# List all passwords (only app name and username).
peevee list
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
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

When you run this command, you will need to enter your administrator password.
This is because Rust needs access to special folders to install correctly.

When this is done, you can check that Rust was installed correctly:

```sh
rustc -V
cargo -V
```

You should see two lines similar to (but not necessarily the same):

```
rustc 1.1.0 (35ceea399 2015-06-19)
cargo 0.2.0-nightly (a483581 2015-05-14) (built 2015-05-15)
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

You may see some yellow, blue or green text appearing with all kinds of weird
technical messages. As long as there is no red text, you're good to go.

**That's it! You did it!**

Peevee is ready to go. Here's how you can use it:
```
./target/debug/peevee-cli list
```

#### Pro tip

You'll probably want to use Peevee with a simple command like `peevee list`
instead of `~/peevee-cli/target/peevee-cli list`. To do that, you'll need to
make what's called a symbolic link. Here's how that might work:

```sh
sudo ln -s $(pwd)/target/debug/peevee-cli /usr/local/bin/peevee
```

Now you can use Peevee like this, from any directory:

```sh
peevee list
```
