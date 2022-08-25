use std::{
    fs::{create_dir, remove_dir_all, File},
    io::Write,
    path::Path,
};

use super::{lib::Info, parser::parser, json_renderer::{write_app_json, write_project_config_json, write_sitmap_json}};

pub fn initial_project(info: Info) {
    let project_name = &info.name;
    let dev_path = format!("mini/{}", &project_name);
    let path = Path::new(&dev_path);
    remove_dir_all(&path).expect("occur at remove_dir_all");
    create_dir(&path).expect("create_root_dir");
    let file_path = format!("{}", path.to_str().unwrap());
    create_pages(&file_path);
    create_utils(&file_path);
    create_basic_file(&file_path);
    create_page(&file_path, "index", &info);
    create_page(&file_path, "try", &info);
}

fn create_pages(file_path: &str) {
    let path = format!("{}/pages", file_path);
    create_dir(path).expect("create_pages");
}

fn create_utils(file_path: &str) {
    let path = format!("{}/utils", file_path);
    create_dir(path).expect("create_utils");
}

// initial the basic file of miniprogram
fn create_basic_file(file_path: &str) {
    // initial file path
    let pathjs = format!("{}/app.js", file_path);
    let pathjson = format!("{}/app.json", file_path);
    let pathwxss = format!("{}/app.wxss", file_path);
    let path_sitmap_json = format!("{}/sitmap.json", file_path);
    let path_project_config = format!("{}/project.config.json", file_path);
    // initial file instance
    let mut app_js = File::create(pathjs).unwrap();
    let mut app_wxss = File::create(pathwxss).unwrap();
    let mut app_json = File::create(pathjson).unwrap();
    let mut project_config = File::create(path_project_config).unwrap();
    let mut sitmap_json = File::create(path_sitmap_json).unwrap();
    // write json file
    write_app_json(&mut app_json).expect("write in app.json");
    write_project_config_json(&mut project_config).expect("write in project.config.json");
    write_sitmap_json(&mut sitmap_json).expect("write in sitmap.json");
    
}

fn create_page(file_path: &str, name: &str, data: &Info) {
    let dir_path = format!("{}/pages/{}", &file_path, &name);
    let path = format!("{}/{}", &dir_path, &name);
    create_dir(&dir_path).unwrap();
    let path_js = format!("{}/{}.js", &dir_path, &name);
    let path_json = format!("{}/{}.json", &dir_path, &name);
    let path_wxml = format!("{}/{}.wxml", &dir_path, &name);
    let path_wxss = format!("{}/{}.wxss", &dir_path, &name);
    let file_js = File::create(path_js).unwrap();
    let file_json = File::create(path_json).unwrap();
    let mut file_wxml = File::create(path_wxml).unwrap();
    let file_wxss = File::create(path_wxss).unwrap();
    let file_wxml_content = parser(data);
    file_wxml.write_all(file_wxml_content.as_bytes());
}