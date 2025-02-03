use bt_string_utils::{get_first_ocurrance, get_first_of_split};

#[test]
fn test_first_occurance(){
    let content = get_first_ocurrance("First:Second:Third",":");
    println!("Content {:?}",&content);
    assert_eq!(content,"First");
}

#[test]
fn test_first_occurance_no_first(){
    let content = get_first_ocurrance("First:Second:Third","*");
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