fn main()
{
   println!("Ms. Anjola Olowokere has recently acquired a brand \nnew TV set. The TV was bought for N510,000. The \nvalue of the TV was depreciated by 5% per annum. \nWrite a rust program to find the value of the TV after \n3 years. (Depreciation means the reduction of value \ndue to use and age of the item).");
   println!("Depreciation = A = P[1 - (R/100)]n");
   println!("\nSolution:");

    let p:f64 = 510000.0;
    let r:f64 = 5.0;
    let t:f64 = 3.0;


    // formula for Depreciation
    let dp = p * (1.0 - (r / 100.0)) * t;
    println!("Depreciation ={}", dp);

}