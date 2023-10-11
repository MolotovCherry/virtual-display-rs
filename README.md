# Virtual Display Driver

[![Build](https://github.com/MolotovCherry/virtual-display-rs/actions/workflows/build.yml/badge.svg?branch=master&event=push)](https://github.com/MolotovCherry/virtual-display-rs/actions/workflows/build.yml) [![GitHub release (with filter)](https://img.shields.io/github/v/release/MolotovCherry/virtual-display-rs)](https://github.com/MolotovCherry/virtual-display-rs/releases)

This is a Windows driver made in Rust which creates a virtual desktop.

It has many uses, such as:
- A private virtual desktop for VR use
- For remote desktops
- For screenshare presentations, to give you more workspace with a second monitor
- Getting a higher resolution (or higher refresh rate) display when you don't have a physical one on-hand (though note you can only use it in software/VR)
- Other uses? Let me know!

Supports: Windows 10 2004+ (x64 only)

_For any bug reports, please see the [debugging or reporting crashes](https://github.com/MolotovCherry/virtual-display-rs#debugging-or-reporting-crashes) section to get the panic message for the bug report_

## Features
- Multiple monitors (up to 10)
- Multiple resolutions per monitor
- Multiple refresh rates per resolution
- App to configure them all, disable all/individual monitors

https://github.com/MolotovCherry/virtual-display-rs/assets/13651622/4a244e40-65d2-4c99-91f7-4e8b352e3ebe

## How to install
1. Go to the [releases](https://github.com/MolotovCherry/virtual-display-rs/releases) section for the latest driver.
2. Download (you may receive a warning, just press accept)
3. [Install certificate](https://github.com/MolotovCherry/virtual-display-rs#installing-the-certificate)
4. Run the msi installer
5. The driver will be installed, started, and you can find a shortcut to the control app in the start menu, named "Virtual Display Driver Control"

## How to install portable version
1. Go to the [releases](https://github.com/MolotovCherry/virtual-display-rs/releases) section for the latest driver.
2. Download (you may receive a warning, just press accept)
3. [Install certificate](https://github.com/MolotovCherry/virtual-display-rs#installing-the-certificate)
4. Install `install.reg`
5. Open device manager
   * click on any item in the list
   * go to `Actions -> Add legacy hardware`
   * next on `Install hardware that I manually select from a list`
   * next on `Show all devices
   * click on `Have Disk...` and select the folder with the driver files file in it
   * finish the setup
7. The driver will be installed and started. The control panel exe you see in the folder will work from anywhere you put it.

## Installing the certificate
The certificate needs installation for Windows to accept the driver
1. In your downloaded zip, there is a file `DriverCertificate.cer` and `install-cert.bat`
2. Open a cmd window as admin and run `install-cert.bat`

_If an install error is occurring and you can't install the driver, check to make sure the certificate got installed properly! Try manually running the commands in the [`install-cert.bat` file](https://github.com/MolotovCherry/virtual-display-rs/blob/master/installer/install-cert.bat)_

## Updating
1. Download the new release
2. Install the msi package

## Using the app
Please see the [wiki](https://github.com/MolotovCherry/virtual-display-rs/wiki/Virtual-Display-Driver-Control) for instructions on using the app.

## How to build
1. Download and install [Visual Studio](https://visualstudio.microsoft.com/downloads/) (use the 2022 edition)
   - Select and install the `Desktop development with C++` workload as well as Windows SDK
2. Install the [WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)
3. Install [`cargo-make`](https://github.com/sagiegurari/cargo-make) if you don't have it
4. Install [`cargo-target-dir`](https://github.com/MolotovCherry/cargo-target-dir)

You can build it with `cargo make build` (debug) or `cargo make -p prod build` (release), and check the `target/output` directory for all the files

To build the installer, do a `cargo make build-installer` (dev) or `cargo make -p prod build-installer` (release). In order to build the installer, you need [wix toolset](https://github.com/wixtoolset/wix3/releases) installed and on `Path`... Or, fork my project and build it with github actions

### Debugging or Reporting Crashes
If you want to debug a problem or need to report a crash, follow the below instructions:

All messages from the driver are logged in the Windows Event Viewer.
1. Open the Event Viewer
3. Go to `Windows Logs` -> `Application`
4. You will see logs for the driver under the source name `VirtualDisplayDriver`

If you want to make them easier to see
1. right click on `Custom Views`
   - click `Create Custom View...`
     - select `By source`
     - find and select `VirtualDisplayDriver` in the list, then press `Ok`
     - type in `VirtualDisplayDriver` for the name, and press `Ok`
2. You should now see any log messages under the `Custom Views` section for `VirtualDisplayDriver`

## Contributions
All contributions are welcome! If you have any questions, feel free to post in the project [Discussion](https://github.com/MolotovCherry/virtual-display-rs/discussions) section
