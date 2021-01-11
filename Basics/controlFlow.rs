//if statement requires no parenthesis
fn main(){
    let mut num=3;
    if isEven(num){println!("Even");}
    else{println!("Odd");}
    println!("Factors of 24 are : {:?}",findFactors(24));
}
fn isEven(num:i32)->bool{
    if num%2==0{
        return true;
    }
    return false;
}
fn findFactors(num:i64)->Vec<i64>{
    let mut v=Vec::new();
    for i in 2..10{
        if(num%i==0){
            v.push(i);
        }
    }
    return v;
}