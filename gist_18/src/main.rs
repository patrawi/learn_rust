trait Player {
    fn increase_score(&mut self, points: u8);
}

struct Gamer {
    current_score: u8,
}

impl Player for Gamer {
    fn increase_score(&mut self, points: u8) {
        self.current_score += points;
    }
}
fn play_game(t: &mut impl Player) {
    t.increase_score(11u8);
}

fn main() {
    let mut gamer = Gamer { current_score: 0 };
    println!("Score before: {}", gamer.current_score);
    play_game(&mut gamer);
    println!("Score after: {}", gamer.current_score);
}
