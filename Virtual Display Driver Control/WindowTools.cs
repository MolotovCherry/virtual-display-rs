using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace Virtual_Display_Driver_Control
{
    internal static class WindowTools
    {
        public const int ICON_SMALL = 0;
        public const int ICON_BIG = 1;
        public const int ICON_SMALL2 = 2;

        public const int WM_GETICON = 0x007F;
        public const int WM_SETICON = 0x0080;

        [DllImport("User32.dll", SetLastError = true, CharSet = CharSet.Auto)]
        public static extern int SendMessage(IntPtr hWnd, uint msg, int wParam, IntPtr lParam);

        public static void SetWindowIcon(object target)
        {
            IntPtr hWnd = WinRT.Interop.WindowNative.GetWindowHandle(target);
            string sExe = System.Diagnostics.Process.GetCurrentProcess().MainModule.FileName;
            System.Drawing.Icon ico = System.Drawing.Icon.ExtractAssociatedIcon(sExe);
            SendMessage(hWnd, WM_SETICON, ICON_BIG, ico.Handle);
        }
    }
}
