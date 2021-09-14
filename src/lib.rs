mod component;

use stylist::Style;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::component::Model;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

pub fn css<'a>(msg: impl stylist::ast::IntoSheet<'a>) -> Style {
    Style::new(msg).unwrap()
}
