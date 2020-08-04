fn main() {
    let a="274982379487238@1.2.3.4:3000,2479823798421694@1.2.3.5:4000";
    let b = a.split(",");
    let mut ret:Vec<(String,String,u32)>=vec![];
    for m in b.into_iter() {
        let c:Vec<String>= m.split("@").map(|a|a.to_string()).collect();
        let nodeid= c[0].clone();
        let tmp= c[1].clone();
        tmp.split(":");
        let d:Vec<String>= tmp.split(":").map(|a|a.to_string()).collect();
        let ip= d[0].clone();
        let port = d[1].parse::<u32>().unwrap();
        ret.push((nodeid, ip, port));
        println!("{:?}", ret.last().unwrap());
       
        println!("*****************************");
    }
}
