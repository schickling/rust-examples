
extern crate ncurses;

use ncurses::*;
use game::*;
use std::thread::sleep_ms as sleep;

mod game;

fn main()
{
    initscr();
    cbreak(); // enable <Ctrl+C> to kill game
    noecho(); // don't show input
    keypad(stdscr, true); // make keys work
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(1);

    let mut bounds = Vector { x: 0, y: 0};
    getmaxyx(stdscr, &mut bounds.y, &mut bounds.x);

    let mut game = Box::new(Game::new(bounds));

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
            KEY_LEFT => game.shift(Direction::Left),
            KEY_RIGHT => game.shift(Direction::Right),
            KEY_UP => game.fire(),
            _ => (),
        }

        if tick_count == 0 {
            match game.tick() {
                GameStatus::Dead => {
                    show_text("You died. Shame on you!");
                    sleep(2000);
                    break;
                },
                GameStatus::Win => {
                    show_text("Wow. I didn't expect that.");
                    sleep(2000);
                    break;
                },
                GameStatus::Running => (),
            };
        }
    }

    endwin();
}

fn draw_char (pos: &Vector, c: char) {
    mvaddch(pos.y, pos.x, c as u64);
}

fn show_text (s: &str) {
    erase();
    addstr(s);
    refresh();
}
