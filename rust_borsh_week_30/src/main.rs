use borsh::{BorshDeserialize, BorshSerialize, to_vec};

#[derive(Debug, BorshDeserialize, BorshSerialize)]

struct User2 {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct User<'a> {
    name: &'a str,
}

impl<'a> User<'a> {
    fn new(name: &'a str) -> Self {
        User { name }
    }
}
fn main() {
    let u2 = User2 {
        name: String::from("Anurag"),
        age: 27,
    };
    let mut store_bytes = Vec::new();
    u2.serialize(&mut store_bytes).unwrap();
    println!("serialize bytes of struct{:?}", store_bytes);
    let bytes2= to_vec(&u2).unwrap();
    println!("serialize bytes of struct using to_vec{:?}", bytes2);

    let deserialized_user = User2::try_from_slice(&store_bytes).unwrap();
    println!("deserialized struct{:?}", deserialized_user);

    let name = String::from("John");
    let u1 = User { name: &name };
    let user = User::new(&name);
    println!("{:?}", u1);
    println!("{}", user.name);
}
