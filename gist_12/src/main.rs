#[derive(Debug)]
struct DessertDetails {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    sweetnewss_level: u8,
}

#[derive(Debug)]
struct SushiRollDetails {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    has_wasabi: bool,
    #[allow(dead_code)]
    ingredients: Vec<String>,
}

#[derive(Debug)]
enum JapaneseFood {
    // Simple and tasty ramen! 🍜
    Ramen,
    // A single piece of nigiri! 🍚
    Nigiri { topping: String },
    // A flavorful onigiri! 🍙 Holding the filling!
    Onigiri(String),
    // Our fancy sushi roll variant, holding the SushiRollDetails struct! ✨
    SpecialRoll(SushiRollDetails),
    // Delicious and crispy tempura! 🍤 Holding the type!
    Tempura(String),
    Dessert(DessertDetails),
}

fn estimate_order_cost(order: Vec<JapaneseFood>) -> f64 {
    let mut total_cost: f64 = 0.0;
    for item in order {
        let cost = match item {
            JapaneseFood::Ramen => 12.00,
            JapaneseFood::Nigiri { topping } => 3.50,
            JapaneseFood::Onigiri(details) => 2.50,
            JapaneseFood::SpecialRoll(details) => 15.00,
            JapaneseFood::Tempura(details) => 8.00,
            JapaneseFood::Dessert(details) => 6.00,
            _ => 0.00,
        };
        total_cost += cost; // Corrected line: removed the '*'
    }
    total_cost
}

fn main() {
    let dinner = JapaneseFood::Ramen;
    // Salmon nigiri! 😋
    let my_sushi = JapaneseFood::Nigiri {
        topping: "Salmon".to_string(),
    };
    // A yummy tuna onigiri! 🍙
    let my_riceball = JapaneseFood::Onigiri("Tuna Mayo".to_string());
    // Our amazing special sushi roll! 🤩
    let dragon_roll = JapaneseFood::SpecialRoll(SushiRollDetails {
        name: "Dragon Roll".to_string(),
        has_wasabi: true,
        ingredients: vec![
            "Eel".to_string(),
            "Avocado".to_string(),
            "Cucumber".to_string(),
        ],
    });
    // Some crispy shrimp tempura! 🍤
    let fried_goodness = JapaneseFood::Tempura("Shrimp".to_string());
    let mochi = JapaneseFood::Dessert(DessertDetails {
        name: "mochi".to_string(),
        sweetnewss_level: 9,
    });
    println!("Dinner: {:?}", dinner);
    println!("Sushi: {:?}", my_sushi);
    println!("Onigiri: {:?}", my_riceball);
    println!("Special Roll: {:?}", dragon_roll);
    println!("Tempura: {:?}", fried_goodness);
    println!("Mochi: {:?}", mochi);
    let my_order: Vec<JapaneseFood> = vec![
        JapaneseFood::Ramen,
        JapaneseFood::Nigiri {
            topping: "Tuna".to_string(),
        },
        dragon_roll,    // Re-use the dragon_roll from earlier
        fried_goodness, // Re-use fried_goodness
        mochi,          // Add the new dessert
        JapaneseFood::Onigiri("Salmon".to_string()),
    ];

    let total = estimate_order_cost(my_order);
    println!("Your estimated total order cost is: ${:.2}", total);
}
