//trait is like interface in java or javascript
trait Shape {
    fn area(&self )->f32;   
}

struct Rect{
    height:f32,
    width:f32,
}

impl Shape for Rect{
    fn area(&self)-> f32{
        return self.height * self.width;
    }
}

struct Circle{
    radius:f32,
}

impl Shape for Circle{
    fn area(&self) ->f32{
        return 3.14*self.radius * self.radius;
    }
}

//it can print the area who implemented Shape trait area function
fn print_area_of_shape<T: Shape>(s:T){

    println!("the area of shape {}",s.area());
}

fn print_area2<T>(s:T)->f32 
where T:Shape
{
   return s.area();
}

fn print_area3(s:impl Shape){
    println!("{}",s.area());
}


fn main(){

    let r1 = Rect{
        height:10.5,
        width:5.5,
    };
    let r2 = Rect{
        height:100.5,
        width:50.5,
    };

    println!("area of rectangle {}",r1.area());

    print_area_of_shape(r1);
    print_area_of_shape(r2);

    let c1 = Circle{
        radius:10.5,
    };
   println!("{}",print_area2(c1));  

    let c2 = Circle{
        radius:10.5,
    };
    print_area3(c2);

}