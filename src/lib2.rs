/// Counts words in a string using rules that closely match
///
/// Word does *not* simply split on spaces. Instead, it uses
/// human‑friendly heuristics:
///
/// ### Rules implemented:
/// - Tokens are separated by **whitespace**.
/// - Leading/trailing punctuation is ignored (e.g., `"hello,"` → `hello`).
/// - Hyphenated words count as **one** (e.g., `"state-of-the-art"` → 1).
/// - Contractions count as **one** (e.g., `"don't"` → 1).
/// - URLs count as **one** word.
/// - Emojis count as **one** word.
/// - CJK (Chinese/Japanese/Korean) characters count as **individual words**,
///   matching Word’s behavior (e.g., `"你好世界"` → 3).
/// - Multiple spaces, tabs, and newlines are ignored.
///
/// ### What this function does *not* handle:
/// - Word document structural features (fields, comments, footnotes, etc.).
///   Those do not apply to plain text.
///
/// ### Examples
/// ```
/// use bt_string_utils::lib2::word_count;
/// assert_eq!(word_count("Hello, world!"), 2);
/// assert_eq!(word_count("state-of-the-art"), 1);
/// assert_eq!(word_count("I'm here"), 2);
/// ```
///
/// # Arguments
/// * `text` – The input string to analyze.
///
/// # Returns
/// The number of words.
pub fn word_count(text: &str) -> usize {
    let mut count = 0;

    for token in text.split_whitespace() {
        // Trim leading/trailing punctuation (Word ignores it)
        let trimmed = token.trim_matches(|c: char| {
            c.is_ascii_punctuation() && c != '\'' && c != '-'
        });

        if trimmed.is_empty() {
            continue;
        }

        // Word treats CJK characters as individual words
        if trimmed.chars().all(|c| is_cjk(c)) {
            count += trimmed.chars().count();
            continue;
        }

        // Hyphenated words and contractions count as one
        count += 1;
    }

    count
}

/// Returns `true` if the character belongs to a CJK (Chinese/Japanese/Korean)
/// Unicode block.
///
///
/// ### Examples
/// ```
/// use bt_string_utils::lib2::is_cjk;
/// assert!(is_cjk('你'));
/// assert!(!is_cjk('a'));
/// assert!(is_cjk('你'));
/// assert!(is_cjk('界'));
/// assert!(!is_cjk('a'));
/// assert!(!is_cjk('🙂'));
///
/// ```
pub fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x4E00..=0x9FFF   | // CJK Unified Ideographs
        0x3400..=0x4DBF   | // CJK Extension A
        0x20000..=0x2A6DF | // CJK Extension B
        0x2A700..=0x2B73F | // CJK Extension C
        0x2B740..=0x2B81F | // CJK Extension D
        0x2B820..=0x2CEAF | // CJK Extension E
        0xF900..=0xFAFF   | // CJK Compatibility Ideographs
        0x2F800..=0x2FA1F   // CJK Compatibility Supplement
    )
}


/// Counts paragraphs in a string using rules that match
///
/// In plain text, this corresponds to:
/// - `\r\n` (Windows newline)
/// - `\n`   (Unix newline)
/// - `\r`   (old Mac newline)
///
/// ### Rules implemented:
/// - Each newline sequence ends a paragraph.
/// - Consecutive newlines create **empty paragraphs**, and Word counts them.
/// - A document with no newline at all counts as **one paragraph**.
/// - An empty document counts as **zero paragraphs**.
///
/// ### Examples
/// ```
/// use bt_string_utils::lib2::count_paragraphs;
/// assert_eq!(count_paragraphs("Hello"), 1);
/// assert_eq!(count_paragraphs("Hello\nWorld"), 2);
/// assert_eq!(count_paragraphs("Line1\n\nLine3"), 3); // empty paragraph in the middle
/// assert_eq!(count_paragraphs(""), 0);
/// ```
pub fn count_paragraphs(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    // Normalize newline types
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");

    let newline_count = normalized.matches('\n').count();

    if newline_count == 0 {
        return 1;
    }

    // If the text starts with a newline, Word counts the number of newlines
    if normalized.starts_with('\n') {
        return newline_count;
    }

    // Otherwise: paragraphs = newlines + 1
    newline_count + 1
}

/// Splits a given string into multiple chunks of safe size while ensuring that UTF-8 multi-byte characters are not split.
/// 
/// This function takes a string and divides it into smaller chunks of `chunk_size_bytes` bytes or less, ensuring that each chunk ends 
/// at a valid UTF-8 character boundary. This helps avoid issues with splitting multi-byte characters (such as emojis or non-Latin 
/// characters), which can lead to invalid UTF-8 sequences. The chunks are returned as a `Vec<String>`, which contains the substrings 
/// of the original content.
/// 
/// # Parameters
/// 
/// - `content`: A reference to a `str` containing the document or text data to be split into chunks. The string must be a valid UTF-8 string.
/// - `chunk_size_bytes: usize`: Size of a chunk in bytes
/// 
/// # Returns
/// 
/// - `Vec<String>`: A vector of `String` instances, each containing one chunk of the original `content`. and the function ensures that no chunk is split in the middle of a multi-byte UTF-8 character.
/// 
/// # Behavior
/// 
/// The function processes the input string byte-by-byte and ensures that each chunk is of safe size and that multi-byte characters 
/// are respected. The chunks are added to the result vector in order, with each chunk being a valid UTF-8 sequence.
/// 
/// # Example
/// 
/// ```rust
/// let document: &str = "Your 70k+ character document..."; // some long document content
/// let chunks = split_into_chunks(document);
/// for chunk in chunks {
///     println!("{}", chunk);
/// }
/// ```
/// 
/// # Limitations
/// 
/// - The function will step backwards within the byte array if necessary to ensure that chunks don't break in the middle of a multi-byte character.
/// - It is optimized to handle **UTF-8** encoded data correctly. 
/// - If the input string is extremely short, only a single chunk will be returned.
pub fn split_into_chunks(content: &str, chunk_size_bytes: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let bytes = content.as_bytes();
    let mut offset = 0;

    while offset < bytes.len() {
        let end = (offset + chunk_size_bytes).min(bytes.len());

        // Ensure UTF-8 boundaries (not cutting in the middle of a multi-byte character)
        let mut valid_end = end;
        while !std::str::from_utf8(&bytes[offset..valid_end]).is_ok() {
            valid_end -= 1; // Step back to avoid splitting a multi-byte character
        }

        let chunk = String::from_utf8_lossy(&bytes[offset..valid_end]).to_string();
        chunks.push(chunk);

        offset = valid_end; // Move to the next chunk start position
    }

    chunks
}
