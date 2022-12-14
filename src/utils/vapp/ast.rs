use hello_macro_derive::Entity;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Entity)]
pub struct Style {
    pub width: String,
    pub height: String,
    pub font_size: String,
    pub color: String,
    pub margin_top: String,
    pub margin_bottom: String,
    pub margin_left: String,
    pub margin_right: String,
    pub padding_top: String,
    pub padding_bottom: String,
    pub padding_left: String,
    pub padding_right: String,
    pub border_radius: String,
    pub border_width: String,
    pub border_color: String,
    pub background_color: String,
    pub opacity: String,
    pub display: String,
    pub flex_direction: String,
    pub justify_content: String,
    pub justify_items: String,
    pub align_content: String,
    pub align_items: String,
}

impl Style {
    pub fn to_style_sheet(&self) -> String {
        self.parser()
    }

    fn parser(&self) -> String {
        let vec_style = self.vec_style();
        let mut style_line = String::from("");
        for (name, value) in vec_style {
            style_line = format!("{}{}:{};", style_line, name, value);
        }
        style_line
    }

    fn vec_style(&self) -> Vec<(&str, &String)> {
        let mut style_vec = vec![];
        style_vec.push(("width", &self.width));
        style_vec.push(("height", &self.height));
        style_vec.push(("font-size", &self.font_size));
        style_vec.push(("color", &self.color));
        style_vec.push(("margin-top", &self.margin_top));
        style_vec.push(("margin-bottom", &self.margin_bottom));
        style_vec.push(("margin-left", &self.margin_left));
        style_vec.push(("margin-right", &self.margin_right));
        style_vec.push(("padding-top", &self.padding_top));
        style_vec.push(("padding-bottom", &self.padding_bottom));
        style_vec.push(("padding-left", &self.padding_left));
        style_vec.push(("padding-right", &self.padding_right));
        style_vec.push(("border-radius", &self.border_radius));
        style_vec.push(("border-width", &self.border_width));
        style_vec.push(("border-color", &self.border_color));
        style_vec.push(("background-color", &self.background_color));
        style_vec.push(("opacity", &self.opacity));
        style_vec.push(("display", &self.display));
        style_vec.push(("flex-direction", &self.flex_direction));
        style_vec.push(("justify-content", &self.justify_content));
        style_vec.push(("justify-items", &self.justify_items));
        style_vec.push(("align-content", &self.align_content));
        style_vec.push(("align-items", &self.align_items));
        style_vec
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Vapp {
    pub project_name: String,
    pub routes: Vec<Routes>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Routes {
    pub id: u8,
    pub name: String,
    pub state: u8,
    pub size: u8,
    pub vnode: Option<VNode>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VNode {
    pub name: String,
    pub tag_name: String,
    pub class: Option<String>,
    pub style: Option<Style>,
    pub content: Option<String>,
    pub children: Vec<VNode>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    pub name: String,
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<Info>,
}
