//! Multiple String related functions

use rand::{distr::Alphanumeric, Rng};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

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
/// use bt_string_utils::get_first_of_split;
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

/// Finds and returns the substring before the first occurrence of a given separator.
///
/// # Arguments
///
/// * `s` - A string slice that holds the text to search within.
/// * `separator` - A string slice that specifies the character(s) to look for as a separator.
///
/// # Returns
///
/// Returns a new `String` containing the substring before the first occurrence of the separator.
/// If the separator is not found, an empty `String` is returned.
///
/// # Examples
///
/// ```
/// use bt_string_utils::get_first_occurrance;
/// let result = get_first_occurrance("Hello, world!", ", ");
/// assert_eq!(result, "Hello");
///
/// let result = get_first_occurrance("No separator here", ",");
/// assert_eq!(result, "");
/// ```

pub fn get_first_occurrance(s: &str, separator: &str) -> String{
    if let Some(position) = s.find(separator){
        s[..position].to_owned()
    }else{
        "".to_owned()
    }
}

/// Finds and returns the value corresponding to a given key in a vector of key-value pairs.
///
/// # Arguments
///
/// * `kv_pairs` - A reference to a vector of strings where each string represents a key-value pair separated by '='.
/// * `key_to_find` - The key for which the corresponding value is to be found.
///
/// # Returns
///
/// Returns an `Option`:
/// - `Some(value)` if a matching key is found, containing the value associated with that key.
/// - `None` if no matching key is found.
///
/// # Examples
///
/// ```
/// use bt_string_utils::find_value_by_key;
/// let pairs = vec!["name=John".to_owned(), "age=30".to_owned(), "city=New York".to_owned()];
/// assert_eq!(find_value_by_key(&pairs, "name"), Some("John".to_string()));
/// assert_eq!(find_value_by_key(&pairs, "gender"), None);
/// ```
pub fn find_value_by_key(kv_pairs: &Vec<String>, key_to_find: &str) -> Option<String> {
    for item in kv_pairs {
        // Split the string at the '=' character
        if let Some((key, value)) = item.split_once('=') {
            if key == key_to_find {
                return Some(value.to_owned());
            }
        }
    }
    None
}

/// Remove Location for remove_char function
pub enum RemoveLocationEnum {
    Begin,
    End,
}

/// Removes the first or last character of a string if it matches the given target character.
///
/// # Arguments
///
/// * `begin` - A boolean indicating whether to remove the first (`true`) or last (`false`) character.
/// * `input` - A `String` to process.
/// * `target` - The character to remove.
///
/// # Returns
///
/// Returns a new `String` with the character removed if it matched.
///
/// # Examples
///
/// ```
/// use bt_string_utils::{remove_char, RemoveLocationEnum};
/// let modified = remove_char(RemoveLocationEnum::Begin, &"hello".to_string(), 'h');
/// assert_eq!(modified, "ello");
///
/// let modified = remove_char(RemoveLocationEnum::End, &"world!".to_string(), '!');
/// assert_eq!(modified, "world");
/// ```
///
/// If the character doesn't match, the original string is returned:
///
/// ```
/// use bt_string_utils::{remove_char, RemoveLocationEnum};
/// let modified = remove_char(RemoveLocationEnum::Begin, &"rust".to_string(), 'x');
/// assert_eq!(modified, "rust");
/// ```
pub fn remove_char(remove_from: RemoveLocationEnum, input: &String, target: char) -> String {
    match remove_from{
        RemoveLocationEnum::Begin => if input.starts_with(target) {
                                        return input.chars().skip(1).collect();
                                     },
        RemoveLocationEnum::End => if input.ends_with(target) {
                                        return input.chars().take(input.len() - 1).collect();
                                    },
    }
    /*if begin {
        if input.starts_with(target) {
            return input.chars().skip(1).collect();
        }
    } else {
        if input.ends_with(target) {
            return input.chars().take(input.len() - 1).collect();
        }
    }*/
    input.to_string() // Return unchanged if no removal occurs
}

/// Generates a random URL-safe string of the specified length.
///
/// # Arguments
///
/// * `n` - The length of the generated string.
///
/// # Returns
///
/// A `String` containing `n` random alphanumeric characters (`A-Z`, `a-z`, `0-9`).
///
/// # Examples
///
/// ```
/// use bt_string_utils::generate_url_safe_string;
/// let random_string = generate_url_safe_string(16);
/// println!("Generated string: {}", random_string);
/// ```
///
/// # Notes
///
/// - Uses the `rand` crate to generate random alphanumeric characters.
/// - Ensures the output contains only **URL-safe** characters.
/// - May require the `rand` crate in your Cargo.toml:
///
/// ```toml
/// [dependencies]
/// rand = "0.8"
/// ```
pub fn generate_url_safe_string(n: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

/// Checks whether a given `haystack` string contains the specified `word`
/// as a whole word, using word boundaries.
///
/// A whole word match means the `word` must be surrounded by non-word characters
/// (e.g., spaces, punctuation) or string boundaries. Substrings within longer words
/// will not match.
///
/// # Arguments
///
/// * `text` - The string to search within.
/// * `word` - The target word to search for.
///
/// # Returns
///
/// * `true` if `word` appears as a whole word in `haystack`.
/// * `false` otherwise.
///
/// # Examples
///
/// ```
/// use bt_string_utils::contains_whole_word;
/// assert_eq!(contains_whole_word("this is a target match", "target"), true);
/// assert_eq!(contains_whole_word("this is a targeted match", "target"), false);
/// assert_eq!(contains_whole_word("no-target", "target"), false);
/// ```
pub fn contains_whole_word(text: &str, word: &str) -> bool {
    let pattern = format!(r"(?:^|[^A-Za-z0-9-]){}(?:[^A-Za-z0-9-]|$)", regex::escape(word));    

    let re = Regex::new(&pattern).unwrap();
    re.is_match(text)
}

/// The remove_first_n function removes the first n characters from a string slice, 
/// returning a new string slice that starts from the character after the nth character. 
/// This function properly handles Unicode characters by working with character indices rather than byte indices, 
/// ensuring that multi-byte UTF-8 characters are correctly handled.
/// 
/// # Parameters
///    s: &str - A string slice from which characters will be removed
///    n: usize - The number of characters to remove from the beginning of the string
/// 
/// # Returns
///    &str - A string slice containing the original string with the first n characters removed
/// 
/// # Examples
/// ```
/// use bt_string_utils::remove_first_n_characters;
/// ```
/// Remove first 3 characters
/// ```
/// use bt_string_utils::remove_first_n_characters;
/// let result = remove_first_n_characters("Hello, World!", 3);
/// assert_eq!(result, "lo, World!");  // 'Hel' removed
/// ```
/// Remove more characters than exist
/// ```
/// use bt_string_utils::remove_first_n_characters;
/// let result = remove_first_n_characters("Hi", 5);
/// assert_eq!(result, "");  // Empty string returned
/// ```
/// Handle Unicode characters
/// ```
/// use bt_string_utils::remove_first_n_characters;
/// let result = remove_first_n_characters("🌟Hello", 1);
/// assert_eq!(result, "Hello");  // The emoji (2 bytes) is properly skipped
/// ```
pub fn remove_first_n_characters(s: &str, n: usize) -> &str {
    let byte_index = s.char_indices().nth(n).map(|(i, _)| i).unwrap_or(s.len());
    &s[byte_index..]
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
/// * `n` — The desired number of substrings. The function will never return
///         more substrings than the number of words in `s`.
///
/// # Returns
/// A `Vec<&str>` containing up to `n` substrings, each containing one or more
/// whole words from the original string.
///
/// # Examples
/// Splitting into fewer groups than words:
/// ```
/// use bt_string_utils::split_upto_n_by_word;
/// let s = "Hello 🙂 World from Rust";
/// let parts = split_upto_n_by_word(s, 3);
/// assert_eq!(parts, vec!["Hello ", "🙂 World ", "from Rust"]);
/// ```
///
/// Requesting more groups than words:
///
/// ```
/// use bt_string_utils::split_upto_n_by_word;
/// let s = "Hello 🙂 World";
/// let parts = split_upto_n_by_word(s, 10);
/// assert_eq!(parts, vec!["Hello ", "🙂 ", "World"]);
/// ```
///
/// Single group:
///
/// ```
/// use bt_string_utils::split_upto_n_by_word;
/// let s = "Hello world";
/// let parts = split_upto_n_by_word(s, 1);
/// assert_eq!(parts, vec!["Hello world"]);
/// ```
///
/// Empty input:
///
/// ```
/// use bt_string_utils::split_upto_n_by_word;
/// let parts = split_upto_n_by_word("", 5);
/// assert!(parts.is_empty());
/// ```
pub fn split_upto_n_by_word(s: &str, n: usize) -> Vec<&str> {
    // Detect word ranges: [start, end)
/*    let mut words = Vec::new();
    let mut in_word = false;
    let mut word_start = 0;

    for (i, ch) in s.char_indices() {
        if ch.is_whitespace() {
            if in_word {
                words.push((word_start, i));
                in_word = false;
            }
        } else if !in_word {
            in_word = true;
            word_start = i;
        }
    }
    if in_word {
        words.push((word_start, s.len()));
    }

    let total = words.len();
    if total == 0 {
        return Vec::new();
    }

    let parts = n.min(total);
    let mut out = Vec::with_capacity(parts);

    for i in 0..parts {
        let start_word = i * total / parts;
        let end_word = (i + 1) * total / parts - 1;

        // Start at the beginning of the first word in this group,
        // but include any whitespace *before* that word.
        let mut start = words[start_word].0;
        while start > 0 && s.as_bytes()[start - 1].is_ascii_whitespace() {
            start -= 1;
        }

        // End at the beginning of the next group's first word,
        // or at the end of the string for the last group.
        let end = if end_word + 1 < total {
            words[end_word + 1].0
        } else {
            s.len()
        };

        out.push(&s[start..end]);
    }

    out*/
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
