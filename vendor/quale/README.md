# rust-quale

A Rust port of the `which` utility. Locates an executable in the user’s path.

‘Quale’ is an Italian translation of the English word ‘which’.

*Note: rust-quale currently only works on Unix-like operating systems.*

## Usage

```toml
[dependencies]
quale = "1.0"
```

```rust
extern crate quale;

use quale::which;

fn main() {
    assert_eq!(
        which("sh"),
        Some("/bin/sh".into()));

    assert_eq!(
        which("foobar"),
        None);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
