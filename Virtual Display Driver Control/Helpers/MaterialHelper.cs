using CSharpFunctionalExtensions;
using Microsoft.UI.Composition.SystemBackdrops;
using Microsoft.UI.Xaml.Media;
using Virtual_Display_Driver_Control.Common;
using Windows.Storage;

namespace Virtual_Display_Driver_Control.Helpers;

class MaterialHelper {
    public static void Initialize() {
        SetMaterial(App.Settings.Material);
    }

    public static void SetMaterial(Material material) {
        IAppSettings Settings = App.Settings;

        if (material == Material.Mica && MicaController.IsSupported()) {
            App.Window.SystemBackdrop = new MicaBackdrop() { Kind = MicaKind.Base };
            Settings.Material = material;
        } else if (material == Material.Acrylic && DesktopAcrylicController.IsSupported()) {
            App.Window.SystemBackdrop = new DesktopAcrylicBackdrop();
            Settings.Material = material;
        } else {
            App.Window.SystemBackdrop = null;
            Settings.Material = Material.None;
        }

        ThemeHelper.ApplyBackground(Settings.Theme);
    }
}
