
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    weight: Option<usize>
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {

    let person =  Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        weight: Some(76)
    };

    println!("{:?}", person);
    println!("Full name: {}", person.full_name());

}