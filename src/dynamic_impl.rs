use super::*;

pub struct AllComponents {
    // Приватное поле - реализация динамическая.
    list: Vec<Box<dyn Draw>>
}

// Публичный интерфейс структуры AllComponents 
// состоит из публичной функции draw и методов для
// добавления элементов. (см строку 20)
impl Draw for AllComponents {
    fn draw(&self, screen: &mut Screen) {
        for dyn_comp in &self.list {
            // этот вызов динамический (см строку 5)
            dyn_comp.draw(screen);
        }
    }
}

impl AllComponents {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    // Делать отдельные методы для каждой структуры не обязательно, но
    // лучше для большего сокрытия деталей реализации. 
    // Оставив только эти методы, мы сможем в будущем заменить
    // одну реализацию на другую, не меняя пользовательский код.
    pub fn add_star(&mut self, star: Star) {
        self.list.push(Box::new(star));
    }
    pub fn add_rect(&mut self, rect: Rectangle) {
        self.list.push(Box::new(rect));
    }
    pub fn add_triangle(&mut self, tr: Triangle) {
        self.list.push(Box::new(tr));
    }
    // Метод возможный только в динамической реализации,
    // служит для добавления любого пользовательского компонента.
    //
    // Он использует дженерики, чтобы получить пользовательский компонент
    // во владение, тогда пользователю не обязательно знать, что мы его
    // аллоцируем.
    // Если бы мы принимали Box<dyn Draw>, то пользователю пришлось бы 
    // делать лишнюю работу, а именно аллоцировать компонент в куче. 
    pub fn add_user_component<T: Draw + 'static>(&mut self, comp: T) {
        self.list.push(Box::new(comp));
    }
}