use std::{
    borrow::{Borrow, Cow},
    marker::PhantomData,
    num::NonZeroU32,
    ops::Deref,
};

use crate::{map::Internalize, pool::Pool};

pub struct Symbol<P>(NonZeroU32, PhantomData<P>);

impl<P> Copy for Symbol<P> {}
impl<P> Clone for Symbol<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> PartialEq for Symbol<P> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P> Eq for Symbol<P> {}

impl<P> Symbol<P> {
    #[doc(hidden)]
    pub const fn new_unchecked(val: NonZeroU32) -> Self {
        Self(val, PhantomData)
    }

    pub fn intern<S: Internalize>(val: S) -> Self
    where
        P: Pool,
    {
        let map = P::get_map();
        Self::new_unchecked(map.internalize(val))
    }

    pub fn as_str(&self) -> &'static str
    where
        P: Pool,
    {
        let map = P::get_map();
        map.get(self.0)
            .expect("Invalid `Symbol` for this Pool (should not happen)")
    }
}

impl<P: Pool> Deref for Symbol<P> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<P: Pool> Borrow<str> for Symbol<P> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<P: Pool> AsRef<str> for Symbol<P> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<P: Pool> core::fmt::Debug for Symbol<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Symbol").field(&self.as_str()).finish()
    }
}

impl<P: Pool> core::fmt::Display for Symbol<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<P: Pool> core::hash::Hash for Symbol<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl<P: Pool> Ord for Symbol<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<P: Pool> PartialOrd for Symbol<P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Pool> PartialEq<str> for Symbol<P> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<P: Pool> PartialEq<Symbol<P>> for str {
    fn eq(&self, other: &Symbol<P>) -> bool {
        self == other.as_str()
    }
}

impl<P: Pool> From<&'_ str> for Symbol<P> {
    fn from(x: &str) -> Self {
        Self::intern(x)
    }
}

impl<P: Pool> From<String> for Symbol<P> {
    fn from(x: String) -> Self {
        Self::intern(x)
    }
}

impl<P: Pool> From<Cow<'_, str>> for Symbol<P> {
    fn from(value: Cow<'_, str>) -> Self {
        Self::intern(value)
    }
}
