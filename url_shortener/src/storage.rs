pub struct Storage {
    urls: Vec<String>,
}

impl Storage {
    pub fn new() -> Self {
        Self { urls: Vec::new() }
    }

    pub fn save(&mut self, item: String) -> usize {
        self.urls.push(item);
        self.urls.len() - 1
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        if index >= self.urls.len() {
            None
        } else {
            Some(&self.urls[index])
        }
    }
}
