#![feature(globs)]

extern crate ncurses;

use ncurses::*;
use game::*;

mod game;

fn main()
{
    initscr();
    cbreak(); // enable <Ctrl+C> to kill game
    noecho(); // don't show input
    keypad(stdscr, true); // make keys work
    curs_set(CURSOR_INVISIBLE);
    timeout(100); // tick speed

    //let mut bounds = Vector { x: 0, y: 0};
    //getmaxyx(stdscr, &mut bounds.y, &mut bounds.x);

    loop {
        erase();
    }

    endwin();
}

//fn draw_char (pos: &Vector, c: char) {
    //mvaddch(pos.y, pos.x, c as u32);
//}

fn show_text (s: &str) {
    erase();
    addstr(s);
    refresh();
}
