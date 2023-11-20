using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Virtual_Display_Driver_Control.Views;

namespace Virtual_Display_Driver_Control {
    public sealed partial class MainWindow : Window {
        public MainWindow() {
            InitializeComponent();

            // set window icon
            WindowTools.SetWindowIcon(this);

            // only supported on windows 11
            if (AppWindowTitleBar.IsCustomizationSupported()) {
                ExtendsContentIntoTitleBar = true;
                SetTitleBar(AppTitleBar);
            }
        }

        private void NavView_Loaded(object sender, RoutedEventArgs e) {
            foreach (NavigationViewItemBase item in NavView.MenuItems) {
                if (item is NavigationViewItem && item.Tag.ToString() == "MonitorsView") {
                    NavView.SelectedItem = item;
                    break;
                }
            }

            ContentFrame.Navigate(typeof(MonitorsView));
        }

        private void NavView_SelectionChanged(NavigationView sender, NavigationViewSelectionChangedEventArgs args) {
            if (args.IsSettingsSelected) {
                ContentFrame.Navigate(typeof(SettingsView));
            } else if (args.SelectedItem is NavigationViewItem item) {
                switch (item.Tag) {
                    case "MonitorsView":
                        ContentFrame.Navigate(typeof(MonitorsView));
                        break;

                    default:
                        break;
                }
            }
        }
    }
}
