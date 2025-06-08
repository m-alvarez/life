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
    cells: Vec<bool>,
}

impl Life {
    fn new(w: i64, h: i64) -> Self {
        assert!(w > 0);
        assert!(h > 0);
        Life {
            h,
            w,
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

    fn draw(&self, frame: &mut Frame) {
        assert!(frame.area().x == 0);
        assert!(frame.area().y == 0);
        let buffer = frame.buffer_mut();
        for x in 0..self.w {
            for y in 0..self.h {
                if self[(x, y)] {
                    buffer
                        .get_mut(x as u16, y as u16)
                        .set_symbol("o")
                        .set_style(Style::default().fg(Color::Red));
                } else {
                    buffer.get_mut(x as u16, y as u16).set_symbol(" ");
                }
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

    loop {
        terminal.draw(|f| life.draw(f))?;
        if !event::poll(Duration::from_millis(10))? {
            if !paused {
                life = life.step();
            }
            continue;
        }
        match event::read()? {
            Event::Key(k) if k.code == KeyCode::Esc => break,
            Event::Key(k) if k.code == KeyCode::Char(' ') => {
                paused = !paused
            },
            _ => (),
        }
    }
    ratatui::restore();
    Ok(())
}
