// Rust program for solving quadratic equations

use std::io;

fn main() 
{
    println!("Given the values of a, b, and c, find the roots of a \nquadratic equation using Rust program.");
    println!("The Quadratic Equation structure is: \nnumber times x² + number times x + number = 0");
    
    println!("\nEnter the number that would be multiplied by x²");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Input is not a number");
    let a:f32 = a.trim().parse().expect("Input not an interger");

    println!("\nEnter the number that would be multiplied by x");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Input is not a number");
    let b:f32 = b.trim().parse().expect("Input not an interger");
    

    println!("\nEnter the last number");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Input is not a number");
    let c:f32 = c.trim().parse().expect("Input not an interger");

    let output1for:f32 = (b * b) - (4.0 * a * c);
    let out1sqr = output1for.sqrt();

    let output1:f32 = ((- b) + out1sqr) / (2.0 * a);

    let output2for:f32 = (b * b) - (4.0 * a * c);
    let out2sqr = output2for.sqrt();

    let output2:f32 = ((- b) - out2sqr) / (2.0 * a);

    if output1for > 0.0
    {
       println!("The Equation has two distinct roots");
       println!("x is = ({} and {})",output1, output2);
    }

    if output1for == 0.0
    {
       println!("The Equation has exaclty one real root");
       println!("x is = ({} and {})",output1, output2);
    }

    if output1for < 0.0
    {
       println!("The Equation has no real roots");
       println!("x is = ({} and {})",output1, output2);
    }    
}
