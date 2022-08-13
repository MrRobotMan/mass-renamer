# Introduction
When you run the application for the first time you will be presented with the main window, located in the center of the screen. The main screen is laid out in a similar way to Windows Explorer. Below the menu bar are two panes, the left of which is a tree view providing access to your system drives and folders. The right-hand pane is a list view of the files and folders contained within the currently selected branch of the tree shown over on the left.

Below the navigation panes are a series of controls. The controls are grouped together according to function - for example, all the facilities to remove text from a filename. Only complete the boxes you are interested in. For example, don't assume you HAVE to put something in the Regular Expressions box, or the New Location box, etc.

The rules used by the Bulk Rename Utility to rename files processed in the numerical order of the fields. Names are not actually changed until the "Rename" button is clicked, but you can always see a preview of the proposed filename in the New Name column. Note that this column is only updated for the files which are selected.


# The Fields
## RegEx (1)

Use a regular expression `Match` to find the offending text and `Replace` it with new. 

Check the `Include Ext.` box to include the file extension in the `Match`.

## Name (2)

Name drop-down:
- `Keep` - Do not change the original file name (default).
- `Remove` - Completely erase the filename from the selected items. This allows it to be rebuilt using components higher than (2).
- `Fixed` - Specify a new filename in the box for all selected items. Only really useful if you're also using the Numbering section.
- `Reverse` - Reverse the name, e.g. 12345.txt becomes 54321.txt.

## Replace (3)

`Replace` the text in this field with the text in the `With` field. `Replace` can be case-sensitive using `Match Case` checkbox. Note that the `With` text is always replaced with the text as written, including any specific text case.

## Case (4)

Case drop-down:
- `Keep` - Do change the capitalization (default).
- `Lower` - change all selected files to lowercase.
- `Upper` - CHANGE ALL SELECTED FILES TO UPPERCASE.
- `Title` - Change All Selected Files To Title Case.
- `Sentence` - Change all selected files to sentence case.
- `Snake` - Change_all_selected_files_to_snake_case_while_keeping_all_other_case_information_the_same.
- `Roman` - Convert Roman Numerals to upper of lower case. For example, if you had a file called "Beethoven's niNTH syMPHONY part iii", you might want to use Title Case to format the filename, but this would impact the "iii". Use this control to handle the Roman Numeral element. This can be buggy.

Exceptions: You can also enter a list of "exceptions", separated by semicolons. So for example if you entered PDF;doc then any occurrence of pdf (or PDF, Pdf, etc) would be converted to upper-case, and every occurrence of DOC (or DoC) would become doc.

## Remove (5)

Remove specific parts of a filename but not file extensions.

- `First n` - Remove the first n characters from the name. 
- `Last n` - Remove the last n characters from the name. 
- `From`/`to` - Remove a string of text, e.g. from the 6th to the 9th characters (0 indexed).
- `Chars` - Remove occurrences of the listed characters from the name (no separator needed).
- `Words` - Remove occurrences of listed words (separated by spaces).
- `Crop` - Remove any text which occurs before (or after) a specific character or word. See note below.
- `Digits` - Remove all occurrences of the digits 0-9 from the filename.
- `High` - Remove high-ASCII characters (chars from 128 to 255).
- `Trim` - Remove leading and trailing spaces.
- `D/S` - Remove occurrences of double spaces, and replace them with single spaces.
- `Accent` - Remove accented characters and replace them with non-accented versions. 
- `Chars` - Remove all characters.
- `Sym` - Remove all symbols.
- `Lead Dots` - Remove the . or .. from the front of filenames.

Note: When you use the `crop` option, you have the ability of specifying a special value using the wildcard (\*). This will remove the specified string, and any characters occupied by the wildcard. So for example, specifying [*] would convert "Hello[ABC] Joe" to just "Hello Joe", as it has removed the two square brackets and everything between.

## Add (6)

Add a fixed `Prefix` or`Suffix` to the filename, or `Insert` text at a specific location (0 indexed).

You may also choose to add a `Word Space`. This will insert a space before any capital letter (except the first character), unless there's a space already there.

## Auto Date (7)

Use the prefix or suffix `Mode` to modify the filename with a date format.
The `Date` that the file was created, modified, or the current date can be added in the format (`FMT`) selected. A `Sep`erator can be specified for the character(s) between the filename and the date as well as a format for setting the character(s) between date `Seg`ments. Select the `YYYY` box to display years as 4 digit instead of the default 2.

You also have the option to specify your own custom date formats using [chrono::format::strftime](https://docs.rs/chrono/0.4.20/chrono/format/strftime/index.html) specifiers.

## Append Folder Name (8)

Add the name of the containing folder or hierarchy of folders. These can be added in prefix or suffix `Mode`, with a `Sep`arator specified and the maximum number of `Levels` selected.

On Windows, if the hierarchy reaches the drive root (i.e. C:\\) the ":\\" characters will be automatically removed.

## Numbering (9)

Add sequential numbers.
- `Mode` - Choose between prefix, suffix, both, or insert at a location (0 indexed).
- `Start` - Specify a starting number for the numbering.
- `Step` - The number to be added to the previous.
- `Pad` - The minimum number of digits occupied by the numeric element. Bases 1-36 will be padded with leading zeros; the a-z and A-Z options will be padded with "a" or "A" as appropriate.
- `Sep`. - A character or characters that you wish to be inserted between the old filename and the number. If you enter the special character ":" (colon) in the Sep. box then this will be replaced with the auto-number. So a separator value of ABC:DEF: would result in ABC1DEF1, ABC2ABC2 etc.
- `Break` - Reset the auto-number when the nth character changes. e.g. enter 4 to cause the number to reset when the 4th character of the NEW name changes.
- `Base` - You can choose to append the auto-number in any numeric base, from base 2 to base 36. e.g. a value of 26 in base 16 would be appended as 1A. Or even use letters, e.g. A-Z or a-z or Roman numerals (upper or lower).

## Extension (10)

Change case of the file name extension.
- `Keep` - Leave the original capitalization intact.
- `Lower` - convert all letters in the extension to lowercase.
- `Upper` - CONVERT ALL LETTERS IN THE EXTENSION INTO UPPERCASE.
- `Title` - Capitalize the first character of the extension.
- `Fixed` - Replace the extension with a fixed extension.
- `Extra` - Add a secondary extension. For example, change all selected files to .bak.
- `Remove` - Remove any file extension. 
