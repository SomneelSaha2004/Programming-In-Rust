/*
Rust is statically typed
Compiler can usually infer types
Two kinds of types:
    1. Scalar : 
        . Integer --> i8,i32,i64
        . Float --> f32 and f64 eg. let f:f32=2.0
        . Boolean --> bool b:bool=true
        . Character ---> 
    2. Compound : 
        . Tuples (val1,val2...valn) can store a sequence of values of different types
        . Arrays have fixed length and store values of a certain type
*/
fn main(){
    let mut a:i8=2;//Integer
    let b:bool=true;
    let t:(i32,f64,bool)=(1,2.0,true);
    let (x,y,z)=t;
    println!("x={},y={},z={}",t.0,t.1,t.2);
    let mut numbers=[1,2,3,4,5,6];
    numbers[0]=5;
    print!("{}",numbers[5]);
}