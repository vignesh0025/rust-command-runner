use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Nil"))]
    cmd: String,

    #[arg(short, long, default_value_t = 1)]
    run_test_times: u8
}

fn main() {
    let args = Args::parse();

    let mut command_args = args.cmd.split_whitespace();
    let cmd = command_args.next().expect("some arg");

    let mut execution = Command::new(cmd);

    /*
    loop {
        match command_args.next() {
            Some(arg) => execution.arg(arg),
            None => break
        };
    } */

    while let Some(arg) = command_args.next() {
        execution.arg(arg);
    }

    for i in 0..args.run_test_times {
        /* Test */
        let result = execution.output();

        if let Ok(output) = result {
            let out = String::from_utf8(output.stdout).expect("Unable to decode utf8");
            println!("{}", out);
        } else {
            println!("Error executing command");
        }
    }

    /*
       if let Ok(output) = Output {
       let out = String::from_utf8(output.stdout);
       match out {
       Ok(s) => println!("Output {s}"),
       Err(_) => println!("Unable to convert to utf8")
       }
       } else if let Err (err) = Output {
       println!("Err: {}", err);
       }
    */
}
