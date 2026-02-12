mod algo;

use crate::algo::bubblesort::{bubblesort, bubblesort_rev};

fn main() {
    let mut sort_vec = vec![1,6,2,8,2,1];
    println!("Hello, world!");
    bubblesort(&mut sort_vec);
    println!("{:?}", sort_vec );
    bubblesort_rev(&mut sort_vec);
    println!("{:?}", sort_vec );
}
