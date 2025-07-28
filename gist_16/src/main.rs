use std::marker::PhantomData;

// --- Marker Types for States ---
// These don't hold data, they just act like labels for our states.
#[derive(Debug, Clone, Copy)] // So we can print and copy them easily
struct Sleeping;

#[derive(Debug, Clone, Copy)]
struct Awake;

#[derive(Debug, Clone, Copy)]
struct Hunting;

#[derive(Debug, Clone)]
struct Full {
    food_eaten: String,
}
// --- Our Cat Struct ---
// It's generic! The `State` tells us what condition the cat is in.
// PhantomData tells Rust "we care about this `State` type, even if we don't store data of that type".
#[derive(Debug)]
struct Cat<S> {
    name: String,
    _state: S, // We use PhantomData because State is just a marker
}

// --- Transitions ---

// Implementation block ONLY for Cats that are Sleeping
impl Cat<Sleeping> {
    // This function takes a Cat<Sleeping>...
    fn wake_up(self) -> Cat<Awake> {
        // ...and returns a NEW Cat<Awake>!
        println!("{} stretches and yawns... now awake! ☀️", self.name);
        Cat {
            name: self.name, // Keep the same name
            _state: Awake,   // Mark it as Awake now
        }
    }

    // Notice: There's no `lull_to_sleep` or `eat` function here! Sleeping cats don't eat!
}
impl Cat<Hunting> {
    fn pounce(self) -> Cat<Awake> {
        println!(
            "{} pounces! ... and comes back with a toy mouse. 🐭 ",
            self.name
        );
        Cat {
            name: self.name,
            _state: Awake,
        }
    }
    fn lull_to_sleep(self) -> Cat<Sleeping> {
        // ...and returns a NEW Cat<Sleeping>!
        println!(
            "{} got tired from hunting and curled up to sleep. 💤... 😴",
            self.name
        );
        Cat {
            name: self.name,  // Keep the same name
            _state: Sleeping, // Mark it as Sleeping now
        }
    }
}
// Implementation block ONLY for Cats that are Awake
impl Cat<Awake> {
    // This function takes a Cat<Awake>...
    fn play(self) -> Self {
        println!("{} playfully chases a laser pointer! 😼", self.name);
        Cat {
            name: self.name, // Keep the same name
            _state: Awake,   // Mark it as Sleeping now
        }
    }
    fn lull_to_sleep(self) -> Cat<Sleeping> {
        // ...and returns a NEW Cat<Sleeping>!
        println!("{} curls up and starts snoozing... 😴", self.name);
        Cat {
            name: self.name,  // Keep the same name
            _state: Sleeping, // Mark it as Sleeping now
        }
    }
    fn start_hunting(self) -> Cat<Hunting> {
        println!(
            "{} spots a target and silently enters hnting mode... stalking...",
            self.name
        );
        Cat {
            name: self.name,
            _state: Hunting,
        }
    }

    // *** NEW *** Let the awake cat eat!
    // It takes any type that can be turned 'Into' a String for the food.
    // It consumes the Cat<Awake> and returns it, still Awake.
    fn eat(self, food: impl Into<String>) -> Cat<Full> {
        // Here we convert the input 'food' into a real String
        let food_item: String = food.into();
        println!("{} happily munches on some {}!", self.name, food_item);

        // The cat ate, but is still awake, so we return the same cat (type).
        Cat {
            name: self.name,
            _state: Full {
                food_eaten: food_item,
            }, // The food is now stored in the state
        }
    }
}

impl Cat<Full> {
    fn digest(self) -> Cat<Awake> {
        println!(
            "{} has finished digesting the delivious {} and is awake again",
            self.name, self._state.food_eaten
        );
        Cat {
            name: self.name,
            _state: Awake,
        }
    }
}

// --- How to create our first Cat ---
impl Cat<Sleeping> {
    // A function to create a new cat, starting in the Sleeping state.
    fn new_sleeping_cat(name: String) -> Self {
        println!("A new cat, {}, appears, already asleep...", name);
        Cat {
            name,
            _state: Sleeping,
        }
    }
}

fn main() {
    // Start with a new sleeping cat named "Mittens"
    let mittens = Cat::new_sleeping_cat("Mittens".to_string());
    println!("*️⃣  Initial state: {:?}\n", mittens);

    // Wake up the cat
    let mittens = mittens.wake_up();
    println!("➡️  After waking up: {:?}\n", mittens);

    // An awake cat can play
    let mittens = mittens.play();
    println!("➡️  After playing: {:?}\n", mittens);

    // An awake cat can go hunting
    let mittens = mittens.start_hunting();
    println!("➡️  After starting to hunt: {:?}\n", mittens);

    // A hunting cat can pounce and return to being awake
    let mittens = mittens.pounce();
    println!("➡️  After pouncing: {:?}\n", mittens);

    // Now that the cat is awake again, it can eat. This transitions to the `Full` state.
    let mittens = mittens.eat("fancy salmon");
    println!("➡️  After eating: {:?}\n", mittens);

    // A full cat can digest its food to become awake again.
    let mittens = mittens.digest();
    println!("➡️  After digesting: {:?}\n", mittens);

    // Finally, an awake cat can be lulled to sleep.
    let mittens = mittens.lull_to_sleep();
    println!("➡️  After falling asleep again: {:?}\n", mittens);

    // This line will now fail to compile, as it should! A sleeping cat can't eat.
    // let mittens = mittens.eat("a midnight snack");
}
