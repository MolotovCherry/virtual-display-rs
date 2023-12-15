use clap::Parser;
use client::Client;
use joinery::JoinableIterator;
use lazy_format::lazy_format;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

mod client;

#[derive(Debug, Parser)]
struct Args {
    #[clap(flatten)]
    options: GlobalOptions,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
struct GlobalOptions {
    /// Format output as JSON.
    #[clap(short, long)]
    json: bool,
}

#[derive(Debug, Parser)]
enum Command {
    /// List currently connected virtual monitors.
    List,
    /// Add a new virtual monitor.
    Add(AddCommand),
    /// Add a new resolution/refresh rate mode to an existing virtual monitor.
    AddMode(AddModeCommand),
    /// Remove a resolution/refresh rate mode to an existing virtual monitor.
    RemoveMode(RemoveModeCommand),
    /// Enable a virtual monitor.
    Enable(EnableCommand),
    /// Disable a virtual monitor.
    Disable(DisableCommand),
    /// Remove one or more virtual monitors.
    Remove(RemoveCommand),
    /// Remove all virtual monitors.
    RemoveAll,
}

#[derive(Debug, Parser)]
struct AddCommand {
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
}

#[derive(Debug, Parser)]
struct AddModeCommand {
    /// ID of the virtual monitor to add a mode to.
    id: driver_ipc::Id,

    /// Width of the new mode.
    width: driver_ipc::Dimen,

    /// Height of the new mode.
    height: driver_ipc::Dimen,

    /// Refresh rate for the new mode. Pass multiple times to support
    /// multiple refresh rates in this mode.
    #[clap(short, long, default_value = "60")]
    refresh_rate: Vec<driver_ipc::RefreshRate>,
}

#[derive(Debug, Parser)]
struct RemoveModeCommand {
    /// ID of the virtual monitor to add a mode to.
    id: driver_ipc::Id,

    /// The index of the mode to remove.
    mode_index: usize,
}

#[derive(Debug, Parser)]
struct EnableCommand {
    id: driver_ipc::Id,
}

#[derive(Debug, Parser)]
struct DisableCommand {
    id: driver_ipc::Id,
}

#[derive(Debug, Parser)]
struct RemoveCommand {
    id: Vec<driver_ipc::Id>,
}

fn main() -> eyre::Result<()> {
    let Args { options, command } = Args::parse();

    match command {
        Command::List => {
            list(&options)?;
        }
        Command::Add(command) => {
            add(&options, command)?;
        }
        Command::AddMode(command) => {
            add_mode(&options, command)?;
        }
        Command::RemoveMode(command) => {
            remove_mode(&options, &command)?;
        }
        Command::Enable(command) => {
            enable(&options, &command)?;
        }
        Command::Disable(command) => {
            disable(&options, &command)?;
        }
        Command::Remove(command) => {
            remove(&options, &command)?;
        }
        Command::RemoveAll => {
            remove_all(&options)?;
        }
    }

    Ok(())
}

fn list(opts: &GlobalOptions) -> eyre::Result<()> {
    let mut client = Client::connect()?;

    let monitors = client.list()?;

    if opts.json {
        let mut stdout = std::io::stdout().lock();
        serde_json::to_writer_pretty(&mut stdout, &monitors)?;
    } else if !monitors.is_empty() {
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
            println!(
                "Monitor {}{name_label}{disabled_label}:",
                monitor.id.green(),
            );

            if monitor.modes.is_empty() {
                println!("{} {}", "-".dimmed(), "No modes".red());
            } else {
                for (index, mode) in monitor.modes.iter().enumerate() {
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
                        "{} Mode {index}: {}x{} @ {}",
                        "-".dimmed(),
                        mode.width.green(),
                        mode.height.green(),
                        refresh_rates
                    );
                }
            }
        }
    } else {
        println!("No virtual monitors found.");
    }

    Ok(())
}

fn add(opts: &GlobalOptions, command: AddCommand) -> eyre::Result<()> {
    if command.more_widths_and_heights.len() % 2 != 0 {
        eyre::bail!("passed a width for an extra resolution without a height");
    }

    let modes = std::iter::once(&[command.width, command.height][..])
        .chain(command.more_widths_and_heights.chunks_exact(2))
        .map(|dim| driver_ipc::Mode {
            width: dim[0],
            height: dim[1],
            refresh_rates: command.refresh_rate.clone(),
        })
        .collect();

    let mut client = Client::connect()?;
    let id = client.new_id(command.id)?;
    let new_monitor = driver_ipc::Monitor {
        id,
        enabled: !command.disabled,
        name: command.name,
        modes,
    };
    client.notify(vec![new_monitor])?;

    if opts.json {
        let mut stdout = std::io::stdout().lock();
        serde_json::to_writer_pretty(&mut stdout, &id)?;
    } else {
        let disabled_footnote = lazy_format!(
            if command.disabled => (" {}", "(disabled)".red())
            else => ""
        );
        println!(
            "Added virtual monitor with ID {}{disabled_footnote}.",
            id.green()
        );
    }

    Ok(())
}

fn add_mode(opts: &GlobalOptions, command: AddModeCommand) -> eyre::Result<()> {
    let mut client = Client::connect()?;
    let mut monitor = client.get(command.id)?;

    let new_mode_index = monitor.modes.len();
    let new_mode = driver_ipc::Mode {
        width: command.width,
        height: command.height,
        refresh_rates: command.refresh_rate,
    };
    monitor.modes.push(new_mode);
    client.notify(vec![monitor])?;

    if opts.json {
        let mut stdout = std::io::stdout().lock();
        serde_json::to_writer_pretty(&mut stdout, &new_mode_index)?;
    } else {
        println!(
            "Added new mode {} to virtual monitor with ID {}.",
            new_mode_index.blue(),
            command.id.green()
        );
    }

    Ok(())
}

fn remove_mode(opts: &GlobalOptions, command: &RemoveModeCommand) -> eyre::Result<()> {
    let mut client = Client::connect()?;
    let mut monitor = client.get(command.id)?;

    if command.mode_index >= monitor.modes.len() {
        eyre::bail!(
            "virtual monitor with ID {} has no mode with index {}",
            command.id,
            command.mode_index
        );
    }
    if monitor.modes.len() == 1 {
        eyre::bail!(
            "cannot remove last mode from virtual monitor with ID {}",
            command.id
        );
    }

    monitor.modes.remove(command.mode_index);
    client.notify(vec![monitor])?;

    if opts.json {
        let mut stdout = std::io::stdout().lock();
        serde_json::to_writer_pretty(&mut stdout, &command.mode_index)?;
    } else {
        println!(
            "Removed mode {} from virtual monitor with ID {}.",
            command.mode_index.blue(),
            command.id.green()
        );
    }

    Ok(())
}

fn enable(opts: &GlobalOptions, command: &EnableCommand) -> eyre::Result<()> {
    let mut client = Client::connect()?;
    let outcome = set_enabled(&mut client, command.id, true)?;

    if opts.json {
        let mut stdout = std::io::stdout().lock();
        serde_json::to_writer_pretty(&mut stdout, &outcome)?;
    } else {
        let footnote = if outcome.toggled {
            ""
        } else {
            " (was already enabled)"
        };
        println!(
            "Enabled virtual monitor with ID {}{footnote}.",
            command.id.green()
        );
    }

    Ok(())
}

fn disable(opts: &GlobalOptions, command: &DisableCommand) -> eyre::Result<()> {
    let mut client = Client::connect()?;
    let outcome = set_enabled(&mut client, command.id, false)?;

    if opts.json {
        let mut stdout = std::io::stdout().lock();
        serde_json::to_writer_pretty(&mut stdout, &outcome)?;
    } else {
        let footnote = if outcome.toggled {
            ""
        } else {
            " (was already disabled)"
        };
        println!(
            "Disabled virtual monitor with ID {}{footnote}.",
            command.id.green()
        );
    }

    Ok(())
}

fn remove(opts: &GlobalOptions, command: &RemoveCommand) -> eyre::Result<()> {
    let mut client = Client::connect()?;
    client.remove(command.id.clone())?;

    if opts.json {
        let mut stdout = std::io::stdout().lock();
        serde_json::to_writer_pretty(&mut stdout, &command.id)?;
    } else if command.id.len() == 1 {
        println!("Removed virtual monitor.");
    } else {
        println!("Removed {} virtual monitors.", command.id.len());
    }

    Ok(())
}

fn remove_all(opts: &GlobalOptions) -> eyre::Result<()> {
    let mut client = Client::connect()?;
    client.remove_all()?;

    if opts.json {
        let mut stdout = std::io::stdout().lock();
        serde_json::to_writer_pretty(&mut stdout, &())?;
    } else {
        println!("Removed all virtual monitors.");
    }

    Ok(())
}

fn set_enabled(
    client: &mut Client,
    id: driver_ipc::Id,
    enabled: bool,
) -> eyre::Result<EnableDisableOutcome> {
    let mut monitor = client.get(id)?;

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
