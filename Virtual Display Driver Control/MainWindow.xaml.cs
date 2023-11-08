using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Data;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Navigation;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.IO.Pipes;
using System.Linq;
using System.Runtime.InteropServices.WindowsRuntime;
using System.Threading;
using Windows.Foundation;
using Windows.Foundation.Collections;

namespace Virtual_Display_Driver_Control {
    public sealed partial class MainWindow : Window {
        public MainWindow() {
            this.InitializeComponent();
        }

        private void myButton_Click(object sender, RoutedEventArgs e) {
            myButton.Content = "Clicked";
        }
    }
}
