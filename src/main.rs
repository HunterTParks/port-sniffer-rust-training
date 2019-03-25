/* Module contains functions to read Environment Variables, Process Arguments, etc... */
use std::env;
/* Module for working with Processes */
use std::process;
/* Module for handling use of multiple threads */
use std::thread;
/* Module for handling asynchronous channels */
use std::sync::mpsc::channel;

/* Arguments module | ~./src/arguments/mod.rs */
mod arguments;
/* Scan Module | ~./src/scanner/mod.rs */
pub mod scanner;

fn main() {
    /* Gets input from command line and collects them into a Vector */
    let args: Vec<String> = env::args().collect();

    /* Get program name used for error handling */
    let program = args[0].clone();

    let arguments = match arguments::Arguments::new(&args) {
        Ok(arguments) => arguments,
        Err(error) => {
            if error.contains("help") {
                /* If arguments contained "help", don't display error message */
                process::exit(0);
            } else {
                /* Print out the error message */
                eprintln!("{} | Problem parsing arguments: {}", program, error);
                process::exit(0);
            }
        }
    };

    /* Declare Variables | # of threads, IP Address, and Sender/Receiver channels from the arguments given from the user */
    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();

    /* Spawns # of threads from the arguments given  */
    for i in 0..num_threads {
        /* Copy Receiver */
        let tx = tx.clone();

        /* Each thread will scan for ports and place them in collection */
        thread::spawn(move || {
            scanner::scan(tx, i, addr, num_threads);
        });
    }

    /* Declare new empty vector */
    let mut out = vec![];

    /* Drop Receiver */
    drop(tx);
    /* Add open ports to vector */
    for p in rx {
        out.push(p);
    }

    /* Print Empty line */
    println!("");
    /* Sort vector */
    out.sort();

    /* Print each port */
    for v in out {
        println!("{} is open", v);
    }

}
