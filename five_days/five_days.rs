fn main() {
    let a: i32 = 3;
    if  a == 3 {
        println!("A equal to three");
    } else {
        println!("A is not equal to three");
    }

    if a != 4 {
        println!("The value of a is {}", a);
    }

    let c: i32 = 6; 
    println!("The value of c is {}", c);

    if c == 1 {
        println!("C equal to one");
    }  else if c == 3 {
        println!("C equal to three");
    }  else if c == 4 {
        println!("C equal to four");
    } else {
        println!("C equal to six");
    }

    let number = if c == 6 {
        6
    } else {
        7
    };

    println!("The value of number is {}.", number);
}


