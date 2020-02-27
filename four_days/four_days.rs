fn main() {
   //let c =  ( let a = 10 );
   let d  = 1 + 2;
   let f = 6;

   a();
   println!("hello");
   let value = {
       let x = 5;
       x + 5
   };
   let num = number();
   println!("The value of num is {}", num);
   println!("value is {}", value);
   let zxc = abc(1, 2);
   println!("The value of zxc is {}", zxc);
}

fn hello() {

}

fn a() {

}

fn number() -> i32 {
    5
}

fn abc(x: i32, y: i32) -> i32 {
    x + y
}


