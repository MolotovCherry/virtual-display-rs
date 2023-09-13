# Virtual Display Driver

This is a Windows driver made in Rust which creates a virtual desktop.

It has many uses, such as:
- A private virtual desktop for VR use
- For remote desktops
- Getting a higher resolution (or higher refresh rate) display when you don't have a physical one on-hand (though note you can only use it in software/VR)
- Other uses? Let me know!

# How to install
- Go to the [releases](https://github.com/MolotovCherry/virtual-display-rs/releases) section for the latest driver.
- Download (you may receive a warning, just press accept)
- Run the `installCert.bat` file to install the certificate. This is required in order to install the driver.
- Open `Device Manager`
- - Click on any item
  - Click on `Action` menu item -> `Add legacy hardware` -> `Next`
  - Click `Install the hardware that I manually select from a list (Advanced)`
  - Click `Next` (`Show All Devices` will be selected)
  - Click `Have Disk`
  - Browse to the location of the folder, press `Ok`, and keep clicking the `Next` buttons

# Future goals
- Work on allowing custom resolution / refresh rate, perhaps with app for easy configuring
- More than 1 monitor
