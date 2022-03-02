use sloth::cmd::init::init;

fn main() {
    env_logger::init();

    let args : Vec<String>= std::env::args().collect();
    let pwd = std::env::current_dir().expect("Unable to obtain the pwd.");
    let command = args.get(1).expect("Invalid command.");
    match &command as &str {
        "init" => init(&pwd.display().to_string(), args.get(2)),
        _ => println!("Command Invalid. - Type help to find the list of commands.")
    }
}
