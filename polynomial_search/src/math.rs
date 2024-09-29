use num_bigint::BigUint;
use num_traits::Zero;

pub fn multiplication(mut a: u32, mut b: u32, primitive: u32, dimension: u32) -> u32 {
    let mut result = 0;

    let mut cutoff = 1;
    for _ in 0..(dimension - 1){
        cutoff <<= 1;
    }

    while a != 0 && b != 0{
        if (b & 1) != 0{ // If lsb == 1
            result ^= a;
        }
        
        if (a & cutoff) != 0{ // If msb == 1
            a = (a << 1) ^ primitive;
        }
        else {
            a <<= 1;
        }
        b >>= 1;
    }
    result
}

pub fn square_and_multiply(original_vector: u32, exponent: u32, irr: u32, dimension: u32) -> u32 {
    // Convert u32 to BigUint as a binary representation
    let biguint_number = BigUint::from(exponent);

    let to_string = biguint_number.to_str_radix(2);

    // Convert binary string to a list of u32 bits
    let mut list_of_bits: Vec<u32> = to_string
        .chars()
        .map(|c| c.to_digit(2).unwrap()) // Convert char to u32
        .collect();

    // Remove first bit
    if !list_of_bits.is_empty() {
        list_of_bits.remove(0);
    }

    // Set y = x
    let mut result_vector = original_vector;

    // Square and multiply
    for bit in list_of_bits {
        // Square if elem == 0
        if bit.is_zero() {
            result_vector = multiplication(result_vector, result_vector, irr, dimension);
        }
        // First square, and then multiply with original vector
        else {
            let temp_vector: u32 = multiplication(result_vector, result_vector, irr, dimension);
            result_vector = multiplication(temp_vector, original_vector, irr, dimension);
        }
    }
    return result_vector;
}

pub fn get_primitive_polynomial(dimension: usize) -> u32 {
    assert!(dimension >= 2);
    assert!(dimension <= 20);

    // let primitive_polys: [u32; 19] = [3, 6, 12, 20, 48, 96, 184, 272, 576, 1280, 3232, 6912, 12448, 24576, 53256, 73728, 132096, 462848, 589824];

    /* The one with one extra bit */
    let primitive_polys: [u32; 19] = [7, 11, 19, 37, 91, 131, 285, 529, 1135, 2053, 4331, 8219, 16553, 32821, 65581, 131081, 267267, 524327, 1050355];
    return primitive_polys[dimension - 2];
}
