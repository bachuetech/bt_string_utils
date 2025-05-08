//! Multiple String related functions

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
/// use bt_string_utils::remove_char;
/// let modified = remove_char(true, "hello".to_string(), 'h');
/// assert_eq!(modified, "ello");
///
/// let modified = remove_char(false, "world!".to_string(), '!');
/// assert_eq!(modified, "world");
/// ```
///
/// If the character doesn't match, the original string is returned:
///
/// ```
/// use bt_string_utils::remove_char;
/// let modified = remove_char(true, "rust".to_string(), 'x');
/// assert_eq!(modified, "rust");
/// ```
pub fn remove_char(begin: bool, input: String, target: char) -> String {
    if begin {
        if input.starts_with(target) {
            return input.chars().skip(1).collect();
        }
    } else {
        if input.ends_with(target) {
            return input.chars().take(input.len() - 1).collect();
        }
    }
    input // Return unchanged if no removal occurs
}
