using Microsoft.UI.Xaml;
using Virtual_Display_Driver_Control.Common;
using Virtual_Display_Driver_Control.Helpers;

namespace Virtual_Display_Driver_Control;

public partial class App : Application {
#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.
    public static Window Window { get; private set; }
    public static AppSettings Settings { get; private set; }
#pragma warning restore CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.

    public App() {
        InitializeComponent();
        Settings = SettingsProvider.Initialize();
    }

    protected override void OnLaunched(LaunchActivatedEventArgs args) {
        Window = new MainWindow();

        ThemeHelper.Initialize();
        MaterialHelper.Initialize();

        Window.Activate();
    }
}
