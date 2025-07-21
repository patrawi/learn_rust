// Welcome to the coding challenge!
// Your goal is to refactor this code to use an Enum for the animal type.
// This will make the code more robust and prevent errors from using incorrect strings.
// Follow the instructions in the comments below.

// #[derive(Debug, Clone)] // We'll need to add another trait here later.
#[derive(Debug, Clone, PartialEq)] // Add PartialEq to allow for direct comparison
struct Animal {
    #[allow(dead_code)]
    name: String,

    #[allow(dead_code)]
    age: u8,

    // TODO: Instruction 2: Change the type of `r#type` from String to the new AnimalType enum.
    r#type: AnimalType, // This should be changed.
}

// TODO: Instruction 1: Define a public enum called `AnimalType`.
// It should have two variants: `Cat` and `Duck`.
// Derive the `Debug`, `Clone`, and `PartialEq` traits for it. This will allow us to print, copy, and compare it.
#[derive(Debug, Clone, PartialEq)]
enum AnimalType {
    Cat,
    Duck,
}
impl Animal {
    // TODO: Instruction 3: Update the `new` function.
    // It should now accept an `AnimalType` as a parameter instead of a string slice for the type.
    // The `new` constructor return 👇 itself call `Self`.
    fn new(name: &str, age: u8, animal_type: AnimalType) -> Self {
        Animal {
            name: name.to_owned(),
            age,
            // This will need to be updated to use the enum.
            r#type: animal_type,
        }
    }

    // This function is a good candidate to be removed after refactoring,
    // but for now, let's update it.
    fn new_cat(name: &str, age: u8) -> Self {
        Animal {
            name: name.to_owned(),
            age,
            // TODO: Instruction 4: Update `new_cat` to use the `AnimalType::Cat` variant.
            r#type: AnimalType::Cat,
        }
    }

    // TODO: Instruction 5: Update the `static_say` function.
    // It should now accept a reference to an `AnimalType` enum.
    // The match statement should then check for the enum variants (`AnimalType::Cat`, `AnimalType::Duck`).
    pub fn static_say(animal_type: &AnimalType) -> &str {
        match animal_type {
            // This &str is bad practice, we will need enum here (later).
            AnimalType::Cat => "meaowww",
            AnimalType::Duck => "quackk",
        }
    }

    // With &self 👇 method.
    pub fn say(&self) -> &str {
        // TODO: Instruction 6: Update the `say` method.
        // It should now pass a reference to `self.r#type` to the `static_say` function.
        // No need to call `.as_str()` anymore.
        let animal_type = &self.r#type;
        Animal::static_say(animal_type)
    }
}

fn main() {
    // TODO: Instruction 7: Update the `main` function to use the new enum.
    // When creating animals, pass the enum variants like `AnimalType::Duck`.
    let animal = Animal::new("foo", 42u8, AnimalType::Duck);
    println!("animal: {:#?}", animal);
    let mut test = "Cat";

    // Call say via static method.
    // This should now use the enum variant.
    let static_say_str = Animal::static_say(&animal.r#type);
    println!("static_say_str: {:#?}", static_say_str);

    // Also can new cat 👇 like this.
    let cat = Animal::new_cat("bar", 24u8);
    println!("cat: {:#?}", cat);

    // Call say via method itself.
    let say_str = cat.say();
    println!("say_str: {:#?}", say_str);

    // Or via Animal 😳
    println!("Animal::say: {:#?}", Animal::say(&cat));

    // You can also clone after derive Clone above 👆
    let mut duck = cat.clone();
    duck.name = "duck the duck".to_owned();
    duck.age = 13;
    // How would you change the duck's type to be `AnimalType::Duck`? Add that line here.
    duck.r#type = AnimalType::Duck;
    //  Destructing from struct.
    let Animal { age, .. } = cat;
    println!("age: {:#?}", age);

    //  Match struct where animal
    // TODO: Instruction 8: Update the match statement to use the enum.
    // You should be able to match directly against the enum variants.
    // For example: `Animal { r#type: AnimalType::Cat, .. }`
    match &duck {
        // Match age at 24
        Animal { age: 24, .. } => println!("match age at 24 : {:#?}", age),

        // Match age between 30-50 range.
        Animal { age: 30..=50, .. } => println!("match age between 30-50 : {:#?}", age),

        // Guard name equal to "foo"
        Animal {
            r#type: AnimalType::Cat,
            ..
        } => println!("It is a cat"),
        Animal {
            r#type: AnimalType::Duck,
            ..
        } => println!("It is a duck"),

        // Other age.
        _ => println!("age not in range"),
    }
}
