pub fn insertionsort(v: &mut Vec<i32>) {
    let n = v.len();
    for i in 0..n {
        let mut ins_index = i;
        let r = v[i];
        for k in 0..n-i-1 {
            if r < v[k] {
                v[k+1] = r;
                ins_index = k
            }
            else {
                break;
            }
        }
        v[ins_index] = r
    }
}