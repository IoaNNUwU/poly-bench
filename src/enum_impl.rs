use super::*;

pub struct AllComponents {
    // Приватное поле - деталь реализации.
    list: Vec<EnumCompWrapper>
}

// Публичный интерфейс структуры AllComponents 
// состоит из публичной функции draw и методов для
// добавления элементов. (см строку 26)
impl Draw for AllComponents {
    fn draw(&self, screen: &mut Screen) {
        for enum_comp in &self.list {
            // этот вызов статический, потому что
            // EnumCompWrapper реализует Draw используя
            // match, а не виртуальный вызов. (см строку 52)
            enum_comp.draw(screen);
        }
    }
}

impl AllComponents {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    // Приходится делать отдельные методы для каждой структуры,
    // потому что enum должен статически знать все типы во время
    // компиляции.
    pub fn add_star(&mut self, star: Star) {
        self.list.push(EnumCompWrapper::from(star));
    }
    pub fn add_rect(&mut self, rect: Rectangle) {
        self.list.push(EnumCompWrapper::from(rect));
    }
    pub fn add_triangle(&mut self, tr: Triangle) {
        self.list.push(EnumCompWrapper::from(tr));
    }
}

// Этот enum приватный, потому что это деталь реализации.
//
// Реализацию легко упростить с помощью макросов, но 
// приведу пример без них для простоты.
enum EnumCompWrapper {
    Rect(Rectangle),
    Trian(Triangle),
    Star(Star),
}

impl Draw for EnumCompWrapper {
    fn draw(&self, screen: &mut Screen) {
        match self {
            EnumCompWrapper::Rect(rect) => rect.draw(screen),
            EnumCompWrapper::Trian(tri) => tri.draw(screen),
            EnumCompWrapper::Star(star) => star.draw(screen),
        }
    }
}

impl From<Rectangle> for EnumCompWrapper {
    fn from(value: Rectangle) -> Self {
        EnumCompWrapper::Rect(value)
    }
}
impl From<Star> for EnumCompWrapper {
    fn from(value: Star) -> Self {
        EnumCompWrapper::Star(value)
    }
}
impl From<Triangle> for EnumCompWrapper {
    fn from(value: Triangle) -> Self {
        EnumCompWrapper::Trian(value)
    }
}