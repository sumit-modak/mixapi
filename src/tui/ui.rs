use super::layoutdata::LayoutData;
use super::tuistate::TuiState;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Direction,
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::rc::Rc;

trait RenderCustomBlocks {
    fn smaller_rect(&self) -> Self;
    fn render_better_border(&self, frame: &mut Frame, cfg: &Color, cbg: &Color);
    fn render_normal_border(&self, frame: &mut Frame, cfg: &Color, cbg: &Color);
}

impl RenderCustomBlocks for Rect {
    #[inline]
    fn smaller_rect(&self) -> Self {
        Rect {
            x: self.x + 1,
            y: self.y,
            width: self.width - 2,
            height: self.height,
        }
    }
    fn render_better_border(&self, frame: &mut Frame, cfg: &Color, cbg: &Color) {
        frame.render_widget(Block::new().bg(*cbg), *self);
        frame.render_widget(
            Block::new().fg(*cfg).borders(Borders::ALL),
            self.smaller_rect(),
        );
    }
    fn render_normal_border(&self, frame: &mut Frame, cfg: &Color, cbg: &Color) {
        frame.render_widget(Block::new().fg(*cfg).bg(*cbg).borders(Borders::ALL), *self);
    }
}

pub fn render(data: &LayoutData, app: &TuiState, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(frame.area());

    let wq_target_block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(layout[1]);

    let wq_target = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(wq_target_block[1]);

    let layout_kb = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(layout[0]);

    let wq_default_block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(layout[2]);

    let wq_default = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(wq_default_block[1]);

    let col1 = Color::Rgb(255, 211, 0);
    let col2 = Color::Rgb(24, 24, 24);
    layout[1].render_better_border(frame, &col1, &col2);
    layout[2].render_better_border(frame, &col1, &col2);

    render_half_layout(frame, layout_kb[0], &data.layout_left);
    render_half_layout(frame, layout_kb[2], &data.layout_right);
    render_word_pair(frame, layout_kb[1]);

    render_lines(frame, &wq_target, &wq_default, &app, 0);
    render_lines(frame, &wq_target, &wq_default, &app, 1);
    render_lines(frame, &wq_target, &wq_default, &app, 2);
}

fn render_half_layout(frame: &mut Frame, r: Rect, a: &[[u8; 5]; 3]) {
    r.render_better_border(frame, &Color::Rgb(196, 196, 196), &Color::Rgb(64, 64, 64));

    let row = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(4),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(4),
        ])
        .split(r);

    let row0 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Max(8),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(row[1]);

    let row1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Max(8),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(row[2]);

    let row2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Max(8),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(row[3]);

    let cfg = Color::Rgb(64, 64, 64);
    let cbg = Color::Rgb(196, 196, 196);
    render_weird(row0[1], frame, &cfg, &cbg);
    render_weird(row0[7], frame, &cfg, &cbg);
    render_weird(row1[1], frame, &cfg, &cbg);
    render_weird(row1[7], frame, &cfg, &cbg);
    render_weird(row2[1], frame, &cfg, &cbg);
    render_weird(row2[7], frame, &cfg, &cbg);
    for i in 0..5 {
        render_char(row0[i + 2], frame, &cfg, &cbg, a[0][i]);
    }
    for i in 0..5 {
        render_char(row1[i + 2], frame, &cfg, &cbg, a[1][i]);
    }
    for i in 0..5 {
        render_char(row2[i + 2], frame, &cfg, &cbg, a[2][i]);
    }
}

pub fn render_char(r: Rect, frame: &mut Frame, cfg: &Color, cbg: &Color, ch: u8) {
    frame.render_widget(
        Paragraph::new(format!(" {}", ch as char))
            .fg(*cfg)
            .bg(*cbg)
            .block(Block::new().borders(Borders::ALL)),
        r,
    );
}

fn render_weird(r: Rect, frame: &mut Frame, cfg: &Color, cbg: &Color) {
    frame.render_widget(Block::new().fg(*cfg).bg(*cbg), r);
}

pub fn render_word_pair(frame: &mut Frame, r: Rect) {
    r.render_normal_border(frame, &Color::Rgb(196, 196, 196), &Color::Rgb(64, 64, 64));
}

pub fn render_lines(frame: &mut Frame, lt: &Rc<[Rect]>, ld: &Rc<[Rect]>, q: &TuiState, lno: usize) {
    frame.render_widget(
        Paragraph::new(&q.tline[lno] as &str)
            .fg(Color::Rgb(255, 211, 0))
            .bg(Color::Rgb(24, 24, 24)),
        lt[lno + 1],
    );
    frame.render_widget(
        Paragraph::new(&q.dline[lno] as &str)
            .fg(Color::Rgb(255, 211, 0))
            .bg(Color::Rgb(24, 24, 24)),
        ld[lno + 1],
    );
}
