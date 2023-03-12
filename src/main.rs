use config::Parse;

mod config;
fn main(){
   let mut cfg = config::Cfg::new("cfg".to_string());
   //初始化
   cfg.load();
   //设置数据
   cfg.set_value("code".to_string(), "data".to_string());
   //修改数据
   cfg.set_value("code".to_string(),"data".to_string());
}