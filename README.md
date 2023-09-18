# Virtual Display Driver

[![Build Driver](https://github.com/MolotovCherry/virtual-display-rs/actions/workflows/build-driver.yml/badge.svg?event=push)](https://github.com/MolotovCherry/virtual-display-rs/actions/workflows/build-driver.yml) [![GitHub release (with filter)](https://img.shields.io/github/v/release/MolotovCherry/virtual-display-rs)](https://github.com/MolotovCherry/virtual-display-rs/releases)

This is a Windows driver made in Rust which creates a virtual desktop.

It has many uses, such as:
- A private virtual desktop for VR use
- For remote desktops
- Getting a higher resolution (or higher refresh rate) display when you don't have a physical one on-hand (though note you can only use it in software/VR)
- Other uses? Let me know!

Support: Windows 10 x64 +

# How to install
- Go to the [releases](https://github.com/MolotovCherry/virtual-display-rs/releases) section for the latest driver.
- Download (you may receive a warning, just press accept)
- Install certificate (see below section)
- Open `Device Manager`
- - Click on any item
  - Click on `Action` menu item -> `Add legacy hardware` -> `Next`
  - Click `Install the hardware that I manually select from a list (Advanced)`
  - Click `Next` (`Show All Devices` will be selected)
  - Click `Have Disk`
  - Browse to the location of the folder, press `Ok`, and keep clicking the `Next` buttons

### Installing the certificate
The certificate needs installation for Windows to accept the driver
- In your downloaded zip, double click on the file `DriverCertificate.cer`
- A window will popup with a `Install Certificate` button (click it)
- Select `Local Machine`
- Select `Place All Certificates in the following store`, click `Browse` and select `Trusted Root Certification Authorities`
- Cick `Next` and `Finish`

# Disabling the display
Disabling it can be done one of two ways
- Disable the driver (and enable it only when you need to use)
- Go to "Display Settings" and select "Show only on 1" (I don't think this setting gets saved across reboots however)

Note: Disabling any/all monitors can be done dynamically at runtime in the upcoming driver/control panel. 😄

# Updating
- Open `Device Manager`
- Under the `Display` section, find the `Virtual Display` driver and double click
- Click the `Driver` tab and the `Update Driver` button
- Click `Browse my computer for drivers`, browse for the right location, and click `Next`

# How to build
- Download and install [Visual Studio](https://visualstudio.microsoft.com/downloads/) (use the 2022 edition)
- Select and install the `Desktop development with C++` workload as well as Windows SDK
- Install the [WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)
- Install [`cargo-make`](https://github.com/sagiegurari/cargo-make) if you don't have it
- You can build it with `cargo make -p dev build` (debug) or `cargo make -p prod build` (release)
- ... Or, fork my project and build it with github actions

### Debugging
To see panic messages and other information, download [DebugViewPP](https://github.com/CobaltFusion/DebugViewPP), run it, click on `Log`->`Capture Global Win32` (note, this requires DebugViewPP be run with admin permissions)

# Future goals
- [X] Work on allowing custom resolution / refresh rate
- [X] More than 1 monitor
- [ ] App for easy configuring monitors

# Extra notes
If you enabled while everything has been booted for awhile, some applications may not be able to see the virtual monitor. You can fix this by rebooting while having it enabled (or enable it quickly after you boot up). This seems to be an issue with Windows itself, but I do not know why. It has affected every single virtual monitor driver I have ever tried. If anyone does know, please feel free to share information 😃

# Contributions
All contributions are welcome! If you have any questions, feel free to post in the project [Discussion](https://github.com/MolotovCherry/virtual-display-rs/discussions) section
