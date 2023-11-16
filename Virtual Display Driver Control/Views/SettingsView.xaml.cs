using Microsoft.UI.Composition.SystemBackdrops;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using System;
using System.ComponentModel;
using Virtual_Display_Driver_Control.Common;
using Virtual_Display_Driver_Control.Helpers;
using Windows.System;

namespace Virtual_Display_Driver_Control.Views;

public sealed partial class SettingsView : Page, INotifyPropertyChanged {
    public string AppInfo {
        get {
            var appTitle = Application.Current.Resources["AppTitleName"] as string;

            var version = $"{GitVersionInformation.Major}.{GitVersionInformation.Minor}.{GitVersionInformation.Patch}";

            #if DEBUG
                string dirty = "";
                uint commitsSince;
                if (uint.TryParse(GitVersionInformation.UncommittedChanges, out commitsSince)) {
                    if (commitsSince > 0) {
                        dirty = "-dirty";
                    }
                }

                var versionString = $"{version}.dev-{GitVersionInformation.ShortSha}{dirty}";
            #else
                var versionString = $"{version}-{GitVersionInformation.ShortSha}";
            #endif

            return $"{appTitle} - v{versionString}";
        }
    }

    public SettingsView() {
        InitializeComponent();

        themeMode_load();
        themeMaterial_load();
    }

    private void themeMode_SelectionChanged(object sender, RoutedEventArgs e) {
        var themeString = ((ComboBoxItem)themeMode.SelectedItem)?.Tag?.ToString();

        if (themeString != null) {
            ElementTheme theme;

            switch (themeString) {
                case "Light":
                    theme = ElementTheme.Light;
                    break;
                case "Dark":
                    theme = ElementTheme.Dark;
                    break;
                default:
                    theme = ElementTheme.Default;
                    break;
            }

            ThemeHelper.SetTheme(theme);
        }
    }

    private void themeMode_load() {
        // do not fire callback when we change the index here
        themeMode.SelectionChanged -= themeMode_SelectionChanged;

        var theme = App.Settings.Theme;
        if (theme == ElementTheme.Light) {
            themeMode.SelectedIndex = 0;
        } else if (theme == ElementTheme.Dark) {
            themeMode.SelectedIndex = 1;
        } else {
            themeMode.SelectedIndex = 2;
        }

        themeMode.SelectionChanged += themeMode_SelectionChanged;
    }

    private void themeMaterial_load() {
        if (!MicaController.IsSupported()) {
            ((ComboBoxItem)themeMaterial.Items[0]).IsEnabled = false;
        }

        if (!DesktopAcrylicController.IsSupported()) {
            ((ComboBoxItem)themeMaterial.Items[1]).IsEnabled = false;
        }

        // do not fire callback when we change the index here
        themeMaterial.SelectionChanged -= themeMaterial_SelectionChanged;

        var material = App.Settings.Material;
        if (material == Material.Mica && MicaController.IsSupported()) {
            themeMaterial.SelectedIndex = 0;
        } else if (material == Material.Acrylic && DesktopAcrylicController.IsSupported()) {
            themeMaterial.SelectedIndex = 1;
        } else {
            themeMaterial.SelectedIndex = 2;
        }

        themeMaterial.SelectionChanged += themeMaterial_SelectionChanged;
    }

    private void themeMaterial_SelectionChanged(object sender, RoutedEventArgs e) {
        var selectedMaterial = ((ComboBoxItem)themeMaterial.SelectedItem)?.Tag?.ToString();

        if (selectedMaterial != null) {
            Material material;
            switch (selectedMaterial) {
                case "Mica":
                    material = Material.Mica;
                    break;
                case "Acrylic":
                    material = Material.Acrylic;
                    break;
                default:
                    material = Material.None;
                    break;
            }

            MaterialHelper.SetMaterial(material);
        }
    }

    private async void donate_Click(object sender, RoutedEventArgs e) {
        await Launcher.LaunchUriAsync(new Uri("https://github.com/sponsors/MolotovCherry"));
    }

    private async void homepage_Click(object sender, RoutedEventArgs e) {
        await Launcher.LaunchUriAsync(new Uri("https://github.com/MolotovCherry/virtual-display-rs/"));
    }

    private async void bugFeatureCard_Click(object sender, RoutedEventArgs e) {
        await Launcher.LaunchUriAsync(new Uri("https://github.com/MolotovCherry/virtual-display-rs/issues/new/choose"));
    }

    //
    // Update check code
    //

    string _updateCheck = "Click to Check for Updates";
    string UpdateCheck {
        get { return _updateCheck;  }
        set {
            if (_updateCheck != value) {
                _updateCheck = value;
                RaisePropertyChanged("UpdateCheck");
            }
        }
    }

    bool needsUpdate = false;
    bool checkedUpdate = false;
    string releaseUrl = "";
    private static readonly Octokit.GitHubClient client = new Octokit.GitHubClient(new Octokit.ProductHeaderValue("VirtualDisplayDriverControl"));
    private async void updates_Click(object sender, RoutedEventArgs e) {
        // update check was already done and succeeded, so launch uri
        if (needsUpdate) {
            await Launcher.LaunchUriAsync(new Uri(releaseUrl));
            return;
        }

        if (checkedUpdate) {
            return;
        }

        // otherwise, do the update check instead

        try {
            var release = await client.Repository.Release.GetLatest("MolotovCherry", "virtual-display-rs");

            var tag = release.TagName[1..6];
            releaseUrl = release.HtmlUrl;

            Version data = Version.Parse(tag);

            var major = uint.Parse(GitVersionInformation.Major);
            var minor = uint.Parse(GitVersionInformation.Minor);
            var patch = uint.Parse(GitVersionInformation.Patch);

            if (data.Major > major || data.Minor > minor || data.Build > patch) {
                UpdateCheck = $"Update is available: v{tag}";
                needsUpdate = true;
                checkedUpdate = true;
            } else {
                UpdateCheck = $"No update is available";
                needsUpdate = false;
                checkedUpdate = true;
            }
        } catch {
            UpdateCheck = "Failed to retrieve latest version information";
            needsUpdate = false;
        }
    }

    public event PropertyChangedEventHandler? PropertyChanged;
    private void RaisePropertyChanged(string name) {
        if (PropertyChanged != null) {
            PropertyChanged(this, new PropertyChangedEventArgs(name));
        }
    }
}
