use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize, Debug,Clone,)]
struct User{
    name: String,
    age: u8,
    email: String,
}

fn main() {
    // println!("Hello, world!");
    let user1 = User{
        name: String::from("Anurag"),
        age: 30,
        email: String::from("anurag@gmail.com"),
    };

    //serialize_data from Struct to String way 1
    let string_data = serde_json::to_string(&user1).unwrap();
    println!("Serialized Data: {:?}", string_data);

    //serialize_data from Struct to String way2
    let serialize_data = serde_json::to_string(&user1);

    match serialize_data{
        Ok(data)=> println!("Serialized Data: {}", data),
        Err(e)=> println!("Error in serialization: {}", e),
    }

    // deserialize_data from String to Struct
    let data:User = serde_json::from_str(&string_data).unwrap();
    println!("Deserialized Data: {:?}", data);


    // deserialize_data from String to Struct way2
    let d1 = serde_json::from_str::<User>(&string_data);
    match d1{
        Ok(user)=> println!("Deserialized Data: {:?}", user),
        Err(e)=> println!("Error in deserialization11: {}", e),
    }
    
    //deserialization from String to Struct way3
    let s = String::from("{\"name\":\"An\", \"age\":10, \"email\":\"An@123\"}");
    // let res:Result<User,serde_json::Error> = serde_json::from_str(&s); //OR
    let res = serde_json::from_str::<User>(&s);
    
    match res{
        Ok(user)=> println!("Deserialized Data: {:?}", user),
        Err(e)=> println!("Error in deserialization22: {}", e),
    }
    
    //deserialization from String to Struct way4
    let s1 = String::from("{\"name\":\"Anur\", \"age\":5, \"email\":\"Anur@123\"}");
    let res2 = serde_json::from_str::<User>(&s1).unwrap();
    println!("Deserialized Data2: {:?}", res2);

}
