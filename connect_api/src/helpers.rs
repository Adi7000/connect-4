/*
 * File: helpers.rs
 * Author: Fahrin Bushra
 * Date: April 11, 2024
 * Adapted From: CHAT GPT
 * Description: HELPER FUNCTIONS FOR VECTOR FIELD CHECKING
*/


pub fn count_different_elements(vector_a: &Vec<Vec<i32>>, vector_b: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;

    // Ensure both vectors have the same length
    if vector_a.len() != vector_b.len() {
        return count;
    }

    // Iterate over each element in the vectors and compare them
    for (row_a, row_b) in vector_a.iter().zip(vector_b.iter()) {
        // Ensure both rows have the same length
        if row_a.len() != row_b.len() {
            continue;
        }

        // Compare each element in the rows
        for (elem_a, elem_b) in row_a.iter().zip(row_b.iter()) {
            if *elem_a != *elem_b {
                count += 1;
            }
        }
    }

    count
}

pub fn count_zeros(matrix: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;

    // Iterate over each row in the matrix
    for row in matrix {
        // Iterate over each element in the row
        for &element in row {
            // Increment the counter if the element is zero
            if element == 0 {
                count += 1;
            }
        }
    }

    count
}

pub fn count_same_elements(vector_a: &Vec<Vec<i32>>, vector_b: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;

    if vector_a.len() != vector_b.len() {
        return count;
    }

    for (row_a, row_b) in vector_a.iter().zip(vector_b.iter()) {
        if row_a.len() != row_b.len() {
            continue;
        }

        for (elem_a, elem_b) in row_a.iter().zip(row_b.iter()) {
            if *elem_a == *elem_b {
                count += 1;
            }
        }
    }
    count
}


pub fn count_non_zero_elements(matrix: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;

    for row in matrix {
        for &element in row {
            if element != 0 {
                count += 1;
            }
        }
    }

    count
}