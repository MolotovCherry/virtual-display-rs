using System;
using System.Runtime.InteropServices;

namespace Virtual_Display_Driver_Control {
    internal static class WindowTools {
        public const int ICON_SMALL = 0;
        public const int ICON_BIG = 1;
        public const int ICON_SMALL2 = 2;

        public const int WM_GETICON = 0x007F;
        public const int WM_SETICON = 0x0080;

        [DllImport("User32.dll", SetLastError = true, CharSet = CharSet.Auto)]
        public static extern int SendMessage(IntPtr hWnd, uint msg, int wParam, IntPtr lParam);

        public static System.Drawing.Icon GetIcon() {
            string sExe = System.Diagnostics.Process.GetCurrentProcess().MainModule.FileName;
            return System.Drawing.Icon.ExtractAssociatedIcon(sExe);
        }

        public static void SetWindowIcon(object target) {
            IntPtr hWnd = WinRT.Interop.WindowNative.GetWindowHandle(target);
            System.Drawing.Icon ico = GetIcon();
            SendMessage(hWnd, WM_SETICON, ICON_BIG, ico.Handle);
        }
    }
}
