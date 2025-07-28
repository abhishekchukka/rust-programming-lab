fn facts(a:i32)->i32 {
    let mut pr:i32=1;
    for j in 2..a+1{
        pr=pr*j;
    }
    pr
}
fn main(){
    let i=5;
    println!("product :{}", facts(i));
}
