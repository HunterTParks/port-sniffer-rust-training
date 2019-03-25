/* Standard Library Enum that contains either an Ipv4Addr or an Ipv6Addr  */
use std::net::IpAddr;
/* Pulls in method that parses a string to return a value of this type */
use std::str::FromStr;

/* Struct that holds Ip Address and # of threads */
pub struct Arguments {
    flag: String,
    pub ipaddr: IpAddr,
    pub threads: u16,
}

/* Contains methods for Arguments Struct */
impl Arguments {
    /*
        @Function   : New (Argument)
        @Returns    : Result - Ok<Vector<string> (Arguments), str> OR Err<str>
        @Parameters : args: &[String]

        @Description: Creates a new insatance of an Argument with the arguments given
    */
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        /* If arguments have too many or too few arguments (from the user), will return Err Result with error message */
        if args.len() < 2 {
            return Err("Not Enough Results");
        } else if args.len() > 4 {
            return Err("Too Many Arguments");
        }

        /* Clone IP Address to be used */
        let f = args[1].clone();

        /* If the from_str function returns a Result Ok type, return a Result that contains a new instance of an Argument */
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});
        } else {
            /* Variable declaration */
            let flag = args[1].clone();

            /* If # of parameters are met AND the user inputs the 'help' flag */
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want\r\n -h or -help to show this help message");
                return Err("help");

            /* If # of parameters are not met AND user inputs the 'help' flag */
            } else if flag.contains("h") || flag.contains("-help") {
                return Err("Too Many Arguments");

            /* If user inputs 'Threads' flag */
            } else if flag.contains("-j") {

                /* Check if IP Address entered is valid */
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IPADDR; must be IPv4 or IPv6")
                };

                /* Check if Threads input entered is valid */
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };

                /* Return Result Ok type with instance of an Argument */
                return Ok(Arguments{threads, flag, ipaddr});
            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}
