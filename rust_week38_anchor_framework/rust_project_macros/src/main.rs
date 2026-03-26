
#[derive(Debug)]
struct Rect{
    width: u32,
    height: u32,
}

//implementation of struct
impl Rect{
    fn area(&self) ->u32{
        self.width * self.height
    }

    pub fn display_value(&self){
        println!("width: {}, height: {}", self.width, self.height);
    }
    pub fn display(){
        println!("This is a rectangle");
    }
}

fn main(){
    let r1 = Rect{
        width:30,
        height: 50,
    };
println!("The rectangle is: {:?}", r1);

    let ans = r1.area();
    println!("The area of the rectangle is: {}", ans);

    r1.display_value();
    Rect::display();
}