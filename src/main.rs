use std::io;

fn main() {

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
            println!("WRONG CHOISE!");
            continue;
        }
    }
    
}
