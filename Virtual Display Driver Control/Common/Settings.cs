using System;
using System.ComponentModel;
using System.IO;
using Config.Net;
using Microsoft.UI.Xaml;

namespace Virtual_Display_Driver_Control.Common;

public interface IAppSettings : INotifyPropertyChanged {
    [Option(DefaultValue = ElementTheme.Default)]
    ElementTheme Theme { get; set; }
    [Option(DefaultValue = Material.Mica)]
    Material Material { get; set; }
}

public enum Material {
    Mica,
    Acrylic,
    None
}

public static class SettingsProvider {
    public static IAppSettings Initialize() {
        var jsonPath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "VirtualDisplayDriver", "settings.json");

        return new ConfigurationBuilder<IAppSettings>()
           .UseJsonFile(jsonPath)
           .Build();
    }
}
