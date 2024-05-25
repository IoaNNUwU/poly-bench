use super::*;

use std::hint::black_box;

// Экран реализофан фиктивно
//
// функция black_box заставляет компилятор думать, что
// мы используем передаваемую фигуру - для верности бенчмарков.
pub struct Screen;

impl Screen {
    pub fn fill_rect(&mut self, rect: Rectangle) {
        black_box(rect);
    }

    pub fn fill_triangle(&mut self, tr: Triangle) {
        black_box(tr);
    }

    pub fn fill_whatever<T>(&mut self, whatever: T) {
        black_box(whatever);
    }
}
