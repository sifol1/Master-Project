use crate::is_3_to_1::is_3_to_1;
use crate::math::{get_primitive_polynomial};
use crate::get_coefficients::filter_coefficients;
use crate::birthday_small_fields::{mapping_cosets, add_tt, compute_power_table};
use std::io;

pub fn compute_tt(
    start_exp: u32,
    coeff: Vec<u32>,
    exp: Vec<u32>,
    num_terms: usize,
    dimension: u32,
) -> io::Result<()> {
    let mut assigned_values = vec![(0, 0); num_terms];

    let primitive = get_primitive_polynomial(dimension as usize);

    assigned_values[0] = (1, start_exp);
    let mut cosetmap = vec![false; 1 << dimension];
    mapping_cosets(start_exp, dimension, &mut cosetmap);

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
            add_terms(
                start_exp,
                &coeff,
                &exp,
                num_terms,
                2,
                &mut assigned_values,
                primitive,
                dimension,
                index_of_e,
                &mut cosetmap,
                same_coset,
            )?;
        }
    }

    Ok(())
}

fn add_terms(
    start_exp: u32,
    coeff: &Vec<u32>,
    exp: &Vec<u32>,
    num_terms: usize,
    count_terms: usize,
    assigned_values: &mut Vec<(u32, u32)>,
    primitive: u32,
    dimension: u32,
    index_of_last_exp: usize,
    coset_map: &mut Vec<bool>,
    belongs_to_same_coset: bool
) -> io::Result<()> { 
    if count_terms == num_terms {
        let mut tt = Vec::new();
        compute_power_table(start_exp, primitive, dimension, &mut tt);
        // Starts at element number 2
        for (c, e) in assigned_values.iter().skip(1) {
            let mut tt_e = Vec::new();
            compute_power_table(*e, primitive, dimension, &mut tt_e);
            add_tt(&mut tt, &tt_e, *c, primitive, dimension);
        }
            if is_3_to_1(tt){
                // print_tt(&mut tt);
                println!("{:?}", assigned_values);

            }
        
        return Ok(());
    }

    for index_of_e in (index_of_last_exp + 1)..exp.len() {
        let e = exp[index_of_e];

        let new_exp_compare_to_coset = belongs_to_same_coset && coset_map[e as usize];

        if count_terms == num_terms && new_exp_compare_to_coset{
            continue;
        }

        for c in coeff {
            assigned_values[count_terms] = (*c, e);
            add_terms(
                start_exp,
                coeff,
                exp,
                num_terms,
                count_terms + 1,
                assigned_values,
                primitive,
                dimension,
                index_of_e,
                coset_map,
                new_exp_compare_to_coset,
            )?;
        }
    }
    Ok(())
}

