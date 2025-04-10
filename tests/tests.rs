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