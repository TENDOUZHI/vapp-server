use std::{fs::{File, create_dir, remove_dir_all}, io::Write, path::Path};

use super::{lib::Info, parser::parser};


pub fn initial_project(info: Info) {
    let project_name = &info.name;
    let path = Path::new(&project_name);
    remove_dir_all(&path).unwrap();
    create_dir(&project_name).unwrap();
    let file_path = format!("{}/",path.to_str().unwrap());
    create_pages(&file_path);
    create_utils(&file_path);
    create_basic_file(&file_path);
    create_page(&file_path, "index",&info);
    create_page(&file_path, "try", &info);
}



fn create_pages (file_path: &str) {
    let path = format!("{}/pages",file_path);
    create_dir(path).unwrap();
}

fn create_utils(file_path: &str) {
    let path = format!("{}/utils",file_path);
    create_dir(path).unwrap();
}

fn create_basic_file(file_path: &str) {
    let pathjs = format!("{}/app.js",file_path);
    File::create(pathjs).unwrap();
    let pathjson = format!("{}/app.json",file_path);
    File::create(pathjson).unwrap();
    let pathwxss = format!("{}/app.wxss",file_path);
    File::create(pathwxss).unwrap();
    let path_project_config = format!("{}/project.config.json",file_path);
    File::create(path_project_config).unwrap();
    let path_sitmap_json = format!("{}/sitmap.json",file_path);
    File::create(path_sitmap_json).unwrap();
}

fn create_page(file_path: &str, name: &str,data: &Info) {
    let dir_path = format!("{}/pages/{}",&file_path,&name);
    let path = format!("{}/{}",&dir_path,&name);
    create_dir(&dir_path).unwrap();
    let path_js = format!("{}/{}.js",&dir_path,&name);
    let file_js = File::create(path_js).unwrap();
    let path_json = format!("{}/{}.json",&dir_path,&name);
    let file_json = File::create(path_json).unwrap();
    let path_wxml = format!("{}/{}.wxml",&dir_path,&name);
    let mut file_wxml = File::create(path_wxml).unwrap();
    let path_wxss = format!("{}/{}.wxss",&dir_path,&name);
    let file_wxss = File::create(path_wxss).unwrap();
    let file_wxml_content = parser(data);
    file_wxml.write_all(file_wxml_content.as_bytes());
}

// fn create_file(file_path: &str,name: &str) {
//     let path = format!("{}/{}",file_path,name);
//     File::create(path).unwrap();

// }