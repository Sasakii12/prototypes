

pub fn bubblesort(vec: &mut Vec<i32>) {
    let len = vec.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if vec[j] > vec[j+1] {
                vec.swap(j, j+1);
            }
        }
    }
}

pub fn bubblesort_rev(vec: &mut Vec<i32>) {
    let len = vec.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if vec[j] < vec[j+1] {
                vec.swap(j, j+1);
            }
        }
    }
}