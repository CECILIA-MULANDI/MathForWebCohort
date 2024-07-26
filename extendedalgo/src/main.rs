fn extended(a:i64,b:i64)->(i64,i64,i64){
    if b==0{
        return (a,1,0);
    }
    let (gcd,x1,y1)=extended(b,a%b);
    let x=y1;
    let y=x1-(a/b)*y1;
    (gcd,x,y)
}
fn main() {
    let (gcd,x,y) = extended(3,10);
    println!("the gcd is :{},x is:{},y is:{} ",gcd,x,y)
}
