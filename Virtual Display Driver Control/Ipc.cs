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



namespace Virtual_Display_Driver_Control {
    public class Ipc : IDisposable {
        public static List<Action> OnConnect = new List<Action>();
        public static List<Action> OnDisconnect = new List<Action>();

        private static PipeClient pipeClient = null;
        private static Task<Ipc> IpcTask = null;

        public static bool IsConnected => pipeClient != null && pipeClient.IsConnected;
        // Gets Ipc, and creates it if it is not created, or if it's disconnected, tries to make new one
        public static Task<Ipc> GetOrCreateIpc() {
            if (IsConnected) {
                return IpcTask;
            } else {
                IpcTask?.Result?.Dispose();

                var tcs = new TaskCompletionSource<Ipc>();

                var result = Task.Run(() => {
                    try {
                        // Would be a bug if it happened
                        if (pipeClient != null) {
                            throw new Exception("Already connected");
                        }

                        pipeClient = new PipeClient();

                        // OnConnect callbacks
                        foreach (var callback in OnConnect) {
                            callback();
                        }

                        // Callbacks to be fired once connection is gone
                        Task.Run(() => {
                            // poll at 50ms intervals for connectivity
                            while (IsConnected) {
                                Thread.Sleep(50);
                            }

                            // Since it's no longer connected, get rid of it
                            IpcTask?.Result?.Dispose();

                            foreach (var callback in OnDisconnect) {
                                callback();
                            }
                        });

                        tcs.SetResult(new Ipc());
                    } catch (Exception e) {
                        tcs.SetException(e);
                    }
                });

                return tcs.Task;
            }
        }

        // Gets Ipc, returns null if it is not created
        public static Ipc GetIpc() {
            if (IsConnected) {
                return IpcTask.Result;
            } else {
                IpcTask?.Result?.Dispose();
                return null;
            }
        }

        private Ipc() { }

        public void DriverNotify(List<Monitor> monitors)
        {
            var command = new SendCommand {
                DriverNotify = monitors
            };

            pipeClient.WriteMessage(command.ToJson());
        }

        public void DriverRemoveAll()
        {
            var command = new SendCommand {
                DriverRemoveAll = true
            };

            pipeClient.WriteMessage(command.ToJson());
        }

        public void DriverRemove(List<uint> monitors)
        {
            var command = new SendCommand {
                DriverRemove = monitors
            };

            pipeClient.WriteMessage(command.ToJson());
        }

        public List<Monitor> RequestState()
        {
            var command = new SendCommand {
                RequestState = true
            };

            pipeClient.WriteMessage(command.ToJson());

            var data = pipeClient.ReadMessage();
            return JsonSerializer.Deserialize<ReplyCommand>(data).ReplyState;
        }

        public void Dispose()
        {
            pipeClient.Dispose();
            pipeClient = null;
            IpcTask = null;
        }
    }
}

//
// IPC Data
//

public class ReplyCommand {
    public List<Monitor> ReplyState { get; set; }
}

public class SendCommand {
    public List<uint> DriverRemove { get;  set; }
    public List<Monitor> DriverNotify { get; set; }
    [JsonIgnore]
    public bool RequestState { get; set; }
    [JsonIgnore]
    public bool DriverRemoveAll { get; set; }

    public string ToJson() {
        var options = new JsonSerializerOptions {
            DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
        };

        if (RequestState) {
            return "\"RequestState\"";
        } else if (DriverRemoveAll) {
            return "\"DriverRemoveAll\"";
        }

        return JsonSerializer.Serialize(this, options);
    }
}

public class Monitor {
    public uint id { get; set; }
    public string name { get; set; }
    public bool enabled { get; set; }
    public List<Mode> modes { get; set; }
}

public class Mode {
    public uint width { get; set; }
    public uint height { get; set; }
    public List<uint> refresh_rates { get; set; }
}

//
// PipeClient
//

public class PipeClient : IDisposable {
    private NamedPipeClientStream pipeClient = null;
    private BlockingCollection<string> Messages = new BlockingCollection<string>();

    public bool IsConnected => pipeClient != null && pipeClient.IsConnected;

    public PipeClient() {
        pipeClient = new NamedPipeClientStream(".", "virtualdisplaydriver", PipeDirection.InOut);

        pipeClient.Connect(50);

        pipeClient.ReadMode = PipeTransmissionMode.Message;

        Task.Run(() => {
            // Read all messages into buffer
            Reader();
        });
    }

    public void WriteMessage(string message) {
        var bytes = Encoding.UTF8.GetBytes(message);

        pipeClient.Write(bytes);
        pipeClient.Flush();
    }

    private string ReadMessageInternal() {
        var buffer = new byte[1024];
        var sb = new StringBuilder();

        int read;

        do {
            read = pipeClient.Read(buffer, 0, buffer.Length);

            if (read > 0) {
                sb.Append(Encoding.UTF8.GetString(buffer, 0, read));
            }
        } while (read > 0 && !pipeClient.IsMessageComplete);

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
        pipeClient.Close();
    }

    #nullable enable
    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool PeekNamedPipe(SafeHandle handle,
        byte[] buffer, uint nBufferSize, ref uint bytesRead,
        ref uint bytesAvail, ref uint BytesLeftThisMessage);
    #nullable restore

    // Check if pipe has anything available
    bool ReadyToRead() {
        byte[] buffer = new byte[1];
        uint aPeekedBytes = 0;
        uint aAvailBytes = 0;
        uint aLeftBytes = 0;

        bool aPeekedSuccess = PeekNamedPipe(
            pipeClient.SafePipeHandle,
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

