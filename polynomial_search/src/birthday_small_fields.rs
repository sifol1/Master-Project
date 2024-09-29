use crate::get_coefficients::filter_coefficients;
use crate::math::{get_primitive_polynomial, multiplication, square_and_multiply};
use crate::is_3_to_1::is_3_to_1;
use rand::seq::IteratorRandom;
use std::collections::HashSet;
use std::io;

/*
Returns a primitive elment of F4, which is always a subfield of an even dimension
*/
pub fn primitive_element_of_f4(dimension: u32, primitive: u32) -> u32 {
    let exponent = (2u32.pow(dimension) - 1) / 3;
    let beta = square_and_multiply(2, exponent, primitive, dimension);

    beta
}

fn birthday_attack_for_small_fields(tt: Vec<u32>, primitive: u32, dimension: u32) -> bool {
    let beta = primitive_element_of_f4(dimension, primitive);
    let beta_square = multiplication(beta, beta, primitive, dimension);
    let mut remaining_elements: HashSet<u32> = HashSet::new();
    let mut observed_outputs = HashSet::new();

    // Finds all the possible elements from 1 - 2^n
    for value in 1..(1 << dimension) {
        remaining_elements.insert(value);
    }

    while !remaining_elements.is_empty() {
        let mut _x = 0;

        // Chooses a random element x from the possible elements
        if let Some(element) = remaining_elements.iter().choose(&mut rand::thread_rng()) {
            _x = *element;
        } else {
            println!("Cannot find a random element in the set of remaining elements");
            return false;
        }

        // Finds the output y
        let y = tt[_x as usize];

        // If you've already found y or y = 0, return false
        if observed_outputs.contains(&y) || y == 0 {
            return false;
        }

        // Only x, beta*x and beta^2*x are the ones that have the same output y (they are the 3 in 3-to-1)
        remaining_elements.remove(&_x);
        remaining_elements.remove(&multiplication(beta, _x, primitive, dimension));
        remaining_elements.remove(&multiplication(beta_square, _x, primitive, dimension));
        observed_outputs.insert(y);
    }
    return true;
}

pub fn mapping_cosets(mut e: u32, dimension: u32, coset_map: &mut Vec<bool>){
    let m = (1 << dimension) - 1;

    for _ in 0..dimension{
        coset_map[e as usize] = true;
        e = (e * 2) % m;
    }
}

pub fn compute_tt_for_small_field(
    start_exp: u32,
    coeff: Vec<u32>,
    exp: Vec<u32>,
    num_terms: usize,
    dimension: u32,
    max_number_of_functions: u32
) -> io::Result<()> {
    let mut assigned_values = vec![(0, 0); num_terms];
    let mut functions_counter = 0;

    let primitive = get_primitive_polynomial(dimension as usize);

    let mut tt = Vec::new();
    compute_power_table(start_exp, primitive, dimension, &mut tt);

    assigned_values[0] = (1, start_exp);

    let mut cosetmap = vec![false; 1 << dimension]; 
    mapping_cosets(start_exp, dimension, &mut cosetmap);  

    // Exp_table contains all monomials with exponentials from exp
    let mut exp_table = vec![vec![0; 1 << dimension]; exp.len()];
    for i in 0..exp.len() {
        // Create a vector for each row
        let mut row = Vec::new();
        // Compute the power table for the current exponent
        compute_power_table(exp[i], primitive, dimension, &mut row);
        exp_table[i] = row;
    }

    let mut false_positive = 0;
    let mut same_coset: bool;  

    for index_of_e in 0..exp.len() {
        if num_terms == 1{
            break;
        }
        let e = exp[index_of_e];
        same_coset = cosetmap[e as usize];  

        if num_terms == 2 && same_coset{ 
            continue; 
        } 

        let mut filter_coeff = Vec::new(); 
        filter_coefficients(start_exp, e, primitive, dimension, coeff.clone(), &mut filter_coeff); 
        for c in filter_coeff.iter() { 

            assigned_values[1] = (*c, e);

            add_tt(&mut tt, &exp_table[index_of_e], *c, primitive, dimension);

            add_terms_for_small_field(
                start_exp,
                &coeff,
                &exp,
                num_terms,
                2,
                &mut assigned_values,
                &exp_table,
                &mut tt,
                primitive,
                dimension,
                index_of_e,
                &mut cosetmap, 
                same_coset,
                &mut false_positive,
                &mut functions_counter,
                max_number_of_functions
            )?;

            add_tt(&mut tt, &exp_table[index_of_e], *c, primitive, dimension);
        }
    }

    println!("Number of false positives: {}", false_positive);
    Ok(())
}

fn add_terms_for_small_field(
    start_exp: u32,
    coeff: &Vec<u32>,
    exp: &Vec<u32>,
    num_terms: usize,
    count_terms: usize,
    assigned_values: &mut Vec<(u32, u32)>,
    exp_table: &Vec<Vec<u32>>,
    tt: &mut Vec<u32>,
    primitive: u32,
    dimension: u32,
    index_of_last_exp: usize,
    coset_map: &mut Vec<bool>, 
    belongs_to_same_coset: bool,
    false_positive: &mut u32,
    functions_counter: &mut u32,
    max_number_of_functions: u32
) -> io::Result<u32> { //-> io::Result<()>
    if count_terms == num_terms {
        let birthday = birthday_attack_for_small_fields(tt.clone(), primitive, dimension);
        if birthday {
            if is_3_to_1(tt.clone()){
                if *functions_counter < max_number_of_functions{
                    // print_tt(tt);
                    println!("{:?}", assigned_values);
                    *functions_counter += 1;
                }
            }
            else {
                *false_positive += 1;
            }
        }
        return Ok(*false_positive);
    }

    for index_of_e in (index_of_last_exp + 1)..exp.len() {
        let e = exp[index_of_e];

        let new_exp_compare_to_coset = belongs_to_same_coset && coset_map[e as usize]; 
        if count_terms == num_terms && new_exp_compare_to_coset{ 
            continue; 
        } 

        for c in coeff {
            assigned_values[count_terms] = (*c, e);

            add_tt(tt, &exp_table[index_of_e], *c, primitive, dimension);
            add_terms_for_small_field(
                start_exp,
                coeff,
                exp,
                num_terms,
                count_terms + 1,
                assigned_values,
                exp_table,
                tt,
                primitive,
                dimension,
                index_of_e,
                coset_map, 
                new_exp_compare_to_coset,
                false_positive,
                functions_counter,
                max_number_of_functions
            )?;
            add_tt(tt, &exp_table[index_of_e], *c, primitive, dimension);
        }
    }
    Ok(*false_positive)
}

pub fn print_tt(tt: &mut Vec<u32>){
    println!("{:?}", tt);
}

pub fn compute_power_table(exp: u32, primitive: u32, dimension: u32, power_table: &mut Vec<u32>) {
    *power_table = vec![0; 1 << dimension];
    for x in 0..(1 << dimension) {
        let v = square_and_multiply(x, exp, primitive, dimension);
        power_table[x as usize] = v;
    }
}

pub fn add_tt(dest: &mut Vec<u32>, src: &Vec<u32>, c: u32, primitive: u32, dimension: u32) {
    for x in 0..dest.len() {
        let multiply = multiplication(src[x], c, primitive, dimension);
        dest[x] ^= multiply;
    }
}
 
