use std::io;

fn main(){
    println!("Fibonnaci Sequence Generator");
    println!("Ennter thr number of terms you want to genetate");

    let num_terms = match get_input_as_u32(){
        Some(value) => value,
        None => {
            println!("Invalid input please enter a Positive number");
            return;
        }
    };

    if num_terms == 0{
        println!("Number of terms must be greater than zero")
    }

    let sequence = genetate_fibonnaci(num_terms);
    println!("Fibinnaci Sequence ({} terms): {:?}",num_terms,sequence);

}

fn get_input_as_u32() ->Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    match input.trim().parse::<u32>(){
        Ok(value) => Some(value),
        Err(_) => None
    }
}

fn genetate_fibonnaci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();
    if n >= 1 {
        sequence.push(0);
    }

    if n>=2 {
        sequence.push(1);
    }

    for i in 2..n {
        let next = sequence[i as usize -1] + sequence[i as usize -2];
        sequence.push(next);
    }

    return sequence;
}