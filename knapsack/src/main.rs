// Implementation of Algorithm to solve Knapsack Problems with Dynamic Programming

use std::cmp::Ordering;

fn main() {

    let weight:  Vec<usize> = vec![2,4,3,1];            // A vector of item weights
    let utility: Vec<usize> = vec![1,4,7,2];            // A vector of item utilities
    let budget:  usize = 5;                             // A contraint on the items sum weight

    println!("Weight vector is: {:?}" , weight);
    println!("Utility vector is: {:?}", utility);
    println!("Budget is: {}", budget);
    println!("Optimal choice vector is: {:?}", knapsack(weight, utility, budget));
}

fn knapsack(weight: Vec<usize>, utility: Vec<usize>, budget: usize) -> Vec<usize> {

    let n: usize = weight.len();

    let mut table:  Vec<usize> = vec![0; budget];       // A vector of optimal budget utilities
    let mut choice: Vec<usize> = vec![0; budget];       // A vector of optimal item choices

    for r in 1..(budget + 1) {                          // For each potential remaining budget...

        let mut value:  Vec<usize> = vec![0; budget];   // A vector of item value add

        for item in 0..n {                              // For each item...

            match (r).cmp(&weight[item]) {              // Compare remaining budget to item weight:
                                                        // If Remaining Budget > Weight:
                                                        //      Value = item utility + optimal w/ remaining budget
                Ordering::Greater => value[item] = utility[item] + table[r - weight[item]],
                                                        // If Remaining Budget = Weight:
                                                        //      Value = exactly the item utility
                Ordering::Equal   => value[item] = utility[item],
                                                        // If Remaining Budget < Weight:
                                                        //      Insufficient budget remains, item can't add value
                Ordering::Less    => value[item] = 0
            }
        }

        let mut max_value: usize = 0;                   // Maximum added value 
        let mut max_index: usize = 0;                   // Maximum added value item

        for item in 0..n {                              // For each item...
            
            if value[item] > max_value {                // Compare to current max
                max_value = value[item];
                max_index = item;
            }
        }

        table[r - 1]  = max_value;                      // Add optimal value for given remaining budget to table
        choice[r - 1] = max_index;                      // Add optimal item for given remaining budget to choice
    }

    choice
}   

