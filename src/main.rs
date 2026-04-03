use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{Color, SetForegroundColor, ResetColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    cmp::min, collections::VecDeque, io::{Write, stdout}
};
use std::time::Duration;
use rand::random_range;

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_inputs() -> Option<KeyCode> {
    if event::poll(Duration::from_millis(300)).unwrap() {
        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            return Some(code);
        }
    }

    None
}

fn draw_gui(max_x: u16, max_y: u16) {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    println!("SNAKE TUI");
    stdout.execute(cursor::MoveTo(0, 1)).unwrap();
    stdout.execute(SetForegroundColor(Color::White)).unwrap();
    println!("┌{}┐", "──".repeat(max_x as usize + 2));

    for i in 2..max_y+3 {
        stdout.execute(cursor::MoveTo(0, i)).unwrap();
        println!("│{}│", " ".repeat((max_x as usize+2) * 2))        
    }

    stdout.execute(cursor::MoveTo(0, max_y+3)).unwrap();
    println!("└{}┘", "──".repeat(max_x as usize + 2));

    stdout.execute(ResetColor).unwrap();
    stdout.flush().unwrap();
}

fn win() {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    println!("WIN");

    loop {
        match read_inputs() {
            Some(KeyCode::Char('r')) => break,
            _ => {},
        }
    }
}

fn game_over() {
    loop {
        match read_inputs() {
            Some(KeyCode::Char('r')) => break,
            _ => {},
        }
    }
}

fn main() {
    const MAX_X : u16 = 7;
    const MAX_Y: u16 = 7;

    let mut head_x: u16 = 4;
    let mut head_y: u16 = 4;
    let mut body_length: usize = 0;
    let mut body: VecDeque<(u16,u16)> = Default::default();
    let mut direction: Direction = Direction::Up;

    let mut apple_x: u16 = random_range(0..MAX_X+1);
    let mut apple_y: u16 = random_range(0..MAX_Y+1);

    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    loop {
        match read_inputs() {
            Some(KeyCode::Up) => direction = if direction != Direction::Down {Direction::Up} else {direction},
            Some(KeyCode::Down) => direction = if direction != Direction::Up {Direction::Down} else {direction},
            Some(KeyCode::Left) => direction = if direction != Direction::Right {Direction::Left} else {direction},
            Some(KeyCode::Right) => direction = if direction != Direction::Left {Direction::Right} else {direction},
            Some(KeyCode::Char('q')) => break,
            _ => {},
        }

        // MOVE
        if body.len() > body_length {
            body.pop_back();
        }
        body.push_front((head_x, head_y));
        
        match direction {
            Direction::Up => head_y = head_y.saturating_sub(1),
            Direction::Down => head_y = min(head_y+1, MAX_Y),
            Direction::Left => head_x = head_x.saturating_sub(1),
            Direction::Right => head_x = min(head_x+1, MAX_X),
        }

        
        // CHECKS
        if body_length == MAX_X as usize * MAX_Y as usize { win(); }
        if (head_x, head_y) == (apple_x, apple_y) {
            body_length += 1;

            loop {
                apple_x = random_range(0..MAX_X+1);
                apple_y = random_range(0..MAX_Y+1);

                if !body.contains(&(apple_x, apple_y)) &&
                    (apple_x, apple_y) != (head_x, head_y) {
                        break; 
                }
            }
        }

        // DRAW
        stdout.execute(Clear(ClearType::All)).unwrap();
        draw_gui(MAX_X, MAX_Y);

        // Apple
        stdout.execute(cursor::MoveTo((apple_x+1)*2, apple_y+2)).unwrap();
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        print!("⯀");

        // Snake
        stdout.execute(cursor::MoveTo((head_x+1)*2, head_y+2)).unwrap();
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        match direction {
            Direction::Up => print!("⯅"),
            Direction::Down => print!("⯆"),
            Direction::Left => print!("⯇"),
            Direction::Right => print!("⯈"),
        }

        for (x, y) in &body {
            stdout.execute(cursor::MoveTo((x+1)*2, y+2)).unwrap();
            print!("⯀")
        }

        stdout.execute(ResetColor).unwrap();
        stdout.flush().unwrap();
    }

    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}
