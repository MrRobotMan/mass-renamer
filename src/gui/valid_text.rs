/*
Borrowed from
https://github.com/BfBB-Clash/BfBB-Clash/blob/ab54cf46c4beb23588b403cf20a0fc06d7ae4d7d/crates/clash/src/gui/val_text.rs
Used in
https://github.com/BfBB-Clash/BfBB-Clash/blob/ab54cf46c4beb23588b403cf20a0fc06d7ae4d7d/crates/clash/src/gui/option_editor.rs
and
https://github.com/BfBB-Clash/BfBB-Clash/blob/ab54cf46c4beb23588b403cf20a0fc06d7ae4d7d/crates/clash/src/gui/lobby/mod.rs
*/

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use egui::TextBuffer;

/// A mutable TextBuffer that will validate it's contents when changed.
///
/// The default validator will simply attempt to parse the text as `T`,
/// but a custom validator function can be provided.
pub struct ValText<T> {
    text: String,
    val: Option<T>,
    prev: Option<T>,
    #[allow(clippy::type_complexity)]
    validator: Box<dyn Fn(&str) -> Option<T>>,
}

impl<T: Debug> Debug for ValText<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValText")
            .field("text", &self.text)
            .field("val", &self.val)
            .finish()
    }
}

impl<T: Copy + Display> ValText<T> {
    pub fn _with_validator(validator: impl Fn(&str) -> Option<T> + 'static) -> Self {
        Self {
            text: Default::default(),
            val: Default::default(),
            prev: Default::default(),
            validator: Box::new(validator),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn get_val(&self) -> Option<T> {
        self.val
    }

    pub fn is_valid(&self) -> bool {
        self.val.is_some()
    }

    pub fn revert(&mut self) {
        if let Some(v) = self.prev {
            self.set_val(v)
        } else {
            self.text = Default::default();
            self.val = Default::default();
        }
    }

    pub fn get_prev(&self) -> Option<T> {
        self.prev
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl<T: Display> ValText<T> {
    pub fn set_val(&mut self, val: T) {
        self.text = val.to_string();
        self.val = Some(val);
    }

    pub fn clear(&mut self) {
        self.text = String::new();
        self.val = None;
        self.prev = None;
    }
}

impl<T: FromStr + Default> Default for ValText<T> {
    fn default() -> Self {
        Self {
            text: Default::default(),
            val: Default::default(),
            prev: Default::default(),
            validator: Box::new(|text| text.parse().ok()),
        }
    }
}

impl<T: Copy> TextBuffer for ValText<T> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.text.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        if self.val.is_some() {
            self.prev = self.val
        }
        let n = self.text.insert_text(text, char_index);
        self.val = (self.validator)(&self.text);
        n
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        if self.val.is_some() {
            self.prev = self.val
        }
        self.text.delete_char_range(char_range);
        self.val = (self.validator)(&self.text);
    }
}
