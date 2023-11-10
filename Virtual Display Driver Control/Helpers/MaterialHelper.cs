using Microsoft.UI.Composition.SystemBackdrops;
using Microsoft.UI.Xaml.Media;
using Windows.Storage;

namespace Virtual_Display_Driver_Control.Helpers;

class MaterialHelper
{
    public static void Initialize() {
        ApplicationDataContainer localSettings = ApplicationData.Current.LocalSettings;

        SetMaterial((string)localSettings.Values["material"] ?? "Mica");
    }

    public static void SetMaterial(string material) {
        var window = App.Window;
        if (window != null) {
            ApplicationDataContainer localSettings = ApplicationData.Current.LocalSettings;

            if (material == "Mica" && MicaController.IsSupported()) {
                window.SystemBackdrop = new MicaBackdrop() { Kind = MicaKind.Base };
                localSettings.Values["material"] = "Mica";
            } else if (material == "Acrylic" && DesktopAcrylicController.IsSupported()) {
                window.SystemBackdrop = new DesktopAcrylicBackdrop();
                localSettings.Values["material"] = "Acrylic";
            } else {
                window.SystemBackdrop = null;
                localSettings.Values["material"] = "None";
            }
        }
    }
}
