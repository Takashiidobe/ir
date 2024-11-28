use clap::Parser;
use ir::error::EvalError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    repl: bool,
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<(), EvalError> {
    // let args = Args::parse();

    // if args.repl {
    //     repl();
    // } else {
    //     match args.file {
    //         Some(file) => {
    //             let program = std::fs::read_to_string(file).unwrap();
    //         }
    //         None => Ok(()),
    //     }
    // }

    Ok(())
}
