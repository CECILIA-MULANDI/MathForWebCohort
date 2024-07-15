
fn main() {
    let base =27;
    let exponent=234;
    let modulus=313;
    let res=mod_exp(base,exponent,modulus);
    println!("{}^{} mod {}={}",base,exponent,modulus,res);
}

fn mod_exp(mut base:u64,mut exp:u64,modulus:u64)->u64{
    if modulus==1{
        return 0;
    }
    let mut result=1;
    base=base%modulus;
    while exp>0{
        if exp%2==1{
            result=(result*base)%modulus;

        }
        exp=exp>>1;
        base=(base*base)%modulus;
    }
    result
}
