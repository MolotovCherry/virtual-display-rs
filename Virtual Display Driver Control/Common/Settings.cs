using System;
using System.ComponentModel;
using System.IO;
using Config.Net;

namespace Virtual_Display_Driver_Control.Common;

public interface IAppSettings : INotifyPropertyChanged {
    [Option(DefaultValue = "Default")]
    string Theme { get; set;  }
    [Option(DefaultValue = "Mica")]
    string Material { get; set; }
}

public static class SettingsProvider {
    public static IAppSettings Initialize() {
        var jsonPath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "VirtualDisplayDriver", "appsettings.json");

        return new ConfigurationBuilder<IAppSettings>()
           .UseJsonFile(jsonPath)
           .Build();
    }
}
