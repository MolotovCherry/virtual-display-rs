using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using Microsoft.Extensions.Configuration;
using Microsoft.UI.Xaml;

namespace Virtual_Display_Driver_Control.Common;

public sealed class AppSettings {
    public ElementTheme Theme { get; set; } = ElementTheme.Default;
    public Material Material { get; set; } = Material.Mica;
    public UpdateVersion UpdateVersion { get; set; } = new UpdateVersion();

    public async void Save() {
        try {
            string json = JsonSerializer.Serialize(this);
            await File.WriteAllTextAsync(SettingsProvider.SettingsPath, json);
        } catch { }
    }
}

public sealed class UpdateVersion {
    public bool IsDirty { get; set; } = false;
    public Version Version { get; set; } = new Version();
    public string ReleaseUrl { get; set; } = "";
    public List<Asset> Assets { get; set; } = new List<Asset>();
}

public sealed class Version {
    public int Major { get; set; }
    public int Minor { get; set; }
    public int Patch { get; set; }
}

public sealed class Asset {
    public string Name { get; set; } = "";
    public string Url { get; set; } = "";
}

public enum Material {
    Mica,
    Acrylic,
    None
}

public static class SettingsProvider {
    public static string AppDir = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "VirtualDisplayDriver");
    public static string Settings = "appsettings.json";
    public static string SettingsPath = Path.Combine(AppDir, Settings);

    public static AppSettings Initialize() {
        if (!Directory.Exists(AppDir)) {
            Directory.CreateDirectory(AppDir);
        }

        IConfiguration configuration;
        try {
            configuration = new ConfigurationBuilder()
                .SetBasePath(AppDir)
                .AddJsonFile(Settings)
                .Build();
        } catch {
            return new AppSettings();
        }

        var appSettings = new AppSettings();
        configuration.Bind(appSettings);

        return appSettings;
    }
}
