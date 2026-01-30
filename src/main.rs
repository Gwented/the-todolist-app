use todol::command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match command::exec(&args) {
        Ok(()) => (),
        Err(err) => eprintln!("{err}"),
    }
}
