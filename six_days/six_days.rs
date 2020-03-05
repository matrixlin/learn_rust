fn main() {
    //loop {
    //    println!("hello world!");
    //}

    let mut counter = 0;
    let result = loop {
        counter += 1; // counter = counter + 1;

        if counter == 20 {
            break counter;
        }
    };
    println!("The value of result is {}", result);
    
    let mut number = 10;
    while number != 0 {
        println!("The value of number is {}", number);

        number -= 1; // number = number -1;
    }

    println!("over!!");

    // range type
    for x in 1..101 {
        println!("the value of x is {}", x);
    }

    for y in 1..102 {
        println!("the value of y is {}", y);
    } 
}


