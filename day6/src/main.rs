use utils::get_input;

struct FishGang {
    groups: [usize; 9],
    cache: [usize; 9],
}

impl FishGang {
    fn new(groups: [usize; 9]) -> Self {
        FishGang {
            groups,
            cache: groups,
        }
    }

    #[allow(unused)]
    fn print(&self) {
        for (idx, value) in self.groups.iter().enumerate() {
            println!("{} Fish in the group {}", value, idx);
        }
    }
}

impl Iterator for FishGang {
    // Total Number of fishs
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.cache[8] = 0;
        for i in 1..=8 {
            self.cache[8 - i] = self.groups[9 - i];
        }
        self.cache[6] += self.groups[0];
        self.cache[8] += self.groups[0];

        self.groups = self.cache;

        Some(self.cache.iter().sum())
    }
}

fn main() {
    let args = get_input::<String>();

    let mut groups: [usize; 9] = [0; 9];
    args.get(0)
        .expect("We need at least a line of data")
        .split(",")
        .filter_map(|e| e.parse::<u32>().ok())
        .for_each(|value| {
            groups[value as usize] += 1;
        });

    let gang = FishGang::new(groups);

    for (i, value) in gang.take(256).enumerate() {
        println!("day {}: {} fishes", i + 1, value);
    }
}
