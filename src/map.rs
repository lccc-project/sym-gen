use std::{
    borrow::Borrow,
    hash::{BuildHasher, RandomState},
    num::NonZeroU32,
    sync::atomic::{AtomicU32, Ordering},
};

use hashbrown::HashMap;
use parking_lot::{RwLock, const_rwlock};

use crate::{Symbol, pool::Pool};

/// A marker trait for types that can be internalized.
///
/// This is a sealed trait, it cannot be implemented outside of the current crate.
///
/// It is implemented for all types that implement [`Into<String>`] and [`Borrow<str>`].
/// It is also implemented for all symbol types to allow efficient sharing of symbols between pools.
pub trait Internalize: Borrow<str> {
    #[doc(hidden)]
    fn internalize(self) -> &'static str;
}

impl<S: Into<String> + Borrow<str>> Internalize for S {
    fn internalize(self) -> &'static str {
        let st: String = self.into();

        st.leak()
    }
}

impl<P: Pool> Internalize for Symbol<P> {
    fn internalize(self) -> &'static str {
        self.as_str()
    }
}

pub struct InternMap<S = RandomState> {
    counter: AtomicU32,
    map: RwLock<(
        HashMap<&'static str, NonZeroU32, S>,
        HashMap<NonZeroU32, &'static str, S>,
    )>,
}

impl InternMap {
    pub fn new(init_counter: NonZeroU32) -> Self {
        Self::new_with_hashers(init_counter, RandomState::new(), RandomState::new())
    }
}

impl<S: BuildHasher> InternMap<S> {
    #[doc(hidden)]
    pub const fn new_with_hashers(init_counter: NonZeroU32, key_to_val: S, val_to_key: S) -> Self {
        Self {
            counter: AtomicU32::new(init_counter.get()),
            map: const_rwlock((
                HashMap::with_hasher(key_to_val),
                HashMap::with_hasher(val_to_key),
            )),
        }
    }

    #[doc(hidden)]
    pub fn insert_mut(&mut self, key: NonZeroU32, val: &'static str) {
        let map = self.map.get_mut();
        map.1.insert(key, val);
        map.0.insert(val, key);
    }

    #[doc(hidden)]
    pub fn insert(&self, key: NonZeroU32, val: &'static str) {
        let mut map = self.map.write();
        map.1.insert(key, val);
        map.0.insert(val, key);
    }

    pub fn internalize<V: Internalize>(&self, st: V) -> NonZeroU32 {
        let map = self.map.read();

        if let Some(key) = map.0.get(st.borrow()) {
            *key
        } else {
            let val = st.internalize();

            let key = self
                .counter
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                    if v == 0 {
                        None
                    } else {
                        Some(v.wrapping_add(1))
                    }
                })
                .map_err(|_| "Intern Map Overflowed")
                .unwrap();

            // SAFETY:
            // The above expression panics if the counter ever wraps to 0.
            let key = unsafe { NonZeroU32::new_unchecked(key) };

            self.insert(key, val);

            key
        }
    }

    pub fn get(&self, key: NonZeroU32) -> Option<&'static str> {
        let lock = self.map.read();
        lock.1.get(&key).copied()
    }
}
