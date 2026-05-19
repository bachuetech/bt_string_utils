#[cfg(test)]
mod finder_n_tests {
    use bt_string_utils::finder::{get_first_n_chars, get_last_n_chars};


    #[test]
    fn returns_empty_for_empty_string() {
        assert_eq!(get_first_n_chars("", 0), "");
        assert_eq!(get_first_n_chars("", 5), "");
    }

    #[test]
    fn returns_empty_when_n_is_zero() {
        assert_eq!(get_first_n_chars("hello", 0), "");
    }

    #[test]
    fn returns_full_string_when_n_equals_length() {
        assert_eq!(get_first_n_chars("hello", 5), "hello");
    }

    #[test]
    fn returns_full_string_when_n_exceeds_length() {
        assert_eq!(get_first_n_chars("hello", 10), "hello");
    }

    #[test]
    fn slices_ascii_correctly() {
        assert_eq!(get_first_n_chars("hello world", 5), "hello");
        assert_eq!(get_first_n_chars("abcdef", 3), "abc");
    }

    #[test]
    fn slices_utf8_multibyte_chars_correctly() {
        assert_eq!(get_first_n_chars("héllo", 2), "hé");
        assert_eq!(get_first_n_chars("åäö", 2), "åä");
    }

    #[test]
    fn slices_emoji_correctly() {
        assert_eq!(get_first_n_chars("😀😃😄😁", 2), "😀😃");
        assert_eq!(get_first_n_chars("👋🌍", 1), "👋");
    }

    #[test]
    fn handles_mixed_ascii_and_unicode() {
        assert_eq!(get_first_n_chars("hello 👋 world", 7), "hello 👋");
    }

    #[test]
    fn handles_combining_characters() {
        // "é" = 'e' + combining acute accent
        let s = "e\u{301}clair";

        // first char only
        assert_eq!(get_first_n_chars(s, 1), "e");

        // first two unicode scalar values
        assert_eq!(get_first_n_chars(s, 2), "e\u{301}");
    }

    #[test]
    fn handles_single_character_strings() {
        assert_eq!(get_first_n_chars("a", 1), "a");
        assert_eq!(get_first_n_chars("😀", 1), "😀");
    }

    #[test]
    fn does_not_panic_on_unicode_boundaries() {
        let s = "こんにちは世界";

        assert_eq!(get_first_n_chars(s, 3), "こんに");
        assert_eq!(get_first_n_chars(s, 20), s);
    }

    #[test]
    fn returned_slice_points_into_original_string() {
        let s = String::from("hello world");

        let slice = get_first_n_chars(&s, 5);

        assert_eq!(slice, "hello");

        // Verify no allocation by checking pointer range
        assert!(std::ptr::eq(slice.as_ptr(), s.as_ptr()));
    }

    #[test]
    fn handles_large_n_gracefully() {
        let s = "test";

        assert_eq!(get_first_n_chars(s, usize::MAX), s);
    }

    #[test]
    fn handles_newlines_and_whitespace() {
        let s = "hello\nworld\t🙂";

        assert_eq!(get_first_n_chars(s, 6), "hello\n");
    }

    #[test]
    fn handles_right_to_left_text() {
        let s = "שלום עולם";

        assert_eq!(get_first_n_chars(s, 4), "שלום");
    }

    #[test]
    fn handles_zero_width_characters() {
        let s = "a\u{200B}b";

        assert_eq!(get_first_n_chars(s, 2), "a\u{200B}");
    }


    #[test]
    fn test_ascii_basic() {
        let s = "HelloWorld";
        assert_eq!(get_last_n_chars(s, 5), "World");
        assert_eq!(get_last_n_chars(s, 1), "d");
        assert_eq!(get_last_n_chars(s, 0), "");
    }

    #[test]
    fn test_unicode_basic() {
        let s = "a💙b💛c";
        assert_eq!(get_last_n_chars(s, 1), "c");
        assert_eq!(get_last_n_chars(s, 2), "💛c");
        assert_eq!(get_last_n_chars(s, 3), "b💛c");
        assert_eq!(get_last_n_chars(s, 4), "💙b💛c");
    }

    #[test]
    fn test_unicode_entire_string() {
        let s = "💙💛💚";
        assert_eq!(get_last_n_chars(s, 3), "💙💛💚");
        assert_eq!(get_last_n_chars(s, 10), "💙💛💚"); // n > len
    }

    #[test]
    fn test_empty_string() {
        let s = "";
        assert_eq!(get_last_n_chars(s, 5), "");
        assert_eq!(get_last_n_chars(s, 0), "");
    }

    #[test]
    fn test_mixed_unicode_and_ascii() {
        let s = "Rust💙Rocks💛!";
        assert_eq!(get_last_n_chars(s, 1), "!");
        assert_eq!(get_last_n_chars(s, 2), "💛!");
        assert_eq!(get_last_n_chars(s, 7), "Rocks💛!");
    }

    #[test]
    fn test_exact_boundary() {
        let s = "abc💙";
        assert_eq!(get_last_n_chars(s, 4), "abc💙");
        assert_eq!(get_last_n_chars(s, 3), "bc💙");
    }    
}