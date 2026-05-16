use std::io;

fn main(){
    println!("Prime Number checker");
    println!("Enter a Positive integer to check it is Prime:");

    let num = match get_input_as_u32(){
        Some(value) => value,
        None => {
            println!("Invalid Input");
            return;
        }
    };

    if num <= 1 {
        println!("Number must be greater than 1");
        return;
    }

    if is_prime(num){
        println!("{} is a Prime Number",num);
    }
    else{
        println!("{} is NOT as Prime Number",num);
    }

}


fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");

    match input.trim().parse::<u32>(){
        Ok(value) => Some(value),
        Err(_) => None
    }
}

fn is_prime(n: u32)-> bool{
    if n<= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }

    let limit = (n as f64).sqrt() as u32 + 1;

    for i in 2..limit {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}