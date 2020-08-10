fn parse_node(a: &str) -> Result<Vec<(String, String, u32)>,failure::Error> {
    let split_nodes = a.split(",");
    let mut all_nodes: Vec<(String, String, u32)> = vec![];
    for a_node in split_nodes.into_iter() {
        let split_nodeid: Vec<String> = a_node.split("@").map(|a| a.to_string()).collect();
        let nodeid = split_nodeid[0].clone();
        let ip_port = split_nodeid[1].clone();
        ip_port.split(":");
        let split_ip: Vec<String> = ip_port.split(":").map(|a| a.to_string()).collect();
        let ip = split_ip[0].clone();
        let port = split_ip[1].parse::<u32>()?;
        all_nodes.push((nodeid, ip, port));
    }
    Ok(all_nodes)
}

fn main() {
    let a = "274982379487238@1.2.3.4:3000,2479823798421694@1.2.3.5:4000";

    let mut ret: Vec<(String, String, u32)> = parse_node(a).unwrap();
    println!("{:?}", ret);
}
