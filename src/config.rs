use std::fs::{self, File, OpenOptions};
use std::io::Write;

pub struct Config {
    path: String,   // 配置文件的路径
    data: Vec<String>,   // 配置文件中的数据
}

impl Config {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            data: Vec::new(),   // 初始化为空向量
        }
    }

    pub fn load(&mut self) {
        if let Err(_) = fs::metadata(&self.path) {
            // 如果文件不存在，则创建一个新文件
            File::create(&self.path).expect("Failed to create file");
        }
        // 读取并解析配置文件
        self.get_data();
    }

    fn get_data(&mut self) {
        let mut contents = String::new();
        if let Ok(data) = fs::read_to_string(&self.path) {
            contents = data;
        }
        // 过滤掉注释行和空行，并将其余行存入向量中
        self.data = contents
            .lines()
            .filter(|line| !line.starts_with('#'))
            .map(String::from)
            .collect();
    }
}

pub trait ParseConfig {
    fn get_value(&self, key: &str) -> Option<String>;
    fn set_value(&mut self, key: &str, value: &str);
}

impl ParseConfig for Config {
    fn get_value(&self, key: &str) -> Option<String> {
        // 查找与指定键匹配的行，并返回其对应的值
        self.data.iter().find_map(|line| {
            let mut parts = line.split('=');
            let current_key = parts.next()?.trim();
            if current_key == key {
                Some(parts.next()?.trim().to_owned())
            } else {
                None
            }
        })
    }

    fn set_value(&mut self, key: &str, value: &str) {
        let new_line = format!("{}={}", key, value);

        if let Some(position) = self.data.iter().position(|line| line.starts_with(&key)) {
            // 如果存在与指定键匹配的行，则替换该行
            self.data[position] = new_line;
        } else {
            // 否则，在末尾添加新行
            self.data.push(new_line);
        }

        // 将更改后的配置写回到文件中
        let new_contents = self.data.join("\n");
        if let Ok(mut file) = OpenOptions::new().write(true).open(&self.path) {
            file.write_all(new_contents.as_bytes()).expect("Unable to write to file");
        }
    }
}
