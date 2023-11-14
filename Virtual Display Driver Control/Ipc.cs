using System;
using System.IO.Pipes;
using System.Collections.Generic;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using System.Threading;
using System.Text.Json.Serialization;
using System.Collections.Concurrent;
using System.Runtime.InteropServices;
using System.Linq;
using CSharpFunctionalExtensions;

namespace Virtual_Display_Driver_Control;

public class Ipc : IDisposable {
    public static List<Action<Ipc>> OnConnect = new List<Action<Ipc>>();
    public static List<Action> OnDisconnect = new List<Action>();

    private static Maybe<PipeClient> pipeClient = Maybe<PipeClient>.None;

    public static bool IsConnected => pipeClient.HasValue && pipeClient.GetValueOrThrow().IsConnected;

    private Ipc() { }

    public static void GetOrCreateIpc() {
        GetOrCreateIpc(null, null);
    }

    public static void GetOrCreateIpc(Action<Ipc> success) {
        GetOrCreateIpc(success, null);
    }

    // Gets Ipc, and creates it if it is not created, or if it's disconnected, tries to make new one
    // Calls success() if it succeeded getting/creating, if it failed calls failed()
    //
    // Each param may be null to ignore the callback
    public static void GetOrCreateIpc(Action<Ipc>? success, Action? failed) {
        if (IsConnected) {
            if (success != null) {
                success(new Ipc());
            }
        } else {
            DisposeInternal();

            var tcs = new TaskCompletionSource<Ipc>();

            var result = Task.Run(() => {
                try {
                    pipeClient = new PipeClient();

                    if (success != null) {
                        success(new Ipc());
                    }

                    // OnConnect callbacks
                    foreach (var callback in OnConnect) {
                        callback(new Ipc());
                    }

                    // Callbacks to be fired once connection is gone
                    Task.Run(() => {
                        // poll at 50ms intervals for connectivity
                        while (IsConnected) {
                            Thread.Sleep(50);
                        }

                        // Since it's no longer connected, get rid of it
                        DisposeInternal();

                        foreach (var callback in OnDisconnect) {
                            callback();
                        }
                    });
                } catch {
                    if (failed != null) {
                        failed();
                    }
                }
            });
        }
    }

    // Gets Ipc, returns null if it is not created
    public static Maybe<Ipc> GetIpc() {
        if (IsConnected) {
            return new Ipc();
        } else {
            DisposeInternal();
            return Maybe<Ipc>.None;
        }
    }

    private void ExecuteConnectedOrDispose(Action<PipeClient> cb) {
        if (IsConnected) {
            cb(pipeClient.GetValueOrThrow());
        } else {
            DisposeInternal();
        }
    }

    public void DriverNotify(List<Monitor> monitors) {
        ExecuteConnectedOrDispose(client => {
            var command = new SendCommand {
                DriverNotify = monitors
            };

            client.WriteMessage(command.ToJson());
        });
    }

    public void DriverRemoveAll() {
        ExecuteConnectedOrDispose(client => {
            var command = new SendCommand {
                DriverRemoveAll = true
            };

            client.WriteMessage(command.ToJson());
        });
    }

    public void DriverRemove(List<uint> monitors) {
        ExecuteConnectedOrDispose(client => {
            var command = new SendCommand {
                DriverRemove = monitors
            };

            client.WriteMessage(command.ToJson());
        });
    }

    public List<Monitor> RequestState() {
        if (IsConnected) {
            PipeClient? out_client;
            if (pipeClient.TryGetValue(out out_client) && out_client is PipeClient client) {
                var command = new SendCommand {
                    RequestState = true
                };

                client.WriteMessage(command.ToJson());

                var data = client.ReadMessage();
                var deserialize = JsonSerializer.Deserialize<ReplyCommand>(data);
                
                return deserialize?.ReplyState ?? new List<Monitor>();
            }
        }

        return new List<Monitor>();
    }

    public void Dispose() {
        DisposeInternal();
    }

    private static void DisposeInternal() {
        pipeClient.Execute(client => {
            client.Dispose();
            pipeClient = Maybe<PipeClient>.None;
        });
    }
}

//
// IPC Data
//

public class ReplyCommand {
    public List<Monitor>? ReplyState { get; set; }
}

public class SendCommand {
    public List<uint>? DriverRemove { get;  set; }
    public List<Monitor>? DriverNotify { get; set; }
    public bool? RequestState { get; set; }
    public bool? DriverRemoveAll { get; set; }

    public string ToJson() {
        var options = new JsonSerializerOptions {
            DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
        };

        if (RequestState.GetValueOrDefault()) {
            return "\"RequestState\"";
        } else if (DriverRemoveAll.GetValueOrDefault()) {
            return "\"DriverRemoveAll\"";
        }

        return JsonSerializer.Serialize(this, options);
    }
}

public class Monitor : ICloneable {
    public uint id { get; set; }
    public string? name { get; set; }
    public bool enabled { get; set; }
    public List<Mode>? modes { get; set; }
    // used to keep track of ui state
    [JsonIgnore]
    public bool pending { get; set; }

    public object Clone() {
        return new Monitor {
            id = id,
            name = name,
            enabled = enabled,
            modes = modes?.Select(mode => (Mode)mode.Clone()).ToList(),
            pending = pending
        };
    }
}

public class Mode : ICloneable {
    public uint width { get; set; }
    public uint height { get; set; }
    public List<uint>? refresh_rates { get; set; }
    // used to keep track of ui state
    [JsonIgnore]
    public bool pending { get; set; }

    public object Clone() {
        return new Mode {
            width = width,
            height = height,
            refresh_rates = refresh_rates?.ToList(),
            pending = pending
        };
    }
}

//
// PipeClient
//

public class PipeClient : IDisposable {
    private Maybe<NamedPipeClientStream> pipeClient = Maybe<NamedPipeClientStream>.None;
    private BlockingCollection<string> Messages = new BlockingCollection<string>();

    public bool IsConnected => pipeClient.HasValue && pipeClient.GetValueOrThrow().IsConnected;

    public PipeClient() {
        var client = new NamedPipeClientStream(".", "virtualdisplaydriver", PipeDirection.InOut);

        client.Connect(50);

        client.ReadMode = PipeTransmissionMode.Message;

        pipeClient = client;

        Task.Run(() => {
            // Read all messages into buffer
            Reader();
        });
    }

    public void WriteMessage(string message) {
        var bytes = Encoding.UTF8.GetBytes(message);

        pipeClient.Execute(client => {
            client.Write(bytes);
            client.Flush();
        });
    }

    private string ReadMessageInternal() {
        // this should never throw, if it does, it's a bug
        var client = pipeClient.GetValueOrThrow();

        var buffer = new byte[1024];
        var sb = new StringBuilder();

        int read;

        do {
            read = client.Read(buffer, 0, buffer.Length);

            if (read > 0) {
                sb.Append(Encoding.UTF8.GetString(buffer, 0, read));
            }
        } while (read > 0 && !client.IsMessageComplete);

        return sb.ToString();
    }

    private void Reader() {
        while (IsConnected) {
            try {
                if (ReadyToRead()) {
                    var msg = ReadMessageInternal();
                    Messages.Add(msg);
                } else {
                    // any error other than 0 means it failed
                    // for example, pipe broken
                    var err = Marshal.GetLastWin32Error();

                    if (err != 0) {
                        Dispose();
                        break;
                    }
                }

                Thread.Sleep(50);
            } catch {
                break;
            }

        }
    }

    public string ReadMessage() {
        // This defaults to FIFO
        var data = Messages.Take();
        return data;
    }

    public void Dispose() {
        pipeClient.Execute(client => {
            client.Close();
        });
    }

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool PeekNamedPipe(SafeHandle handle,
        byte[] buffer, uint nBufferSize, ref uint bytesRead,
        ref uint bytesAvail, ref uint BytesLeftThisMessage);

    // Check if pipe has anything available
    bool ReadyToRead() {
        if (pipeClient.HasNoValue) {
            return false;
        }

        var client = pipeClient.GetValueOrThrow();

        byte[] buffer = new byte[1];
        uint aPeekedBytes = 0;
        uint aAvailBytes = 0;
        uint aLeftBytes = 0;

        bool aPeekedSuccess = PeekNamedPipe(
            client.SafePipeHandle,
            buffer,
            (uint)buffer.Length,
            ref aPeekedBytes,
            ref aAvailBytes,
            ref aLeftBytes
        );

        if (aPeekedSuccess && aAvailBytes > 0) {
            return true;
        } else {
            return false;
        }
    }
}
