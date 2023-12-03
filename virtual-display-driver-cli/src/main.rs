use clap::Parser;
use client::Client;
use joinery::JoinableIterator;
use lazy_format::lazy_format;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

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
        /// Width of the virtual monitor.
        width: driver_ipc::Dimen,

        /// Height of the virtual monitor.
        height: driver_ipc::Dimen,

        /// More resolutions to add as extra modes for the virtual monitor.
        more_widths_and_heights: Vec<driver_ipc::Dimen>,

        /// Refresh rate of the virtual monitor. Pass multiple times to
        /// support multiple refresh rates.
        #[clap(short, long, default_value = "60")]
        refresh_rate: Vec<driver_ipc::RefreshRate>,

        /// Manual ID to set for the monitor. Must not conflict with an
        /// existing virtual monitor's ID.
        #[clap(long)]
        id: Option<driver_ipc::Id>,

        /// Optional label for the virtual monitor.
        #[clap(long)]
        name: Option<String>,

        /// Set the virtual monitor to disabled on creation.
        #[clap(long)]
        disabled: bool,
    },
    /// Enable a virtual monitor.
    Enable { id: driver_ipc::Id },
    /// Disable one or more virtual monitors.
    Disable { id: driver_ipc::Id },
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
                serde_json::to_writer_pretty(&mut stdout, &monitors)?;
            } else {
                if monitors.len() > 0 {
                    println!("{}", "Virtual monitors".underline());
                    for (i, monitor) in monitors.iter().enumerate() {
                        if i > 0 {
                            println!();
                        }

                        let name_label = lazy_format!(match (&monitor.name) {
                            Some(name) => (" {}{name}{}", "[".dimmed(), "]".dimmed()),
                            None => "",
                        });
                        let disabled_label = lazy_format!(if monitor.enabled => ""
                        else =>
                            (" {}", "(disabled)".red())
                        );
                        println!("{}{name_label}{disabled_label}:", monitor.id.blue(),);

                        if monitor.modes.len() > 0 {
                            for mode in &monitor.modes {
                                let refresh_rate_labels = mode
                                    .refresh_rates
                                    .iter()
                                    .map(|rate| lazy_format!("{}", rate.blue()))
                                    .join_with("/");
                                let refresh_rates = lazy_format!(if mode.refresh_rates.is_empty() =>
                                    ("{}Hz", "?".red())
                                else =>
                                    ("{}Hz", refresh_rate_labels)
                                );
                                println!(
                                    "{} {}x{} @ {}",
                                    "-".dimmed(),
                                    mode.width.green(),
                                    mode.height.green(),
                                    refresh_rates
                                );
                            }
                        } else {
                            println!("{} {}", "-".dimmed(), "No modes".red());
                        }
                    }
                } else {
                    println!("No virtual monitors found.");
                }
            }
        }
        Command::Add {
            width,
            height,
            more_widths_and_heights,
            refresh_rate,
            id,
            name,
            disabled,
        } => {
            if more_widths_and_heights.len() % 2 != 0 {
                eyre::bail!("passed a width for an extra resolution without a height");
            }

            let modes = std::iter::once(&[width, height][..])
                .chain(more_widths_and_heights.chunks_exact(2))
                .map(|dim| driver_ipc::Mode {
                    width: dim[0],
                    height: dim[1],
                    refresh_rates: refresh_rate.clone(),
                })
                .collect();

            let mut client = Client::connect()?;
            let id = client.new_id(id)?;
            let new_monitor = driver_ipc::Monitor {
                id,
                enabled: !disabled,
                name,
                modes,
            };
            client.notify(vec![new_monitor])?;

            if args.json {
                let mut stdout = std::io::stdout().lock();
                serde_json::to_writer_pretty(&mut stdout, &id)?;
            } else {
                let disabled_footnote = lazy_format!(
                    if disabled => (" {}", "(disabled)".red())
                    else => ""
                );
                println!(
                    "Added virtual monitor with ID {}{disabled_footnote}.",
                    id.green()
                );
            }
        }
        Command::Enable { id } => {
            let mut client = Client::connect()?;
            let outcome = set_enabled(&mut client, id, true)?;

            if args.json {
                let mut stdout = std::io::stdout().lock();
                serde_json::to_writer_pretty(&mut stdout, &outcome)?;
            } else {
                let footnote = if outcome.toggled {
                    ""
                } else {
                    " (was already enabled)"
                };
                println!("Enabled virtual monitor with ID {}{footnote}.", id.green());
            }
        }
        Command::Disable { id } => {
            let mut client = Client::connect()?;
            let outcome = set_enabled(&mut client, id, false)?;

            if args.json {
                let mut stdout = std::io::stdout().lock();
                serde_json::to_writer_pretty(&mut stdout, &outcome)?;
            } else {
                let footnote = if outcome.toggled {
                    ""
                } else {
                    " (was already disabled)"
                };
                println!("Disabled virtual monitor with ID {}{footnote}.", id.green());
            }
        }
        Command::Remove { id } => {
            let mut client = Client::connect()?;
            client.remove(id.clone())?;

            if args.json {
                let mut stdout = std::io::stdout().lock();
                serde_json::to_writer_pretty(&mut stdout, &id)?;
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
                serde_json::to_writer_pretty(&mut stdout, &())?;
            } else {
                println!("Removed all virtual monitors.");
            }
        }
    }

    Ok(())
}

fn set_enabled(
    client: &mut Client,
    id: driver_ipc::Id,
    enabled: bool,
) -> eyre::Result<EnableDisableOutcome> {
    let monitors = client.list()?;
    let monitor = monitors.into_iter().find(|monitor| monitor.id == id);
    let Some(mut monitor) = monitor else {
        eyre::bail!("no virtual monitor with ID {} found", id);
    };

    let should_toggle = enabled != monitor.enabled;

    if should_toggle {
        monitor.enabled = enabled;
        client.notify(vec![monitor.clone()])?;
    }

    Ok(EnableDisableOutcome {
        monitor,
        toggled: should_toggle,
    })
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct EnableDisableOutcome {
    monitor: driver_ipc::Monitor,
    toggled: bool,
}
