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