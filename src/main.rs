use std::io;

fn main() {
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
    // ch03-01

    // let mut x = 5;
    // println!("The value x is {x}");
    // x = 6;
    // println!("The value x is {x}");

    // let space = "    ";
    // let space = space.len();
    // println!("space is: {space}");        

    // let mut space = "   ";
    // space = space.len();
    // println!("space is : {space}");

    // let x = 6;
    // println!("the x is: {x}");
    // let x = x + 2;
    // println!("the x is: {x}");
    // {
    //     let x = x +10;
    //     println!("in new scope: the x is: {x}");
    // }
    // println!("after scope: the x is: {x}");

    // ch03-02
    // https://doc.rust-lang.org/book/ch03-02-data-types.html

    // Data types

    // let guess = "42".parse().expect("Not a number");
    // println!("the guess is: {guess}")

    // Scaler types
    // Integer type

    // compound types
    // tuple

    // let tup = (500, 6.1 , 1);
    // let (x, y, z) = tup;

    // println!("the y is: {y}");

    // let tup: (i32, f64, u8) = (500, 6.5, 1);
    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let one = tup.2;

    // println!("the numbers {one}");

    // The Array Type
    // let array: [i32; 5] = [1,2,3,4,5];
    // let first = array[0];
    // println!("the array is: {first}");

    // Invalid Array Element Access
    // let a = [1,2,3,4,5];
    // println!("please enter the index");

    // let mut index = String::new();

    // io::stdin()
    // .read_line(&mut index )
    // .expect("failed to read the line");

    // let index: usize = index
    // .trim()
    // .parse()
    // .expect("please enter a number");

    // let element = a[index];
    // println!("the element is: {element}");

    // https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
    // ch03-03




}