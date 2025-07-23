use async_trait::async_trait;
use std::time::{Duration, SystemTime};
use tokio::{spawn, time::sleep};
// 👇 We need this.
#[derive(Clone)]
struct Cat;
#[derive(Clone)]
struct Dog;
#[async_trait]
trait Animal {
    // 👇 Because of this async.
    async fn sleep(&self);
    async fn eat(&self, food_item: String) -> String;
    async fn make_sound(&self) -> String;
}

#[async_trait]
impl Animal for Dog {
    async fn sleep(&self) {
        sleep(Duration::new(1, 0)).await;
    }
    async fn eat(&self, food_item: String) -> String {
        sleep(Duration::new(0, 700)).await;
        format!("The dog gulped down the {food_item}")
    }
    async fn make_sound(&self) -> String {
        sleep(Duration::new(0, 500)).await;
        format!("Woof!")
    }
}
// 👇 Also here.
#[async_trait]
impl Animal for Cat {
    async fn sleep(&self) {
        // Will sleep for 2 seconds.
        sleep(Duration::new(2, 0)).await;
    }
    async fn eat(&self, food_item: String) -> String {
        sleep(Duration::new(1, 0)).await;
        format!("The cat happily ate the {food_item}")
    }
    async fn make_sound(&self) -> String {
        sleep(Duration::new(0, 1000)).await;
        format!("Meow!")
    }
}

// This `async fn main` need `tokio::main`.
#[tokio::main]
async fn main() {
    // Wait for sleepy cat for 2 sec.
    let now = SystemTime::now();
    println!("Start time for animal sleep");
    let cat_sleep_handle = spawn(async move {
        Cat {}.sleep().await;
    });
    let dog_sleep_handle = spawn(async move {
        Dog {}.sleep().await;
    });
    cat_sleep_handle.await.expect("Cat sleeps task panicked");
    dog_sleep_handle.await.expect("Dog sleeps task panicked");
    let now_sec = now.elapsed().ok().unwrap().as_secs();
    println!("Total time for animal sleep: {} seconds.", now_sec);

    println!("Start time for concurrent eating");
    let eat_start_time = SystemTime::now();
    let cat_eat_handle = spawn(async move {
        Cat {}.eat("fish".to_string()).await;
    });
    let dog_eat_handle = spawn(async move {
        Dog {}.eat("meat".to_string()).await;
    });
    cat_eat_handle.await.expect("Cat eats task panicked");
    dog_eat_handle.await.expect("Dog eats task panicked");
    let eat_elasped_time = eat_start_time.elapsed().ok().unwrap().as_secs();
    println!(
        "Total time for concurrent eating: {} seconds.",
        eat_elasped_time
    );

    println!("Start time for animal making sound");
    let sound_start_time = SystemTime::now();
    let cat_make_sound_handle = spawn(async move {
        Cat {}.make_sound().await;
    });

    let dog_make_sound_handle = spawn(async move {
        Dog {}.make_sound().await;
    });

    cat_make_sound_handle
        .await
        .expect("Cat makes sound task panicked");

    dog_make_sound_handle
        .await
        .expect("Dog makes sound task panicked");
    let make_sound_elapsed_time = sound_start_time.elapsed().ok().unwrap().as_micros();
    println!(
        "Total time for animal making sound: {} microseconds.",
        make_sound_elapsed_time
    );
}
