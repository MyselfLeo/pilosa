//! Algorithms for common operations on unsigned big ints.
//! Those functions are used by the BigNum struct to represent and manipulate
//! arbitrary long/precise numbers.

use crate::assert_err;

/// Clean the unsigned big int (vec of digits from least to most significant) by removing useless zeroes
/// Will keep one zero if it is already here (ex: `vec![0, 0, 0]` -> `vec![0]`)
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

/// Return a cleaned version of the unsigned big int by removing useless zeroes.  
/// Will keep one zero if it is already here (ex: `vec![0, 0, 0] -> vec![0]`)
/// 
/// # Arguments
/// 
/// * `ubint` - the unsigned big int (vec of digits from least to most significant)
/// 
/// # Examples
/// 
/// ```
/// use sloth_num::core;
/// 
/// let mut number = vec![0, 2, 4, 9, 6, 0, 0];                 // 0069420
/// assert_eq!(core::ub_cleaned(number), vec![0, 2, 4, 9, 6]);  // -> 69420
/// 
/// // works with empty UBI
/// let mut number = vec![];
/// assert_eq!(core::ub_cleaned(number), vec![]); 
/// 
/// ```
pub fn ub_cleaned(ubint: Vec<u8>) -> Vec<u8> {
    let mut res = ubint.clone();
    while let Some(0) = res.last() {
        if res.len() > 1 {res.pop();}
        else {break}
    };

    res
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







/// Return the (cleaned) sum of the 2 unsigned big int u and v.
/// Requires that u >= v
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
/// let n3 = vec![0, 0, 1, 0];     // 100
/// let n4 = vec![0];              // 0
/// 
/// assert_eq!(core::ub_add(n1, n2), vec![7, 2, 1, 3]); // 2763 + 364 = 3127
/// assert_eq!(core::ub_add(n3, n4), vec![0, 0, 1]);    // 100
/// ```
pub fn ub_add(u: Vec<u8>, v: Vec<u8>) -> Vec<u8> {
    // the algorithm requires that u.len() >= v.len()
    if u.len() < v.len() {return ub_add(v, u)}

    // various optimization
    if u == vec![0] {return ub_cleaned(v);}
    if v == vec![0] {return ub_cleaned(u);}

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



// todo: maybe allow the user to specify whether they want the result cleaned or not (?)



/// Substract an unsigned big int v to an unsigned big int u.
/// requires u >= v and u and v of the same size (panics otherwise)  
/// returns a value of the same length, NOT CLEANED  
/// Based on the substraction algorithm in the Art of Computer Programming
/// 
/// # Arguments
/// 
/// * `u` & `v` - unsigned big ints (represented by vecs of u8, from least to most significant digit)
/// 
/// # Examples
/// 
/// ```
/// use sloth_num::core;
/// 
/// // the numbers must have the same amount of digits
/// let n1 = vec![3, 6, 7, 2];     // 2763
/// let n2 = vec![4, 6, 3, 0];     // 364
/// let n3 = vec![0, 0, 1, 0];     // 100
/// let n4 = vec![4, 0, 0, 0];     // 4
/// 
/// assert_eq!(core::ub_sub(n1, n2), Ok(vec![9, 9, 3, 2]));
/// assert_eq!(core::ub_sub(n3, n4), Ok(vec![6, 9, 0, 0]));
/// ```
pub fn ub_sub(u: Vec<u8>, v: Vec<u8>) -> Result<Vec<u8>, String> {
    // the algorithm requires that u.len() == v.len()
    assert_err!(u.len() == v.len(), "Both unsigned big ints must have the same amount of digits");

    // optimization
    if v == vec![0] {return Ok(u);}

    let n = u.len();
    let mut w = vec![0; n];

    let mut k: i16 = 0; // carry
    for j in 0..n {
        let t = u[j] as i16 - v[j] as i16 + k;

        w[j] = t.rem_euclid(10) as u8;
        k = -((t < 0) as i16);
    }

    if k != 0 {panic!("Expected u >= v")}

    Ok(w)
}







/// Multiply 2 unsigned big ints u and v
/// (represented by vecs of u8, from least to most significant digit)
/// Based on the multiplication algorithm in the Art of Computer Programming
/// 
/// # Arguments
/// 
/// * `u` & `v` - unsigned big ints (represented by vecs of u8, from least to most significant digit)
/// 
/// # Examples
/// 
/// ```
/// use sloth_num::core;
/// 
/// // the numbers must have the same amount of digits
/// let n1 = vec![3, 6, 7, 2];   // 2763
/// let n2 = vec![4, 6, 3];      // 364
/// let n3 = vec![0, 0, 1, 0];   // 100
/// let n4 = vec![0];            // 0
/// 
/// assert_eq!(core::ub_mul(&n1, &n2), vec![2, 3, 7, 5, 0, 0, 1]);
/// assert_eq!(core::ub_mul(&n3, &n4), vec![0]);
/// assert_eq!(core::ub_mul(&n3, &n2), vec![0, 0, 4, 6, 3]);
/// ```
pub fn ub_mul(u: &Vec<u8>, v: &Vec<u8>) -> Vec<u8> {
    // the algorithm requires that u.len() >= v.len()
    if u.len() < v.len() {return ub_mul(v, u)}


    // various optimisation
    if v == &vec![1] {return u.clone();}
    if u == &vec![1] {return v.clone();}
    if u == &vec![0] || v == &vec![0] {return vec![0];}

    
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










/// Compute the division of u by v, return the quotient q and the remainder r.  
/// Simpler division algorithm compared to `ub_div` as v is only 1 digit
/// 
/// # Arguments
/// * `u` - unsigned big ints (a Vec of digits, from least to most significant)
/// * `v` - a digit != 0
/// 
/// # Examples
/// 
/// ```
/// use sloth_num::core;
/// 
/// let n1 = vec![3, 6, 7, 2];     // 2763
/// let n2 = vec![4, 6, 3];        // 364
/// let n3 = vec![0, 0, 1, 0];     // 100
/// 
/// assert_eq!(core::ub_shortdiv(n1.clone(), 3), Ok((vec![1, 2, 9], 0)));
/// assert_eq!(core::ub_shortdiv(n2, 9), Ok((vec![0, 4], 4)));
/// assert_eq!(core::ub_shortdiv(n3, 1), Ok((vec![0, 0, 1], 0)));
/// assert!(core::ub_shortdiv(n1, 0).is_err());
/// ```
pub fn ub_shortdiv(u: Vec<u8>, v: u8) -> Result<(Vec<u8>, u8), String> {
    if v == 0 {return Err("Division by zero".to_string())}
    if v == 1 {return Ok((ub_cleaned(u), 0))}

    let n = u.len();
    let mut res = vec![0u8; n];


    let mut r = 0u8;
    for i in (0..n).rev() {
        let x = r * 10 + u[i];

        res[i] = x / v;
        r = x % v;
    }

    ub_clean(&mut res);
    Ok((res, r))
}












/// Returns u // v and u % v.  
/// If `v` is only 1 digit, it is preferable to use `core::shortdiv` instead.
/// 
/// # Arguments
/// 
/// * `u` - the dividend of the operation, a **cleaned** unsigned bit int (a Vec of digits, from least to most significant)
/// * `v` - the divisor, a **cleaned** unsigned big int too
/// 
/// 
/// # Examples
/// 
/// ```
/// use sloth_num::core;
/// 
/// let n1 = vec![3, 6, 7, 2];     // 2763
/// let n2 = vec![4, 6, 3];        // 364
/// let n3 = vec![0, 0, 1];        // 100
/// let n4 = vec![0];              // 0
/// 
/// assert_eq!(core::ub_div(&n1, &n2), Ok((vec![7], vec![5, 1, 2])));
/// //assert_eq!(core::ub_div(&n2, &n3));
/// ```
pub fn ub_div(u: &Vec<u8>, v: &Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), String> {
    assert_err!(v.len() > 1, "v needs to be of length 2 at least");
    assert_err!(v.len() > 1, "v needs to be of length 2 at least");
    assert_err!(u.len() >= v.len(), "m can't be negative");

    let n = v.len();



    // v[n-1] must be < 5 to work with inner_div
    // if it is not, we need to normalize the dividend and divisor so that v[n-1] >= 5
    // we can later 


    // will be > 1 if normalisation is needed
    let mut normaliser = 9 / v[n-1];



    let mut nv = ub_mul(v, &vec![normaliser]);



    // we normalized too much (got one more digit), so we substract v to nv and 1 to normaliser
    while nv.len() > n && nv.last() != Some(&0) {
        normaliser -= 1;
        let mut rhs = v.clone();
        rhs.push(0);
        nv = ub_sub(nv, rhs).expect("internal error in ub_div");


    }

    // remove the last digit (which must be 0) if nv.len() > v.len()
    if nv.len() > v.len() {
        assert_err!(nv.last() == Some(&0), "last is not 0 after normal correction");
        nv.pop();
    }

    // multiply nu by normaliser too
    let mut nu = ub_mul(u, &vec![normaliser]);

    // inner_div requires that nu is AT LEAST one digit longer than nv
    if nu.len() == nv.len() {nu.push(0);}

    assert_err!(nv[nv.len() - 1] >= 5, "last digit of nv = {} < 5", nv[nv.len() - 1]);







    // the quotient did not change after the normalisation
    // only the remainder needs to be unnormalized
    let (quotient, remainder) = inner_div(&nu, &nv);


    let (remainder, r0) = ub_shortdiv(remainder, normaliser).expect("internal error in ub_div"); // normaliser =/= 0 so no risk of error

    assert_err!(r0 == 0, "r0 = {r0} != 0");

   Ok((quotient, remainder))
}












/// Compute u / v and u % v.  
/// This algorithm is based on the division algorithm in the Art of Computer Programming.
/// However, the function expect the division to be normalised (i.e most significant digit of v > 5).
/// The function `ub_div` manages this normalisation.
/// 
/// # Arguments
/// 
/// * `u` - the dividend of the operation, an unsigned bit int (a Vec of digits, from least to most significant)
/// * `v` - the divisor, an unsigned big int too
/// 
/// # Conditions:
/// `u.len() = m + n + 1 (n > 2, m >= 0)`
/// `v.len() = n`
/// `v[n-1] > 5`
fn inner_div(u: &Vec<u8>, v: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    debug_assert!(v.len() > 1, "v needs to be of length 2 at least");
    debug_assert!(u.len() >= v.len(), "m can't be negative");

    // clone u as it needs to be mutable
    let mut u = u.clone();
    u.push(0);

    let n = v.len();
    let m = u.len() - n - 1;

    debug_assert!(v[n-1] > 5, "v[n-1] should be > 5");

    

    // quotient that will be returned
    let mut q = vec![0u8; m+2];



    println!("n: {}  m: {}", n, m);




    for j in (0..m+1).rev() { // j goes from m to 0 (included)
        println!("value of j: {j}");

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
        let mut v_slice = ub_mul(v, &vec![q_est]);       // q_est(Vn-1 ... V1 V0) of length n+1
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
            let mut lhs = ub_sub(v_slice, u_slice).expect("internal error in inner_div"); // lhs.len() = n+1
            lhs.push(0);                                                                                    // lhs.len() = n+2
            




            // compute substraction
            let mut sub = ub_sub(ten_pow, lhs).expect("internal error in inner_div");
            debug_assert!(sub[n+1] == 0, "Sub is full n+2 in size (no good)");
            sub.pop(); // remove last "0" that SHOULD be here if everything is ok

            sub
        }
        else {
            ub_sub(u_slice, v_slice).expect("internal error in inner_div") // length n+1
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
/// 
/// # Arguments
/// * `b` - a cleaned (no useless zero) unsigned big int (a Vec of digits, from least to most significant)
/// 
/// # Examples
/// 
/// ```
/// use sloth_num::core;
/// 
/// let n1 = vec![0, 0, 0, 1]; // 1000
/// let n2 = vec![1, 0, 3];    // 301
/// let n3 = vec![1];          // 1
/// let n4 = vec![0, 0, 1, 0]; // 100 (not cleaned)
/// 
/// assert_eq!(core::is_power_of_ten(&n1), Some(3)); // 1000 = 10^3
/// assert_eq!(core::is_power_of_ten(&n2), None);
/// assert_eq!(core::is_power_of_ten(&n3), Some(0)); // 1 = 10^0
/// assert_eq!(core::is_power_of_ten(&n4), None);    // will not work on non-cleaned numbers
/// ```
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