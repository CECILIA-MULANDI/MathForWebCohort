fn main() {
    println!("{:?}",factorize(12));
}
fn factorize(mut n:u128)->Vec<u128>{
    let mut p =2;
    let mut res=Vec::new();

    while n>=p*p{
        if n%p==0{
            res.push(p);
            n=n/p;
            
        }
        else{
            p=p+1;
        }
    }
    res.push(n);
    res


}