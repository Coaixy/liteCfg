mod file;
mod parse;
fn main(){
   let mut a = file::file_info::new("cfg".to_string());
   a.load();
}