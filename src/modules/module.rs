pub trait Module {
    fn render(&self, gap: usize) -> String;
    fn width(&self, gap: usize) -> usize;
    fn label(&self) -> Option<&str>;
}
