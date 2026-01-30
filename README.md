# Project Title
BT STRING UTILS

## Description
Simple string utility to split and return parts of the string.

## Usage
```
let content_first = get_first_ocurrance("First:Second:Third",":");
let content = get_first_of_split("First:Second:Third",":");
```

## Version History
* 0.1.0
    * Initial Release
* 0.2.0
    * Add a function to find the value of a given key in a Vector of Strings with the form "key=value"
    * Breaking change: Change the name of the function due to a typo. get_first_occurrance
* 0.2.1
    * Add remove_char function to remove a character from the beginning or end of the string
* 0.2.2
    * Breaking change: Add Enum remove_location (Begin or End) for remove_char
* 0.2.3
    * Add a function to generate a random string of size n (generate_url_safe_string).
* 0.2.4 
    * Update dependencies. Add a function to find whole-word matches
* 0.2.5
    * Update dependencies.
* 0.2.6
    * Add function remove first n characters from a &str
* 0.2.7
    * Add function split_upto_n_by_word to evenly split a string into n groups by words
* 0.2.8
    * Function split_upto_n_by_word split now using also punctuation marks and whitespace that separate words are at the beginning of a group
* 0.2.9
    * Add function to remove Tags and the content between tags   
* 0.2.10
    * Add functions to count words and paragraphs.  
* 0.2.11
    * Add functions that splits a given string into multiple chunks of n bytes

## License
GPL-3.0-only