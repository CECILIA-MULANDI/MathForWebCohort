fn euclidean_algo(mut a:u32,mut b:u32)->u32{
 
    while b !=0{
        let temp=b;
        b=a%b;
        a=temp;
    }
    a   

}

fn lcm(a:u32,b:u32)->u32{
    let res=(a*b)/euclidean_algo(a,b);
    res

}
fn main() {
    println!("The lcm is {:?}",lcm(3,10));
}
