/// Clean useless zeroes of the big int
fn ub_clean(ubint: &mut Vec<u8>) {
    while let Some(0) = ubint.last() {ubint.pop();}
}




/// Return true if u < v
pub fn ub_is_lower(u: Vec<u8>, v: Vec<u8>) -> bool {
    if u.len() < v.len() {return true}
    if u.len() > v.len() {return false}

    for (du, dv) in std::iter::zip(&u, &v) {
        if du < dv {return true}
        if du > dv {return false}
    }

    false
}





/// Add 2 unsigned big ints u and v
/// (represented by vecs of u8, from least to most significant digit)
pub fn ub_add(u: Vec<u8>, v: Vec<u8>) -> Vec<u8> {
    // the algorithm requires that u.len() >= v.len()
    if u.len() < v.len() {return ub_add(v, u)}

    // various optimization
    if u == vec![0] {return v;}
    if v == vec![0] {return u;}

    let m = u.len();
    let n = v.len();

    let mut w = vec![0; m+1];

    // addition by iterating over digits of u
    let mut k = 0;
    for i in 0..m {
        let vi = if i < n {v[i]} else {0};
        let t = u[i] + vi + k;

        w[i] = t % 10;
        k = t / 10;
    }
    w[m] = k; // final carry

    ub_clean(&mut w);
    w
}





/// Substract an unsigned big int u to an unsigned big int v
/// (represented by vecs of u8, from least to most significant digit)
/// requires u >= v and u and v of the same size (panics otherwise)
/// Based on the substraction algorithm in the Art of Computer Programming
pub fn ub_sub(u: Vec<u8>, v: Vec<u8>) -> Vec<u8> {
    if u.len() != v.len() {panic!("Both unsigned big ints must have the same amount of digits")}

    let n = u.len();
    let mut w = vec![0; n];

    let mut k: i16 = 0; // carry
    for j in 0..n {
        let t = u[j] as i16 - v[j] as i16 + k;

        w[j] = t.rem_euclid(10) as u8;
        k = -((t < 0) as i16);
    }

    if k != 0 {panic!("Expected u >= v")}

    ub_clean(&mut w);
    w
}







/// Multiply 2 unsigned big ints u and v
/// (represented by vecs of u8, from least to most significant digit)
/// Based on the multiplication algorithm in the Art of Computer Programming
pub fn ub_mul(u: Vec<u8>, v: Vec<u8>) -> Vec<u8> {
    // the algorithm requires that u.len() >= v.len()
    if u.len() < v.len() {return ub_mul(v, u)}


    // various optimisation
    if v == vec![1] {return u;}
    if u == vec![1] {return v;}
    if u == vec![0] || v == vec![0] {return vec![0];}

    
    let m = u.len();
    let n = v.len();

    let mut w = vec![0; m+n];   // final big uint

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
        w[j+m] = k; // final carry
    }
    ub_clean(&mut w);
    w
}