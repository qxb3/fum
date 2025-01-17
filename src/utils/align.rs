use ratatui::{layout::{Constraint, Flex, Layout, Rect}, Frame};

use crate::config::Align;

pub fn center(frame: &mut Frame<'_>, width: u16, height: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(frame.area());

    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);

    area
}

pub fn top(frame: &mut Frame<'_>, width: u16, height: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(frame.area());

    let [area] = Layout::vertical([Constraint::Length(height)])
        .areas(area);

    area
}

pub fn left(frame: &mut Frame<'_>, width: u16, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(frame.area());

    let [area] = Layout::horizontal([Constraint::Length(width)])
        .areas(area);

    area
}

pub fn bottom(frame: &mut Frame<'_>, width: u16, height: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(frame.area());

    let [_, area] = Layout::vertical([Constraint::Min(0), Constraint::Length(height)])
        .areas(area);

    area
}

pub fn right(frame: &mut Frame<'_>, width: u16, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(frame.area());

    let [_, area] = Layout::horizontal([Constraint::Min(0), Constraint::Length(width)])
        .areas(area);

    area
}

pub fn top_left(frame: &mut Frame<'_>, width: u16, height: u16) -> Rect {
    let [area, _] = Layout::horizontal([Constraint::Length(width), Constraint::Min(0)])
        .areas(frame.area());

    let [area, _] = Layout::vertical([Constraint::Length(height), Constraint::Min(0)])
        .areas(area);

    area
}

pub fn top_right(frame: &mut Frame<'_>, width: u16, height: u16) -> Rect {
    let [_, area] = Layout::horizontal([Constraint::Min(0), Constraint::Length(width)])
        .areas(frame.area());

    let [area, _] = Layout::vertical([Constraint::Length(height), Constraint::Min(0)])
        .areas(area);

    area
}

pub fn bottom_left(frame: &mut Frame<'_>, width: u16, height: u16) -> Rect {
    let [area, _] = Layout::horizontal([Constraint::Length(width), Constraint::Min(0)])
        .areas(frame.area());

    let [_, area] = Layout::vertical([Constraint::Min(0), Constraint::Length(height)])
        .areas(area);

    area
}

pub fn bottom_right(frame: &mut Frame<'_>, width: u16, height: u16) -> Rect {
    let [_, area] = Layout::horizontal([Constraint::Min(0), Constraint::Length(width)])
        .areas(frame.area());

    let [_, area] = Layout::vertical([Constraint::Min(0), Constraint::Length(height)])
        .areas(area);

    area
}

pub fn get_align(frame: &mut Frame<'_>, align: &Align, width: u16, height: u16) -> Rect {
    match align {
        Align::Center           => center(frame, width, height),
        Align::Top              => top(frame, width, height),
        Align::Left             => left(frame, width, height),
        Align::Bottom           => bottom(frame, width, height),
        Align::Right            => right(frame, width, height),
        Align::TopLeft          => top_left(frame, width, height),
        Align::TopRight         => top_right(frame, width, height),
        Align::BottomLeft       => bottom_left(frame, width, height),
        Align::BottomRight      => bottom_right(frame, width, height)
    }
}
