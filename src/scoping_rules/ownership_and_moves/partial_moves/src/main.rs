fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // `name` 从 person 中移走，`age` 只是引用（通过ref）
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error person部分值已经移动
    // println!("The person struct is {:?}", person);

    println!("The person's age from person struct is {}", person.age);
}
