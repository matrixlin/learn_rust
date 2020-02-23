use std::{i8, i16, i32, i64, i128, u8, u16, u32, u64, u128,isize, usize, f64, f32};

fn main() {
    let number = 10; //i32

    let number1: i8 = 10; //i8

    let number3: u8 = 111; //u8

    let number4 = 10i8;  //i8
    let number5 = 123u32; //u32
    let number6: i32 = 0xff; // Hex
    let number7: u128 = 0o11; //Octal
    let number8: u64 = 0b1010; //Binary

    let number9: u8 = 255; //error  0 ~ 2^n -1  0 ~ 255
    let number10: i8 = 127; // -128 ~ 127

    println!("i8 smallest value: {}, largest value: {}",i8::MIN, i8::MAX); 
    println!("i16 smallest value: {}, largest value: {}",i16::MIN, i16::MAX); 
    println!("i32 smallest value: {}, largest value: {}",i32::MIN, i32::MAX); 
    println!("i64 smallest value: {}, largest value: {}",i64::MIN, i64::MAX); 
    println!("i128 smallest value: {}, largest value: {}",i128::MIN, i128::MAX); 
    println!("i8 smallest value: {}, largest value: {}",i8::min_value(), i8::max_value());
    
    //Floating point type
    let a = 11.11;  //f32
    let c: f32 = 1.11; //f32
    let d: f64 = 1.1111; // f64
    let f = 1.1f32;  //f32
    let b = 1.112341234f64; //f64
   
    println!("f32 smallest value: {}, largest value: {}",f32::MIN, f32::MAX); 
    println!("f64 smallest value: {}, largest value: {}",f64::MIN, f64::MAX);

    let sum = a + c;
    println!("THe value of sum is {}", sum);

    let dou = 12 * 10;
    println!("THe value of dou is {}", dou);

    let abc = 120 / 10; //12
    let ax  = 10 % 3;  // 1
    println!("abs is {}, as is {}", abc, ax);
   

    //Boolean type
    let t = true;
    let b: bool = false;

    //Character type
    let ch = 'a';
    let cha: char = 'a';

    //Tuple type
    let tuple1  = (1, 2, 3, 3.3);
    let tuple2: (i32, f64, u8) = (12, 44.3, 1);

    let tuple3 = (1, 2.2, 3.3);
    let (x, y, z) = tuple3;
    println!("THe value of y is {}", y);
    println!("THe value of z is {}", tuple3.2);


    // Array type

    let aa = [1, 2, 3, 4];
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let f = a1[2];
    println!("The value of f is {}", f);


}
