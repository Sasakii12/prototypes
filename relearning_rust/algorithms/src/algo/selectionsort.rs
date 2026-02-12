pub fn selection_sort(v: &mut Vec<i32>) {
    let n = v.len();

    for i in 0..n {
        for k in i..n {
            if v[k] < v[i] {
                v.swap(i,k);
            }
        }
    }
}