use std::collections::HashSet;

// fn gcd(a: u64, b: u64) -> u64 {
//     if b == 0 {
//         a
//     } else {
//         gcd(b, a % b)
//     }
// }

fn pow_mod(base: u64, exp: u64, modu: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modu;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modu;
        }
        base = (base * base) % modu;
        exp /= 2;
    }
    result
}

fn is_primitive_root(g: u64, p: u64) -> bool {
    let phi = p - 1;
    let factors = prime_factors(phi);
    
    for factor in &factors {
        if pow_mod(g, phi / factor, p) == 1 {
            return false;
        }
    }
    true
}

fn prime_factors(mut n: u64) -> HashSet<u64> {
    let mut factors = HashSet::new();
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            factors.insert(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        factors.insert(n);
    }
    factors
}

fn find_primitive_roots(p: u64) -> Vec<u64> {
    if p == 2 {
        return vec![1];
    }
    
    let mut roots = Vec::new();
    for g in 2..p {
        if is_primitive_root(g, p) {
            roots.push(g);
        }
    }
    roots
}

fn main() {
    let p: u64 = 7; // Replace with any prime number
    let primitive_roots = find_primitive_roots(p);
    println!("Primitive roots of {} are: {:?}", p, primitive_roots);
}
