#[cfg(test)]
mod sub_strings_test{
    use bt_string_utils::{get_first_occurrance, get_first_of_split};

    #[test]
    fn test_first_occurance(){
        let content = get_first_occurrance("First:Second:Third",":");
        println!("Content {:?}",&content);
        assert_eq!(content,"First");
    }

    #[test]
    fn test_first_occurance_no_first(){
        let content = get_first_occurrance("First:Second:Third","*");
        println!("Content {:?}",&content);
        assert_eq!(content,"");
    }

    #[test]
    fn test_first_split(){
        let content = get_first_of_split("First:Second:Third",":");
        println!("Content {:?}",&content);
        assert_eq!(content, ("First".to_owned(),"Second:Third".to_owned()));
    }

    #[test]
    fn test_first_split_no_split(){
        let content = get_first_of_split("First:Second:Third","*");
        println!("Content {:?}",&content);
        assert_eq!(content, ("First:Second:Third".to_owned(),"".to_owned()));
    }
}

//**************/
//UNIT TEST
//*************/
#[cfg(test)]
mod strings_test{
    use bt_string_utils::find_value_by_key;

    #[test]
    fn find_value_by_key_exist_test(){
        let v = vec!["k1=a".to_owned(),"k2=b".to_owned(),"k3=c".to_string()];
        assert_eq!(find_value_by_key(&v, "k3").unwrap(),"c");
    }

    #[test]
    fn find_value_by_key_nofound_test(){
        let v = vec!["k1=a".to_owned(),"k2=b".to_owned(),"k3=c".to_string()];
        assert_eq!(find_value_by_key(&v, "k5"),None);
    }    
}

#[cfg(test)]
mod removed_tests {
    use bt_string_utils::{remove_char, RemoveLocationEnum};

    #[test]
    fn test_remove_first_char() {
        assert_eq!(remove_char(RemoveLocationEnum::Begin, &"hello".to_string(), 'h'), "ello");
        assert_eq!(remove_char(RemoveLocationEnum::Begin, &"rust".to_string(), 'r'), "ust");
    }

    #[test]
    fn test_remove_last_char() {
        assert_eq!(remove_char(RemoveLocationEnum::End, &"world!".to_string(), '!'), "world");
        assert_eq!(remove_char(RemoveLocationEnum::End, &"test".to_string(), 't'), "tes");
    }

    #[test]
    fn test_no_removal() {
        assert_eq!(remove_char(RemoveLocationEnum::Begin, &"rust".to_string(), 'x'), "rust");
        assert_eq!(remove_char(RemoveLocationEnum::End, &"mars".to_string(), 'z'), "mars");
    }
}

#[cfg(test)]
mod rand_string_tests {
    use bt_string_utils::generate_url_safe_string;

    #[test]
    fn test_generate_string_length() {
        let length = 16;
        let result = generate_url_safe_string(length);
        assert_eq!(result.len(), length, "Generated string should be {} characters long", length);
    }

    #[test]
    fn test_generate_string_is_alphanumeric() {
        let result = generate_url_safe_string(20);
        assert!(result.chars().all(|c| c.is_ascii_alphanumeric()), "Generated string should contain only alphanumeric characters");
        assert_eq!(result.len(),20);
    }

    #[test]
    fn test_generate_string_with_zero_length() {
        let result = generate_url_safe_string(0);
        assert_eq!(result.len(), 0, "Generated string for length 0 should be an empty string");
    }

    #[test]
    fn test_generate_string_uniqueness() {
        let result1 = generate_url_safe_string(10);
        let result2 = generate_url_safe_string(10);
        assert_ne!(result1, result2, "Two generated strings should be different");
    }

}
