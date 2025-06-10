use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame,
    style::{Color, Style},
};

#[derive(Clone)]
struct Life {
    w: i64,
    h: i64,
    x: i64,
    y: i64,
    cells: Vec<bool>,
}

impl Life {
    fn new(w: i64, h: i64) -> Self {
        assert!(w > 0);
        assert!(h > 0);
        Life {
            h,
            w,
            x: 0,
            y: 0,
            cells: vec![false; (w * h) as usize],
        }
    }

    fn count_live_neighbours(&self, x: i64, y: i64) -> usize {
        // Toroidal topology I guess
        let mut sum = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx != 0 || dy != 0) && self[(x + dx, y + dy)] {
                    sum += 1
                }
            }
        }
        sum
    }

    fn step(&self) -> Life {
        let mut new = self.clone();
        for x in 0..self.w {
            for y in 0..self.h {
                let neigh = self.count_live_neighbours(x, y);
                if neigh == 3 {
                    new[(x, y)] = true
                } else if neigh != 2 {
                    new[(x, y)] = false
                }
            }
        }
        new
    }

    fn draw(&self, frame: &mut Frame, cursor: &Cursor) {
        assert!(frame.area().x == 0);
        assert!(frame.area().y == 0);
        let buffer = frame.buffer_mut();
        for x in 0..self.w {
            // Being so cavalier will come back to haunt us, I'm sure
            let rx = x - self.x;
            for y in 0..self.h {
                let ry = y - self.y;
                let style = if x == cursor.x && y == cursor.y {
                    Style::default().fg(Color::Red).bg(Color::White)
                } else {
                    Style::default().fg(Color::Red)
                };
                if self[(rx, ry)] {
                    buffer
                        .get_mut(x as u16, y as u16)
                        .set_symbol("o")
                        .set_style(style)
                } else {
                    buffer
                        .get_mut(x as u16, y as u16)
                        .set_symbol(" ")
                        .set_style(style)
                };
            }
        }
    }
}

impl std::ops::Index<(i64, i64)> for Life {
    type Output = bool;
    fn index(&self, (x, y): (i64, i64)) -> &bool {
        let x = x.rem_euclid(self.w);
        let y = y.rem_euclid(self.h);
        return &self.cells[(x + y * self.w) as usize];
    }
}
impl std::ops::IndexMut<(i64, i64)> for Life {
    fn index_mut(&mut self, (x, y): (i64, i64)) -> &mut bool {
        let x = x.rem_euclid(self.w);
        let y = y.rem_euclid(self.h);
        return &mut self.cells[(x + y * self.w) as usize];
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Cursor {
    x: i64,
    y: i64,
}

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();
    let size = terminal.size()?;
    let mut life = Life::new(size.width as i64, size.height as i64);
    life[(0, 0)] = true;
    life[(0, 1)] = true;
    life[(0, 2)] = true;
    life[(1, 2)] = true;
    life[(2, 1)] = true;

    let mut paused = false;
    let mut cursor = Cursor { x: 0, y: 0 };

    let mut delay = 400;

    loop {
        terminal.draw(|f| life.draw(f, &cursor))?;
        if !paused {
            life = life.step();
        }
        if !event::poll(Duration::from_millis(delay))? {
            continue;
        }
        match event::read()? {
            Event::Key(k) if k.code == KeyCode::Esc => break,
            Event::Key(k) if k.code == KeyCode::Char(' ') => paused = !paused,
            Event::Key(k) if k.code == KeyCode::Char('h') => cursor.x -= 1,
            Event::Key(k) if k.code == KeyCode::Char('l') => cursor.x += 1,
            Event::Key(k) if k.code == KeyCode::Char('j') => cursor.y += 1,
            Event::Key(k) if k.code == KeyCode::Char('k') => cursor.y -= 1,
            Event::Key(k) if k.code == KeyCode::Char('f') => {
                life[(cursor.x, cursor.y)] = !life[(cursor.x, cursor.y)]
            },
            Event::Key(k) if k.code == KeyCode::Char('+') => if delay > 100 {
                delay -= 100
            }
            Event::Key(k) if k.code == KeyCode::Char('-') => delay += 100,
            _ => (),
        }

        cursor.x = cursor.x.rem_euclid(life.w);
        cursor.y = cursor.y.rem_euclid(life.h);
    }
    ratatui::restore();
    Ok(())
}
