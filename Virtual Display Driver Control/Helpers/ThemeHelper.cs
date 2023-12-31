﻿using Microsoft.UI;
using Microsoft.UI.Composition.SystemBackdrops;
using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using System.Runtime.InteropServices;
using Virtual_Display_Driver_Control.Common;
using Windows.UI;
using Windows.UI.ViewManagement;

namespace Virtual_Display_Driver_Control.Helpers;

public static class ThemeHelper {
    [DllImport("UXTheme.dll", SetLastError = true, EntryPoint = "#138")]
    public static extern bool ShouldSystemUseDarkMode();

    private static UISettings uiSettings = new UISettings();

    public static ElementTheme GetTheme() {
        if (App.Window.Content is FrameworkElement frameworkElement) {
            return frameworkElement.ActualTheme;
        }

        return ElementTheme.Default;
    }

    public static void SetTheme(ElementTheme theme) {
        if (App.Window.Content is FrameworkElement frameworkElement) {
            App.Settings.Theme = theme;

            SetThemeTitlebar(theme);

            uiSettings.ColorValuesChanged -= ColorChangedCb;
            frameworkElement.RequestedTheme = theme;

            if (theme == ElementTheme.Default) {
                uiSettings.ColorValuesChanged += ColorChangedCb;
            }

            ApplyBackground(theme);
        }
    }

    private static void ColorChangedCb(UISettings sender, object args) {
        var dispatcher = App.Window.DispatcherQueue;

        // run it on the main window thread
        dispatcher?.TryEnqueue(() => {
            ElementTheme theme;
            if (ShouldSystemUseDarkMode()) {
                theme = ElementTheme.Dark;
            } else {
                theme = ElementTheme.Light;
            }

            SetThemeTitlebar(theme);
            ApplyBackground(theme);
        });
    }

    private static void SetThemeTitlebar(ElementTheme theme) {
        if (App.Window.AppWindow.TitleBar is AppWindowTitleBar titleBar && AppWindowTitleBar.IsCustomizationSupported()) {
            var resources = (ResourceDictionary)Application.Current.Resources.ThemeDictionaries;

            ResourceDictionary resourceTheme;
            if (theme == ElementTheme.Light) {
                resourceTheme = (ResourceDictionary)resources["Light"];
            } else if (theme == ElementTheme.Dark) {
                resourceTheme = (ResourceDictionary)resources["Dark"];
            // The rest are ElementTheme.Default
            } else if (ShouldSystemUseDarkMode()) {
                resourceTheme = (ResourceDictionary)resources["Dark"];
            } else {
                resourceTheme = (ResourceDictionary)resources["Light"];
            }
            
            titleBar.ButtonForegroundColor = (Color)resourceTheme["ButtonForegroundColor"];
            titleBar.ButtonInactiveForegroundColor = (Color)resourceTheme["ButtonInactiveForegroundColor"];
            titleBar.ButtonHoverForegroundColor = (Color)resourceTheme["ButtonHoverForegroundColor"];
            titleBar.ButtonHoverBackgroundColor = (Color)resourceTheme["ButtonHoverBackgroundColor"];
            titleBar.ButtonPressedBackgroundColor = (Color)resourceTheme["ButtonPressedBackgroundColor"];
            titleBar.ButtonPressedForegroundColor = (Color)resourceTheme["ButtonPressedForegroundColor"];
        }
    }

    public static void ApplyBackground(ElementTheme theme) {
        var appResources = (ResourceDictionary)Application.Current.Resources.ThemeDictionaries;

        ResourceDictionary resourceTheme;
        if (theme == ElementTheme.Dark) {
            resourceTheme = (ResourceDictionary)appResources["Dark"];
        } else if (theme == ElementTheme.Light) {
            resourceTheme = (ResourceDictionary)appResources["Light"];
        // The rest are ElementTheme.Default
        } else if (ShouldSystemUseDarkMode()) {
            resourceTheme = (ResourceDictionary)appResources["Dark"];
        } else {
            resourceTheme = (ResourceDictionary)appResources["Light"];
        }

        var material = App.Settings.Material;
        SolidColorBrush brush;
        if (MaterialHelper.isMicaSupported(material)) {
            brush = new SolidColorBrush(Colors.Transparent);
        } else if (material == Material.Acrylic && DesktopAcrylicController.IsSupported()) {
            brush = new SolidColorBrush(Colors.Transparent);
        } else if (material == Material.Blurred && DesktopAcrylicController.IsSupported()) {
            brush = (SolidColorBrush)resourceTheme["BackgroundBlurred"];
        } else if (material == Material.Transparent) {
            brush = (SolidColorBrush)resourceTheme["BackgroundTransparent"];
        } else {
            brush = (SolidColorBrush)resourceTheme["Background"];
        }

        Grid rootGrid = (Grid)App.Window.Content;

        rootGrid.Background = brush;
    }

    public static void Initialize() {
        SetTheme(App.Settings.Theme);
    }

    public static bool IsEnabled() {
        // high contrast theme does not allow themes to be changed
        var accessibilitySettings = new AccessibilitySettings();
        return !accessibilitySettings.HighContrast;
    }
}
