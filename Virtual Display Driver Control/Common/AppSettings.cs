using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using CSharpFunctionalExtensions;
using Humanizer;
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
    public DateTime? LastUpdate { get; set; }

    public string LastUpdateHuman() {
        return LastUpdate.Humanize();
    }
}

public sealed class Version : IComparable<Version>, IEquatable<Version> {
    public int Major { get; set; }
    public int Minor { get; set; }
    public int Patch { get; set; }

    public static bool operator <(Version version1, Version version2) {
        return version1.CompareTo(version2) < 0;
    }

    public static bool operator >(Version version1, Version version2) {
        return version1.CompareTo(version2) > 0;
    }

    public int CompareTo(Version? version) {
        if (version == null)
            return 1;

        if (ReferenceEquals(version, this))
            return 0;

        var MajorCmp = Major.CompareTo(version.Major);
        if (MajorCmp != 0)
            return MajorCmp;

        var MinorCmp = Minor.CompareTo(version.Minor);
        if (MinorCmp != 0)
            return MinorCmp;

        var PatchCmp = Patch.CompareTo(version.Patch);
        if (PatchCmp != 0)
            return PatchCmp;

        return 0;
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

    public bool Equals(Version? version) {
        return CompareTo(version) == 0;
    }
}

public sealed class Asset {
    public string Name { get; set; } = "";
    public string Url { get; set; } = "";
}

public enum Material {
    Mica,
    MicaAlt,
    Acrylic,
    Blurred,
    Transparent,
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
