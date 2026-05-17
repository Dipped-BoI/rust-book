use std::io;

fn main() {
    
    let choice:u32 = ask_for_choice("Enter which operation you want to do: 0 for C to F, 1 for F to C ");

    let mut temp:f32 = ask_for_temp("Enter temperature to change");

    temp = if choice==0 {
        temp*9.0/5.0 + 32.0
    } else {
        (temp-32.0)*5.0/9.0
    };

    println!("Your desired temperature is {temp:.2}")
}

fn ask_for_choice(prompt:&str) -> u32 {
    loop{
        println!("{}",prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }

}
    
fn ask_for_temp(prompt:&str) -> f32 {
    loop{
        println!("{}",prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
    
}

