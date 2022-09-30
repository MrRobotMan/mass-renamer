use crate::file::{Process, RenameFile};

pub struct NumberingOptions {}

impl Process for NumberingOptions {
    fn process(&self, file: &mut RenameFile) {}
}

#[cfg(test)]
mod numbering_test {}
