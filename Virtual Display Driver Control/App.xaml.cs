using Microsoft.UI.Xaml;
using Serilog;
using System;
using System.Diagnostics;
using System.Text;
using Virtual_Display_Driver_Control.Common;
using Virtual_Display_Driver_Control.Helpers;

namespace Virtual_Display_Driver_Control;

public partial class App : Application {
#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.
    public static Window Window { get; private set; }
    public static AppSettings Settings { get; private set; }
#pragma warning restore CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.

    public App() {
        Logging.Initialize();

        UnhandledException += App_UnhandledException;
        AppDomain.CurrentDomain.UnhandledException += Domain_UnhandledException;
        InitializeComponent();
        Settings = SettingsProvider.Initialize();
    }

    protected override void OnLaunched(LaunchActivatedEventArgs args) {
        Window = new MainWindow();

        ThemeHelper.Initialize();
        MaterialHelper.Initialize();

        Window.Activate();
        Window.Closed += OnClosed;
    }

    private void OnClosed(object sender, WindowEventArgs e) {
        // cleanup ops
        Log.CloseAndFlush();
    }

    void App_UnhandledException(object sender, Microsoft.UI.Xaml.UnhandledExceptionEventArgs e) {
        _UnhandledException(sender, e.Exception);
    }

    void Domain_UnhandledException(object sender, System.UnhandledExceptionEventArgs e) {
        _UnhandledException(sender, (Exception)e.ExceptionObject);
    }

    async void _UnhandledException(object sender, Exception ex) {
        StringBuilder formattedException = new StringBuilder() { Capacity = 200 };

        formattedException.Append("\n--------- UNHANDLED EXCEPTION ---------");

        if (ex is not null) {
            formattedException.Append($"\n>>>> HRESULT: {ex.HResult}\n");
            if (ex.Message is not null) {
                formattedException.Append("\n--- MESSAGE ---\n");
                formattedException.Append(ex.Message);
            }
            if (ex.StackTrace is not null) {
                formattedException.Append("\n--- STACKTRACE ---\n");
                formattedException.Append(ex.StackTrace);
            }
            if (ex.Source is not null) {
                formattedException.Append("\n--- SOURCE ---\n");
                formattedException.Append(ex.Source);
            }
            if (ex.InnerException is not null) {
                formattedException.Append("\n--- INNER ---\n");
                formattedException.Append(ex.InnerException);
            }
        } else {
            formattedException.Append("\nException is null!\n");
        }

        formattedException.Append("\n---------------------------------------\n");

        Log.Fatal(formattedException.ToString());

        Log.CloseAndFlush();

        // Please check "Output Window" for exception details (View -> Output Window) (CTRL + ALT + O)
        Debugger.Break();
    }
}
