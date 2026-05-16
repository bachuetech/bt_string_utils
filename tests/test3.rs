/***************** */
//UNIT TEST
/**************** */
#[cfg(test)]
mod remove_whitespace_tests {
    use bt_string_utils::cleanser::remove_whitespace;


    #[test]
    fn removes_spaces() {
        let input = "Hello World";
        assert_eq!(remove_whitespace(input), "HelloWorld");
    }

    #[test]
    fn removes_tabs_and_newlines() {
        let input = "Hello\tWorld\nRust\rLang";
        assert_eq!(remove_whitespace(input), "HelloWorldRustLang");
    }

    #[test]
    fn removes_unicode_whitespace() {
        // Includes non‑breaking space (U+00A0)
        let input = "Hello\u{00A0}World";
        assert_eq!(remove_whitespace(input), "HelloWorld");
    }

    #[test]
    fn handles_no_whitespace() {
        let input = "RustLang";
        assert_eq!(remove_whitespace(input), "RustLang");
    }

    #[test]
    fn handles_empty_string() {
        let input = "";
        assert_eq!(remove_whitespace(input), "");
    }
}

#[cfg(test)]
mod replace_whitespaces_tests {
    use bt_string_utils::cleanser::replace_whitespace;


    #[test]
    fn replaces_spaces() {
        let input = "Hello world";
        let output = replace_whitespace(input, '_');
        assert_eq!(output, "Hello_world");
    }

    #[test]
    fn replaces_multiple_whitespace() {
        let input = "Hello   world";
        let output = replace_whitespace(input, '-');
        assert_eq!(output, "Hello---world");
    }

    #[test]
    fn replaces_tabs() {
        let input = "Hello\tworld";
        let output = replace_whitespace(input, '*');
        assert_eq!(output, "Hello*world");
    }

    #[test]
    fn replaces_newlines() {
        let input = "Hello\nworld";
        let output = replace_whitespace(input, '#');
        assert_eq!(output, "Hello#world");
    }

    #[test]
    fn replaces_unicode_whitespace() {
        let input = "Hello\u{2003}world"; // EM SPACE
        let output = replace_whitespace(input, '_');
        assert_eq!(output, "Hello_world");
    }

    #[test]
    fn no_whitespace_no_change() {
        let input = "Helloworld";
        let output = replace_whitespace(input, '_');
        assert_eq!(output, "Helloworld");
    }

    #[test]
    fn empty_string() {
        let input = "";
        let output = replace_whitespace(input, '_');
        assert_eq!(output, "");
    }
}

#[cfg(test)]
mod initials_tests {
    use bt_string_utils::finder::initials_uppercase;


    #[test]
    fn basic_sentence() {
        let s = "Rust language is fast";
        assert_eq!(initials_uppercase(s), "RLIF");
    }

    #[test]
    fn handles_multiple_spaces() {
        let s = "  multiple   spaces   here ";
        assert_eq!(initials_uppercase(s), "MSH");
    }

    #[test]
    fn empty_string_returns_empty() {
        let s = "";
        assert_eq!(initials_uppercase(s), "");
    }

    #[test]
    fn single_word() {
        let s = "Hello";
        assert_eq!(initials_uppercase(s), "H");
    }

    #[test]
    fn unicode_words() {
        let s = "árbol niño corazón";
        assert_eq!(initials_uppercase(s), "ÁNC");
    }

    #[test]
    fn unicode_mixed_scripts() {
        let s = "東京 Rust язык";
        assert_eq!(initials_uppercase(s), "東RЯ");
    }

    #[test]
    fn punctuation_attached_to_words() {
        let s = "Hello, world!";
        assert_eq!(initials_uppercase(s), "HW");
    }
}