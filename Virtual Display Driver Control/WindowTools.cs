using CSharpFunctionalExtensions;
using System;
using System.Drawing;
using System.Runtime.InteropServices;

namespace Virtual_Display_Driver_Control;

internal static class WindowTools {
    public const int ICON_SMALL = 0;
    public const int ICON_BIG = 1;
    public const int ICON_SMALL2 = 2;

    public const int WM_GETICON = 0x007F;
    public const int WM_SETICON = 0x0080;

    [DllImport("User32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    public static extern int SendMessage(IntPtr hWnd, uint msg, int wParam, IntPtr lParam);

    public static Maybe<Icon> GetIcon() {
        string? sExe = System.Diagnostics.Process.GetCurrentProcess().MainModule?.FileName;
        if (sExe is not null) {
            return Icon.ExtractAssociatedIcon(sExe)!;
        } else {
            return Maybe<Icon>.None;
        }
    }

    public static void SetWindowIcon(object target) {
        IntPtr hWnd = WinRT.Interop.WindowNative.GetWindowHandle(target);
        GetIcon().Execute(icon => {
            SendMessage(hWnd, WM_SETICON, ICON_BIG, icon.Handle);
        });
    }
}
