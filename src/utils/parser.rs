use super::lib::{Info, Style};

pub fn parser(data: &Info) -> String {
    parse_figment(data)
}

fn parse_figment(data: &Info) -> String {
    let figment = parse_node(data);
    let res = format!("<view>{figment}</view>");
    res
}

fn parse_node(vnode: &Info) -> String {
    let mut el = String::from("");
    for node in &vnode.children {
        // 同一树枝上的节点
        let deep_node = parse_node(node);
        // 同一层内的节点
        let back_node:String;
        if node.children.len() == 0 {
            back_node = write_tag(&node.name, node.content.as_ref().unwrap(),node.style.to_owned());
        } else {
            back_node = write_tag(&node.name, &deep_node,node.style.to_owned());
        }

        el = format!("{}{}", el, back_node)
    }
    el
}
fn write_tag(name: &str, content: &str, style: Style) -> String {
    format!("<{name} style={:?}>{content}</{name}>",content)
}




