use std::io;

fn main(){
    println!("PALINDROME CHECKER");
    println!("Please enter a word or phrase to check if it's a palindrome:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let cleaned_input = clean_string(&input);

    if cleaned_input.is_empty(){
        println!("Please Enter a valid non-empty String");
        return;
    }

    if is_palindrome(&cleaned_input){
        println!("'{}' is a plaindrome",input.trim());
    }
    else{
        println!("'{}' id NOT a plaindrome",input.trim());
    }
}

fn clean_string(input: &str)->String {
    input
        .chars() // iterate over characters
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect::<String>() // collect into a new String
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}