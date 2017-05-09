use super::super::{
    Demand,
    Widget,
};
use base::{
    Cursor,
    Window,
};
use super::{
    count_grapheme_clusters,
};

pub struct LineLabel {
    text: String,
}
impl LineLabel {
    pub fn new<S: Into<String>>(text: S) -> Self {
        LineLabel {
            text: text.into(),
        }
    }

    pub fn set<S: Into<String>>(&mut self, text: S) {
        self.text = text.into();
    }
}

impl Widget for LineLabel {
    fn space_demand(&self) -> (Demand, Demand) {
        (Demand::exact(count_grapheme_clusters(&self.text)), Demand::exact(1))
    }
    fn draw(&mut self, mut window: Window) {
        let mut cursor = Cursor::new(&mut window);
        cursor.write(&self.text);
    }
}