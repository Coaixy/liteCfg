#![allow(unused_variables)]
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs;
use std::string;
use std::vec;
pub struct Cfg {
    pub data: Vec<String>,
    pub path: String,
}
impl Cfg {
    pub fn load(&mut self) {
        //文件是否存在
        let flag = match fs::metadata(&self.path) {
            Ok(_) => true,
            Err(_) => false,
        };
        //不存在则创建
        if !flag {
            let file = File::create(&self.path).unwrap();
        }
        //获取数据
        self.get_data();
    }

    pub fn new(file_path: String) -> Self {
        Self {
            path: file_path,
            data: vec![],
        }
    }
    //获取文本
    fn read_text(&mut self) -> String {
        let file_path = &self.path;
        match fs::read_to_string(file_path) {
            Ok(contents) => return contents,
            Err(e) => panic!("Error reading file: {}", e),
        }
    }
    //解析文本
    fn get_data(&mut self) {
        let mut vec: Vec<String> = vec![];
        for i in self.read_text().lines() {
            let str = String::from(i);
            if str.contains("#") == false {
                vec.push(str);
            }
        }
        self.data = vec;
    }
}
pub trait Parse{
    fn get_value(&mut self,code:String) -> String;
    fn set_value(&mut self,code:String,data:String);
}
impl Parse for Cfg{
    fn get_value(&mut self,code:String) -> String{
        self.get_data();
        for i in &self.data {
            let mut line_data = i.split("=");
            if code == line_data.nth(0).unwrap().trim(){
                return line_data.nth(0).unwrap().trim().to_string();
            }
            // println!("{}.{}",line_data.nth(0).unwrap().trim(),line_data.nth(0).unwrap().trim())
        }
        "".to_string()
    }
    fn set_value(&mut self,code:String,data:String){
        let mut text = String::from("");
        let mut flag = false;
        self.get_data();
        for i in &self.data{
            let mut line_data = i.split("=");
            //存在的话就修改
            if code == line_data.nth(0).unwrap().trim(){
                flag = true;
                text += &code;
                text += &String::from("=");
                text += &data;
                text += &String::from("\n");
            //其他的直接加入
            }else{
                text += &i.to_owned();
                text += &String::from("\n");
            }
            
        }
        //如果不存在的话就加入
        if !flag {
            text += &code;
            text += &String::from("=");
            text += &data;
            text += &String::from("\n");
        }
        //写入文件
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path).unwrap();
        file.write_all(text.as_bytes());
    }
}