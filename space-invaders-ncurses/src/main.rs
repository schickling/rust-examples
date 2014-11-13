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
    timeout(1);

    let mut bounds = Vector { x: 0, y: 0};
    getmaxyx(stdscr, &mut bounds.y, &mut bounds.x);

    let mut game = box Game::new(bounds);

    // TODO rewrite async
    let mut tick_count = 0u8;

    loop {
        tick_count += 1;
        tick_count %= 30;

        erase();

        {
            let player = game.get_player_vector();
            draw_char(player, 'X');
        }

        {
            let invaders = game.get_invader_vectors();
            for invader in invaders.iter() {
                draw_char(invader, '#');
            }
        }

        {
            let bullets = game.get_bullet_vectors();
            for bullet in bullets.iter() {
                draw_char(bullet, '.');
            }
        }

        match getch() {
            KEY_LEFT => game.shift(Left),
            KEY_RIGHT => game.shift(Right),
            KEY_UP => game.fire(),
            _ => (),
        }

        if tick_count == 0 {
            match game.tick() {
                Dead => {
                    show_text("You died. Shame on you!");
                    std::io::timer::sleep(std::time::duration::Duration::seconds(2));
                    break;
                },
                Win => {
                    show_text("Wow. I didn't expect that.");
                    std::io::timer::sleep(std::time::duration::Duration::seconds(2));
                    break;
                },
                Running => (),
            };
        }
    }

    endwin();
}

fn draw_char (pos: &Vector, c: char) {
    mvaddch(pos.y, pos.x, c as u32);
}

fn show_text (s: &str) {
    erase();
    addstr(s);
    refresh();
}
