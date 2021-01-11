fn main(){
    let mut i=0;
    while i<10{
        println!("{}",i);
        i+=1;
    }
    let nums=[1,2,3,4,5,6,7,8,9];
    for i in nums.iter(){
        println!("{}",i);
    }
}
/*
loops can be implemented using loop, for and while keywords
loop - no condition
*/