use std::{fs, panic};
#[allow(unused_must_use)]
pub(crate) fn run_test<T>(files: &Vec<&str>, test: T)
where
    T: FnOnce() + panic::UnwindSafe,
{
    for file in files {
        fs::File::create(file);
    }
    let result = panic::catch_unwind(test);
    for file in files {
        fs::remove_file(file);
    }
    assert!(result.is_ok())
}
