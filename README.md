# Virtual Display Driver

[![Build](https://github.com/MolotovCherry/virtual-display-rs/actions/workflows/build.yml/badge.svg?branch=master&event=push)](https://github.com/MolotovCherry/virtual-display-rs/actions/workflows/build.yml) [![GitHub release (with filter)](https://img.shields.io/github/v/release/MolotovCherry/virtual-display-rs)](https://github.com/MolotovCherry/virtual-display-rs/releases)

This is a Windows driver made in Rust which creates a virtual desktop.

It has many uses, such as:
- A private virtual desktop for VR use
- For remote desktops
- Getting a higher resolution (or higher refresh rate) display when you don't have a physical one on-hand (though note you can only use it in software/VR)
- Other uses? Let me know!

Support: Windows 10 x64 +

## Features
- Multiple monitors (up to 10)
- Multiple resolutions per monitor
- Multiple refresh rates per resolution
- App to configure them all, disable all/individual monitors

https://github.com/MolotovCherry/virtual-display-rs/assets/13651622/4a244e40-65d2-4c99-91f7-4e8b352e3ebe

# How to install
- Go to the [releases](https://github.com/MolotovCherry/virtual-display-rs/releases) section for the latest driver.
- Download (you may receive a warning, just press accept)
- Install certificate (see below section)
- Run the msi installer

_Note about driver install:  
If you're getting an error about an unverified driver during install, it's either because the provided certificate isn't installed, or was installed incorrectly. You can also just install it anyways and it should still work._

### Installing the certificate
The certificate needs installation for Windows to accept the driver
- In your downloaded zip, there is a file `DriverCertificate.cer` and `install-cert.bat`
- Open a cmd window as admin and run `install-cert.bat`

# Updating
- Download the new release
- Install the msi package

# Using the app
Please see the [wiki](https://github.com/MolotovCherry/virtual-display-rs/wiki/Virtual-Display-Driver-Control) for instructions on using the app.

# How to build
- Download and install [Visual Studio](https://visualstudio.microsoft.com/downloads/) (use the 2022 edition)
- Select and install the `Desktop development with C++` workload as well as Windows SDK
- Install the [WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)
- Install [`cargo-make`](https://github.com/sagiegurari/cargo-make) if you don't have it
- Install [`cargo-target-dir`](https://github.com/MolotovCherry/cargo-target-dir)
- You can build it with `cargo make build` (debug) or `cargo make -p prod build` (release), and check the `target/output` directory for all the files
- ... Or, fork my project and build it with github actions

### Debugging or Reporting Crashes
If you want to debug a problem or need to report a crash, follow the below instructions:

All messages from the driver are logged in the Windows Event Viewer. Open the Event Viewer, go to `Windows Logs` -> `Application`, and you will see logs for the driver under the source name "VirtualDisplayDriver".

If you want to make them easier to see, right click on `Custom Views`, click `Create Custom View...`, select `By source`, find and select `VirtualDisplayDriver` in the list, then press `Ok`, type in `VirtualDisplayDriver` for the name, and press `Ok`. You should now see any log messages under the `Custom Views` section for `VirtualDisplayDriver`

# Contributions
All contributions are welcome! If you have any questions, feel free to post in the project [Discussion](https://github.com/MolotovCherry/virtual-display-rs/discussions) section
