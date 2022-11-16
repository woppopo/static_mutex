#![feature(adt_const_params)]
#![feature(const_caller_location)]
#![feature(const_heap)]
#![feature(const_mut_refs)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(core_intrinsics)]
#![feature(inline_const)]

mod id;
mod location;

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::{LockResult, Mutex, MutexGuard, PoisonError},
};

pub use id::{End, Id, Locked, Unlocked};

use crate::id::IsLocked;

pub struct Log<List>(PhantomData<List>);

impl Log<End> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<List> Log<List> {
    const fn into<NextList>(self) -> Log<NextList> {
        Log(PhantomData)
    }
}

pub struct StaticMutex<const ID: Id, T>(Mutex<T>);

impl<const ID: Id, T> StaticMutex<ID, T> {
    pub const fn new(value: T) -> Self {
        Self(Mutex::new(value))
    }

    #[track_caller]
    pub fn lock<List>(
        &self,
        log: Log<List>,
    ) -> LockResult<(Log<Locked<ID, List>>, StaticMutexGuard<ID, T>)>
    where
        List: IsLocked<ID>,
    {
        #[track_caller]
        const fn assertion(is_locked: bool) {
            if is_locked {
                panic!("Double-lock occured.");
            }
        }

        const { assertion(List::RESULT) };

        match self.0.lock() {
            Ok(guard) => {
                let guard = StaticMutexGuard(guard);
                Ok((log.into(), guard))
            }
            Err(poison) => {
                let guard = StaticMutexGuard(poison.into_inner());
                Err(PoisonError::new((log.into(), guard)))
            }
        }
    }

    pub fn unlock<List>(log: Log<List>, _: StaticMutexGuard<'_, ID, T>) -> Log<Unlocked<ID, List>> {
        log.into()
    }
}

pub struct StaticMutexGuard<'a, const ID: Id, T>(MutexGuard<'a, T>);

impl<'a, const ID: Id, T> Deref for StaticMutexGuard<'a, ID, T> {
    type Target = MutexGuard<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, const ID: Id, T> DerefMut for StaticMutexGuard<'a, ID, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
