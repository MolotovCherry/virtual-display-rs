using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Windows.Devices.Bluetooth;

namespace Virtual_Display_Driver_Control.Common;

public class Version {
    public uint Major;
    public uint Minor;
    public uint Patch;

    public Version(uint major, uint minor, uint patch) {
        Major = major;
        Minor = minor;
        Patch = patch;
    }
}
