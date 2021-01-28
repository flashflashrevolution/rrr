#[derive(Debug, Clone)]
pub struct ListState {
    offset: usize,
    selected: Option<usize>,
}

impl Default for ListState {
    fn default() -> ListState {
        ListState {
            offset: 0,
            selected: None,
        }
    }
}

impl ListState {
    fn selected(&mut self) -> Option<usize> {
        self.selected
    }

    fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }
}

pub struct List<T> {
    pub items: Vec<T>,
}

pub trait SingleSelectList {
    fn deselect(&mut self);
    fn next(&mut self);
    fn previous(&mut self);
    fn select(&mut self, targetIndex: u32);
}

pub trait MultiSelectList {
    fn deselect(&mut self);
    fn deselect(&mut self, targetIndex: u32);
    fn deselect_all(&mut self);
    fn highlight_next(&mut self);
    fn highligh_previous(&mut self);
    fn select(&mut self);
    fn select(&mut self, targetIndex: u32);
}
