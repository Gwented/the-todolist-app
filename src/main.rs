use todol::command;
use todol::config::GlobalConfig;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // You don't need detailed errors if you don't
    // make mistakes.
    let global_cfg = GlobalConfig::new();
    match command::exec(&global_cfg, &args) {
        Ok(()) => (),
        Err(err) => eprintln!("{err}"),
    }
}
