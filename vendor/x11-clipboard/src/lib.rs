#[macro_use] extern crate error_chain;
pub extern crate xcb;

#[macro_use] pub mod error;
mod run;

use std::thread;
use std::time::{ Duration, Instant };
use std::sync::{ Arc, RwLock };
use std::sync::mpsc::{ Sender, channel };
use std::collections::HashMap;
use xcb::{ Connection, Window, Atom };


pub const INCR_CHUNK_SIZE: usize = 4000;
const POLL_DURATION: u64 = 50;
type SetMap = Arc<RwLock<HashMap<Atom, (Atom, Vec<u8>)>>>;

#[derive(Clone, Debug)]
pub struct Atoms {
    pub primary: Atom,
    pub clipboard: Atom,
    pub property: Atom,
    pub targets: Atom,
    pub string: Atom,
    pub utf8_string: Atom,
    pub incr: Atom
}

/// X11 Clipboard
pub struct Clipboard {
    pub getter: Context,
    pub setter: Arc<Context>,
    setmap: SetMap,
    send: Sender<Atom>
}

pub struct Context {
    pub connection: Connection,
    pub window: Window,
    pub atoms: Atoms
}

#[inline]
fn get_atom(connection: &Connection, name: &str) -> error::Result<Atom> {
    xcb::intern_atom(connection, false, name)
        .get_reply()
        .map(|reply| reply.atom())
        .map_err(Into::into)
}

impl Context {
    pub fn new(displayname: Option<&str>) -> error::Result<Self> {
        let (connection, screen) = Connection::connect(displayname)?;
        let window = connection.generate_id();

        {
            let screen = connection.get_setup().roots().nth(screen as usize)
                .ok_or_else(|| err!(XcbConn, ::xcb::ConnError::ClosedInvalidScreen))?;
            xcb::create_window(
                &connection,
                xcb::COPY_FROM_PARENT as u8,
                window, screen.root(),
                0, 0, 1, 1,
                0,
                xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                screen.root_visual(),
                &[(
                    xcb::CW_EVENT_MASK,
                    xcb::EVENT_MASK_STRUCTURE_NOTIFY | xcb::EVENT_MASK_PROPERTY_CHANGE
                )]
            );
            connection.flush();
        }

        macro_rules! intern_atom {
            ( $name:expr ) => {
                get_atom(&connection, $name)?
            }
        }

        let atoms = Atoms {
            primary: xcb::ATOM_PRIMARY,
            clipboard: intern_atom!("CLIPBOARD"),
            property: intern_atom!("THIS_CLIPBOARD_OUT"),
            targets: intern_atom!("TARGETS"),
            string: xcb::ATOM_STRING,
            utf8_string: intern_atom!("UTF8_STRING"),
            incr: intern_atom!("INCR")
        };

        Ok(Context {
            connection: connection,
            window: window,
            atoms: atoms
        })
    }

    pub fn get_atom(&self, name: &str) -> error::Result<Atom> {
        get_atom(&self.connection, name)
    }
}


impl Clipboard {
    /// Create Clipboard.
    pub fn new() -> error::Result<Self> {
        let getter = Context::new(None)?;
        let setter = Arc::new(Context::new(None)?);
        let setter2 = Arc::clone(&setter);
        let setmap = Arc::new(RwLock::new(HashMap::new()));
        let setmap2 = Arc::clone(&setmap);

        let (sender, receiver) = channel();
        let max_length = setter.connection.get_maximum_request_length() as usize * 4;
        thread::spawn(move || run::run(setter2, setmap2, max_length, &receiver));

        Ok(Clipboard {
            getter: getter,
            setter: setter,
            setmap: setmap,
            send: sender
        })
    }

    /// load value.
    pub fn load<T>(&self, selection: Atom, target: Atom, property: Atom, timeout: T)
        -> error::Result<Vec<u8>>
        where T: Into<Option<Duration>>
    {
        let mut buff = Vec::new();
        let mut is_incr = false;
        let timeout = timeout.into();
        let start_time =
            if timeout.is_some() { Some(Instant::now()) }
            else { None };

        xcb::convert_selection(
            &self.getter.connection, self.getter.window,
            selection, target, property,
            xcb::CURRENT_TIME
                // FIXME ^
                // Clients should not use CurrentTime for the time argument of a ConvertSelection request.
                // Instead, they should use the timestamp of the event that caused the request to be made.
        );
        self.getter.connection.flush();

        loop {
            if timeout.into_iter()
                .zip(start_time)
                .next()
                .map(|(timeout, time)| (Instant::now() - time) >= timeout)
                .unwrap_or(false)
            {
                return Err(err!(Timeout));
            }

            let event = match self.getter.connection.poll_for_event() {
                Some(event) => event,
                None => {
                    thread::park_timeout(Duration::from_millis(POLL_DURATION));
                    continue
                }
            };

            match event.response_type() & !0x80 {
                xcb::SELECTION_NOTIFY => {
                    let event = unsafe { xcb::cast_event::<xcb::SelectionNotifyEvent>(&event) };
                    if event.selection() != selection || event.property() != property { continue };

                    let reply =
                        xcb::get_property(
                            &self.getter.connection, false, self.getter.window,
                            event.property(), xcb::ATOM_ANY, buff.len() as u32, ::std::u32::MAX // FIXME reasonable buffer size
                        )
                        .get_reply()?;

                    if reply.type_() == self.getter.atoms.incr {
                        if let Some(&size) = reply.value::<i32>().get(0) {
                            buff.reserve(size as usize);
                        }
                        xcb::delete_property(&self.getter.connection, self.getter.window, property);
                        self.getter.connection.flush();
                        is_incr = true;
                        continue
                    } else if reply.type_() != target {
                        continue
                    }

                    buff.extend_from_slice(reply.value());
                    break
                },
                xcb::PROPERTY_NOTIFY if is_incr => {
                    let event = unsafe { xcb::cast_event::<xcb::PropertyNotifyEvent>(&event) };
                    if event.state() != xcb::PROPERTY_NEW_VALUE as u8 { continue };

                    let length =
                        xcb::get_property(
                            &self.getter.connection, false, self.getter.window,
                            property, xcb::ATOM_ANY, 0, 0
                        )
                        .get_reply()
                        .map(|reply| reply.bytes_after())?;

                    let reply =
                        xcb::get_property(
                            &self.getter.connection, true, self.getter.window,
                            property, xcb::ATOM_ANY, 0, length
                        )
                        .get_reply()?;

                    if reply.type_() != target { continue };

                    if reply.value_len() != 0 {
                        buff.extend_from_slice(reply.value());
                    } else {
                        break
                    }
                },
                _ => ()
            }
        }

        xcb::delete_property(&self.getter.connection, self.getter.window, property);
        self.getter.connection.flush();
        Ok(buff)
    }

    /// store value.
    pub fn store<T: Into<Vec<u8>>>(&self, selection: Atom, target: Atom, value: T) -> error::Result<()> {
        self.send.send(selection)?;
        self.setmap
            .write()
            .map_err(|_| err!(Lock))?
            .insert(selection, (target, value.into()));

        xcb::set_selection_owner(
            &self.setter.connection,
            self.setter.window, selection,
            xcb::CURRENT_TIME
        );
        self.setter.connection.flush();

        if xcb::get_selection_owner(&self.setter.connection, selection)
            .get_reply()
            .map(|reply| reply.owner() == self.setter.window)
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err(err!(SetOwner))
        }
    }
}


#[test]
fn it_work() {
    let data = format!("{:?}", Instant::now());
    let clipboard = Clipboard::new().unwrap();

    let atom_clipboard = clipboard.setter.atoms.clipboard;
    let atom_utf8string = clipboard.setter.atoms.utf8_string;
    let atom_property = clipboard.setter.atoms.property;

    clipboard.store(atom_clipboard, atom_utf8string, data.as_bytes()).unwrap();

    let output = clipboard.load(atom_clipboard, atom_utf8string, atom_property, None).unwrap();
    assert_eq!(output, data.as_bytes());

    let data = format!("{:?}", Instant::now());
    clipboard.store(atom_clipboard, atom_utf8string, data.as_bytes()).unwrap();

    let output = clipboard.load(atom_clipboard, atom_utf8string, atom_property, None).unwrap();
    assert_eq!(output, data.as_bytes());

    let output = clipboard.load(atom_clipboard, atom_utf8string, atom_property, None).unwrap();
    assert_eq!(output, data.as_bytes());
}
