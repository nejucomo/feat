#[derive(Debug, Default)]
pub struct Task {
    title: Option<String>,
}

impl Task {
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }
}
