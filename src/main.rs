use std::io;

fn main() {
    println!("Welcome to FarenCel!");
    let opt = get_opt();
    println!("Enter the temperatue:");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("error while getting temperatue.");
    let temp:f64 = temp.trim().parse().expect("ENTER NUMBERS ONLY PLEASE!");

    if opt == 1 {
        println!("Result: {} Degree Celsius.", f_to_c(temp));
    } else {
        println!("Result: {} Degree Fahrenheit.", c_to_f(temp));
    }
}

fn f_to_c(temp:f64) -> f64{
    (temp - 32.0) * (5.0/9.0)
}
fn c_to_f(temp:f64) -> f64{
    (temp * (9.0/5.0)) + 32.0
}

fn get_opt() -> u8 {
    println!("Please choose:");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    loop {
        let mut opt = String::new();
        io::stdin().read_line(&mut opt)
            .expect("error occured.");
        let opt:u8 = opt.trim().parse().expect("ENTER NUMBERS ONLY!");
        if opt == 1 {
            return 1;
        } else if opt == 2 {
            return 2;
        } else {
            println!("WRONG CHOICE!\ntry again.");
            continue;
        }
    }
    
}
