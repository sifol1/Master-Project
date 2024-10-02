mod math;
mod get_exponents;
mod get_coefficients;
mod is_3_to_1;
mod birthday_big_fields;
mod birthday_small_fields;
mod three_to_one;

use get_coefficients::get_coefficients;
use get_exponents::get_exponents;
use chrono::{Local, DateTime};
// use core::num;
use std::env;


fn main() -> Result<(), String>{
    let args: Vec<String> = env::args().collect();
    
    
    let n:  u32 = args.get(1).map_or(6, |s| s.parse().unwrap_or(6)); // Default is 6
    let m: u32 = args.get(2).map_or(6, |s| s.parse().unwrap_or(6)); // Default is 6
    let e: u32 = args.get(3).map_or(3, |s| s.parse().unwrap_or(3)); // Default is 3
    let k: usize = args.get(4).map_or(3, |s| s.parse().unwrap_or(3)); // Default is 3
    let number_of_tests: u32 = args.get(5).map_or(700, |s| s.parse().unwrap_or(700)); // Default is 700
    let max_number_of_functions: u32 = args.get(6).map_or(1200000, |s| s.parse().unwrap_or(1200000)); // Default is 1,200,000

    if n % m != 0{
        return Err("Invalid value: m must be a divisor of n".to_string())
    }

    if m > 20 || n > 20{ //m == 1 ||
        return Err("The field/subfield cannot be 1 or higher than 20, it has to be at least 2".to_string());
    }
    
    let coefficients = get_coefficients(n, m);
    let exponents = get_exponents(n, e);

    // println!("Coeff: {:?}", coefficients);
    // println!("Exp: {:?}", exponents);

    if n < 10{
        let start_time = Local::now();
        println!("Start time {}", format_asctime(start_time));
        let _ = birthday_small_fields::compute_tt_for_small_field(e, coefficients, exponents, k, n, max_number_of_functions);
        let end_time = Local::now();
        println!("End time {}", format_asctime(end_time));
    }
    else{
        let start_time = Local::now();
        println!("Start time: {}", format_asctime(start_time));
        let _ = birthday_big_fields::compute_tt_for_big_field(e, coefficients, exponents, k, n, number_of_tests, max_number_of_functions);
        let end_time = Local::now();
        println!("End time: {}", format_asctime(end_time));
    }
    
    /* 
    Run the original triplicate method
    */
    // let start_time = Local::now();
    // println!("Start time: {}", format_asctime(start_time));
    // let _ = three_to_one::compute_tt(e, coefficients, exponents, k, n);
    // let end_time = Local::now();
    // println!("End time: {}", format_asctime(end_time));
    
     
    Ok(())
}
 
fn format_asctime(time: DateTime<Local>) -> String {
    let naive_time = time.naive_local();
    let formatted_time = format!("{}", naive_time.format("%a %b %e %T %Y"));
    formatted_time
}