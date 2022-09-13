use std::fmt;

#[derive(Clone, Copy)]
enum Block {
    Red,
    Blue,
    Green,
    Yellow,
    Empty,
}

#[derive(Clone, Copy)]
struct Cell<'a>(Block, Option<&'a Player>);


impl fmt::Display for Block {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Block::Red => 'R',
            Block::Blue => 'B',
            Block::Green => 'G',
            Block::Yellow => 'Y',
            Block::Empty => ' ',

        };
        write!(f, "{}", printable)
    }
}

impl<'a> fmt::Display for Cell<'a> {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self.1 {
            Some(player) => write!(f, "#"),
            None => write!(f, "{}", self.0)
        }
    }
}

type Grid<'a> = [[Cell<'a>; 80]; 16];

struct Player {
    x: i32,
    y: i32,
}

struct Game<'a> {
    player: &'a Player,
    grid: Grid<'a>,
}

impl<'a> Game<'a> {
    fn print(&self) {
        for i in self.grid {
            for j in i {
                print!("{}",j);
            }
            println!();
        }
    }
}

fn main() {
    let player = Player{x:39, y:7};
    let grid: Grid = [[Cell(Block::Empty, None); 80]; 16];
    let mut game = Game{player: &player, grid};

    game.grid[15][79].0 = Block::Red;

    for i in 0..16 {
        game.grid[i][0].0 = Block::Blue;
        game.grid[i][79].0 = Block::Blue;
    }

    for i in 0..80 {
        game.grid[0][i].0 = Block::Blue;
        game.grid[15][i].0 = Block::Blue;
    }

    game.print()

}
