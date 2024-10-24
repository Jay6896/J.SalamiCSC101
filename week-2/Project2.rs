fn main ()
{
   println!("Chief Donatus and Sons Ltd is downsizing and \nre adjusting their products sales due to an ongoing \nrecession. You have been consulted to write a rust \nprogram to calculate the sum and average of the \nfollowing sales record.");
   let tq:f64 = 2.0;
   let t:f64 = 450000.00;
   let mq:f64 = 1.0;
   let m:f64 = 1500000.00;
   let hq:f64 = 3.0;
   let h:f64 = 750000.00;
   let dq:f64 = 3.0;
   let d:f64 = 2850000.00;
   let aq:f64 = 1.0;
   let a:f64 = 250000.00;
   println!("Sum = Total Summation of (Qty*Amount)");
   println!("\nSolution:");

    // formula of summation of sales
    let ss = (tq * t) + (mq * m) + (hq * h) + (dq * d) + (aq * a);
    println!("Sum of Sales ={}", ss);

     // formula of average of sales
    println!("Average = Total Summation of (Qty*Amount) / Total Qty ");
    let aos = ss / (tq + mq + hq + dq + aq);
    println!("Average of Sales ={}", aos);

}