use inflector::Inflector;

/// Change the case of the file.
/// - `Keep` - Do change the capitalization (default).
/// - `Lower` - change all selected files to lowercase.
/// - `Upper` - CHANGE ALL SELECTED FILES TO UPPERCASE.
/// - `Title` - Change All Selected Files To Title Case.
/// - `Sentence` - Change all selected files to sentence case.
/// - `Snake` - Flag_to_change_all_selected_files_to_snake_case.
/// Exceptions: You can also enter a list of "exceptions", separated by semicolons.
/// So for example if you entered PDF;doc then any occurrence of pdf (or PDF, Pdf,
/// etc) would be converted to upper-case, and every occurrence of DOC (or DoC)
/// would become doc.
pub struct CaseOptions<'a> {
    pub case: Case,
    pub snake: bool,
    pub exceptions: Option<&'a str>,
}

pub enum Case {
    Keep,
    Lower,
    Upper,
    Title,
    Sentence,
}

impl CaseOptions<'_> {
    pub fn process(&self, file: &mut String) {
        *file = match self.case {
            Case::Keep => file.to_owned(),
            Case::Lower => file.to_lowercase(),
            Case::Upper => file.to_uppercase(),
            Case::Title => file.to_title_case(),
            Case::Sentence => file.to_sentence_case(),
        };
        if let Some(exceptions) = self.exceptions {
            for exception in exceptions.split(";") {
                let mod_exception = match self.case {
                    Case::Keep => exception.to_owned(),
                    Case::Lower => exception.to_lowercase(),
                    Case::Upper => exception.to_uppercase(),
                    Case::Title => exception.to_title_case(),
                    Case::Sentence => exception.to_sentence_case(),
                };
                *file = file.replace(&mod_exception, &exception);
            }
        }
        if self.snake {
            *file = file.replace(" ", "_")
        };
    }
}

#[cfg(test)]
mod case_tests {
    use super::*;
    #[test]
    fn test_keep_case() {
        let mut file = String::from("test file");
        let opt = CaseOptions {
            case: Case::Keep,
            snake: false,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("test file"));
    }

    #[test]
    fn test_keep_case_snake() {
        let mut file = String::from("test file");
        let opt = CaseOptions {
            case: Case::Keep,
            snake: true,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("test_file"));
    }

    #[test]
    fn test_lower_case() {
        let mut file = String::from("TEST FILE");
        let opt = CaseOptions {
            case: Case::Lower,
            snake: false,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("test file"));
    }

    #[test]
    fn test_lower_case_snake() {
        let mut file = String::from("TEST FILE");
        let opt = CaseOptions {
            case: Case::Lower,
            snake: true,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("test_file"));
    }

    #[test]
    fn test_upper_case() {
        let mut file = String::from("test file");
        let opt = CaseOptions {
            case: Case::Upper,
            snake: false,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("TEST FILE"));
    }

    #[test]
    fn test_upper_case_snake() {
        let mut file = String::from("test file");
        let opt = CaseOptions {
            case: Case::Upper,
            snake: true,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("TEST_FILE"));
    }

    #[test]
    fn test_title_case() {
        let mut file = String::from("test file");
        let opt = CaseOptions {
            case: Case::Title,
            snake: false,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("Test File"));
    }

    #[test]
    fn test_title_case_snake() {
        let mut file = String::from("test file");
        let opt = CaseOptions {
            case: Case::Title,
            snake: true,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("Test_File"));
    }

    #[test]
    fn test_sentence_case() {
        let mut file = String::from("test file");
        let opt = CaseOptions {
            case: Case::Sentence,
            snake: false,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("Test file"));
    }

    #[test]
    fn test_sentence_case_snake() {
        let mut file = String::from("test file");
        let opt = CaseOptions {
            case: Case::Sentence,
            snake: true,
            exceptions: None,
        };
        opt.process(&mut file);
        assert_eq!(file, String::from("Test_file"));
    }

    #[test]
    fn test_exceptions_with_upper() {
        let mut files = (String::from("test file.doc"), String::from("test file.pdf"));
        let opt = CaseOptions {
            case: Case::Upper,
            snake: false,
            exceptions: Some(&"doc;PDF"),
        };
        (opt.process(&mut files.0), opt.process(&mut files.1));
        let expected = (String::from("TEST FILE.doc"), String::from("TEST FILE.PDF"));
        assert_eq!(files, expected);
    }
}
