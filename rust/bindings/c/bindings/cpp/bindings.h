#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// A thin api client over the driver api with all the essential api.
/// Does the bare minimum required. Does not track state
struct Client;

using Id = uint32_t;

using Dimen = uint32_t;

using RefreshRate = uint32_t;

/// Cannot be freed if allocated/created on c side and passed to rust
/// If you received this type from a fn call, then it must be freed
struct Mode {
  Dimen width;
  Dimen height;
  /// array of refresh rates. cannot be null, but len may be 0
  const RefreshRate *refresh_rates;
  /// length of refresh_rates array
  uintptr_t refresh_rates_len;
};

/// Cannot be freed if allocated/created on c side and passed to rust
/// If you received this type from a fn call, then it must be freed
struct Monitor {
  /// identifier
  Id id;
  /// null if there's no name. non null if there is. must be null terminated
  const char *name;
  /// length of name array
  uintptr_t name_len;
  bool enabled;
  /// array of modes. cannot be null. but len may be 0
  const Mode *modes;
  /// length of modes array
  uintptr_t modes_len;
};

/// You must call free on it when done
struct ReplyCommand {
  enum class Tag {
    /// Reply to previous current system monitor state request
    /// ptr to array of monitor + len of array
    State,
  };

  struct State_Body {
    Monitor *arr;
    uintptr_t len;
    uintptr_t _reserved;
  };

  Tag tag;
  union {
    State_Body state;
  };
};

extern "C" {

/// # SAFETY
/// - ptr must be a valid, unfreed, Client
/// - must not use ptr after it is freed
/// - must have been a ptr given to you from this library
void client_free(Client *ptr);

/// create client
/// connect to pipe virtualdisplaydriver
///
/// returns null ptr if connection failed
Client *client_connect();

/// choose which pipe name you connect to
/// pipe name is ONLY the name, only the {name} portion of \\.\pipe\{name}
///
/// # SAFETY
/// - name arg must be null terminated
/// - must be valid char data
/// - must contain valid utf8 (won't be ub, but function will fail)
///
/// returns null ptr if function failed
Client *client_connect_to(const char *name);

/// Notifies driver of changes (additions/updates/removals)
///
/// # SAFETY
/// - ptr must be a valid, unfreed, Client
/// - monitors is a ptr to an valid array of Monitor
/// - len must be a valid len for the array
/// - this is thread safe, but will fail if any functions are called simultaneously
///
/// returns if function succeeded or not
bool client_notify(Client *ptr, const Monitor *monitors, uintptr_t len);

/// Remove specific monitors by id
///
/// # SAFETY
/// - ptr must be a valid, unfreed, Client
/// - `ids` is an array of Id
/// - `ids_len` must be valid len for the array
/// - this is thread safe, but will fail if any functions are called simultaneously
bool client_remove(Client *ptr, const Id *ids, uintptr_t ids_len);

/// Remove all monitors
///
/// # SAFETY:
/// - ptr must be a valid, unfreed, Client
/// - this is thread safe, but will fail if any functions are called simultaneously
bool remove_all(Client *ptr);

/// Receive generic reply
///
/// If `last` is false, will only receive new messages from the point of calling
/// If `last` is true, will receive the the last message received, or if none, blocks until the next one
///
/// The reason for the `last` flag is that replies are auto buffered in the background, so if you send a
/// request, the reply may be missed
///
/// # SAFETY
/// - ptr must be a valid, unfreed, Client
/// - returns null ptr if function failed
/// - this is thread safe, but will fail if any functions are called simultaneously
ReplyCommand *receive_reply(Client *ptr,
                            bool last);

} // extern "C"
