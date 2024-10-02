# Master-Project

This is the official code and results for Silje Folkestad's Master thesis at University of Bergen, 2024.

## Overview

### polynomial_search
The folder polynomial_search/src contains the code for doing a polynomial search over any field. It finds quadratic triplicate APN functions. It uses an approach called triplicate birthday attack, which is a probabilistic test. It tests whether a function is 3-to-1 based on *T* tests. If the triplicate birthday attack assumes that the function is 3-to-1, it uses the original 3-to-1 approach to verify.

Upon running the code, you write args commands referring to *dimension, subfield, start exponent, number of terms, number of tests*, and *maximum number of functions*. The default values are 6, 6, 3, 3, 700 and 2,000,000.

If the dimension is lower than 10, it will *birthday_small_fields.rs*, while using dimension 10 or higher will run *birthday_big_fields.rs*. The difference is in how they go through checking whether they are 3-to-1. birthday_small_fields go through the entire field, only picking random elements and keeping track of all the triplicates it has found. birthday_big_fields uses *T* tests; it picks *T* random elements, and keeps track of all the triplicates it has found.

You could also use this code to run the original triplicate method. Right now it is written as a comment inside *main.rs*, so to be able to run it you have to uncomment it, and comment out the if-else above. It uses the same args methods as described above. 

The output of this code is quadratic triplicate APN functions of the specific dimension. Because of storage space, the functions are printed out as univariate polynomials on the form [(coeff1, exp1), (coeff2, exp2), ...].

### orthoderivative_diff_spectrum
The folder orthoderivative_diff_spectrum/src contains the code for calculating the orthoderivative differential spectrum of a function. 

This code takes a file containing functions as an input, where the default is *default.o*. In order for the code to read the file, the file's first line has to contain the dimension of the field, while the rest of the lines have to contain exactly one function. The functions have to be on the form described above. 

The code outputs orthoderivative differential spectra of each function inside the input file. The first orthoderivative differential spectra corresponds to the first function inside the input file.

### results
The folder results contains my results doing the polynomial searches. The names of each of the files corresponds to *dimension_subfield_startExponent_numberOfTerms_numberOfTests*.

The folder *different_T* contains polynomial searches using different *T*s in dimension 14. Each file has a start and stop time, as it was used to decide on the optimal *T*. 

The folder birthday_VS_3_to_1 contains the results from doing polynomial searches over different dimensions. It is recorded to be able to compare the computation time for the triplicate birthday attack and the original triplicate method.

The folder dim14 contains the polynomial searches in dimension 14 that I was able to do before the deadline of this thesis. The folder odds contains the corresponding orthoderivative differential spectrum to the functions inside dim14. odds/sorted contains the same orthoderivative differential spectrum as in odds, but the duplicates are removed and counted.
