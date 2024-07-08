fn euclidean_algo(mut a:u32,mut b:u32)->u32{
    
    while b !=0{
        let temp=b;
        b=a%b;
        a=temp;
    }
    a

    
    

}

fn main() {
    println!("{:?}",euclidean_algo(3,10));
    let rem=|mut a,mut b|{
        while b!=0{
            let temp =b;
            b=a%b;
            a=temp;
        }
        println!("{a}");
        
        
        
    };
    rem(3,10)
}
