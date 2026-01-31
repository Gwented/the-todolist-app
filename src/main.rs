use todol::{command, iyo::config::GlobalConfig};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let global_cfg = GlobalConfig::new();

    match command::exec(&global_cfg, &args) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
            //DID THIS MAKE A DIFFERENCE?
        }
    }
}
