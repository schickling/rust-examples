#![feature(globs)]

extern crate ncurses;

use std::char;
use ncurses::*;
use board::*;

mod board;

fn main()
{
    initscr();
    cbreak();
    noecho();
    keypad(stdscr, true); // make keys work
    curs_set(CURSOR_INVISIBLE);
    timeout(100);

    let mut screen_height = 0i32;
    let mut screen_width = 0i32;
    getmaxyx(stdscr, &mut screen_height, &mut screen_width);

    let mut board = Board::new(screen_width, screen_height);

    let mut direction = Up;

    loop {
        erase();

        {
            let bullet = board.get_bullet_position();
            draw_char(bullet, 'o');
        }

        {
            let segments = board.get_snake_positions();
            for segment in segments.iter() {
                draw_char(segment, '#');
            }
        }

        direction = get_new_direction(&direction);
        board.set_direction(direction);

        match board.tick() {
            Error => break,
            _ => (),
        }

    }

    endwin();
}

fn draw_char (pos: &Position, c: char) {
    mvaddch((*pos).y, (*pos).x, c as u32);
}

fn get_new_direction (prev_dir: &Direction) -> Direction {

    match getch() {
        KEY_UP if *prev_dir != Down => Up,
        KEY_DOWN if *prev_dir != Up => Down,
        KEY_LEFT if *prev_dir != Right => Left,
        KEY_RIGHT if *prev_dir != Left => Right,
        _ => *prev_dir,
    }

}
