use std::sync::LazyLock;
static t:LazyLock<u8>=LazyLock::new(||{1+1});
fn main(){
  println!("{}",*t)
  
}
