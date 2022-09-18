#![feature(adt_const_params)]
#![feature(const_caller_location)]
#![feature(const_heap)]
#![feature(const_mut_refs)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(core_intrinsics)]
#![feature(generic_arg_infer)]
#![feature(generic_const_exprs)]

mod id;
mod location;

use std::{
    ops::{Deref, DerefMut},
    sync::{LockResult, Mutex, MutexGuard, PoisonError},
};

pub use id::{IDList, IDs, ID};

pub struct StaticMutex<T, const U: ID>(Mutex<T>);

impl<T, const U: ID> StaticMutex<T, U> {
    pub const fn new(value: T) -> Self {
        Self(Mutex::new(value))
    }

    pub fn lock<const LIST: IDList>(
        &self,
        _: IDs<LIST>,
    ) -> LockResult<(IDs<{ IDs::<LIST>::push(LIST, U) }>, StaticMutexGuard<T, U>)> {
        match self.0.lock() {
            Ok(guard) => {
                let guard = StaticMutexGuard(guard);
                Ok((IDs, guard))
            }
            Err(poison) => {
                let guard = StaticMutexGuard(poison.into_inner());
                Err(PoisonError::new((IDs, guard)))
            }
        }
    }

    pub fn unlock<const LIST: IDList>(
        _: IDs<LIST>,
        _: StaticMutexGuard<'_, T, U>,
    ) -> IDs<{ IDs::<LIST>::remove(LIST, U) }> {
        IDs
    }
}

pub struct StaticMutexGuard<'a, T, const U: ID>(MutexGuard<'a, T>);

impl<'a, T, const U: ID> Deref for StaticMutexGuard<'a, T, U> {
    type Target = MutexGuard<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T, const U: ID> DerefMut for StaticMutexGuard<'a, T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
