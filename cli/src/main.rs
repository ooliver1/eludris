mod clean;
mod deploy;
mod logs;
mod static_attachments;
mod stop;

use std::{env, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
#[command(next_line_help = true)]
struct Cli {
    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploys your Eludris instance
    Deploy,
    /// Stops your Eludris instance
    Stop,
    /// Shows you your instance's logs
    Logs,
    /// Static attachment related commands
    Static {
        #[command(subcommand)]
        command: StaticSubcommand,
    },
    /// Removes all info related to your Eludris instance
    #[command(alias = "clear")]
    Clean,
}

#[derive(Subcommand)]
enum StaticSubcommand {
    /// Adds a static attachment
    Add {
        /// Path of the file you want to add
        path: PathBuf,
    },
    /// Removes a static attachment
    Remove {
        /// Name of the attachment you want to remove
        name: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.debug {
        0 => {}
        1 => env::set_var("RUST_LOG", "error"),
        2 => env::set_var("RUST_LOG", "warn"),
        3 => env::set_var("RUST_LOG", "debug"),
        _ => env::set_var("RUST_LOG", "trace"), // >= 4
    };
    env_logger::init();

    match cli.command {
        Commands::Deploy => deploy::deploy()?,
        Commands::Stop => stop::stop()?,
        Commands::Logs => logs::logs()?,
        Commands::Static { command } => match command {
            StaticSubcommand::Add { path } => static_attachments::add(path)?,
            StaticSubcommand::Remove { name } => static_attachments::remove(name)?,
        },
        Commands::Clean => clean::clean()?,
    }

    Ok(())
}
