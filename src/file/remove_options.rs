/// Options for removing parts of the filename.
/// Remove specific parts of a filename but not file extensions.
///
/// - `First n` - Remove the first n characters from the name.
/// - `Last n` - Remove the last n characters from the name.
/// - `From`/`to` - Remove a string of text, e.g. from the 6th to the 9th characters (0 indexed).
/// - `Chars` - Remove occurrences of the listed characters from the name (no separator needed).
/// - `Words` - Remove occurrences of listed words (separated by spaces).
/// - `Crop` - Remove any text which occurs before (or after) a specific character or word. See note below.
/// - `Digits` - Remove all occurrences of the digits 0-9 from the filename.
/// - `High` - Remove high-ASCII characters (chars from 128 to 255).
/// - `Trim` - Remove leading and trailing spaces.
/// - `D/S` - Remove occurrences of double spaces, and replace them with single spaces.
/// - `Accent` - Remove accented characters and replace them with non-accented versions.
/// - `Chars` - Remove all characters.
/// - `Sym` - Remove all symbols.
/// - `Lead Dots` - Remove the . or .. from the front of filenames.
///
/// Note: When you use the `crop` option, you have the ability of specifying a special
/// value using the wildcard (\*). This will remove the specified string, and any
/// characters occupied by the wildcard. So for example, specifying [*] would convert
/// "Hello[ABC] Joe" to just "Hello Joe", as it has removed the two square brackets and
/// everything between.

pub enum Crop {
    Before,
    After,
}

#[cfg(test)]
mod remove_tests {
    use super::*;
}
