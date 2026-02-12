mod algo;

use crate::algo::*;

fn main() {
    let mut sort_vec = vec![7,12,9,11,3];
    println!("Hello, world!");
    bubblesort::bubblesort(&mut sort_vec);
    println!("{:?}", sort_vec );
    bubblesort::bubblesort_rev(&mut sort_vec);
    println!("{:?}", sort_vec );
    selectionsort::selection_sort(&mut sort_vec);
    println!("{:?}", sort_vec );
}
