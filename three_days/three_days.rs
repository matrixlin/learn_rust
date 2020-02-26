fn can_speak() {
    println!("Hi, this is a function");
}
fn main() {
    can_speak();
    another_function();

    fn sleep() {
        println!("sleep sleep");
    }

    sleep();
    my_function(6); //argument
    add(4, 5);
}

fn another_function() {
    println!("another function");
}

//parameter
fn my_function(x: i32) {
    println!("The value of x is {}", x);
}

fn add(x: i32, y: i32) {
    println!(" x + y = {}", x + y);
}

