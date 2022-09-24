use std::intrinsics::const_allocate;

use crate::location::Location;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ID(&'static Location);

impl ID {
    #[track_caller]
    pub const fn new() -> Self {
        Self(Location::caller())
    }

    const fn eq(a: &Self, b: &Self) -> bool {
        Location::eq(a.0, b.0)
    }
}

pub type IDList = &'static [ID];
type IDListMut = &'static mut [ID];

pub struct IDs<const LIST: IDList>;

impl IDs<{ &[] }> {
    pub const fn empty() -> Self {
        Self
    }
}

impl<const LIST: IDList> IDs<LIST> {
    const fn allocate_list(len: usize) -> IDListMut {
        unsafe {
            let ptr = const_allocate(
                core::mem::size_of::<ID>() * len,
                core::mem::align_of::<ID>(),
            );
            core::slice::from_raw_parts_mut(ptr.cast::<ID>(), len)
        }
    }

    #[track_caller]
    pub const fn push(ids: IDList, id: ID) -> IDList {
        let new = Self::allocate_list(ids.len() + 1);

        let mut i = 0;
        while i < ids.len() {
            if ID::eq(&ids[i], &id) {
                panic!("Double locks.");
            }

            new[i] = ids[i];
            i += 1;
        }

        new[i] = id;
        new
    }

    #[track_caller]
    pub const fn remove(ids: IDList, id: ID) -> IDList {
        let new = Self::allocate_list(ids.len() - 1);

        let mut i = 0;
        let mut found = false;
        while i < ids.len() {
            if ID::eq(&ids[i], &id) {
                found = true;
                break;
            }
            i += 1;
        }

        if !found {
            panic!("The ID doesn't exist.");
        }

        let mut i = 0;
        let mut j = 0;
        while i < ids.len() {
            if ID::eq(&ids[i], &id) {
                i += 1;
                continue;
            }

            new[j] = ids[i];
            i += 1;
            j += 1;
        }

        new
    }
}
