fn main() 
{

    // while true

    let mut x = 0;
    loop 
    {
      x+=1;
      println!("x={}", x);

      if x ==15
      {
        break;
      }
    }
    let mut count = 0;

    for num in 1..21 
    {
       if num > 10 
       {
         println!("{:?}", num);
         continue;
       }
       count+=1;
    }
       println!("The count of values greater than 10 (between 1 and 20) is: {}", count);
       //outputs 10
}
