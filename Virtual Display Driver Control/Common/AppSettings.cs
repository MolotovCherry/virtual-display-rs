using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using CSharpFunctionalExtensions;
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
    public Version Version { get; set; } = new Version();
    public string ReleaseUrl { get; set; } = "";
    public List<Asset> Assets { get; set; } = new List<Asset>();
}

public sealed class Version : IComparable<Version>, IEquatable<Version> {
    public int Major { get; set; }
    public int Minor { get; set; }
    public int Patch { get; set; }

    public static bool operator <(Version version1, Version version2) {
        int result = version1.CompareTo(version2);
        return result < 0;
    }

    public static bool operator >(Version version1, Version version2) {
        int result = version1.CompareTo(version2);
        return result > 0;
    }

    public int CompareTo(Version? obj) {
        if (obj != null) {
            var version = new System.Version(obj.Major, obj.Minor, obj.Patch);
            int result = version.CompareTo(new System.Version(this.Major, this.Minor, this.Patch));

            switch (result) {
                case -1: return 1;
                case 0: return 0;
                case 1: return -1;
            }
        }

        throw new ArgumentNullException(nameof(obj));
    }

    public static Maybe<Version> Parse(string version) {
        try {
            System.Version parsedVersion;
            if (System.Version.TryParse(version, out parsedVersion!)) {
                return new Version {
                    Major = parsedVersion.Major,
                    Minor = parsedVersion.Minor,
                    Patch = parsedVersion.Build
                };
            }

            return Maybe<Version>.None;
        } catch {
            return Maybe<Version>.None;
        }
    }

    public bool Equals(Version? other) {
        if (other is Version version) {
            return version.CompareTo(this) == 0;
        }

        return false;
    }
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
