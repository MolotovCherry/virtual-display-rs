# Virtual Display Driver

[![Build](https://github.com/MolotovCherry/virtual-display-rs/actions/workflows/build.yml/badge.svg?branch=master&event=push)](https://github.com/MolotovCherry/virtual-display-rs/actions/workflows/build.yml) [![GitHub release (with filter)](https://img.shields.io/github/v/release/MolotovCherry/virtual-display-rs)](https://github.com/MolotovCherry/virtual-display-rs/releases)

This is a Windows driver made in Rust which creates a virtual desktop.

It has many uses, such as:
- A private virtual desktop for VR use
- For remote desktops
- Getting a higher resolution (or higher refresh rate) display when you don't have a physical one on-hand (though note you can only use it in software/VR)
- Other uses? Let me know!

Supports: Windows 10 version 2004+ (x64 only)

_For any bug reports, please see the [debugging or reporting crashes](https://github.com/MolotovCherry/virtual-display-rs#debugging-or-reporting-crashes) section to get the panic message for the bug report_

## Features
- Multiple monitors (up to 10)
- Multiple resolutions per monitor
- Multiple refresh rates per resolution
- App to configure them all, disable all/individual monitors

https://github.com/MolotovCherry/virtual-display-rs/assets/13651622/4a244e40-65d2-4c99-91f7-4e8b352e3ebe

# How to install
1. Go to the [releases](https://github.com/MolotovCherry/virtual-display-rs/releases) section for the latest driver.
1. Download (you may receive a warning, just press accept)
1. Install certificate (see below section)
1. Run the msi installer

_Note about driver install:  
If you're getting an error about an unverified driver during install, it's either because the provided certificate isn't installed, or was installed incorrectly. You can also just install it anyways and it should still work._

### Installing the certificate
The certificate needs installation for Windows to accept the driver
1. In your downloaded zip, there is a file `DriverCertificate.cer` and `install-cert.bat`
1. Open a cmd window as admin and run `install-cert.bat`

# Updating
1. Download the new release
1. Install the msi package

# Using the app
Please see the [wiki](https://github.com/MolotovCherry/virtual-display-rs/wiki/Virtual-Display-Driver-Control) for instructions on using the app.

# How to build
1. Download and install [Visual Studio](https://visualstudio.microsoft.com/downloads/) (use the 2022 edition)
   - Select and install the `Desktop development with C++` workload as well as Windows SDK
1. Install the [WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)
1. Install [`cargo-make`](https://github.com/sagiegurari/cargo-make) if you don't have it
1. Install [`cargo-target-dir`](https://github.com/MolotovCherry/cargo-target-dir)

You can build it with `cargo make build` (debug) or `cargo make -p prod build` (release), and check the `target/output` directory for all the files

To build the installer, do a `cargo make build-installer` (dev) or `cargo make -p prod build-installer` (release). In order to build the installer, you need [wix toolset](https://github.com/wixtoolset/wix3/releases) installed and on `Path`... Or, fork my project and build it with github actions

### Debugging or Reporting Crashes
If you want to debug a problem or need to report a crash, follow the below instructions:

All messages from the driver are logged in the Windows Event Viewer.
1. Open the Event Viewer
1. Go to `Windows Logs` -> `Application`
1. You will see logs for the driver under the source name `VirtualDisplayDriver`

If you want to make them easier to see
1. right click on `Custom Views`
   - click `Create Custom View...`
     - select `By source`
     - find and select `VirtualDisplayDriver` in the list, then press `Ok`
     - type in `VirtualDisplayDriver` for the name, and press `Ok`
1. You should now see any log messages under the `Custom Views` section for `VirtualDisplayDriver`

# Contributions
All contributions are welcome! If you have any questions, feel free to post in the project [Discussion](https://github.com/MolotovCherry/virtual-display-rs/discussions) section
