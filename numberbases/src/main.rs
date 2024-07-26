fn change_to_base_two(mut n:u32){
    // n/2
    // 53/2
    let mut res=Vec::new();
    
    while n !=0{
        let remainder=n%2;
        n=n/2;
        res.push(remainder);
    }
    res.reverse();
    println!("{:?}",res);


}
fn main() {
   change_to_base_two(8)
}
