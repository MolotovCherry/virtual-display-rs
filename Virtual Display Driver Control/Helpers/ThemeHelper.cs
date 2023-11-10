using Microsoft.UI;
using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media.Animation;
using System.Runtime.InteropServices;
using Windows.Storage;
using Windows.UI;
using Windows.UI.ViewManagement;

namespace Virtual_Display_Driver_Control.Helpers;

public static class ThemeHelper {
    [DllImport("UXTheme.dll", SetLastError = true, EntryPoint = "#138")]
    public static extern bool ShouldSystemUseDarkMode();

    private static UISettings uiSettings = new UISettings();

    public static ElementTheme GetTheme() {
        if (App.Window?.Content is FrameworkElement frameworkElement) {
            return frameworkElement.ActualTheme;
        }

        return ElementTheme.Default;
    }

    public static void SetTheme(ElementTheme theme) {
        if (App.Window?.Content is FrameworkElement frameworkElement) {
            ApplicationDataContainer localSettings = ApplicationData.Current.LocalSettings;

            if (theme == ElementTheme.Light || (!ShouldSystemUseDarkMode() && theme != ElementTheme.Dark)) {
                localSettings.Values["theme"] = theme == ElementTheme.Light ? "Light" : "Default";
            } else if (theme == ElementTheme.Dark || (ShouldSystemUseDarkMode() && theme != ElementTheme.Light)) {
                localSettings.Values["theme"] = theme == ElementTheme.Dark ? "Dark" : "Default";
            }

            SetThemeTitlebar(theme);

            uiSettings.ColorValuesChanged -= ColorChangedCb;
            frameworkElement.RequestedTheme = theme;

            if (theme == ElementTheme.Default) {
                uiSettings.ColorValuesChanged += ColorChangedCb;
            }
        }
    }

    private static void ColorChangedCb(UISettings sender, object args) {
         var dispatcher = App.Window?.DispatcherQueue;

        // run it on the main window thread
        dispatcher.TryEnqueue(() => {
            ElementTheme theme;
            if (ShouldSystemUseDarkMode()) {
                theme = ElementTheme.Dark;
            } else {
                theme = ElementTheme.Light;
            }

            SetThemeTitlebar(theme);
        });
    }

    private static void SetThemeTitlebar(ElementTheme theme) {
        if (App.Window?.AppWindow.TitleBar is AppWindowTitleBar titleBar && AppWindowTitleBar.IsCustomizationSupported()) {
            if (theme == ElementTheme.Light || (!ShouldSystemUseDarkMode() && theme != ElementTheme.Dark)) {
                titleBar.ButtonForegroundColor = Colors.Black;
                titleBar.ButtonInactiveForegroundColor = Colors.Black;
                titleBar.ButtonHoverForegroundColor = Colors.Black;
                titleBar.ButtonHoverBackgroundColor = Color.FromArgb(127, 233, 233, 233);
                titleBar.ButtonPressedBackgroundColor = Color.FromArgb(127, 237, 237, 237);
                titleBar.ButtonPressedForegroundColor = Colors.Black;
            } else if (theme == ElementTheme.Dark || (ShouldSystemUseDarkMode() && theme != ElementTheme.Light)) {
                titleBar.ButtonForegroundColor = Colors.White;
                titleBar.ButtonInactiveForegroundColor = Colors.White;
                titleBar.ButtonHoverForegroundColor = Colors.White;
                titleBar.ButtonHoverBackgroundColor = Color.FromArgb(127, 25, 25, 25);
                titleBar.ButtonPressedBackgroundColor = Color.FromArgb(127, 29, 29, 29);
                titleBar.ButtonPressedForegroundColor = Colors.White;
            }
        }
    }

    public static void Initialize() {
        ElementTheme theme;
        ApplicationDataContainer localSettings = ApplicationData.Current.LocalSettings;

        var themeString = (string)localSettings.Values["theme"];

        if (themeString == "Light") {
            theme = ElementTheme.Light;
        } else if (themeString == "Dark") {
            theme = ElementTheme.Dark;
        } else {
            theme = ElementTheme.Default;
        }

        SetTheme(theme);
    }

    public static bool IsEnabled() {
        // high contrast theme does not allow themes to be changed
        var accessibilitySettings = new AccessibilitySettings();
        return !accessibilitySettings.HighContrast;
    }
}
