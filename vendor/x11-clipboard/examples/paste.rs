extern crate x11_clipboard;

use x11_clipboard::Clipboard;


fn main() {
    let clipboard = Clipboard::new().unwrap();
    let val =
        clipboard.load(
            clipboard.setter.atoms.clipboard,
            clipboard.setter.atoms.utf8_string,
            clipboard.setter.atoms.property,
            None
        )
        .unwrap();
    let val = String::from_utf8(val).unwrap();

    print!("{}", val);
}
