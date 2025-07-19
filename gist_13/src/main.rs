struct Animal {}
struct Human {}
trait Movable {
    fn move_sound(&self) -> String;
}
struct Car {}
impl Movable for Car {
    fn move_sound(&self) -> String {
        "vroom vroom".to_owned()
    }
}
fn do_move<T: Movable + ?Sized>(arg: &T) {
    println!("{}", arg.move_sound());
}
impl Movable for Animal {
    fn move_sound(&self) -> String {
        "pitter pitter".to_owned()
    }
}
impl Movable for Human {
    fn move_sound(&self) -> String {
        "thump thump".to_owned()
    }
}
fn main() {
    let human = Human {};
    let animal = Animal {};
    let car = Car {};
    let mut data: Vec<&dyn Movable> = vec![];
    data.push(&human);
    data.push(&animal);
    data.push(&car);
    for item in data {
        do_move(item);
    }
}
