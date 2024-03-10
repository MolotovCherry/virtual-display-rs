use std::collections::{BTreeSet, HashMap};

use eyre::Context as _;
use joinery::JoinableIterator as _;

const DEFAULT_REFRESH_RATE: driver_ipc::RefreshRate = 60;

#[derive(Debug, Clone)]
pub struct Mode {
    pub width: driver_ipc::Dimen,
    pub height: driver_ipc::Dimen,
    pub refresh_rates: BTreeSet<driver_ipc::RefreshRate>,
}

impl From<driver_ipc::Mode> for Mode {
    fn from(value: driver_ipc::Mode) -> Self {
        Self {
            width: value.width,
            height: value.height,
            refresh_rates: value.refresh_rates.into_iter().collect(),
        }
    }
}

impl From<Mode> for driver_ipc::Mode {
    fn from(mut value: Mode) -> Self {
        if value.refresh_rates.is_empty() {
            value.refresh_rates.insert(DEFAULT_REFRESH_RATE);
        }

        Self {
            width: value.width,
            height: value.height,
            refresh_rates: value.refresh_rates.into_iter().collect(),
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.refresh_rates.is_empty() {
            write!(f, "{}x{}", self.width, self.height)?;
        } else {
            write!(
                f,
                "{}x{}@{}",
                self.width,
                self.height,
                self.refresh_rates.iter().join_with("/"),
            )?;
        }

        Ok(())
    }
}

impl std::str::FromStr for Mode {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (resolution, refresh_rate_list) = match s.split_once('@') {
            Some((resolution, refresh_rate_list)) => (resolution, Some(refresh_rate_list)),
            None => (s, None),
        };

        let (width, height) = resolution.split_once('x').ok_or_else(|| {
            eyre::eyre!("invalid resolution in {s:?}, expected a string like \"1920x1080\"",)
        })?;
        let width = width
            .parse()
            .with_context(|| format!("invalid width in {s:?}, expected a number"))?;
        let height = height
            .parse()
            .with_context(|| format!("invalid height in {s:?}, expected a number"))?;

        let refresh_rates = match refresh_rate_list {
            Some(refresh_rate_list) => refresh_rate_list
                .split('/')
                .map(|s| {
                    s.parse().with_context(|| {
                        format!("failed to parse refresh rate in {s:?}, expected a number")
                    })
                })
                .collect::<eyre::Result<_>>()?,
            None => BTreeSet::new(),
        };

        Ok(Self {
            width,
            height,
            refresh_rates,
        })
    }
}

pub fn merge(modes: impl IntoIterator<Item = Mode>) -> Vec<Mode> {
    let mut resolutions =
        HashMap::<(driver_ipc::Dimen, driver_ipc::Dimen), BTreeSet<driver_ipc::RefreshRate>>::new();

    for mode in modes {
        let refresh_rates = resolutions.entry((mode.width, mode.height)).or_default();
        refresh_rates.extend(&mode.refresh_rates);
    }

    resolutions
        .into_iter()
        .map(|((width, height), refresh_rates)| Mode {
            width,
            height,
            refresh_rates,
        })
        .collect()
}

pub fn remove(
    modes: impl IntoIterator<Item = Mode>,
    remove_mode: &Mode,
) -> eyre::Result<Vec<Mode>> {
    let mut resolutions =
        HashMap::<(driver_ipc::Dimen, driver_ipc::Dimen), BTreeSet<driver_ipc::RefreshRate>>::new();

    for mode in modes {
        let refresh_rates = resolutions.entry((mode.width, mode.height)).or_default();
        refresh_rates.extend(&mode.refresh_rates);
    }

    if remove_mode.refresh_rates.is_empty() {
        let removed = resolutions.remove(&(remove_mode.width, remove_mode.height));
        if removed.is_none() {
            eyre::bail!("mode {remove_mode} not found");
        }
    } else {
        let Some(refresh_rates) = resolutions.get_mut(&(remove_mode.width, remove_mode.height))
        else {
            eyre::bail!("mode {remove_mode} not found");
        };
        for refresh_rate in &remove_mode.refresh_rates {
            let removed = refresh_rates.remove(refresh_rate);
            if !removed {
                eyre::bail!("mode {remove_mode} not found");
            }
        }
    }

    let modes = resolutions
        .into_iter()
        .map(|((width, height), refresh_rates)| Mode {
            width,
            height,
            refresh_rates,
        })
        .collect();
    Ok(modes)
}
