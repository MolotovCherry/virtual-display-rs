use clap::Parser;
use client::Client;
use lazy_format::lazy_format;
use owo_colors::OwoColorize;

mod client;

#[derive(Debug, Parser)]
struct Args {
    /// Format output as JSON.
    #[clap(short, long)]
    json: bool,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    /// List currently connected virtual monitors.
    List,
    /// Add a new virtual monitor.
    Add {
        width: driver_ipc::Dimen,

        height: driver_ipc::Dimen,

        #[clap(short, long, default_value = "60")]
        refresh_rates: Vec<driver_ipc::RefreshRate>,

        #[clap(long)]
        id: Option<driver_ipc::Id>,

        #[clap(long)]
        name: Option<String>,
    },
    /// Remove one or more virtual monitors.
    Remove { id: Vec<driver_ipc::Id> },
    /// Remove all virtual monitors.
    RemoveAll,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::List => {
            let mut client = Client::connect()?;

            let monitors = client.list()?;

            if args.json {
                let mut stdout = std::io::stdout().lock();
                serde_json::to_writer(&mut stdout, &monitors)?;
            } else {
                if monitors.len() > 0 {
                    println!("{}", "Virtual monitors".underline());
                    for monitor in monitors {
                        let name_label = lazy_format!(match (&monitor.name) {
                            Some(name) => (" {}{name}{}", "[".dimmed(), "]".dimmed()),
                            None => "",
                        });
                        let primary_mode = monitor.modes.get(0);
                        let primary_width = lazy_format!(match (primary_mode) {
                            Some(mode) => ("{}", mode.width.green()),
                            None => ("{}", "?".red()),
                        });
                        let primary_height = lazy_format!(match (primary_mode) {
                            Some(mode) => ("{}", mode.height.green()),
                            None => ("{}", "?".red()),
                        });
                        let primary_refresh =
                            primary_mode.and_then(|mode| mode.refresh_rates.get(0));
                        let primary_refresh = lazy_format!(match (primary_refresh) {
                            Some(refresh) => ("{}Hz", refresh.blue()),
                            None => ("{}Hz", "?".red()),
                        });
                        println!(
                            "  {}{name_label}: {} x {} @ {}",
                            monitor.id.blue(),
                            primary_width,
                            primary_height,
                            primary_refresh,
                        );
                    }
                } else {
                    println!("No virtual monitors found.");
                }
            }
        }
        Command::Add {
            width,
            height,
            refresh_rates,
            id,
            name,
        } => {
            let mut client = Client::connect()?;
            let id = match id {
                Some(id) => id,
                None => client.next_id()?,
            };
            let new_monitor = driver_ipc::Monitor {
                id,
                enabled: true,
                name,
                modes: vec![driver_ipc::Mode {
                    width,
                    height,
                    refresh_rates,
                }],
            };
            client.notify(vec![new_monitor])?;

            if args.json {
                let mut stdout = std::io::stdout().lock();
                serde_json::to_writer(&mut stdout, &id)?;
            } else {
                println!("Added virtual monitor with id {}.", id.green());
            }
        }
        Command::Remove { id } => {
            let mut client = Client::connect()?;
            client.remove(id.clone())?;

            if args.json {
                let mut stdout = std::io::stdout().lock();
                serde_json::to_writer(&mut stdout, &id)?;
            } else {
                if id.len() == 1 {
                    println!("Removed virtual monitor.");
                } else {
                    println!("Removed {} virtual monitors.", id.len());
                }
            }
        }
        Command::RemoveAll => {
            let mut client = Client::connect()?;
            client.remove_all()?;

            if args.json {
                let mut stdout = std::io::stdout().lock();
                serde_json::to_writer(&mut stdout, &())?;
            } else {
                println!("Removed all virtual monitors.");
            }
        }
    }

    Ok(())
}
