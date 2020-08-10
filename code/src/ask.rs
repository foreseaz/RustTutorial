type UserFunc = Box<dyn Fn(u64, String)->bool>;

fn check(myfunc: UserFunc)
{
  
    for i in 0..3 {
        println!("=======  {}", i);
        let r= myfunc(i,format!("my number {}",i));
        println!("result {}", r);
    }

}

fn main() {
    let user_func = | number:u64, info:String| -> bool {
        let a= quest::ask(format!("{}  {}   proceed=", number, info).as_str());
        let b= quest::yesno(true).unwrap().unwrap();
        b
    };

    check(Box::new(user_func));
 
   /* check(

    );
    let a= quest::ask("proceed=");
    let b= quest::yesno(true);
    println!("{:?}", b);*/

}
