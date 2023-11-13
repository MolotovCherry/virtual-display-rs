using CSharpFunctionalExtensions;
using Microsoft.UI.Xaml;
using Virtual_Display_Driver_Control.Common;
using Virtual_Display_Driver_Control.Helpers;

namespace Virtual_Display_Driver_Control {
    public partial class App : Application {
        public static Maybe<Window> Window { get; private set; }

        public App() {
            Logger.Initialize();
            InitializeComponent();
        }

        protected override void OnLaunched(LaunchActivatedEventArgs args) {
            var window = new MainWindow();
            Window = window;

            ThemeHelper.Initialize();
            MaterialHelper.Initialize();

            window.Activate();
        }
    }
}
