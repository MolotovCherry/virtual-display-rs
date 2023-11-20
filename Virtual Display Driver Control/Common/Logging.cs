using Serilog;
using System;
using System.IO;

namespace Virtual_Display_Driver_Control.Common; 
class Logging {
    public static void Initialize() {
        var log = new LoggerConfiguration()
            // Always log to debug regardless
            .WriteTo.Debug();

        // Write output log if not in debug mode
        #if !DEBUG
            log.WriteTo.File(Path.Combine(SettingsProvider.AppDir, "app.log"),
                rollingInterval: RollingInterval.Day,
                rollOnFileSizeLimit: true);
        #endif
            
        Log.Logger = log.CreateLogger();
    }

    public static void Dispose() {
        Log.CloseAndFlush();
    }
}
