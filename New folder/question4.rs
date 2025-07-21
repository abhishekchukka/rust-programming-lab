fn main() {
 let scope_var = 10;
    println!("Before inner scope: {}", scope_var);
    {
        let inner_var = 20;
        println!("Inside inner scope: {}", inner_var);
        let scope_var = 30; 
        println!("Shadowed inside scope: {}", scope_var);
    }
    println!("After inner scope: {}", scope_var);
}