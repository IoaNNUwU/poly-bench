// Модуль с реализацией через dyn Trait (см файл dynamic_impl.rs)
pub mod dynamic_impl;

// Модуль с реализацией через enum (см файл enum_impl.rs)
pub mod enum_impl;

// Модуль с экраном-пустышкой (см файл screen.rs)
pub mod screen;

pub use screen::Screen;

// Идиоматичное имя для трейта Component
pub trait Draw {
    fn draw(&self, screen: &mut Screen);
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub pos: Point,
    pub w: f32, 
    pub h: f32,
    pub color: Color,
}

impl Draw for Rectangle {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_rect(*self);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub color: Color,
}

impl Draw for Triangle {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_triangle(*self);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Star {
    pub pos: Point,
    pub radius: f32,
    pub color: Color,
}

impl Draw for Star {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_whatever(*self);
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Color(pub u32);

impl Color {
    pub const RED: Color = Color(0xFF000000);
    pub const GREEN: Color = Color(0x00FF0000);
    pub const BLUE: Color = Color(0x0000FF00);
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}