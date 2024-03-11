use std::collections::{BTreeSet, HashMap};

use eyre::Context as _;
use joinery::JoinableIterator as _;

const DEFAULT_REFRESH_RATE: driver_ipc::RefreshRate = 60;

/// Represent a mode as specified by the user as a CLI argument. Can be parsed
/// from a string such as `1920x1080` or `3840x2160@60/120`, or converted
/// from/to the type [`driver_ipc::Mode`].
///
/// This type is very similar to [`driver_ipc::Mode`], but with a few key
/// differences:
///
/// - The list of refresh rates can be empty by design. When converting to
///   [`driver_ipc::Mode`], the refresh rate is set to [`DEFAULT_REFRESH_RATE`]
///   if one isn't specified. See [`remove`] for a use-case where the empty list
///   of refresh rates is used.
/// - It implements [`std::str::FromStr`], with a convenient user-facing error
///   message using `eyre`.
/// - It implements [`std::fmt::Display`] with a nice output format.
/// - The list of resolutions is represented with a set, meaning that duplicate
///   resolutions will be ignored.
#[derive(Debug, Clone)]
pub struct Mode {
    pub width: driver_ipc::Dimen,
    pub height: driver_ipc::Dimen,
    pub refresh_rates: BTreeSet<driver_ipc::RefreshRate>,
}

impl Mode {
    /// Add the default refresh rate if the list of refresh rates is empty.
    fn ensure_refresh_rate(&mut self) {
        if self.refresh_rates.is_empty() {
            self.refresh_rates.insert(DEFAULT_REFRESH_RATE);
        }
    }
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
        value.ensure_refresh_rate();

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

/// Merge together a list of modes. Multiple modes with the same resolution
/// will be merged into one, and the sets of refresh rates will be combined.
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

/// Remove a mode from a list of modes. If `remove_mode` includes a refresh
/// rate, then only that refresh rate will be removed from the mode; otherwise,
/// the entire mode will be removed. Returns an error if no mode matches
/// `remove_mode`.
///
/// This function will also implicitly merge together the list of provided
/// modes, see [`merge`] for more details.
pub fn remove(
    modes: impl IntoIterator<Item = Mode>,
    remove_mode: &Mode,
) -> eyre::Result<Vec<Mode>> {
    let mut resolutions =
        HashMap::<(driver_ipc::Dimen, driver_ipc::Dimen), BTreeSet<driver_ipc::RefreshRate>>::new();

    for mut mode in modes {
        mode.ensure_refresh_rate();

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
        .filter(|(_, refresh_rates)| !refresh_rates.is_empty())
        .map(|((width, height), refresh_rates)| Mode {
            width,
            height,
            refresh_rates,
        })
        .collect();
    Ok(modes)
}
