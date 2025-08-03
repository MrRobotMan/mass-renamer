use std::{fs, panic};
pub(crate) fn run_test<T>(files: &Vec<&str>, test: T)
where
    T: FnOnce() + panic::UnwindSafe,
{
    for file in files {
        if fs::File::create(file).is_err() {
            return;
        };
    }
    let result = panic::catch_unwind(test);
    for file in files {
        if fs::remove_file(file).is_err() {
            println!("Could not delete {file:?}");
        };
    }
    assert!(result.is_ok())
}
