/// Clean useless zeroes of the big int
fn ub_clean(ubint: &mut Vec<u8>) {
    while let Some(0) = ubint.last() {
        if ubint.len() > 1 {ubint.pop();}
        else {break}
    }
}




/// Return true if u < v
pub fn ub_is_lower(u: &Vec<u8>, v: &Vec<u8>) -> bool {
    if u.len() < v.len() {return true}
    if u.len() > v.len() {return false}


    for (du, dv) in std::iter::zip(u, v).rev() {
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
    // the algorithm requires that u.len() == v.len()n
    assert!(u.len() == v.len(), "Both unsigned big ints must have the same amount of digits");

    // optimization
    if v == vec![0] {return u;}

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









/// u.len() = m+n
/// v.len() = n
/// both cleaned and n > 1
/// 
/// returns q = floor(u/v)
///     and r = u mod v
/// 
/// Based on the division algorithm in the Art of Computer Programming
pub fn ub_div(u: Vec<u8>, v: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let n = v.len();
    let m = u.len() - n;

    assert!(n > 1, "n should be > 1");
    assert!(v[n-1] != 0, "v[n-1] should not be 0");

    // normalisation so that nv[n-1] > b/2 in any case
    // note that d < 10 (important for the end of the algorithm)
    let d = 9 / v[n-1];

    let mut nu = ub_mul(u, vec![d]);
    
    if nu.len() < n+m+1 {nu.push(0);}

    let nv = ub_mul(v, vec![d]);


    let mut q = vec![0u8; m+1];

    debug_assert!(nu.len() == n+m+1, "nu is not n+m+1 in length");
    debug_assert!(nv.len() == n, "nv is not n in length");

    debug_assert!(nv[n-1] != 0, "nv[n-1] should not be 0");

    for j in (0..m+1).rev() { // m -> 1
        

        // estimation of q (called q_est) and r (r_est)
        let mut q_est = (nu[j+n] * 10 + nu[j+n-1]) / nv[n-1];
        let mut r_est = (nu[j+n] * 10 + nu[j+n-1]).rem_euclid(nv[n-1]);

        // i think i need a do-while here so pretend it's one
        'do_while: loop {
            if q_est == 10 || q_est * nv[n-2] > 10 * r_est + nu[j+n-2] {
                q_est -= 1;
                r_est += nv[n-1];

                if r_est >= 10 {break 'do_while;}
            }
            else {
                break 'do_while;
            }
        }



        let u_slice = nu[j..j+n+1].to_vec();
        let mut v_slice = ub_mul(nv.clone(), vec![q_est]);

        debug_assert!(v_slice.len() <= nv.len()+1, "v_slice.len() is > nv.len()+1");
      
        // assure that v_slice is the same length as nv
        while v_slice.len() < nv.len()+1 {v_slice.push(0);}

        debug_assert!(v_slice.len() == nv.len()+1, "v_slice.len() != nv.len()+1 (even after rectification)");

        let borrow = ub_is_lower(&u_slice, &v_slice);

        // computes u_slice - v_slice (if u_slice >= v_slice) or u_slice - v_slice + 10^(n+1) (if u_slice < v_slice)
        let mut sub = if borrow {                           // u_slice - v_slice + 10^(n+1) <=> 10^(n+1) - (v_slice - u_slice)
            let mut ten_pow = vec![0u8; n+1]; // 10^(n+1)
            ten_pow.push(1);

            let lhs = ub_sub(v_slice, u_slice);
            ub_sub(ten_pow, lhs)
        }
        else {
            ub_sub(u_slice, v_slice)                                 // u_slice - v_slice (>0)
        };

        debug_assert!(sub.len() <= n, "sub is too long");

        // assure that sub is the of length n
        while sub.len() < n {sub.push(0);}
        
        
        // replace the values in nu by the values of sub (between j and j+n)
        for i in 0..n {
            nu[i+j] = sub[i];
        }

        q[j] = q_est;
        debug_assert!(q[j] < 10, "q_est was not a digit");

        
        if borrow {
            q[j] -= 1;

            // todo: can be refactored as an in-place addition on nu
            let mut slice = nv.clone();
            slice.push(0);
            let add = ub_add(slice, nu[j..n+j+1].to_vec());

            // add should be of length n+1, but we ignore the nth digit (created by a carry) as
            // it cancels the borrow that occured before
            for i in 0..n {
                nu[i+j] = add[i];
            }
        }
    }


    // resize nu so it is of size n
    nu.resize(n, 0);


    // unnormalize
    let (r, r0) = ub_shortdiv(nu, d);

    debug_assert!(r0 == 0, "I believe r0 should be 0 ????");


    // clean and return the results
    ub_clean(&mut q);
    (q, r)
}











// todo: algorithm for when n=1

/// Compute the division of u by v, return the quotient q and the remainder r
pub fn ub_shortdiv(u: Vec<u8>, v: u8) -> (Vec<u8>, u8) {
    let n = u.len();
    let mut res = vec![0u8; n];


    let mut r = 0u8;
    for i in (0..n).rev() {
        let x = r * 10 + u[i];

        res[i] = x / v;
        r = x % v;
    }

    ub_clean(&mut res);
    (res, r)
}