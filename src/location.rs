// TODO: https://github.com/rust-lang/rust/pull/101030
#[derive(PartialEq, Eq)]
pub struct Location {
    pub file: &'static str,
    pub line: u32,
    pub col: u32,
}

impl Location {
    #[track_caller]
    pub const fn caller() -> &'static Self {
        let caller = std::panic::Location::caller();
        unsafe { &*<*const _>::cast::<Location>(caller) }
    }
}
