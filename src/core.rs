/// Clean the unsigned big int (vec of digits from least to most significant) by removing useless zeroes
/// 
/// # Arguments
/// 
/// * `ubint` - the unsigned big int
/// 
/// # Examples
/// 
/// ```
/// use sloth_num::core;
/// 
/// let mut number = vec![0, 2, 4, 9, 6, 0, 0]; // 0069420
/// core::ub_clean(&mut number);                // -> 69420
/// 
/// let mut number = vec![];                    // works with empty UBI
/// core::ub_clean(&mut number);
/// 
/// ```
pub fn ub_clean(ubint: &mut Vec<u8>) {
    while let Some(0) = ubint.last() {
        if ubint.len() > 1 {ubint.pop();}
        else {break}
    }
}






/// Return true if u < v. Only works if both numbers are cleaned.
/// 
/// # Arguments
/// 
/// * `u` & `v` - unsigned big ints (a Vec of digits, from least to most significant)
/// 
/// # Examples
/// ```
/// use sloth_num::core;
/// 
/// let n1 = vec![3, 6, 7, 2];     // 2763
/// let n2 = vec![4, 6, 3];        // 364
/// let n3 = vec![0, 4, 1, 5];     // 5140
/// let n4 = vec![];               // 0 (`vec![0] works too`)
/// 
/// assert_eq!(core::ub_is_lower(&n1, &n2), false);
/// assert_eq!(core::ub_is_lower(&n1, &n3), true);
/// assert_eq!(core::ub_is_lower(&n4, &n2), true);
/// ```
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
/// returns a value of the same length
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










/// Compute the division of u by v, return the quotient q and the remainder r
/// Simpler division algorithm as v is only 1 digit
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












/// Perform the division of u / v (returns also u % v)
pub fn ub_div(u: Vec<u8>, v: Vec<u8>) -> (Vec<u8>, Vec<u8>) {


    debug_assert!(v.len() > 1, "v needs to be of length 2 at least");
    debug_assert!(u.len() >= v.len(), "m can't be negative");

    let v = v.clone();

    let n = v.len();



    // v[n-1] must be < 5 to work with inner_div
    // if it is not, we need to normalize the dividend and divisor so that v[n-1] >= 5
    // we can later 


    // will be > 1 if normalisation is needed
    let mut normaliser = 9 / v[n-1];



    let mut nv = ub_mul(v.clone(), vec![normaliser]);



    // we normalized too much (got one more digit), so we substract v to nv and 1 to normaliser
    while nv.len() > n && nv.last() != Some(&0) {
        normaliser -= 1;
        let mut rhs = v.clone();
        rhs.push(0);
        nv = ub_sub(nv, rhs);


    }

    // remove the last digit (which must be 0) if nv.len() > v.len()
    if nv.len() > v.len() {
        debug_assert!(nv.last() == Some(&0), "last is not 0 after normal correction");
        nv.pop();
    }

    // multiply nu by normaliser too
    let mut nu = ub_mul(u, vec![normaliser]);

    

    // inner_div requires that nu is AT LEAST one digit longer than nv
    if nu.len() == nv.len() {nu.push(0);}

    debug_assert!(nv[nv.len() - 1] >= 5, "last digit of nv = {} < 5", nv[nv.len() - 1]);







    // the quotient did not change after the normalisation
    // only the remainder needs to be unnormalized
    let (quotient, remainder) = inner_div(nu, nv);

    let (remainder, r0) = ub_shortdiv(remainder, normaliser);

    debug_assert!(r0 == 0, "r0 = {r0} != 0");

   (quotient, remainder)
}












/// Compute u / v and u % v
/// 
/// Conditions:
/// u.len() = m + n + 1
/// v.len() = n
/// 
/// v[n-1] > 5
fn inner_div(u: Vec<u8>, v: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    debug_assert!(v.len() > 1, "v needs to be of length 2 at least");
    debug_assert!(u.len() >= v.len(), "m can't be negative");

    let n = v.len();
    let m = u.len() - n - 1;

    debug_assert!(v[n-1] > 5, "v[n-1] should be > 5");

    // clone u as it needs to be mutable
    let mut u = u.clone();

    // quotient that will be returned
    let mut q = vec![0u8; m+1];








    for j in (0..m+1).rev() { // j goes from m to 0 (included)


        // estimation of q (called q_est) and r (r_est)
        let mut q_est = (u[j+n] * 10 + u[j+n-1]) / v[n-1];
        let mut r_est = (u[j+n] * 10 + u[j+n-1]).rem_euclid(v[n-1]);

        'do_while: loop { // correct estimation of q_est (so q_est is not too large)
            if q_est == 10 || q_est * v[n-2] > 10 * r_est + u[j+n-2] {
                q_est -= 1;
                r_est += v[n-1];

                if r_est >= 10 {break 'do_while;}
            }
            else {
                break 'do_while;
            }
        }



        // multiply and substract (hard part)
        // from The Art of Computer Programming:
        // "Replace (Uj+n Uj+n-1 ... Uj)b by (Uj+n Uj+n-1 ... Uj) - q_est(Vn-1 ... V1 V0)"
        // if this substraction is negative, compute it's 10's complement (add to it 10^(n+1))
        // and remember the borrowing that occured for later use
        
        let u_slice = u[j..j+n+1].to_vec();                          // (Uj+n Uj+n-1 ... Uj) of length n+1
        let mut v_slice = ub_mul(v.clone(), vec![q_est]);       // q_est(Vn-1 ... V1 V0) of length n+1
        v_slice.resize(n+1, 0); // the result of the ub_mul can be [0], so we resize to n+1





        // wether we must borrow or not
        let borrow = ub_is_lower(&u_slice, &v_slice);

        
        let sub = if borrow {

            // u_slice < v_slice so u_slice - v_slice < 0
            // we must compute u_slice - v_slice + 10^(n+1)
            //
            // => u_slice - v_slice + 10^(n+1)
            // => -(v_slice - u_slice) + 10^(n+1)
            // => 10^(n+1) - (v_slice - u_slice)

            // ten_pow = 10^(n+1) (length n+2)
            let mut ten_pow = vec![0u8; n+1];
            ten_pow.push(1);


            // lhs = v_slice - u_slice
            let mut lhs = ub_sub(v_slice, u_slice); // lhs.len() = n+1
            lhs.push(0);                                          // lhs.len() = n+2
            




            // compute substraction
            let mut sub = ub_sub(ten_pow, lhs);
            debug_assert!(sub[n+1] == 0, "Sub is full n+2 in size (no good)");
            sub.pop(); // remove last "0" that SHOULD be here if everything is ok

            sub
        }
        else {
            ub_sub(u_slice, v_slice) // length n+1
        };
        debug_assert!(sub.len() == n+1, "sub is not n+1 in length ({:?})", sub);


        // replace the values in u by those in sub
        for i in 0..n {
            u[j+i] = sub[i];
        }

        debug_assert!(q[j] < 10, "q_est is not a digit ({q_est})");
        q[j] = q_est;

        
        // Add back after the borrow
        if borrow {
            q[j] -= 1;

            // todo: can be refactored as an in-place addition on nu
            let mut slice = v.clone();
            slice.push(0);
            let add = ub_add(slice, u[j..n+j+1].to_vec());

            // add should be of length n+1, but we ignore the nth digit (created by a carry) as
            // it cancels the borrow that occured before
            for i in 0..n {
                u[i+j] = add[i];
            }
        }
    }

    // resize u so we only keep the nth first elements
    u.resize(n, 0);


    // clean and return the results
    ub_clean(&mut q);
    (q, u)
}













/// If b is a power of ten, returns this power
/// ex: [0, 0, 1] => 2 (because 100 = 10^2)
/// Does not work if b is not cleaned
pub fn is_power_of_ten(b: &Vec<u8>) -> Option<usize> {
    match b.last()? {
        1 => {
            for d in &b[0..b.len()-1] {
                if d != &0 {return None}
            }
            Some(b.len()-1)
        },
        _ => None
    }
}