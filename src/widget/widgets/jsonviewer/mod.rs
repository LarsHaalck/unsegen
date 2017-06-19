//#[macro_use]
//extern crate json;

use super::super::{
    Demand,
    Demand2D,
    RenderingHints,
    Widget,
};
use base::{
    Cursor,
    ExtentEstimationWindow,
    StyleModifier,
    Window,
};

use input::{
    Scrollable,
    OperationResult,
};

pub use json as json_ext;

use json::{
    JsonValue,
};

mod path;
mod displayvalue;

use self::path::*;
use self::displayvalue::*;

pub struct JsonViewer {
    value: DisplayValue,
    active_element: Path,
    indentation: u16,
    active_focused_style: StyleModifier,
    inactive_focused_style: StyleModifier,
}

impl JsonViewer {
    pub fn new(value: &JsonValue) -> Self {
        let mut res = JsonViewer {
            value: DisplayValue::from_json(&value),
            active_element: Path::Scalar, //Will be fixed ...
            indentation: 2,
            active_focused_style: StyleModifier::new().invert().bold(true),
            inactive_focused_style: StyleModifier::new().bold(true),
        };
        res.fix_active_element_path(); //... here!
        res
    }

    pub fn reset(&mut self, value: &JsonValue) {
        self.value = DisplayValue::from_json(value);
        self.fix_active_element_path();
    }

    pub fn replace(&mut self, value: &JsonValue) {
        self.value = self.value.replace(value);
        self.fix_active_element_path();
    }

    pub fn select_next(&mut self) -> Result<(),()> {
        if let Some(new_path) = self.active_element.clone().find_next_path(&self.value) {
            self.active_element = new_path;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn select_previous(&mut self) -> Result<(),()> {
        if let Some(new_path) = self.active_element.clone().find_previous_path(&self.value) {
            self.active_element = new_path;
            Ok(())
        } else {
            Err(())
        }
    }

    fn fix_active_element_path(&mut self) {
        let mut tmp = Path::Scalar;
        ::std::mem::swap(&mut self.active_element, &mut tmp);
        self.active_element = tmp.fix_path_for_value(&self.value)
    }

    pub fn toggle_active_element(&mut self) -> Result<(),()> {
        self.active_element.find_and_act_on_element(&mut self.value)
    }
}

impl Widget for JsonViewer {
    fn space_demand(&self) -> Demand2D {
        let mut window = ExtentEstimationWindow::unbounded();
        //TODO: We may want to consider passing hints to space_demand as well for an accurate estimate
        {
            let mut cursor = Cursor::<ExtentEstimationWindow>::new(&mut window);
            let info = RenderingInfo {
                hints: RenderingHints::default(),
                active_focused_style: self.active_focused_style,
                inactive_focused_style: self.inactive_focused_style,
            };
            self.value.draw(&mut cursor, Some(&self.active_element), &info, self.indentation);
        }
        Demand2D {
            width: Demand::at_least(window.extent_x()),
            height: Demand::exact(window.extent_y()),
        }
    }
    fn draw(&mut self, mut window: Window, hints: RenderingHints) {
        let mut cursor = Cursor::new(&mut window);
        let info = RenderingInfo {
            hints: hints,
            active_focused_style: self.active_focused_style,
            inactive_focused_style: self.inactive_focused_style,
        };
        self.value.draw(&mut cursor, Some(&self.active_element), &info, self.indentation);
    }
}

impl Scrollable for JsonViewer {
    fn scroll_forwards(&mut self) -> OperationResult {
        self.select_next()
    }
    fn scroll_backwards(&mut self) -> OperationResult {
        self.select_previous()
    }
}