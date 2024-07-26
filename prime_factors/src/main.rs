fn main() {
    // println!("{:?}",find_factors(600851475143));
    // println!("{:?}",is_prime(600851475143));
    println!("{:?}", find_factors(12));
    println!("Largest prime factor: {}", find_largest(&find_factors(12)));
    
}

// factors of 10
// 1,2,5,10;
fn find_largest(res: &[i64]) -> i64 {
    *res.iter().max().unwrap()
}

fn find_factors(n:i64)->Vec<i64>{
    let mut res=Vec::new();
    let limit = (n as f64).sqrt() as i64;
    for i in 2..=limit{
        if n%i==0{
            if  is_prime(i ){
                res.push(i)
            }
            let pair_factor = n / i;
            
            // Check if the pair factor is different from i and is prime
            if pair_factor != i && is_prime(pair_factor ) {
                // If the pair factor is prime, add it to the result vector
                res.push(pair_factor);
            }
            

        }
    }
    res

}

fn is_prime(n:i64)->bool{
    let mut flag=true;

    if n<=1{
        flag=false;
    }
    for i in 2..=(n as f64).sqrt() as i64 {
        if n%i==0{
            flag=false;
        }
    }
    flag
}