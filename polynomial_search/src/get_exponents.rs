pub fn get_cyclotomic_exponents(n: u32) -> Vec<u32>{
    let mut get_potential_exponents = Vec::new();
    for i in 0..n{ //doesn't include n, so this is n-1
        for j in i+1..n{
            let number = 2u32.pow(i) + 2u32.pow(j);
            if number % 3 == 0 && !get_potential_exponents.contains(&number){
                get_potential_exponents.push(number);
            }
        }
    }

    let mut get_exponents= get_potential_exponents.clone();
    for i in &get_potential_exponents{
        for &j in &get_potential_exponents{
            if i == &j{
                continue;
            }
            if i*2 == j{
                get_exponents.retain(|&x| x != j);
            }
        }
    }
    get_exponents.sort();

    return get_exponents;
}

pub fn get_exponents(n: u32, avoid_exp: u32) -> Vec<u32>{
    let mut get_exponents = Vec::new();
    for i in 0..n{
        for j in i+1..n{
            let number = 2u32.pow(i) + 2u32.pow(j);
            if number % 3 == 0 && !get_exponents.contains(&number) && number != avoid_exp{
                get_exponents.push(number);
            }
        }
    }
    get_exponents.sort();
    return get_exponents;
}
