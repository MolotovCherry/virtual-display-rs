using Microsoft.UI.Composition.SystemBackdrops;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using System;
using System.Collections.Generic;
using Virtual_Display_Driver_Control.Common;
using Virtual_Display_Driver_Control.Helpers;
using Windows.System;

namespace Virtual_Display_Driver_Control.Views;

public sealed partial class SettingsView : Page {
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
        updates_load();
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

            App.Settings.Save();
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

            App.Settings.Save();
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

    bool clickToRelease = false;
    private async void updates_Click(object sender, RoutedEventArgs e) {
        if (clickToRelease) {
            await Launcher.LaunchUriAsync(new Uri(App.Settings.UpdateVersion.ReleaseUrl));
        } else {
            await Launcher.LaunchUriAsync(new Uri("https://github.com/MolotovCherry/virtual-display-rs/releases"));
        }
    }

    private static readonly Octokit.GitHubClient client = new Octokit.GitHubClient(new Octokit.ProductHeaderValue("VirtualDisplayDriverControl"));
    private async void updateBtn_Click(object sender, RoutedEventArgs e) {
        try {
            UpdateDownloadBtn.IsEnabled = false;

            var settings = App.Settings.UpdateVersion;
            var release = await client.Repository.Release.GetLatest("MolotovCherry", "virtual-display-rs");

            var tag = release.TagName[1..6];
            System.Version data = System.Version.Parse(tag);

            // Update the settings latest checked version
            settings.IsDirty = true;
            settings.ReleaseUrl = release.HtmlUrl;
            settings.Version.Major = data.Major;
            settings.Version.Minor = data.Minor;
            settings.Version.Patch = data.Build;

            List<Asset> assets = new List<Asset>();
            foreach (Octokit.ReleaseAsset asset in release.Assets) {
                settings.Assets.Add(new Asset {
                    Name = asset.Name,
                    Url = asset.BrowserDownloadUrl
                });
            }

            App.Settings.Save();

            updates_load();

            UpdateDownloadBtn.IsEnabled = true;
        } catch {
            updateCard.Header = "Failed to retrieve latest version information";
            UpdateDownloadBtn.IsEnabled = true;
        }
    }

    private void updates_load() {
        if (App.Settings.UpdateVersion.IsDirty && !clickToRelease) {
            try {
                var data = App.Settings.UpdateVersion.Version;

                var major = uint.Parse(GitVersionInformation.Major);
                var minor = uint.Parse(GitVersionInformation.Minor);
                var patch = uint.Parse(GitVersionInformation.Patch) - 2;

                if (data.Major > major || data.Minor > minor || data.Patch > patch) {
                    updateCard.Header = $"Update is available: v{data.Major}.{data.Minor}.{data.Patch}";
                    
                    var flyout = new MenuFlyout { Placement = FlyoutPlacementMode.BottomEdgeAlignedRight };

                    foreach (Asset asset in App.Settings.UpdateVersion.Assets) {
                        var item = new MenuFlyoutItem();
                        item.Text = asset.Name;
                        item.Click += async (object sender, RoutedEventArgs e) => {
                            await Launcher.LaunchUriAsync(new Uri(asset.Url));
                        };

                        flyout.Items.Add(item);
                    }

                    UpdateDownloadBtn.Flyout = flyout;
                    UpdateDownloadBtn.Content = "Download";

                    clickToRelease = true;
                    // won't use this anymore
                    UpdateDownloadBtn.Click -= updateBtn_Click;
                }
            } catch { }
        }
    }
}
