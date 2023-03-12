use std::clone;
use std::fs::File;
use std::io::Write;
use std::string;
use std::fs;
use std::vec;
pub struct file_info{
    pub data:Vec<String>,
    pub path:String
}
impl file_info{

    pub fn load(&mut self){
        let mut flag = false;
        match fs::metadata(&self.path){
            Ok(_) => flag = true ,
            Err(_) => flag = false,
        }
        //文件不存在
        if !flag {
            let mut file = File::create(&self.path).unwrap();
            file.write_all(b"#Created By liteCfg\n");
        }
        self.get_data();
    }

    pub fn new(file_path:String) -> Self{
        Self{
            path:file_path,
            data:vec![]
        }
    }
    fn read_text(&mut self) -> String{
        let file_path = &self.path;
        match fs::read_to_string(file_path) {
            Ok(contents) => return contents,
            Err(e) =>panic!("Error reading file: {}", e),
        }
    }

    fn get_data(&mut self){
        let mut vec:Vec<String> = vec![];
        for i in self.read_text().lines(){
            let str = String::from(i);
            if str.contains("#") == false {
                vec.push(str);
            }
        }
        self.data = vec;
    }
}