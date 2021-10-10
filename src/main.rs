use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, SetForegroundColor};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown,
};
use crossterm::{event, QueueableCommand};
use rand::seq::SliceRandom;
use rand::Rng;

const CHARS: [char; 56] = [
    'ﾊ', 'ﾐ', 'ﾋ', 'ｰ', 'ｳ', 'ｼ', 'ﾅ', 'ﾓ', 'ﾆ', 'ｻ', 'ﾜ', 'ﾂ', 'ｵ', 'ﾘ', 'ｱ', 'ﾎ', 'ﾃ', 'ﾏ', 'ｹ',
    'ﾒ', 'ｴ', 'ｶ', 'ｷ', 'ﾑ', 'ﾕ', 'ﾗ', 'ｾ', 'ﾈ', 'ｽ', 'ﾀ', 'ﾇ', 'ﾍ', '0', '1', '2', '3', '4', '5',
    '7', '8', '9', 'Z', '"', ':', '.', '=', '*', '+', '-', '<', '>', '¦', '╌', 'ç', 'ﾘ', 'ｸ',
];

// TODO
//
//  - Lines "falling" at different speeds
//  - Highlight colors can move within a line at a different speed
//  - Start new lines in the middle of the terminal

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();

    // Setup
    stdout.queue(EnterAlternateScreen {})?;
    stdout.queue(Hide {})?;
    stdout.flush()?;
    enable_raw_mode()?;

    let (cols, rows) = size().map(|(c, r)| (c as usize, r as usize))?;

    let mut rng = rand::thread_rng();
    let mut table: Vec<Vec<char>> = vec![vec![' '; cols]; rows];

    loop {
        for x in 0..cols {
            let r = rng.gen::<f32>();

            table[0][x] = if (
                // If the character on the row below isn't a space we have a higher chance of
                // printing another character to continue the line.
                table[1][x] != ' ' && r > 0.2
            ) || (
                // Otherwise we have a lower chance of printing a character.
                r > 0.98
            ) {
                *CHARS.choose(&mut rng).expect("missing char")
            } else {
                ' '
            };
        }

        // Write to the screen
        for y in 0..rows {
            for x in 0..cols {
                let pos = MoveTo(x as u16, y as u16);
                let color = SetForegroundColor(Color::Rgb {
                    r: rng.gen_range(0..10),
                    g: rng.gen_range(150..200),
                    b: rng.gen_range(90..110),
                });
                write!(stdout, "{}{}{}", pos, color, table[y][x])?;
            }
        }

        // Move the bottom row back to the top
        let old = table.pop().expect("missing row");
        table.insert(0, old);

        // Quit on any input
        if event::poll(Duration::from_millis(100))? && event::read().is_ok() {
            break;
        }

        stdout.flush()?;

        // Scroll down the terminal, this should help with redraw flickering
        stdout.queue(ScrollDown(1))?;
    }

    // Teardown
    disable_raw_mode()?;
    stdout.queue(LeaveAlternateScreen {})?;
    stdout.queue(Show {})?;
    stdout.flush()?;

    Ok(())
}
