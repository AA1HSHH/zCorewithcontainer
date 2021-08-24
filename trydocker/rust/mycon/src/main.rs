use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let command = &args[1];
    match command.as_str() {
        "run"   => run(),
        "child" => child(),
        _       => {}
    }
}

fn run() {
    println!("run");
}

fn child() {
    println!("child");
}
