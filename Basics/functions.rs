fn main(){
    println!("{}",add(2,3));
    helloName("Somneel Saha".to_string());
}
fn helloName(name:String){
    println!("Hello {}",name);
}
fn add(a:i32,b:i32)->i32{
    a+b
}
/*
Use the fn keyword to define a function
eg function definition
fn <function name>(<var1>:<type>,<var2>:<type>..<varn>:<type>)-><return type>(optional)
*/