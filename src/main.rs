use config::{ParseConfig};

mod config;
fn main(){
   let mut cfg = config::Config::new("cfg");
   //初始化
   cfg.load();
   //设置数据
   cfg.set_value("code","test");
   //修改数据
   cfg.set_value("code","data");
   //测试输出
   println!("{}",cfg.get_value("code").unwrap())
}