# Master-Project

This is the official code and results for Silje Folkestad's Master thesis at University of Bergen, 2024.

## Overview

### polynomial_search
The folder polynomial_search/src contains the code for doing a polynomial search over any field. It finds quadratic triplicate APN functions. It uses an approach called triplicate birthday attack, which is a probabilistic test. It tests whether a function is 3-to-1 based on *T* tests. If the triplicate birthday attack assumes that the function is 3-to-1, it uses the original 3-to-1 approach to verify.

Upon running the code, you write args commands referring to *dimension, subfield, start exponent, number of terms, number of tests*, and *maximum number of functions*. The default values are 6, 6, 3, 3, 700 and 2,000,000.

If the dimension is lower than 10, it will *birthday_small_fields.rs*, while using dimension 10 or higher will run *birthday_big_fields.rs*. The difference is in how they go through checking whether they are 3-to-1. birthday_small_fields go through the entire field, only picking random elements and keeping track of all the triplicates it has found. birthday_big_fields uses *T* tests; it picks *T* random elements, and keeps track of all the triplicates it has found.

You could also use this code to run the original 3-to-1 method. Right now it is written as a comment inside *main.rs*, so to be able to run it you have to uncomment it, and comment out the if-else above. It uses the same args methods as described above. 
