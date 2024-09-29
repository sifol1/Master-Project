use crate::math::{multiplication, dot, primitive_element_of_f4};
use std::collections::HashSet;

pub fn orthoderivative(dimension: u32, tt: &[u32], primitive: u32) -> Vec<u32> {
    let beta = primitive_element_of_f4(dimension, primitive);
    let beta_square = multiplication(beta, beta, primitive, dimension);
    let mut od = vec![0; 1 << dimension];
    od[0] = 0;
    for a in 1..(1 << dimension){
        if od[a as usize] != 0{
            continue;
        }
        for possible_val in 1..(1 << dimension){
            let mut _problem = false;
            for x in 0..(1 << dimension){ 
                let derivative = tt[0] ^ tt[a] ^ tt[x] ^ tt[x ^ a];
                if dot(possible_val, derivative) {
                    _problem = true;
                    break;
                }
            }
            if !_problem {
                od[a as usize] = possible_val;
                od[multiplication(beta, a as u32, primitive, dimension) as usize] = possible_val;
                od[multiplication(beta_square, a as u32, primitive, dimension) as usize] = possible_val;
                break;
            }
        }
    }
    od
}

pub fn differential_spectrum(od: &[u32], dimension: u32) {
    let mut counts = vec![0; 1 << dimension];

    for a in 1..(1 << dimension) {
        let mut solutions = vec![0; 1 << dimension];
        for x in 0..(1 << dimension) {
            let output_diff = od[x] ^ od[x ^ a];
            solutions[output_diff as usize] += 1;
        }

        for num_output_diff in solutions {
            counts[num_output_diff] += 1;
        }
    }

    for (c, &count) in counts.iter().enumerate() {
        if count != 0 {
            print!("{}^^{}, ", c, count);
        }
    }
}

/*
The below is a different way to find the orthoderivative using the basis, but it's slower than the other method
*/

fn compute_basis(a: u32, dimension: u32, tt: &[u32], basis: &mut HashSet<u32>){
    let mut b: HashSet<u32> = HashSet::new(); // Contains the bases of the image set of delta_aF(x) and the linearly dependent vectors of these bases
    basis.clear();

    b.insert(0);

    for x in 0..(1 << dimension){
        let derivative = tt[0] ^ tt[a as usize] ^ tt[x] ^ tt[x ^ a as usize];

        if b.contains(&derivative){
            continue;
        }

        basis.insert(derivative);
        if basis.len() == dimension as usize - 1{
            break;
        }

        for key in b.clone(){
            b.insert(key ^ derivative);
        }

        b.insert(derivative);


    }

}

pub fn orthoderivative_with_basis(dimension: u32, tt: &[u32], primitive: u32) -> Vec<u32>{
    let beta = primitive_element_of_f4(dimension, primitive);
    let beta_square = multiplication(beta, beta, primitive, dimension);
    let mut od = vec![0; 1 << dimension];
    od[0] = 0;
    let mut bases = HashSet::new(); // Contains the bases of the image set of delta_aF(x)
    /*
    for i in 0..dimension{
        let basis_el = 1 << i;
        bases.insert(basis_el);
    }
     */
    for a in 1..(1 << dimension){
        if od[a as usize] != 0{
            continue;
        }
        // println!("a: {}", a);
        compute_basis(a, dimension, tt, &mut bases);
        // println!("length: {}", bases.len());
        // println!("basis: {:?}", bases);
        // break;
        for possible_val in 1..(1 << dimension){
            let mut problem = false;
            for basis_el in bases.clone(){
                // let derivative = tt[0] ^ tt[a as usize] ^ tt[basis_el] ^ tt[basis_el ^ a as usize];
                // println!("derivative: {}", derivative);
                if dot(possible_val, basis_el) { //returns true if 1, false if 0
                    problem = true;
                    break;
                }
            }
            if !problem{
                od[a as usize] = possible_val;
                od[multiplication(beta, a as u32, primitive, dimension) as usize] = possible_val;
                od[multiplication(beta_square, a as u32, primitive, dimension) as usize] = possible_val;
                break;
            }
        }
        if od[a as usize] == 0{
            eprintln!("Couldn't find any orthoderivative");
            break;
        }
    }
    od
}