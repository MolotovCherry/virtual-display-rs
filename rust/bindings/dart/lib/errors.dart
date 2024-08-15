export 'src/generated/api/client.dart'
    show
        ConnectionError,
        ConnectionError_Failed,
        PersistError,
        PersistError_Open,
        PersistError_Serialize,
        PersistError_Set,
        ReceiveError,
        RequestError,
        RequestError_Receive,
        RequestError_Send,
        RequestError_Timeout,
        SendError,
        SendError_PipeBroken;

export 'src/generated/api/driver_client.dart'
    show
        AddModeError,
        AddModeError_ModeExists,
        AddModeError_MonitorNotFound,
        AddModeError_RefreshRateExists,
        DuplicateError,
        DuplicateError_Monitor,
        DuplicateError_Mode,
        DuplicateError_RefreshRate,
        InitError,
        InitError_Connect,
        InitError_RequestState,
        MonitorNotFoundError;
