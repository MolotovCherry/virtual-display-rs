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

_For any bug reports, please see the [debugging or reporting crashes](#debugging-or-reporting-crashes) section to get the panic message for the bug report_

## Features
- Multiple monitors (up to 10)
- Multiple resolutions per monitor
- Multiple refresh rates per resolution
- App to configure them all, disable all/individual monitors

https://github.com/MolotovCherry/virtual-display-rs/assets/13651622/4a244e40-65d2-4c99-91f7-4e8b352e3ebe

## How to install
1. Go to the [releases](https://github.com/MolotovCherry/virtual-display-rs/releases) section for the latest driver.
2. Download (you may receive a warning, just press accept)
3. [Install certificate](#installing-the-certificate)
4. Run the msi installer
5. The driver will be installed, started, and you can find a shortcut to the control app in the start menu, named "Virtual Display Driver Control"

## How to install portable version
1. Go to the [releases](https://github.com/MolotovCherry/virtual-display-rs/releases) section for the latest driver.
2. Download (you may receive a warning, just press accept)
3. [Install certificate](#installing-the-certificate)
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
2. Open a cmd prompt as admin and run `install-cert.bat`
3. Verify the certificate installed properly. *

\* If the certificate didn't install properly, then the driver won't install. If driver installation fails, this is most likely the reason why; and you should check that the certificate is actually installed. Try manually running the commands in the [`install-cert.bat` file](https://github.com/MolotovCherry/virtual-display-rs/blob/master/installer/install-cert.bat) (below) in an admin cmd prompt to make sure the certificate is installed correctly (for both root and TrustedPublisher stores). The commands will tell you if they successfully added it or not.
```
certutil -addstore -f root "DriverCertificate.cer"
certutil -addstore -f TrustedPublisher "DriverCertificate.cer"
```
You can also search for `Manage Computer Certificates`, look in `Trusted Publishers` and `Trusted Root Certification`, you will see the certificate named `DriverCertficate`.

![image](https://github.com/MolotovCherry/virtual-display-rs/assets/13651622/f63d24dd-a61d-42f4-b491-5123fd480d38)

You can manually import it by right clicking on the menu entry -> `All Tasks` -> `Import`, and following the instructions in the import wizard

![image](https://github.com/MolotovCherry/virtual-display-rs/assets/13651622/3a2f7704-12ae-4d66-963c-68c44c66bde4)

Why is it so difficult? The reason I didn't add auto certificate installation is because I believe certificates are a personal thing, and should not be added automatically without the users knowledge.

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

To build the installer, do a `cargo make build-installer` (dev) or `cargo make -p prod build-installer` (release). In order to build the installer, you need [wix toolset](https://github.com/wixtoolset/wix3/releases) installed and on `Path`

... Or, fork my project and build it with github actions. You will require 2 repository secrets:
* `PRIVATE_KEY` - a windows code signing pfx certificate encoded in base64 (use `certutil -encode`)
* `PRIVATE_KEY_PASSWORD` - self explanatory

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

If you're using a driver compiled in debug mode, you can see panic messages and other information in a live logger: download [DebugViewPP](https://github.com/CobaltFusion/DebugViewPP), run it, click on `Log`->`Capture Global Win32` (note, this requires DebugViewPP be run with admin permissions). As long as the program is open and capturing, the messages will appear live as they are logged. This is a bit easier to use than the event log when you are trying to debug something.

## Contributions
All contributions are welcome!

For first time contributors, please read our [contributing](CONTRIBUTING.md) guide.

## Forking or using in other projects
You are welcome to use this project in your own projects.

If you do so, please contribute back to the main project with your code changes, and even by [sponsoring](https://github.com/sponsors/MolotovCherry). Every little bit helps us make an even better project. We appreciate it, thank you!

## Where to talk or get help
If you have any questions, need support, need to collaborate on development, or any other use-case, you may join our [discord server](https://discord.gg/pDDt78wYQy) (see related `#virtual-display-driver` channels). This is the quickest and easiest way to communicate.

You may also post in the project [discussion](https://github.com/MolotovCherry/virtual-display-rs/discussions) section. Though note that using the discord channel will get your messages seen and responded to quicker.

## Supporting the project
If this project has helped you, or you want to say thanks and help continued development, [sponsorships](https://github.com/sponsors/MolotovCherry) are very welcome. ❤️
