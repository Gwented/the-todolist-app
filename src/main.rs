use todol::{command, error::TodoError, iyo::config::GlobalConfig};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let global_cfg = GlobalConfig::new();

    match command::exec(&global_cfg, &args) {
        Ok(()) => (),
        Err(err) => match err {
            TodoError::InvalidSyntax(err_ctx) => {
                eprintln!("{}", TodoError::InvalidSyntax(err_ctx));
            }
            // todol::error::TodoError::TitleNotFound(_) => todo!(),
            // todol::error::TodoError::IO(error_kind) => todo!(),
            e => eprintln!("{e}"),
        },
    }
}
