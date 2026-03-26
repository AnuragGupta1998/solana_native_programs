#[derive(Debug,Clone)]
struct User{
    is_male:bool,
    age:u32,
    name:String,
}

fn main(){
    let u1 = User{
        is_male:true,
        age:26,
        name:String::from("zhangsan"),
    };

    let u2 = &u1;   //or we can use u1.clone()
    //try to avoid .clone() in real projects because of performance issues and it takes lots of time to cloning large data
    // let u2 = u1.clone();

    println!("{:?},{:?}",u1,u2);
}
