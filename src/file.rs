use std::clone;
use std::string;
use std::fs;
use std::vec;
struct file_info{
    data:Vec<String>,
    path:String
}
impl file_info{

    pub fn load(mut self,file_path:String){
        match fs::metadata(&file_path){
            Ok(_) => (),
            Err(_) => panic!("file not exists"),
        }
        self.path = file_path
    }

    pub fn get_object(self) -> file_info{
        self
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
            vec.push(str);
        }
        self.data = vec;
    }
}