fn main()
{
	println!("The Ibeju Local Government Chairman has received a \nmortgage loan of N520,000,000 from Sterling Bank for \nthe construction of the Lekki Free Trade Zone \nindustrial estate. Find the compound interest for 5 \nyears at 10% per annum compounded annually.");
    println!("\nSolution:");
    // formula
    println!("A = P[1 + (R/100)n");
    let p:f64 = 520000000.0;
    let r:f64 = 10.0;
    let t:f64 = 5.0;

    // amount calculation
    let a = p * (1.0 + (r / 100.0)) * t;
    println!("The amount is {}", a);

    println!("CI = A - P");
    // compound interest calculation
    let ci = a - p;
    println!("The Compound Interest is {}", ci);
}