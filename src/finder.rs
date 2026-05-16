use regex::Regex;

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
/// use bt_string_utils::finder::get_first_occurrance;
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

/// Checks whether a given string contains the specified `word`
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
/// use bt_string_utils::finder::contains_whole_word;
/// assert_eq!(contains_whole_word("this is a target match", "target"), true);
/// assert_eq!(contains_whole_word("this is a targeted match", "target"), false);
/// assert_eq!(contains_whole_word("no-target", "target"), false);
/// ```
pub fn contains_whole_word(text: &str, word: &str) -> bool {
    let pattern = format!(r"(?:^|[^A-Za-z0-9-]){}(?:[^A-Za-z0-9-]|$)", regex::escape(word));    

    let re = Regex::new(&pattern).unwrap();
    re.is_match(text)
}

/// Returns a UTF-8 safe slice containing the first `n` characters of `s`.
/// If `s` contains fewer than `n` characters, the entire string is returned.
/// # Arguments
///
/// * `s` - The input string slice.
/// * `n` - The number of Unicode characters to include.
///
/// # Returns
///
/// A `&str` slice containing at most the first `n` Unicode characters.
/// 
/// ```
/// use bt_string_utils::finder::get_first_n_chars; 
/// assert_eq!(get_first_n_chars("hello world", 5), "hello");
/// assert_eq!(get_first_n_chars("héllo", 2), "hé");
/// assert_eq!(get_first_n_chars("short", 20), "short");
/// ```
pub fn get_first_n_chars(s: &str, n: usize) -> &str {
    match s.char_indices().nth(n) {
        Some((idx, _)) => &s[..idx],
        None => s,
    }
}

/// Extracts the first letter of every word in a string and returns
/// the collected initials in uppercase.
///
/// Words are defined as sequences of non‑whitespace characters.
/// Leading, trailing, and repeated whitespace are ignored.
///
/// # Examples
///
/// ```
/// use bt_string_utils::finder::initials_uppercase;
/// let result = initials_uppercase("Rust language is fast");
/// assert_eq!(result, "RLIF");
/// ```
///
/// ```
/// use bt_string_utils::finder::initials_uppercase;
/// let result = initials_uppercase("  multiple   spaces   here ");
/// assert_eq!(result, "MSH");
/// ```
///
/// ```
/// use bt_string_utils::finder::initials_uppercase;
/// let result = initials_uppercase("");
/// assert_eq!(result, "");
/// ```
pub fn initials_uppercase(input: &str) -> String {
    input
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_uppercase()
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
/// use bt_string_utils::finder::find_value_by_key;
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