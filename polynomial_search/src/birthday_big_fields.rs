use crate::is_3_to_1::is_3_to_1;
use crate::math::{get_primitive_polynomial, multiplication, square_and_multiply};
use crate::get_coefficients::filter_coefficients;
use crate::birthday_small_fields::{mapping_cosets, add_tt, compute_power_table, print_tt, primitive_element_of_f4};
use rand::Rng;
use std::collections::HashSet;
use std::io;

fn evaluate_y(polynomial: Vec<(u32, u32)>, x: u32, primitive: u32, dimension: u32) -> u32 {
    let mut result = 0;

    for (c, e) in polynomial {
        let exponentiate_x = square_and_multiply(x, e, primitive, dimension);
        result ^= multiplication(c, exponentiate_x, primitive, dimension);
    }

    result
}

fn birthday_attack_for_big_fields(
    polynomial: Vec<(u32, u32)>,
    primitive: u32,
    dimension: u32,
    num_tests: u32,
) -> bool {
    let beta = primitive_element_of_f4(dimension, primitive);
    let beta_square = multiplication(beta, beta, primitive, dimension);
    let mut checked_elements= HashSet::new(); // Should contain x, x*beta, x*beta^2
    let mut observed_outputs = HashSet::new(); // Should contain y
    let mut iter = 0;
    let mut rng = rand::thread_rng();
    let mut x;

    while iter < num_tests && checked_elements.len() != ((1 << dimension) - 1) {
        loop {
            x = rng.gen_range(1..=(1 << dimension) - 1);
            if !checked_elements.contains(&x) {
                break;
            }
        }

        let y = evaluate_y(polynomial.clone(), x, primitive, dimension);

        if observed_outputs.contains(&y) || y == 0 {
            return false;
        }

        checked_elements.insert(x);
        checked_elements.insert(multiplication(beta, x, primitive, dimension));
        checked_elements.insert(multiplication(beta_square, x, primitive, dimension));
        observed_outputs.insert(y);

        iter += 1;
    }
    true
}

pub fn compute_tt_for_big_field(
    start_exp: u32,
    coeff: Vec<u32>,
    exp: Vec<u32>,
    num_terms: usize,
    dimension: u32,
    num_tests: u32,
    max_number_of_functions: u32
) -> io::Result<()> {
    let mut assigned_values = vec![(0, 0); num_terms];
    let mut functions_counter = 0;

    let primitive = get_primitive_polynomial(dimension as usize);

    assigned_values[0] = (1, start_exp);
    let mut cosetmap = vec![false; 1 << dimension];
    mapping_cosets(start_exp, dimension, &mut cosetmap);

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
            add_terms_for_big_field(
                start_exp,
                &coeff,
                &exp,
                num_terms,
                2,
                &mut assigned_values,
                primitive,
                dimension,
                index_of_e,
                num_tests,
                &mut cosetmap,
                same_coset,
                &mut false_positive,
                &mut functions_counter,
                max_number_of_functions
            )?;
        }
    }
    let mut _percentage = 0.0;
    let denominator = (false_positive + functions_counter) as f32;

    if false_positive == 0{
        _percentage = false_positive as f32;
    }
    else{
        _percentage = ((false_positive as f32) / denominator) * 100.000;
    }

    println!("Number of functions found by birthday attack: {}", denominator as u32);
    println!("Number of false positives: {}", false_positive);
    println!("Number of false positives in percent: {:.3}%", _percentage);
    Ok(())
}

fn add_terms_for_big_field(
    start_exp: u32,
    coeff: &Vec<u32>,
    exp: &Vec<u32>,
    num_terms: usize,
    count_terms: usize,
    assigned_values: &mut Vec<(u32, u32)>,
    primitive: u32,
    dimension: u32,
    index_of_last_exp: usize,
    num_tests: u32,
    coset_map: &mut Vec<bool>,
    belongs_to_same_coset: bool,
    false_positive: &mut u32,
    functions_counter: &mut u32,
    max_number_of_functions: u32
) -> io::Result<u32> { //-> io::Result<()>
    if count_terms == num_terms {
        let birthday = birthday_attack_for_big_fields(
            assigned_values.clone(),
            primitive,
            dimension,
            num_tests,
        );
        if birthday {
            let mut tt = Vec::new();
            compute_power_table(start_exp, primitive, dimension, &mut tt);
            // Starts at element number 2
            for (c, e) in assigned_values.iter().skip(1) {
                let mut tt_e = Vec::new();
                compute_power_table(*e, primitive, dimension, &mut tt_e);
                add_tt(&mut tt, &tt_e, *c, primitive, dimension);
            }
            if is_3_to_1(tt){
                if *functions_counter < max_number_of_functions{
                    // print_tt(&mut tt);
                    println!("{:?}", assigned_values);
                    *functions_counter += 1;
                }
            }
            else {
                *false_positive += 1;
            }
        }
        return Ok(*false_positive)
        // return Ok(());
    }

    for index_of_e in (index_of_last_exp + 1)..exp.len() {
        let e = exp[index_of_e];

        let new_exp_compare_to_coset = belongs_to_same_coset && coset_map[e as usize];

        if count_terms == num_terms && new_exp_compare_to_coset{
            continue;
        }

        for c in coeff {
            assigned_values[count_terms] = (*c, e);

            add_terms_for_big_field(
                start_exp,
                coeff,
                exp,
                num_terms,
                count_terms + 1,
                assigned_values,
                primitive,
                dimension,
                index_of_e,
                num_tests,
                coset_map,
                new_exp_compare_to_coset,
                false_positive,
                functions_counter,
                max_number_of_functions
            )?;
        }
    }
    Ok(*false_positive)
}

