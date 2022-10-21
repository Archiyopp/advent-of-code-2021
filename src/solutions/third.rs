struct BitCounter(u32, u32);

impl BitCounter {
    const fn new() -> Self {
        Self(0, 0)
    }
    fn add_zero(&mut self) {
        self.0 += 1;
    }

    fn add_one(&mut self) {
        self.1 += 1;
    }

    const fn get_bigger(&self) -> char {
        if self.0 > self.1 {
            '0'
        } else {
            '1'
        }
    }
    const fn get_smaller(&self) -> char {
        if self.0 > self.1 {
            '1'
        } else {
            '0'
        }
    }
}

pub fn first_puzzle() {
    let input = advent_of_code::read_file("inputs", "third");
    let mut vec = Vec::new();
    for line in input.lines() {
        for (index, bit) in line.chars().enumerate() {
            if vec.len() < index + 1 {
                vec.push(BitCounter::new())
            }
            match bit {
                '0' => vec.get_mut(index).expect("To get a BitCounter").add_zero(),
                '1' => vec.get_mut(index).expect("To get a BitCounter").add_one(),
                _ => panic!("Expected to get binary numbers"),
            }
        }
    }
    let gamma = vec.iter().map(|b| b.get_bigger()).collect::<String>();
    let epsilon = vec.iter().map(|b| b.get_smaller()).collect::<String>();
    let gamma = u32::from_str_radix(&gamma, 2).expect("Binary number");
    let epsilon = u32::from_str_radix(&epsilon, 2).expect("Binary number");
    println!(
        "Gamma: {}, Epsilon: {}, Consumption: {}",
        gamma,
        epsilon,
        gamma * epsilon
    )
}
