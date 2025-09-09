trait Creator {
    fn get_genre(&self) -> String;
}

trait Artist: Creator {
    fn get_style(&self) -> String;
}

struct Musician {}

impl Artist for Musician {
    fn get_style(&self) -> String {
        "Rock".to_string()
    }
}

impl Creator for Musician {
    fn get_genre(&self) -> String {
        "Rock".to_string()
    }
}

fn showcase_trait(someone: &dyn Artist) -> String {
    format!(
        "{} is a {} musician",
        someone.get_genre(),
        someone.get_style()
    )
}

fn showcase_artist_generic<T: Artist>(someone: &T) -> String {
    format!(
        "{} is a {} musician",
        someone.get_genre(),
        someone.get_style()
    )
}
fn main() {
    println!("{}", showcase_artist_generic(&artist_information));
}
