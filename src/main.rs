use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// translates text forwards
    Forwards {
        /// text to translate
        #[arg(short, long)]
        text: String,
    },
    /// translates text backwards
    Backwards {
        /// text to translate
        #[arg(short, long)]
        text: String
    }
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Forwards { text } => {
            let result = text.chars().into_iter().map(|character| {
                match character {
                    'a' => '4',
                    'c' => '€',
                    'e' => '3',
                    'i' => '1',
                    'L' => '!',
                    'n' => 'π',
                    'o' => '0',
                    's' => 'z',
                    't' => '7',
                    'v' => 'V',
                    'x' => '×',
                    x => x,
                }
            }).collect::<String>();
            println!("{}", result);
        },
        Commands::Backwards { text } => {
            let result = text.chars().into_iter().map(|character| {
                match character {
                    '4' => 'a',
                    '€' => 'c',
                    '3' => 'e',
                    '1' => 'i',
                    '!' => 'L',
                    'π' => 'n',
                    '0' => 'o',
                    'z' => 's',
                    '7' => 't',
                    'V' => 'v',
                    '×' => 'x',
                    x => x,
                }
            }).collect::<String>();
            println!("{}", result);
        }
    }
}
