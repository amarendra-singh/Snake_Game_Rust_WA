
#[derive(Debug)]
enum PersonId{
    Passport,
    IdentityCard,
}
struct Person {
    name : String,
    last_name : String,
    age : u32,
    id: PersonId,
}

impl Person{
    fn new () -> Person{
        Person { 
            name:"Ram".to_string(), 
            last_name:"Sharma".to_string(), 
            age: 32,
            id:PersonId::IdentityCard
        }
    }


    fn from(name: String, last_name: String, age: u32, id:PersonId) -> Person{
        Person{
            name,
            last_name,
            age,
            id,
        }
    }

        fn change_age(&mut self, new_age:u32){
            self.age = new_age;
    }
}



    fn main(){
        let mut person = Person::new();
        let person_2 = Person::from(
        String::from("John"),
        String::from("Snow"),
        35,
        PersonId::Passport
        );

        person.change_age(50);
        println!("{} {} {}", person.name, person.last_name, person.age);
        println!("{} {} {}", person_2.name, person_2.last_name, person_2.age);
    }
