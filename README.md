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
- Install certificate (see below section)
- Open `Device Manager`
- - Click on any item
  - Click on `Action` menu item -> `Add legacy hardware` -> `Next`
  - Click `Install the hardware that I manually select from a list (Advanced)`
  - Click `Next` (`Show All Devices` will be selected)
  - Click `Have Disk`
  - Browse to the location of the folder, press `Ok`, and keep clicking the `Next` buttons

### Installing the certificate
The certificate needs installation for Windows to accept it.
- In your downloaded zip, double click on the file `DriverCertificate.cer`
- A window will popup with a `Install Certificate` button (click it)
- Select `Local Machine`
- Select `Place All Certificates in the following store`, click `Browse` and select `Trusted Root Certification Authorities`

# Updating
- Open `Device Manager`
- Under the `Display` section, find the `Virtual Display` driver and double click
- Click the `Driver` tab and the `Update Driver` button
- Click `Browse my computer for drivers`, browse for the right location, and click `Next`

# Future goals
- Work on allowing custom resolution / refresh rate, perhaps with app for easy configuring
- More than 1 monitor
