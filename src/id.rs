use core::marker::PhantomData;

use crate::location::Location;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Id(&'static Location);

impl Id {
    #[track_caller]
    pub const fn new() -> Self {
        Self(Location::caller())
    }

    const fn eq(a: Self, b: Self) -> bool {
        Location::eq(a.0, b.0)
    }
}

pub struct End;

pub struct Locked<const ID: Id, Prev>(PhantomData<Prev>);

pub struct Unlocked<const ID: Id, Prev>(PhantomData<Prev>);

pub trait IsLocked<const ID: Id> {
    const RESULT: bool;
}

impl<const ID: Id> IsLocked<ID> for End {
    const RESULT: bool = false;
}

impl<const ID: Id, const HOLD: Id, Prev> IsLocked<ID> for Locked<HOLD, Prev>
where
    Prev: IsLocked<ID>,
{
    const RESULT: bool = if Id::eq(ID, HOLD) { true } else { Prev::RESULT };
}

impl<const ID: Id, const HOLD: Id, Prev> IsLocked<ID> for Unlocked<HOLD, Prev>
where
    Prev: IsLocked<ID>,
{
    const RESULT: bool = if Id::eq(ID, HOLD) {
        false
    } else {
        Prev::RESULT
    };
}
