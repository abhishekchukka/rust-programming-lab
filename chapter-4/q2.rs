fn div3(a:i32)->bool {
   if a%3==0{
       return true;
   }
   return false;
}
fn div4(a:i32)->bool {
   if a%4==0{
       return true;
   }
   return false;
}
fn main(){
    let i=24;
    if div3(i) && div4(i){
        println!("2");
    }
    else if div3(i) || div4(i)
{
     println!("1");
}    
else{
    println!("-1");
}
    
}
