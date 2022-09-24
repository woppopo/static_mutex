use core::intrinsics::const_allocate;

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

    pub const fn eq(a: &Self, b: &Self) -> bool {
        eq_str(a.file, b.file) && a.line == b.line && a.col == b.col
    }
}

const fn eq_str(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a = unsafe {
        let ptr = const_allocate(
            core::mem::size_of::<u8>() * a.len(),
            core::mem::align_of::<u8>(),
        );
        core::ptr::copy(a.as_ptr(), ptr, a.len());
        core::slice::from_raw_parts(ptr, a.len())
    };

    let b = unsafe {
        let ptr = const_allocate(
            core::mem::size_of::<u8>() * b.len(),
            core::mem::align_of::<u8>(),
        );
        core::ptr::copy(b.as_ptr(), ptr, b.len());
        core::slice::from_raw_parts(ptr, b.len())
    };

    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }

    true
}
