use hello_macro_derive::Entity;
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize, Debug,Clone,Entity)]
pub struct Style {
    pub width: String,
    pub height: String,
    pub fontSize: String,
    pub color: String,
    pub marginTop: String,
    pub marginBottom: String,
    pub marginLeft: String,
    pub marginRight: String,
    pub paddingTop: String,
    pub paddingBottom: String,
    pub paddingLeft: String,
    pub paddingRight: String,
    pub borderRadius: String,
    pub borderWidth: String,
    pub borderColor: String,
    pub backgroundColor: String,
    pub opacity: String,
    pub display: String,
    pub flexDirection: String,
    pub justifyContent: String,
    pub justifyItems: String,
    pub alignContent: String,
    pub alignItems: String,
}

impl Style {
    pub fn to_style_sheet(&self) -> String {
        self.parser()
    }

    fn parser(&self) -> String {
        let vec_style = self.vec_style();
        let mut style_line = String::from("");
        for (name, value) in vec_style {
            style_line = format!("{}{}:{};",style_line,name,value);
        }
        style_line
    }

    fn vec_style(&self) -> Vec<(&str, &String)> {
        let mut style_vec = vec![];
        style_vec.push(("width",&self.width));
        style_vec.push(("height",&self.height));
        style_vec.push(("font-size",&self.fontSize));
        style_vec.push(("color",&self.color));
        style_vec.push(("margin-top",&self.marginTop));
        style_vec.push(("margin-bottom",&self.marginBottom));
        style_vec.push(("margin-left",&self.marginLeft));
        style_vec.push(("margin-right",&self.marginRight));
        style_vec.push(("padding-top",&self.paddingTop));
        style_vec.push(("padding-bottom",&self.paddingBottom));
        style_vec.push(("padding-left",&self.paddingLeft));
        style_vec.push(("padding-right",&self.paddingRight));
        style_vec.push(("border-radius",&self.borderRadius));
        style_vec.push(("border-width",&self.borderWidth));
        style_vec.push(("border-color",&self.borderColor));
        style_vec.push(("background-color",&self.backgroundColor));
        style_vec.push(("opacity",&self.opacity));
        style_vec.push(("display",&self.display));
        style_vec.push(("flex-direction",&self.flexDirection));
        style_vec.push(("justify-content",&self.justifyContent));
        style_vec.push(("justify-items",&self.justifyItems));
        style_vec.push(("align-content",&self.alignContent));
        style_vec.push(("align-items",&self.alignItems));
        style_vec
    }


}

#[derive(Deserialize,Serialize, Debug)]
pub struct Vapp {
    pub project_name: String,
    pub routes: Vec<Routes>
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Routes {
    pub id: u8,
    pub name: String,
    pub state: u8,
    pub size: u8,
    pub vnode: Option<VNode>
}

#[derive(Deserialize,Serialize, Debug)]
pub struct VNode {
    pub name: String,
    pub tag_name: String,
    pub class: Option<String>,
    pub style: Option<Style>,
    pub content: Option<String>,
    pub children: Vec<VNode>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Info {
    pub name: String,
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<Info>,
}