use std::io;

#[deriving(PartialEq)]
enum Command {
    Go(Direction),
    Unlock(Direction),
}

#[deriving(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Exit {
    direction: Direction,
    target: u32, // the room number
    locked: bool,
}

impl Exit {
    fn can_go(&self, direction: Direction) -> bool {
        self.direction == direction &&
        !self.locked
    }
}

struct Room {
    description: String,
    exits: Vec<Exit>,
    items: Vec<Item>,
}

struct Item {
    name: String,
}

impl Room {
    fn can_go(&self, direction: Direction) -> bool {
        self.exits.iter().find(|e| e.can_go(direction)).is_some()
    }

    fn exit_to(&self, direction: Direction) -> u32 {
        self.exits.iter()
                  .find(|e| e.direction == direction)
                  .unwrap()
                  .target
    }

    fn is_escape(&self) -> bool {
        self.exits.len() == 0
    }
}

fn main() {
    let rooms = vec![
        Room {
            description: "You find yourself in a room. There is a door to the south and a door to the east.".to_string(),
            exits: vec![
                Exit {
                    direction: South,
                    target: 2,
                    locked: false,
                },
                Exit {
                    direction: East,
                    target: 1,
                    locked: false,
                },
            ],
            items: vec![],
        },
        Room {
            description: "You find yourself in a room. There is a door to the west and a door to the south.".to_string(),
            exits: vec![
                Exit {
                    direction: West,
                    target: 0,
                    locked: false,
                },
                Exit {
                    direction: South,
                    target: 3,
                    locked: false,
                },
            ],
            items: vec![],
        },
        Room {
            description: "You find yourself in a room. There is a door to the north. A key is here.".to_string(),
            exits: vec![
                Exit {
                    direction: North,
                    target: 0,
                    locked: false,
                },
            ],
            items: vec![
                Item {
                    name: "Key".to_string(),
                }
            ],
        },
        Room {
            description: "You find yourself in a room. There is a door to the north. The door to the south is locked.".to_string(),
            exits: vec![
                Exit {
                    direction: North,
                    target: 1,
                    locked: false,
                },
                Exit {
                    direction: South,
                    target: 4,
                    locked: true,
                },
            ],
            items: vec![],
        },
        Room {
            description: "Dungeon exit".to_string(),
            exits: vec![],
            items: vec![],
        }
    ];

    let mut current_room = 0;

    println!("* * * A D V E N T U R E * * *\n\n");

    while !rooms[current_room].is_escape() {
        current_room = enter(&rooms[current_room]);
    }

    println!("Congrats! You've escaped.");
}

fn enter(room: &Room) -> uint {
    let mut command: Option<Command> = None;

    while command == None {
        println!("{}", room.description);
        println!("\nWhat do you do?\n");

        for exit in room.exits.iter() {
            match exit.direction {
                North => println!("* Go (n)orth"),
                East  => println!("* Go (e)ast"),
                South => println!("* Go (s)outh"),
                West  => println!("* Go (w)est"),
            }
        }

        let input = io::stdin().read_line().ok().expect("Failed to read line");

        command = match input.as_slice().trim() {
            "n" if room.can_go(North) => Some(Go(North)),
            "e" if room.can_go(East)  => Some(Go(East)),
            "s" if room.can_go(South) => Some(Go(South)),
            "w" if room.can_go(West)  => Some(Go(West)),
            _   => {
                println!("Please type a valid command.");
                continue;
            }
        };
    }

    let next_room = match command.unwrap() {
        Go(North) => room.exit_to(North),
        Go(East)  => room.exit_to(East),
        Go(South) => room.exit_to(South),
        Go(West)  => room.exit_to(West),
        Unlock(_) => 1, // lol
    };

    next_room as uint
}
