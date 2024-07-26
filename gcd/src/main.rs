fn main() {
    
    println!("{:?}",get_gcd(0, 60));
}

fn get_gcd(mut num1:u32,mut num2:u32)->u32{
    while num2 !=0{
            let temp=num2;
            num2=num1%num2;
            num1=temp;
    }
    return num1;

 
}
