
/// Clean useless zeroes of the integer
fn clean_uint(u: &mut Vec<u8>) {
    while let Some(0) = u.last() {u.pop();}
}





/// Multiply 2 unsigned integers
/// (represented by vecs of u8, from least to most significant digit)
/// Base on the multiplication algorithm in the Art of Computer Programming
pub fn u_mul(u: Vec<u8>, v: Vec<u8>) -> Vec<u8> {
    // the algorithm requires that u.len() >= v.len()
    if u.len() < v.len() {return u_mul(v, u)}


    // various optimisation
    if v == vec![1] {return u;}
    if u == vec![0] || v == vec![0] {return vec![0];}

    
    let m = u.len();
    let n = v.len();

    let mut w = vec![0; m+n];

    // multiplication by iterating over each digit of v
    for j in 0..n {
        if v[j] == 0 {
            w[j+m] = 0;
            continue;
        }

        let mut k = 0;
        for i in 0..m {
            let t = u[i] * v[j] + w[i+j] + k;
            w[i+j] = t % 10;
            k = t / 10;
        }
        w[j+m] = k;
    }

    clean_uint(&mut w);
    w
}