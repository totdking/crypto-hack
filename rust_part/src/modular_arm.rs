/// To find the modular exponentiation of a number in a finite field
/// 
/// Under Fermat little therorem
pub fn mod_pow(mut num: u128, mut pow: u128, p: u128) -> u128{
    if p == 1{return 0;}
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
    if !roots.is_empty(){
        return Some(roots);
    }
    None
}
pub fn valid_square(arr: &[u128], p: u128) {
    for j in arr.iter() {
        let roots = root_find(*j, p);
        if !roots.clone().unwrap().is_empty() {
            println!("value {:?} is a valid square in field {:?} with root(s) {:?}", j, p, roots);
        }
    }
    let (_, invalid): (Vec<u128>, Vec<u128>) = arr.iter().partition(|&val| !root_find(*val, p).unwrap().is_empty());
    println!("invalid squares for value {:?}", invalid)
}