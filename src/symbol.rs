use std::fmt;
use std::sync::Mutex;

use rustc_hash::FxHashMap;

use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(usize);

impl Symbol {
    const fn new(n: u32) -> Self {
        Symbol(n as usize)
    }

    pub fn intern(string: &str) -> Self {
        with_interner(|interner| interner.intern(string))
    }

    pub fn from_char(c: char) -> Option<Self> {
        with_interner(|interner| interner.get(c.to_string().as_str()))
    }

    pub fn as_str(&self) -> &'static str {
        with_interner(|interner| unsafe {
            std::mem::transmute::<&str, &str>(interner.get_str(*self))
        })
    }

    pub fn as_u32(&self) -> u32 {
        self.0 as u32
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_str(), f)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), f)
    }
}

#[derive(Default)]
pub struct Interner {
    names: FxHashMap<&'static str, Symbol>,
    strings: Vec<&'static str>,
}

impl Interner {
    pub fn exist(&mut self, string: &str) -> bool {
        self.names.get(string).is_some()
    }

    pub fn get(&mut self, string: &str) -> Option<Symbol> {
        match self.names.get(string) {
            Some(&name) => Some(name),
            None => None,
        }
    }

    #[inline]
    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&name) = self.names.get(string) {
            return name;
        }

        let name = Symbol::new(self.strings.len() as u32);

        let string: &'static str = unsafe { &*(string as *const str) };
        self.strings.push(string);
        self.names.insert(string, name);
        name
    }

    pub fn get_str(&self, symbol: Symbol) -> &str {
        self.strings[symbol.0]
    }
}

pub struct SessionGlobals {
    symbol_interner: std::cell::RefCell<Interner>,
}

impl Default for SessionGlobals {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionGlobals {
    pub fn new() -> SessionGlobals {
        SessionGlobals {
            symbol_interner: std::cell::RefCell::new(Interner::default()),
        }
    }
}

lazy_static! {
    static ref SESSION_GLOBALS: Mutex<SessionGlobals> = Mutex::new(SessionGlobals::new());
}

fn with_interner<T, F: FnOnce(&mut Interner) -> T>(f: F) -> T {
    f(&mut *SESSION_GLOBALS.lock().unwrap().symbol_interner.borrow_mut())
}
