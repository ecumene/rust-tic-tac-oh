use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use std::io::{Write, stdout, stdin};

const AI_PLAYER: u8 = 2;
const HUMAN_PLAYER: u8 = 1;

fn cell_format(data: u8) -> char {
    match data {
        HUMAN_PLAYER => return 'O',
        AI_PLAYER => return 'X',
        _ => return ' ',
    }
}

fn winning(data: [u8; 9], player: u8) -> bool {
    return
        (data[0] == player && data[1] == player && data[2] == player) || // Horizontals
        (data[3] == player && data[4] == player && data[5] == player) ||
        (data[6] == player && data[7] == player && data[8] == player) ||
        (data[0] == player && data[3] == player && data[6] == player) || // Diagonal
        (data[1] == player && data[4] == player && data[7] == player) ||
        (data[2] == player && data[5] == player && data[8] == player) || // Vertical
        (data[0] == player && data[4] == player && data[8] == player) ||
        (data[2] == player && data[4] == player && data[6] == player);
}

fn avail(data: [u8; 9]) -> Vec<u8> {
    let mut avail: Vec<u8> = Vec::new();
    for x in 0..9 {
        if data[x] == 0 {avail.push(x as u8)}
    }
    return avail;
}

struct Move {
    index: u8,
    score: i32,
}

fn minimax(data: [u8; 9], player: u8) -> Move {
    let array = avail(data);
    if winning(data, HUMAN_PLAYER) { return Move { index: 0, score: -10 }; }
    if winning(data, AI_PLAYER)    { return Move { index: 0, score: 10 }; }
    if array.len() == 0 { return Move { index: 0, score: 0 }; }
    let mut best_move: Move = Move {index: 0, score: 0};
    if player == AI_PLAYER {best_move.score = -10000} 
    else                   {best_move.score = 10000} 
    for i in array {
        let mut new_data = data.clone();
        new_data[i as usize] = player;
        if player == AI_PLAYER {
            let g: i32 = minimax(new_data, HUMAN_PLAYER).score;
            if g > best_move.score {
                best_move = Move { index: i, score: g };
            }
        } else {
            let g: i32 = minimax(new_data, AI_PLAYER).score;
            if g < best_move.score {
                best_move = Move { index: i, score: g };
            }
        }
    }
    return best_move;
}

fn draw_state(data: [u8; 9], i: i8) {
    println!("{}{}q to exit. Use the arrow keys to navagate. Press space to place.", termion::clear::All, termion::cursor::Goto(1, 1));
    println!("{}You will either tie or lose :)", termion::cursor::Goto(1, 2));
    println!("{}╔═══════════╗", termion::cursor::Goto(1, 3));
    println!("{}║ {} | {} | {} ║", termion::cursor::Goto(1, 4), cell_format(data[0]), cell_format(data[1]), cell_format(data[2]));
    println!("{}║ {} | {} | {} ║", termion::cursor::Goto(1, 5),  cell_format(data[3]), cell_format(data[4]), cell_format(data[5]));
    println!("{}║ {} | {} | {} ║", termion::cursor::Goto(1, 6),  cell_format(data[6]), cell_format(data[7]), cell_format(data[8]));
    println!("{}╚═══════════╝", termion::cursor::Goto(1, 7));
    println!("{}", termion::cursor::Goto(((i as u16%3)*4)+2, ((i as u16/3))+3));
    if winning(data, AI_PLAYER)||avail(data).len() == 0 {
        println!("{}You have lost :( Try again? Press r!\n", termion::cursor::Goto(1, 8));
    }
}

fn main() {
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap()); // Alternate screen makes for cleaner raw input
    let mut data: [u8; 9] = [0,0,0,0,0,0, 0,0,0]; // Goes from the top left, to the bottom right
    let mut i: i8 = 0;
    draw_state(data, i);
    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('r') => data = [0 as u8; 9],
            Key::Left      => i -= 1,
            Key::Right     => i += 1,
            Key::Up        => if i > 2 {i -= 3}, // Loop around top
            Key::Down      => if i < 6 {i += 3},
            Key::Char(' ') => {
                if data[i as usize] == 0 {
                    data[i as usize] = HUMAN_PLAYER;
                    if avail(data).len() != 0 {data[minimax(data, AI_PLAYER).index as usize] = AI_PLAYER;}
                }
            },
            _=> {}
        }
        i = (i % 9).abs(); // Avoids seg fault
        draw_state(data, i);
        stdout.flush().unwrap();
    }
}