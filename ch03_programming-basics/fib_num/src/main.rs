use std::io;

fn main() {
    
    let n = get_n("Enter Fibonacci index");
    
    let n_th_fibonacci = get_fibonacci(n);
    
    println!("The required fibonacci number is {n_th_fibonacci}");


}

fn get_n(prompt: &str) -> u32 {

    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not readline");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn get_fibonacci(index: u32) -> u32 {

    let mut main_num = 1;
    let mut helper_num = 1;
    let mut main_num_copy = 1;
    
    if index == 1 || index == 2 {
    
    } else {
        for _ in 3..=index {
            main_num_copy = main_num;
            main_num+=helper_num;
            helper_num = main_num_copy;

        }
    }
    return main_num;
}
