#[cfg(test)]
mod word_count_tests {
    use bt_string_utils::lib2::word_count;


    #[test]
    fn basic_words() {
        assert_eq!(word_count("Hello world"), 2);
        assert_eq!(word_count("One two three"), 3);
    }

    #[test]
    fn punctuation_handling() {
        assert_eq!(word_count("Hello, world!"), 2);
        assert_eq!(word_count("(test)"), 1);
        assert_eq!(word_count("\"quoted\""), 1);
    }

    #[test]
    fn multiple_whitespace() {
        assert_eq!(word_count("a   b\tc\nd"), 4);
        assert_eq!(word_count("   spaced   out   "), 2);
    }

    #[test]
    fn hyphenated_words() {
        assert_eq!(word_count("state-of-the-art"), 1);
        assert_eq!(word_count("mother-in-law"), 1);
    }

    #[test]
    fn contractions() {
        assert_eq!(word_count("don't stop"), 2);
        assert_eq!(word_count("I'm here"), 2);
        assert_eq!(word_count("they're coming"), 2);
    }

    #[test]
    fn urls() {
        assert_eq!(word_count("Visit https://example.com now"), 3);
        assert_eq!(word_count("example.com/test"), 1);
    }

    #[test]
    fn emojis() {
        assert_eq!(word_count("🙂"), 1);
        assert_eq!(word_count("Hello 🙂 world"), 3);
    }

    #[test]
    fn empty_and_whitespace_only() {
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("     "), 0);
        assert_eq!(word_count("\n\t  "), 0);
    }
}

#[cfg(test)]
mod count_paragraphs_tests {
    use bt_string_utils::lib2::count_paragraphs;

    use super::*;

    #[test]
    fn single_paragraph_no_newline() {
        assert_eq!(count_paragraphs("Hello world"), 1);
    }

    #[test]
    fn two_paragraphs_unix_newline() {
        assert_eq!(count_paragraphs("Hello\nWorld"), 2);
    }

    #[test]
    fn two_paragraphs_windows_newline() {
        assert_eq!(count_paragraphs("Hello\r\nWorld"), 2);
    }

    #[test]
    fn two_paragraphs_old_mac_newline() {
        assert_eq!(count_paragraphs("Hello\rWorld"), 2);
    }

    #[test]
    fn empty_document() {
        assert_eq!(count_paragraphs(""), 0);
    }

    #[test]
    fn newline_only() {
        // Word counts a single newline as one empty paragraph
        assert_eq!(count_paragraphs("\n"), 1);
    }

    #[test]
    fn cr_only() {
        // Word counts a single newline as one empty paragraph
        assert_eq!(count_paragraphs("\r"), 1);
    }    


    #[test]
    fn crnl_only() {
        // Word counts a single newline as one empty paragraph
        assert_eq!(count_paragraphs("\r\n"), 1);
    }    

    #[test]
    fn trailing_newline_creates_empty_paragraph() {
        // Word counts the trailing empty paragraph
        assert_eq!(count_paragraphs("Hello\n"), 2);
        assert_eq!(count_paragraphs("Hello\r\n"), 2);
    }

    #[test]
    fn multiple_empty_paragraphs() {
        // Two newlines = two paragraph breaks = three paragraphs
        assert_eq!(count_paragraphs("A\n\nB"), 3);

        // Three consecutive newlines = four paragraphs
        assert_eq!(count_paragraphs("A\n\n\nB"), 4);
    }

    #[test]
    fn paragraphs_with_whitespace_only_lines() {
        // Word counts whitespace-only lines as paragraphs
        assert_eq!(count_paragraphs("A\n   \nB"), 3);
    }

    #[test]
    fn mixed_newline_types() {
        let text = "A\r\nB\nC\rD";
        assert_eq!(count_paragraphs(text), 4);
    }
}