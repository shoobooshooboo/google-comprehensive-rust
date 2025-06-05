use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

struct Chopstick;

struct Philosopher {
    name: String,
    left_chopstick: Arc<Mutex<Chopstick>>,
    right_chopstick: Arc<Mutex<Chopstick>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        let (mut left,mut right) = (None, None);
        while left.is_none() || right.is_none(){
            (left, right) = (None, None);
            left = match self.left_chopstick.try_lock(){
                Ok(l) => Some(l),
                Err(_) => None
            };
            right = match self.right_chopstick.try_lock(){
                Ok(r) => Some(r),
                Err(_) => None
            };
        }

        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // Create chopsticks
    let mut chopsticks = Vec::new();
    for i in 0..5{
        chopsticks.push(Arc::new(Mutex::new(Chopstick)));
    }

    // Create philosophers
    let (sender, receiver) = mpsc::channel();
    let mut philosophers = Vec::new();

    let mut handlers = Vec::new();
    for i in 0..5{
        philosophers.push(Philosopher{
            name: PHILOSOPHERS[i].to_string(),
            left_chopstick: chopsticks[i].clone(),
            right_chopstick: chopsticks[(i + 1) % 5].clone(),
            thoughts: sender.clone(),
        });
    }

    // Make each of them think and eat 100 times
    for _ in 0..5{
        let philosopher = philosophers.pop().unwrap();
        handlers.push(thread::spawn(move || {
            for _ in 0..100{
                philosopher.eat();
                philosopher.think();
            }
        }));
    }

    // Output their thoughts
    let mut counter = 0;
    drop(sender);
    for thought in receiver.iter(){
        counter += 1;
        println!("{counter}: {thought}");
    }
}