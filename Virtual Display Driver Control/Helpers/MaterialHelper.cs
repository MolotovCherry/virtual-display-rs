using Microsoft.UI.Composition.SystemBackdrops;
using Microsoft.UI.Xaml.Media;
using Virtual_Display_Driver_Control.Common;
using WinUIEx;

namespace Virtual_Display_Driver_Control.Helpers;

class MaterialHelper {
    private static MicaBackdrop micaBackdrop = new MicaBackdrop() { Kind = MicaKind.Base };
    private static MicaBackdrop micaBackdropAlt = new MicaBackdrop() { Kind = MicaKind.BaseAlt };
    private static DesktopAcrylicBackdrop acrylicBackdrop = new DesktopAcrylicBackdrop();
    private static TransparentTintBackdrop transparentTintBackdrop = new TransparentTintBackdrop();
    private static BlurredBackdrop blurredBackdrop = new BlurredBackdrop();

    private class BlurredBackdrop : CompositionBrushBackdrop {
        protected override Windows.UI.Composition.CompositionBrush CreateBrush(Windows.UI.Composition.Compositor compositor)
            => compositor.CreateHostBackdropBrush();
    }

    public static void Initialize() {
        SetMaterial(App.Settings.Material);
    }

    public static void SetMaterial(Material material) {
        AppSettings Settings = App.Settings;

        if (material == Material.Mica && MicaController.IsSupported()) {
            App.Window.SystemBackdrop = micaBackdrop;
            Settings.Material = material;
        } else if (material == Material.MicaAlt && MicaController.IsSupported()) {
            App.Window.SystemBackdrop = micaBackdropAlt;
            Settings.Material = material;
        } else if (material == Material.Acrylic && DesktopAcrylicController.IsSupported()) {
            App.Window.SystemBackdrop = acrylicBackdrop;
            Settings.Material = material;
        } else if (material == Material.Blurred && DesktopAcrylicController.IsSupported()) {
            App.Window.SystemBackdrop = blurredBackdrop;
            Settings.Material = material;
        } else if (material == Material.Transparent) {
            App.Window.SystemBackdrop = transparentTintBackdrop;
            Settings.Material = material;
        } else {
            App.Window.SystemBackdrop = null;
            Settings.Material = Material.None;
        }

        ThemeHelper.ApplyBackground(Settings.Theme);
    }

    // Checks if material is supported
    public static bool isSupported(Material material) {
        if (material == Material.Mica || material == Material.MicaAlt) {
            return MicaController.IsSupported();
        } else if (material == Material.Acrylic || material == Material.Blurred) {
            return DesktopAcrylicController.IsSupported();
        } else {
            return true;
        }
    }

    public static bool isMicaSupported(Material material) {
        if (material == Material.Mica || material == Material.MicaAlt) {
            return MicaController.IsSupported();
        }

        return false;
    }

    public static bool isAcrylicSupported(Material material) {
        if (material == Material.Acrylic || material == Material.Blurred) {
            return DesktopAcrylicController.IsSupported();
        }

        return false;
    }
}
