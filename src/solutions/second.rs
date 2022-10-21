use std::fmt::Display;

struct Instructions<'a>(&'a str, i32);
#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    fn go_forward(&mut self, value: i32) {
        self.x += value;
        self.y += self.aim * value;
    }

    fn go_down(&mut self, value: i32) {
        self.aim -= value;
    }

    fn go_up(&mut self, value: i32) {
        self.aim += value;
    }

    pub fn update_position(&mut self, dir: &str, value: i32) {
        match dir {
            "forward" => self.go_forward(value),
            "down" => self.go_down(value),
            "up" => self.go_up(value),
            _ => panic!("Wrong input: {}", dir),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position: x: {}, y: {}, aim: {}",
            self.x, self.y, self.aim
        )
    }
}

pub fn puzzle() {
    let input = advent_of_code::read_file("inputs", "second");
    // let mut position = Position::new();
    // for line in input.lines() {
    //     let (direction, value) = match line.trim().split_once(' ') {
    //         Some(ins) => ins,
    //         None => break,
    //     };
    //     let value: i32 = value.parse().expect("value should be a number");
    //     position.update_position(direction, value)
    // }
    let position = input
        .lines()
        .map(to_instruction)
        .fold(Position::new(), |mut pos, ins| {
            pos.update_position(ins.0, ins.1);
            pos
        });
    println!(
        "Day 2: {}; first result is: {}, second result is: {}",
        position,
        (position.x * position.aim).abs(),
        (position.x * position.y).abs()
    )
}

fn to_instruction(line: &str) -> Instructions {
    let (direction, value) = line.trim().split_once(' ').expect("input to be correct");
    let value: i32 = value.parse().expect("value should be a number");
    Instructions(direction, value)
}
