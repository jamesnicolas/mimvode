use std::io::{Write, stdout};
use crossterm::{queue, QueueableCommand, cursor};
use crossterm::event::{read, Event};
use crossterm::event::KeyCode;
use crossterm::terminal::{Clear, ClearType};
use std::fmt;
// use rand::Rng;

#[derive(Clone, Copy)]
enum Block {
    Red,
    Blue,
    Green,
    Yellow,
    Empty,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => Direction::None,
        }
    }
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

impl<'a> fmt::Display for Game<'a> {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let mut y = 0;
        for row in self.grid {
            let mut x = 0;
            for cell in row {
                if self.player.x == x && self.player.y == y {
                    write!(f, "#").unwrap()
                } else {
                    write!(f, "{}", cell).unwrap()
                }
                x += 1;
            }
            write!(f, "\n").unwrap();
            y += 1;
        }
        Ok(())
    }
}

type Grid = [[Block; 80]; 16];

struct Player {
    x: i32,
    y: i32,
}

struct Game<'a> {
    player: &'a mut Player,
    grid: Grid,
}

impl<'a> Game<'a> {
    fn step(&mut self, d: Direction, m: i32) {
        use crate::Direction::*;
        match d {
            Up => self.player.y -= m,
            Right => self.player.x += m,
            Down => self.player.y += m,
            Left => self.player.x -= m,
            None => ()
        }
    }

    fn fill_row(&mut self, r: usize, b: Block) {
        for i in 0..self.grid[0].len() {
            self.grid[r][i] = b
        }
    }

    fn fill_column(&mut self, c: usize, b: Block) {
        for i in 0..self.grid.len() {
            self.grid[i][c] = b
        }
    }
}


fn main() -> Result<(), std::io::Error> {

    let mut stdout = stdout();

    // move operation is performed only if we flush the buffer.
    stdout.flush().expect("Failed to flush queue");
    let mut player = Player{x:39, y:7};
    let grid: Grid = [[Block::Empty; 80]; 16];
    let mut game = Game{player: &mut player, grid};

    game.grid[15][79] = Block::Red;
    game.fill_row(0, Block::Blue);
    game.fill_row(game.grid.len()-1, Block::Blue);

    game.fill_column(0, Block::Blue);
    game.fill_column(game.grid[0].len()-1, Block::Blue);

    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char('k') | KeyCode::Char('w') | KeyCode::Up => game.step(Direction::Up, 1),
                KeyCode::Char('h') | KeyCode::Char('a') | KeyCode::Left => game.step(Direction::Left, 1),
                KeyCode::Char('j') | KeyCode::Char('s') | KeyCode::Down => game.step(Direction::Down, 1),
                KeyCode::Char('l') | KeyCode::Char('d') | KeyCode::Right => game.step(Direction::Right, 1),
                _ => (),
            },
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
            _ => (),
        };
        queue!(stdout, Clear(ClearType::All)).expect("Failed to move cursor");
        queue!(stdout, cursor::MoveTo(game.player.x.try_into().unwrap(), game.player.y.try_into().unwrap())).expect("Failed to move cursor");
        stdout.flush().expect("Failed to flush queue");
    }

}
