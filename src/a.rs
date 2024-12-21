struct 细菌{
   数量:u32
}
t
impl Iterator for 细菌{
typ
fn next(&mut self) ->u32 {
        self.数量*2
        
}
}
fn main(){
  let mut 细菌a=细菌{数量:50};
  let s =细菌a.next().next().next().next()
  println!("{:?}",s)
}
