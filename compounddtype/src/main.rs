// array , tuples, slices and strings (sliced string)
// arrays -> fixed element {homogeneous types}
fn main() {
    let numbers:[i32;5]=[1,2,3,4,5];
    println!("numbers{:?}",numbers);
    let fruits:[&str; 3]=["Apple","banana","Orange"];
    println!("fruits:{:?}",fruits);
    println!("Hello, world!");

    // tuple -> multiple datatype ,also array 
    let human:(String,i32,bool) =("Alice".to_string(),30,false);
    println!("Human tuple! {:?}",human);

    // slices :[1,2,3,4,5] {contigionous array we dont need to jump between memory and memery allocation}
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("number:{:?}",number_slices);
    
    //Strings vs Strings slices
    //Strings [growable,mutable ,owned string type] memory allocation is done in heap in backend
    // By default any data in rust is inmutable 
    // adding mut making it to mutable datatype
    // Expandable Dynamic
    let mut stone_cold: String = String::from("Hell,"); //stored in heap memory 
    stone_cold.push_str("yeah!");
    println!("Stone cold Says:{}",stone_cold);

    //&str(String slice)
    //immutable reference string without copy same variables without taking ownership of the data
    // no number of bytes to the stack so stack remember the bytes and react very quickly
    // Quicker 
    // stack memory 
    let string: String = String::from("Nandu world");
    let slice: &str = &string;
    println!("Slice value {}",slice);

}
// all the variables are local bound 