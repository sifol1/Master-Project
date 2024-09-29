use crate::math::{get_primitive_polynomial, square_and_multiply, multiplication};

pub fn get_coefficients(n: u32, m: u32) -> Vec<u32> {
    let mut get_coefficients: Vec<u32> = Vec::new();
    let primitive = get_primitive_polynomial(n as usize);

    // Checks whether vector^(2^m) == vector
    for vector in 1..2u32.pow(n) {
        let exponent = 2u32.pow(m); // 2^m
        if square_and_multiply(vector, exponent, primitive, n) == vector {
            get_coefficients.push(vector);
        }
    }

    return get_coefficients;
}


 pub fn filter_coefficients(e1: u32, e2: u32, primitive: u32, dimension: u32, coefficients: Vec<u32>, filtered_coeff: &mut Vec<u32>){

    let mut coeffs = vec![false; 1 << dimension];

    // Sets all the values that is in coefficients to true in coeffs
    for index in coefficients {
        coeffs[index as usize] = true;
    }

    // Remove cyclotomic coset
    for x in 1..(1 << dimension){
        if coeffs[x]{
            for j in 1..dimension{
                let v = square_and_multiply(x as u32, 1 << j, primitive, dimension);
                coeffs[v as usize] = false; 
            }
            coeffs[x] = true;
        }
    }

    // Remove multipliers
    let mut multipliers = vec![false; 1 << dimension];
    let exponent_diff = if e1 > e2 { e1 - e2 } else { e2 - e1 }; // The abs of e2 - e1
    for x in 1..(1 << dimension){ // 1 to 2^dim
        let m = square_and_multiply(x, exponent_diff, primitive, dimension); // m = x^exponent_diff
        multipliers[m as usize] = true;
        for i in 1..dimension{
            let powers_of_m = square_and_multiply(m, 1 << i, primitive, dimension);
            multipliers[powers_of_m as usize] = true;
        }
    }
    
    for x in 1..(1 << dimension){
        if coeffs[x]{
            for m in 1..(1 << dimension){
                if multipliers[m]{
                    let v = multiplication(x as u32, m as u32, primitive, dimension);
                    coeffs[v as usize] = false;
                }
            }
            coeffs[x] = true;
        }
    }

    // Combining the coset and multiplier filtering, so that coeffs[x^(2^i)*m^(2^j)] = false
    for x in 1..(1 << dimension){
        if coeffs[x]{
            for i in 1..dimension{
                let v = square_and_multiply(x as u32, 1 << i, primitive, dimension);
                for m in 1..(1 << dimension){
                    if multipliers[m]{
                        let power_of_x_times_m = multiplication(v, m as u32, primitive, dimension);
                        coeffs[power_of_x_times_m as usize] = false;
                    }
                }
            }
            coeffs[x] = true;
        }
    }

    for x in 1..(1 << dimension){
        if coeffs[x]{
            filtered_coeff.push(x as u32);
        }
    }

 }



