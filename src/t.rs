use std::convert::TryInto;
struct NumPlus {
  len:usize,
  sign:bool,
  num:Vec<u128>,
}
impl NumPlus{
   fn raw_new(len:usize,sign:bool,num:Vec<u128>)->NumPlus{
      NumPlus{len,sign,num}
   }
   fn new(new_num:isize) ->NumPlus{
      let n=if let Ok(n)=new_num.try_into(){
        n
      }else{
        panic!("cant creat because new_num are too large ")
      }
      let sign=n>=0;
      NumPlus{len:1,sign,num:vec!(n)}
   };
}
impl fmt::Display{
}
fn main(){
    let a=NumPlus::new();
     
}
