#![feature(globs)]

extern crate ncurses;

use std::old_io::timer::sleep;
use std::time::duration::Duration;
use ncurses::*;
use game::*;

mod game;

fn main()
{
    initscr();
    cbreak(); // enable <Ctrl+C> to kill game
    noecho(); // don't show input
    keypad(stdscr, true); // make keys work
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(100); // tick speed

    let mut bounds = Vector { x: 0, y: 0 };
    getmaxyx(stdscr, &mut bounds.y, &mut bounds.x);

    let mut board = Board::new(bounds);

    let mut direction = Direction::Up;

    loop {
        erase();

        {
            let bullet = board.get_bullet_vector();
            draw_char(bullet, 'o');
        }

        {
            let segments = board.get_snake_vectors();
            for segment in segments.iter() {
                draw_char(segment, '#');
            }
        }

        direction = get_new_direction(direction);
        board.set_direction(direction);


        match board.tick() {
            Err(err) => {
                match err {
                    GameError::Wall => show_text("You hit the wall, stupid."),
                    GameError::Suicide => show_text("Damn it. Stop eating yourself."),
                }
                let two_secs = Duration::seconds(2);
                sleep(two_secs);
                break;
            },
            Ok(_) => (),
        };
    }

    endwin();
}

fn draw_char (pos: &Vector, c: char) {
    mvaddch(pos.y, pos.x, c as u64);
}

fn get_new_direction (prev_dir: Direction) -> Direction {
    match getch() {
        KEY_UP if prev_dir != Direction::Down => Direction::Up,
        KEY_DOWN if prev_dir != Direction::Up => Direction::Down,
        KEY_LEFT if prev_dir != Direction::Right => Direction::Left,
        KEY_RIGHT if prev_dir != Direction::Left => Direction::Right,
        _ => prev_dir,
    }
}

fn show_text (s: &str) {
    erase();
    addstr(s);
    refresh();
}
