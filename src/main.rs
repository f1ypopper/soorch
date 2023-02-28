mod soorch;

use soorch::{create_index, run_server, write_index};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("SOORCH Usage:\nindex <dir> <index_path>: index a directory into a json");
        println!("serve <dir> <address:port>: start an http server on address");
    } else {
        use std::time::Instant;
        let now = Instant::now();
        match args[1].as_str() {
            "index" =>{
                let index = create_index(&args[2]).unwrap();
                write_index(&index, &args[3]).unwrap();
            },
            "serve" => {},
            _ => eprintln!("unknown command"),
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}
