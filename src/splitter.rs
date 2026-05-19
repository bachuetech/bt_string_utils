/// Splits the given string at the first occurrence of the specified separator.
///
/// # Arguments
///
/// * `s` - A string slice to be split.
/// * `separator` - The substring used as a separator.
///
/// # Returns
///
/// A tuple containing two strings:
/// - The first part of the string before the separator.
/// - The second part of the string after the separator.
///
/// If the separator is not found, returns the original string and an empty string.
///
/// # Examples
///
/// ```
/// use bt_string_utils::splitter::get_first_of_split;
/// let (part1, part2) = get_first_of_split("hello=world", "=");
/// assert_eq!(part1, "hello");
/// assert_eq!(part2, "world");
///
/// let (part1, part2) = get_first_of_split("key:value", ":");
/// assert_eq!(part1, "key");
/// assert_eq!(part2, "value");
///
/// let (part1, part2) = get_first_of_split("no=separator", " ");
/// assert_eq!(part1, "no=separator");
/// assert_eq!(part2, "");
/// ```
pub fn get_first_of_split(s: &str, separator: &str) -> (String, String){
    if let Some(position) = s.find(separator){
        let str1 = s[..position].to_owned();
        let str2 = s[position + 1..].to_owned();
        (str1, str2)
    }else{
        (s.to_owned(),"".to_owned())
    }
}

/// Splits a string into at most `n` substrings, grouped by whole words.
///
/// This function performs **word‑based splitting**, never character‑based.
/// It guarantees:
///
/// - Words are never broken apart.
/// - The number of returned substrings is **min(n, word_count)**.
/// - Unicode and emoji are handled safely (because splitting happens on
///   whitespace boundaries, which are always valid UTF‑8 boundaries).
/// - The original string is never copied; all substrings are `&str` slices.
///
///
/// # Arguments
/// * `s` — The input string to split.
/// * `n` — The desired number of substrings. The function will never return more substrings than the number of words in `s`.
///
/// # Returns
/// A `Vec<&str>` containing up to `n` substrings, each containing one or more
/// whole words from the original string.
///
/// # Examples
/// Splitting into fewer groups than words:
/// ```
/// use bt_string_utils::splitter::split_upto_n_by_word;
/// let s = "Hello 🙂 World from Rust";
/// let parts = split_upto_n_by_word(s, 3);
/// assert_eq!(parts, vec!["Hello", " 🙂 World", " from Rust"]);
/// ```
///
/// Requesting more groups than words:
///
/// ```
/// use bt_string_utils::splitter::split_upto_n_by_word;
/// let s = "Hello 🙂 World";
/// let parts = split_upto_n_by_word(s, 10);
/// assert_eq!(parts, vec!["Hello", " 🙂", " World"]);
/// ```
///
/// Single group:
///
/// ```
/// use bt_string_utils::splitter::split_upto_n_by_word;
/// let s = "Hello world";
/// let parts = split_upto_n_by_word(s, 1);
/// assert_eq!(parts, vec!["Hello world"]);
/// ```
///
/// Empty input:
///
/// ```
/// use bt_string_utils::splitter::split_upto_n_by_word;
/// let parts = split_upto_n_by_word("", 5);
/// assert!(parts.is_empty());
/// ```
pub fn split_upto_n_by_word(s: &str, n: usize) -> Vec<&str> {
    // 1. Identify words including trailing punctuation
    let mut words = Vec::new();
    let mut in_word = false;
    let mut start = 0;

    for (i, ch) in s.char_indices() {
        if ch.is_whitespace() {
            if in_word {
                words.push((start, i));
                in_word = false;
            }
        } else {
            if !in_word {
                in_word = true;
                start = i;
            }
        }
    }
    if in_word {
        words.push((start, s.len()));
    }

    let total = words.len();
    if total == 0 {
        return Vec::new();
    }

    let parts = n.min(total);
    let mut out = Vec::with_capacity(parts);

    // 2. Precompute whitespace map
    let mut is_space = vec![false; s.len()];
    for (i, ch) in s.char_indices() {
        if ch.is_whitespace() {
            is_space[i] = true;
        }
    }

    // 3. Build groups
    for i in 0..parts {
        let start_word = i * total / parts;
        let end_word = (i + 1) * total / parts - 1;

        // Start at the beginning of the first word in this group,
        // but include ALL whitespace before it.
        let mut start_idx = words[start_word].0;
        while start_idx > 0 && is_space[start_idx - 1] {
            start_idx -= 1;
        }

        // End at the end of the last word in this group.
        let end_idx = words[end_word].1;

        out.push(&s[start_idx..end_idx]);
    }

    out
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
/// use bt_string_utils::splitter::split_into_chunks;
/// let document: &str = "Your 70k+ character document..."; // some long document content
/// let chunks = split_into_chunks(document,5);
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
        while std::str::from_utf8(&bytes[offset..valid_end]).is_err() {
            valid_end -= 1; // Step back to avoid splitting a multi-byte character
        }

        let chunk = String::from_utf8_lossy(&bytes[offset..valid_end]).to_string();
        chunks.push(chunk);

        offset = valid_end; // Move to the next chunk start position
    }

    chunks
}