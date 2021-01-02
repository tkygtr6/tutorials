struct Dice {
    bottom: i32,
    top: i32,
    north: i32,
    south: i32,
    west: i32,
    east: i32,
}

fn rotate(dice: Dice, direction: char) -> Dice {
    match direction {
        'S' => Dice {
            bottom: dice.south,
            top: dice.north,
            north: dice.bottom,
            south: dice.top,
            west: dice.west,
            east: dice.east,
        },
        'N' => Dice {
            bottom: dice.north,
            top: dice.south,
            north: dice.top,
            south: dice.bottom,
            west: dice.west,
            east: dice.east,
        },
        'W' => Dice {
            bottom: dice.west,
            top: dice.east,
            north: dice.north,
            south: dice.south,
            west: dice.top,
            east: dice.bottom,
        },
        'E' => Dice {
            bottom: dice.east,
            top: dice.west,
            north: dice.north,
            south: dice.south,
            west: dice.bottom,
            east: dice.top,
        },
        _ => dice,
    }
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let dice: Vec<i32> = line
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let mut commands = String::new();
    std::io::stdin().read_line(&mut commands).unwrap();
    let commands: Vec<char> = commands.chars().collect();

    let mut dice = Dice {
        bottom: dice[5],
        top: dice[0],
        north: dice[4],
        south: dice[1],
        west: dice[3],
        east: dice[2],
    };
    for c in commands {
        dice = rotate(dice, c);
    }
    println!("{}", dice.top);
}
