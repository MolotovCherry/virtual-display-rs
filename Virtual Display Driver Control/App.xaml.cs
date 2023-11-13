using Microsoft.UI.Xaml;
using Virtual_Display_Driver_Control.Helpers;

namespace Virtual_Display_Driver_Control {
    public partial class App : Application {
        public static Window? Window { get; private set; }

        public App() {
            InitializeComponent();
        }

        protected override void OnLaunched(Microsoft.UI.Xaml.LaunchActivatedEventArgs args) {
            Window = new MainWindow();

            ThemeHelper.Initialize();
            MaterialHelper.Initialize();

            Window.Activate();
        }
    }
}
