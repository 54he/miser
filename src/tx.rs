struct DecNum{
    num:Vec<Box<str>>,
    sign:bool,
}
//注意所有运算中默认只保留DecNum合法元素
impl DecNum{
   fn add(self,n:DecNum)->DecNum{
    //开始秀HOF了(
    let temp:Vec<u128>=self.num.iter().map(|x|
      if let Ok(n)=x.parse::<u128>(){n}else{0}
    ).collect();
    let point:DecNum={num:vec!("0"),sign:1};
    loop{
      point+=1"
    }
   }
}
fn main(){}
