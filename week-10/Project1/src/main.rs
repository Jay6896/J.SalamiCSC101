struct Laptops
{
   brand:String,
   price:u32,
   no:u32,
}

impl Laptops 
{
    fn pricetotal(&self)->u32 
    {
       self.price * self.no
    }
}

fn main() 
{
    println!("Mr Ogbeifuna runs a series of electronics \nshops at Alaba International Market in Lagos. \nHe has recently received a consignment of 30 \nlaptop devices at the following cost: \n10 HP Laptops as 650,000 each, \n6 IBM Laptops at 755,000 each, \n10 Toshiba Laptops at 550,000 each, \nAnd 4 Dell Laptops at 850,000 each. \n\nUsing your knowledge of Rust struct and \nmethod, calculate the total cost supposing a \ncustomer purchases 3 from each brand.\n\n");
    let brand1 = Laptops
    {
        brand:String::from("HP"),
        price:650000,
        no:3
    };

    let brand2 = Laptops
    {
        brand:String::from("IBM"),
        price:755000,
        no:3
    };

    let brand3 = Laptops
    {
        brand:String::from("Toshiba"),
        price:550000,
        no:3
    };

    let brand4 = Laptops
    {
        brand:String::from("Dell"),
        price:850000,
        no:3
    };

    displayinfo(brand1);
    displayinfo(brand2);
    displayinfo(brand3);
    displayinfo(brand4);


println!("Total Cost of 3 Laptops From each Brand:8415000");

}

fn displayinfo(brandinfo:Laptops)
{
   println!("Laptop Brand Name:{} \nPrice:{} \nAmount Bought:{}", brandinfo.brand, brandinfo.price, brandinfo.no);
   println!("Total Price of 3 {} Laptops:{}\n\n",brandinfo.brand, brandinfo.pricetotal());
}