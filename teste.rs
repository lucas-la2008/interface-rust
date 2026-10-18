fn main(){
    let mut palavra = String::from("Ola");
    let ptr1 = &palavra as *mut String;
    let ptr2 = &palavra as *mut String;
    unsafe{
    println!("Ponteiro 1: {}\nPonteiro 2: {}",ptr1 as usize,*ptr2 as usize);
}}