use std::hash::BuildHasher;

use crate::map::InternMap;

pub trait Pool: Sized {
    type Hasher: BuildHasher + 'static;
    fn get_map() -> &'static InternMap<Self::Hasher>;
}
