/* Standard Library module that handles basic I/O */
use std::io::{self, Write};
/* Standard Library module that handles TCP/UDP functionality  */
use std::net::{IpAddr, TcpStream};
/* Sender from the Asynchronous Channel type */
use std::sync::mpsc::{Sender};

/* Constant variable that contains max # of ports available */
const MAX: u16 = 65535;

/*
    @Function   : Scan
    @Returns    : Nothing
    @Parameters : tx: Sender<u16>, start_port: integer, addr: IpAddr, num_threads: integer

    @Description: Connects to each port and scans them. If connection is successful (port is open), send port number to receiver. If connection is not successful, do nothing.
*/
pub fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    /* Variable declaration */
    let mut port: u16 = start_port + 1;

    /* Loop through each port and attempt connection */
    loop {
        /* Match port | If connection is successful (port is open), send port number to receiver. If connection is not successful, do nothing. */
        match TcpStream::connect((addr, port)) {
            /* If port is open */
            Ok(_) => {
                print!(".");
                /*
                    stdout() : Output port info
                    flush()  : Ensure contents reach destination
                    unwrap() : If no value, stop application (convert to Expect)
                */
                io::stdout().flush().unwrap();

                /* Send to receiver */
                tx.send(port).unwrap();
            }
            Err(_) => {
                /* Do nothing */
            }
        }

        /* If the only ports left to scan are already handled by separate threads, break out of loop */
        if (MAX - port) <= num_threads {
            break;
        }

        /* Increment through loop by the amount of threads in use to avoid 2 threads working on same port */
        port += num_threads;
    }
}
