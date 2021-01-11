//By defualt variables are immutable 
fn main(){
    let mut a=1;
    println!("Value if a is {}",a);
    //Use curly braces in strings to insert variable values in strings during printing
    a=2;
    println!("Value if a is {}",a);
    let mut s="Hello World!";
    println!("{}{}",s,a);
}
//let to declare a variable(immutable by default)
// add mut after let to make a variable mutable