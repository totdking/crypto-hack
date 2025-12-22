use crypto_bigint::NonZero;
use crypto_bigint::U1024;

/// To find the modular exponentiation of a number in a finite field
///
/// Under Fermat little therorem
pub fn mod_pow(mut num: u128, mut pow: u128, p: u128) -> u128 {
    if p == 1 {
        return 0;
    }
    let mut result = 1;
    num = num % p;
    while pow > 0 {
        if pow % 2 == 1 {
            result = (result * num) % p;
        }
        pow >>= 1;
        num = (num.pow(2)) % p;
    }
    return result;
}

/// Brute forcing a root find of a square in a finite field p
pub fn root_find(square: u128, p: u128) -> Option<Vec<u128>> {
    let mut roots = Vec::new();
    for i in 0..p {
        if mod_pow(i, 2, p) == square {
            roots.push(i);
        }
    }
    if !roots.is_empty() {
        return Some(roots);
    }
    None
}

/// Need to use legendre's symbol test to find out if they are valid squares.
pub fn valid_square(arr: &[u128], p: u128) {
    for j in arr.iter() {
        let roots = root_find(*j, p);
        if !roots.clone().unwrap().is_empty() {
            println!(
                "value {:?} is a valid square in field {:?} with root(s) {:?}",
                j, p, roots
            );
        }
    }
    let (_, invalid): (Vec<u128>, Vec<u128>) = arr
        .iter()
        .partition(|&val| !root_find(*val, p).unwrap().is_empty());
    println!("invalid squares for value {:?}", invalid)
}


// ============ Legendre Symbol Test ============
/// Find which value in the array is a quadratic residue
/// Returns the index and value of the quadratic residue, or None if not found
pub fn find_quadratic_residue(a: &[U1024], p: U1024) -> Option<(usize, U1024)> {
    let one = U1024::ONE;
    let two = U1024::from_u8(2);

    // Calculate (p - 1) / 2
    let exponent = (p - one) / two;

    // Check each value individually
    for (idx, &value) in a.iter().enumerate() {
        let result = legendre_exp(value, exponent, p);

        // If result == 1, it's a quadratic residue
        if result == one {
            println!("Found quadratic residue at index {}: {}", idx, value);
            return Some((idx, value));
        }
    }

    None
}

/// Calculate square root for primes where p ≡ 3 mod 4
/// For such primes, if a is a quadratic residue, then:
/// sqrt(a) ≡ a^((p+1)/4) mod p
pub fn sqrt_mod_prime_3mod4(a: U1024, p: U1024) -> U1024 {
    let one = U1024::ONE;
    let four = U1024::from_u8(4);

    // Calculate (p + 1) / 4
    let exponent = (p + one) / four;

    // Calculate a^((p+1)/4) mod p
    legendre_exp(a, exponent, p)
}

/// helper function for legendre symbol finding
pub fn legendre_exp(mut a: U1024, mut pow: U1024, p: U1024) -> U1024 {
    let p_nonzero = NonZero::new(p).expect("Modulus cannot be zero");

    if p == U1024::from_u8(1) {
        return U1024::from_u8(0);
    }
    let mut res = U1024::from_u8(1);
    a = a % p;
    while pow > U1024::from_u8(0) {
        if pow % U1024::from_u8(2) == U1024::from_u8(1) {
            res = (res.mul_mod(&a, &p_nonzero)) % p;
        }
        pow = pow / U1024::from_u8(2);
        a = (a.mul_mod(&a, &p_nonzero)) % p;
    }
    return res;
}
