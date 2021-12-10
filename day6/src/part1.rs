use utils::get_input;

#[derive(Debug, Default, Clone)]
struct LanternFish {
    timer: u32,
}

impl LanternFish {
    fn new(timer: u32) -> Self {
        Self { timer }
    }

    // Compute a day of the life of LanternFish.
    fn process(&mut self) -> Option<LanternFish> {
        if self.timer == 0 {
            self.timer = 6;
            Some(LanternFish::new(8))
        } else {
            self.timer -= 1;
            None
        }
    }
}

impl From<u32> for LanternFish {
    fn from(nb: u32) -> Self {
        Self { timer: nb }
    }
}

fn main() {
    let args = get_input::<String>();
    let mut fishs: Vec<LanternFish> = args
        .get(0)
        .expect("We need at least a line of data")
        .split(",")
        .filter_map(|e| e.parse::<u32>().ok())
        .map(LanternFish::from)
        .collect();

    let mut new_fishes = Vec::new();
    for _ in 0..256 {
        for fish in &mut fishs {
            if let Some(new_fish) = fish.process() {
                new_fishes.push(new_fish);
            }
        }
        fishs.extend_from_slice(&new_fishes);
        new_fishes.clear();
    }

    println!("After 80 days, there are {} fishes", fishs.len());
}
