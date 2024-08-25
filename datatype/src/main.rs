//primitive data types 
//int float bool char 

//integer 
//rust has signed (+ and -) and unsigned integer (only +) type of different sizes
//i8 i16 i32 i64 i128: signed integer(+ and -)
//u8 u16 u32 u64 u128: unsigned integer (+)
fn main() {
    let x: i32 = -42;
    let y:u64 = 100;
    let is_showing: bool = true;
    let letter: char = 'a';
    println!("Signed integer:{}",x);
    println!("unSigned integer:{}",y);
    println!("Is it snowing:{}",is_showing);
    println!("show letter:{}",letter);

}


