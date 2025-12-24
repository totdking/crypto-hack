use crypto_bigint::{NonZero, U2048};
// use num_bigint::BigInt;

/// To check if it is a valid square in a finite field p
pub fn check_quad(a: U2048, p: U2048) -> bool {
    legendre_symbol(a, p) == 1
}

/// To find out what type of square it is
/// Returns 1 if it is a quadratic residue
/// Returns -1 if it is a quadratic non-residue
/// Returns 0 if it is not a square
pub fn legendre_symbol(a: U2048, p: U2048) -> i32 {
    let (one, two) = (U2048::ONE, U2048::from_u8(2));
    let exp = (p - one) / two;
    let ls = legendre_exp(a, exp, p);
    if ls == U2048::from_u8(1) {
        return 1;
    } else if ls == (p - one) {
        return -1;
    }
    0
}

pub fn legendre_exp(mut a: U2048, mut pow: U2048, p: U2048) -> U2048 {
    let p_nonzero = NonZero::new(p).expect("Modulus cannot be zero");

    if p == U2048::from_u8(1) {
        return U2048::from_u8(0);
    }
    let mut res = U2048::from_u8(1);
    a = a % p;
    while pow > U2048::from_u8(0) {
        if pow % U2048::from_u8(2) == U2048::from_u8(1) {
            res = (res.mul_mod(&a, &p_nonzero)) % p;
        }
        pow = pow / U2048::from_u8(2);
        a = (a.mul_mod(&a, &p_nonzero)) % p;
    }
    return res;
}

/// For 3mod(4) prime field case
pub fn root_3mod4(a: U2048, p: U2048) -> U2048 {
    let one = U2048::ONE;
    let four = U2048::from_u8(4);
    let exponent = (p + one) / four;
    legendre_exp(a, exponent, p)
}

/// For both complexities
pub fn gen_root_find(a: U2048, p: U2048) -> Option<U2048> {
    let (one, two, three, four) = (
        U2048::ONE,
        U2048::from_u8(2),
        U2048::from_u8(3),
        U2048::from_u8(4),
    );
    if a == U2048::ZERO {
        return Some(U2048::ZERO);
    }
    if (p % four) == three {
        return Some(root_3mod4(a, p));
    }

    //Decomposition of (p-1) = Q *2^s
    let mut s = 0_u32;
    let mut q = p - one;
    // while q is even
    //@NOTE, This may give an unbounded recursive loop
    while q % two == U2048::from_u8(0) {
        q >>= 1;
        s += 1;
    }
    // Write p-1 = Q * 2 ^ s where q is odd

    // Find a quadratic non-residue z
    // We need a number z that does not have square root mod p
    let mut z = two;
    while legendre_symbol(z, p) != -1 {
        z += one
    }

    // Initialize variables
    // Tracks the order of t (How many times we square t to get 1)
    let mut m = s;
    // correction factor derived from the non-residue z
    let mut c = legendre_exp(z, q, p);
    // value we're trying to reduce to one, when t == 1, we are done
    let mut t = legendre_exp(a, q, p);
    // current candidate for the square root
    let mut r = legendre_exp(a, (q + one) / two, p);

    while t != one {
        // Find lowest i such that t^(2^i) = 1
        let mut i = 1u32;
        let mut temp = t.clone();
        while i < m {
            // Square temp: temp = temp^2 mod p
            let p_nonzero = NonZero::new(p).expect("Modulus cannot be zero");
            temp = temp.mul_mod(&temp, &p_nonzero);
            if temp == one {
                break;
            }
            i += 1;
        }

        // b = c^(2^(m-i-1)) mod p
        // We need to square c exactly (m-i-1) times
        let num_squarings = m - i - 1;
        let mut b = c;
        let p_nonzero = NonZero::new(p).expect("Modulus cannot be zero");
        for _ in 0..num_squarings {
            b = b.mul_mod(&b, &p_nonzero);
        }
        m = i;
        c = (b * b) % p;
        t = (t * c) % p;
        r = (r * b) % p;
    }
    Some(r)
}
