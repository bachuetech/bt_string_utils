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
/// use bt_string_utils::cleanser::{remove_char, RemoveLocationEnum};
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
/// use bt_string_utils::cleanser::{remove_char, RemoveLocationEnum};
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
/// use bt_string_utils::cleanser::remove_first_n_characters;
/// ```
/// Remove first 3 characters
/// ```
/// use bt_string_utils::cleanser::remove_first_n_characters;
/// let result = remove_first_n_characters("Hello, World!", 3);
/// assert_eq!(result, "lo, World!");  // 'Hel' removed
/// ```
/// Remove more characters than exist
/// ```
/// use bt_string_utils::cleanser::remove_first_n_characters;
/// let result = remove_first_n_characters("Hi", 5);
/// assert_eq!(result, "");  // Empty string returned
/// ```
/// Handle Unicode characters
/// ```
/// use bt_string_utils::cleanser::remove_first_n_characters;
/// let result = remove_first_n_characters("🌟Hello", 1);
/// assert_eq!(result, "Hello");  // The emoji (2 bytes) is properly skipped
/// ```
pub fn remove_first_n_characters(s: &str, n: usize) -> &str {
    let byte_index = s.char_indices().nth(n).map(|(i, _)| i).unwrap_or(s.len());
    &s[byte_index..]
}

/// Removes all `<custom> ... </custom>` sections from the input string,
/// returning a new `String` with the tags and their contents removed.
/// # Parameters
///
/// * `s: &str`  
///   The input string to process.  
///   This may contain zero, one, or multiple `<custom> ... </custom>`
///   tag pairs. The function treats tags literally and does not interpret
///   nested or malformed markup.
/// * `open_tag: &str` The open tag (e.g. <custom>).  `<custom> ... </custom>`
/// 
/// * `close_tag: &str` The close tag (e.g. </custom>). `<custom> ... </custom>`
/// 
/// # Behavior
/// - Every occurrence of `<custom>` starts a removal region.
/// - The function removes everything from `<custom>` up to and including
///   the next `</custom>` tag.
/// - If a `<custom>` tag appears **without** a matching `</custom>`,
///   the function removes the `<custom>` tag and **drops the remainder**
///   of the string.
/// - Text outside of `<custom> ... </custom>` is preserved exactly,
///   including whitespace and UTF‑8 characters.
/// - The first closing tag always ends the removal region.
///
/// # Returns
///
/// A new `String` containing the original text with all `<custom>`
/// sections removed.
///
/// # Performance
///
/// # Examples
///
/// ## Basic removal
/// ```
/// use bt_string_utils::cleanser::remove_tags;
/// let result = remove_tags("Hello <custom>secret</custom> world", "<custom>","</custom>");
/// assert_eq!(result, "Hello  world");
/// ```
///
/// ## Multiple tags
/// ```
/// use bt_string_utils::cleanser::remove_tags;
/// let result = remove_tags("1 <custom>a</custom> 2 <custom>b</custom> 3", "<custom>","</custom>");
/// assert_eq!(result, "1  2  3");
/// ```
///
/// ## Missing closing tag
/// ```
/// use bt_string_utils::cleanser::remove_tags;
/// let result = remove_tags("before <custom>unfinished", "<custom>","</custom>");
/// assert_eq!(result, "before ");
/// ```
///
/// ## UTF‑8 characters preserved
/// ```
/// use bt_string_utils::cleanser::remove_tags;
/// let result = remove_tags("héllo <custom>x</custom> wørld 🌍", "<custom>","</custom>");
/// assert_eq!(result, "héllo  wørld 🌍");
/// ```
///
/// ## No tags present
/// ```
/// use bt_string_utils::cleanser::remove_tags;
/// let result = remove_tags("nothing to remove", "<custom>","</custom>");
/// assert_eq!(result, "nothing to remove");
/// ```
pub fn remove_tags(text: &str, open_tag: &str, close_tag: &str) -> String {
    //const OPEN: &str = "<custom>";
    //const CLOSE: &str = "</custom>";

    let mut out = String::with_capacity(text.len());
    let text_bytes = text.as_bytes();
    let mut i = 0;

    while i < text_bytes.len() {
        // Check for opening tag
        if text_bytes[i..].starts_with(open_tag.as_bytes()) {
            // Found an opening tag
            i += open_tag.len();

            // We are inside a removal region; support nesting
            let mut depth = 1;
            while i < text_bytes.len() && depth > 0 {
                if text_bytes[i..].starts_with(open_tag.as_bytes()) {
                    depth += 1;
                    i += open_tag.len();
                } else if text_bytes[i..].starts_with(close_tag.as_bytes()) {
                    depth -= 1;
                    i += close_tag.len();
                } else {
                    // Just advance by one byte inside the removed region
                    i += 1;
                }
            }

            // If depth > 0 here, there was no matching closing tag:
            // we just drop the remainder.
            if depth > 0 {
                break;
            }

        } else {
            // Copy one UTF‑8 character safely
            let ch = text[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }

    }
    out
}

/// Replaces all Unicode whitespace characters in the given string slice
/// with the specified replacement character.
///
/// This function treats any character for which `char::is_whitespace()`
/// returns `true` as whitespace. That includes spaces, tabs, newlines,
/// and a variety of Unicode whitespace characters.
///
/// # Arguments
///
/// * `input` - The string slice to process.
/// * `replacement` - The character that will replace each whitespace character.
///
/// # Returns
///
/// A new `String` where every whitespace character has been replaced
/// by the provided `replacement` character.
///
/// # Examples
///
/// ```
/// use bt_string_utils::cleanser::replace_whitespace;
/// let result = replace_whitespace("Hello   world\nRust!", '_');
/// assert_eq!(result, "Hello___world_Rust!");
/// ```
pub fn replace_whitespace(input: &str, replacement: char) -> String {
    input
        .chars()
        .map(|c| if c.is_whitespace() { replacement } else { c })
        .collect()
}

///Removed any no visible UTF-8 character from string
pub fn remove_whitespace(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    out.extend(input.chars().filter(|c| { 
        if c.is_whitespace() || c.is_control() {return false}

        let u = *c as u32;
        // Remove zero-width characters
        if matches!(u,
            0x200B | // zero-width space
            0x200C | // zero-width non-joiner
            0x200D | // zero-width joiner
            0xFEFF   // zero-width no-break space (BOM)
        ) { return false }

        // Remove soft hyphen
        if u == 0x00AD { return false;  }

        // Remove Private Use Area characters
        if (0xE000..=0xF8FF).contains(&u)
            || (0xF0000..=0xFFFFD).contains(&u)
            || (0x100000..=0x10FFFD).contains(&u)
        { return false;  }

        true
    })); 
    out
}