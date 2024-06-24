mod cd;
mod echo;
mod find;
mod grep;
mod ls;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "coreutil",
    about = "simple cli",
    version = "0.1.0",
    author = "Your Name"
)]
struct CoreUtil {
    #[command(subcommand)]
    commands: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Echo {
        input: Vec<String>,
        #[arg(short, long, required = false, default_value_t = false)]
        upper: bool,
        #[arg(short, long, required = false, default_value_t = 1)]
        count: isize,
    },
    Ls {
        dir_path: String,
        #[arg(short, long, required = false, default_value_t = false)]
        all: bool,
    },
}

fn main() {
    let c_util = CoreUtil::parse();

    match &c_util.commands {
        Commands::Echo {
            input,
            upper,
            count,
        } => {
            for _ in 0..*count {
                echo::echo(input, upper);
            }
        },
        Commands::Ls {

        }
    }
}
