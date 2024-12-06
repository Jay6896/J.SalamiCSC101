fn main() 
{
    // mutable array
    let mut colors = ["red", "green","yellow", "white"];

    println!("\n Original array = {:?}", colors);

    // mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}", sliced_colors);

    // change the value of the orignal slice at the first index
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors);
}
