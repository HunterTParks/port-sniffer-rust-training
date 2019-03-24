use std::net::IpAddr;
use std::str::FromStr;

pub struct Arguments {
    flag: String,
    pub ipaddr: IpAddr,
    pub threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not Enough Results");
        } else if args.len() > 4 {
            return Err("Too Many Arguments");
        }

        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});
        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want\r\n -h or -help to show this help message");
                return Err("help");
            } else if flag.contains("h") || flag.contains("-help") {
                return Err("Too Many Arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IPADDR; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}
