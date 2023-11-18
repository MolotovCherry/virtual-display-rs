using Microsoft.UI.Xaml.Controls;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Virtual_Display_Driver_Control.Views;

public sealed partial class MonitorsView : Page {
    private static Ipc? _ipc = null;

    static bool s_IsConnected {
        get {
            return _ipc != null && Ipc.IsConnected;
        }
    }

    static Ipc? s_Ipc {
        get {
            if (s_IsConnected) {
                return _ipc;
            } else {
                return null;
            }
        }
    }

    static List<Monitor>? savedData = null;
    static List<Monitor>? monitorList = null;

    public MonitorsView() {
        InitializeComponent();

        // Now initialize Ipc since we have defined the callback
        Ipc.GetOrCreateIpc((Ipc ipc) => {
            System.Diagnostics.Debug.WriteLine("Init view");
            savedData = ipc.RequestState();
            // an actual clone, we don't want to touch the original savedData
            monitorList = savedData.Select(monitor => (Monitor)monitor.Clone()).ToList();
            _ipc = ipc;
        });
    }

    // callback is fired once it reconnects (if it does). can be null if not desired
    public void ReconnectIpc(Action<Ipc> callback) {
        if (!s_IsConnected) {
            Ipc.GetOrCreateIpc((Ipc ipc) => {
                _ipc = ipc;

                if (callback != null) {
                    callback(ipc);
                }
            }, null);
        }
    }
}
