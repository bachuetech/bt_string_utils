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

#[cfg(test)]
mod split_chunk_tests {
    use bt_string_utils::lib2::split_into_chunks;

    // The function to split content into chunks
    const CHUNK_SIZE_BYTES: usize = 30_000; // 30 KB (bytes per chunk)

    // Test 1: Basic Test - String fits into one chunk
    #[test]
    fn test_single_chunk() {
        let input = "Hello, world!";
        let chunks = split_into_chunks(input, CHUNK_SIZE_BYTES);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], input);
    }

    // Test 2: Medium Test - String exceeds 30,000 bytes and gets split into multiple chunks
    #[test]
    fn test_multiple_chunks() {
        // A string with exactly 60,000 characters (which will be split into two chunks)
        let input = "a".repeat(60_000);
        let chunks = split_into_chunks(&input, CHUNK_SIZE_BYTES);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].len(), CHUNK_SIZE_BYTES);
        assert_eq!(chunks[1].len(), CHUNK_SIZE_BYTES);
    }

    // Test 3: Multi-byte Characters Test - Ensure multi-byte characters aren't split
    #[test]
    fn test_multi_byte_characters() {
        let input = "This is a test with emoji: 🦄🚀";
        let chunks = split_into_chunks(input, CHUNK_SIZE_BYTES);
        assert_eq!(chunks.len(), 1); // Only one chunk because it's under the size limit
        assert!(chunks[0].contains("🦄"));
        assert!(chunks[0].contains("🚀"));
    }

    // Test 4: Single Character Test - Small input
    #[test]
    fn test_single_character() {
        let input = "A";
        let chunks = split_into_chunks(input, CHUNK_SIZE_BYTES);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], input);
    }

    // Test 5: Empty String Test
    #[test]
    fn test_empty_string() {
        let input = "";
        let chunks = split_into_chunks(input, CHUNK_SIZE_BYTES);
        assert_eq!(chunks.len(), 0); // An empty string should result in no chunks
    }

    // Test 6: Large Input Test - Large string that needs multiple chunks
    #[test]
    fn test_large_input() {
        let input = "a".repeat(100_000); // String of 100,000 characters
        let chunks = split_into_chunks(&input, CHUNK_SIZE_BYTES);
        assert_eq!(chunks.len(), 4); // Should split into 4 chunks, each of 30,000 bytes
        assert_eq!(chunks[0].len(), CHUNK_SIZE_BYTES);
        assert_eq!(chunks[1].len(), CHUNK_SIZE_BYTES);
        assert_eq!(chunks[2].len(), CHUNK_SIZE_BYTES);
        assert_eq!(chunks[3].len(), 10_000); // Last chunk is smaller
    }

    // Test 7: Check that chunks never break multi-byte characters
    #[test]
    fn test_dont_split_multi_byte_characters() {
        let input = "This is a test with a Chinese character: 字";
        let chunks = split_into_chunks(input, CHUNK_SIZE_BYTES);
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].contains("字")); // Ensure that the Chinese character is intact
    }
}
