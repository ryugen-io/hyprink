# Kitchn Log Architecture (Revised 2025)

## Philosophy
- "Sweet Dracula" theme is the standard.
- **Unified Stream**: All logs (user-facing and debug) flow through a unified `tracing` pipeline.
- **Smart Client, Dumb Server**: `kitchn` CLI formats logs (colors, timestamps) and sends them to a debug viewer over a Unix Domain Socket.

## Components
- **Tracing**: Replaced `log` facade in CLI with `tracing`.
    - `tracing-log`: Bridges legacy `log` calls from `kitchn_lib` into the tracing system to capture deep hook details.
- **Socket Logging**:
    - Debug logs are broadcast to `$XDG_RUNTIME_DIR/kitchn-debug.sock`.
    - No disk I/O latency for debug stream.
- **Viewer**: `kitchn --debug` spawns a separate terminal running `kitchn internal-watch` which acts as the socket server/listener.

## Interaction
- `kitchn` CLI imports `kitchn_lib::logger` directly for user-facing output (no `kitchn-log` subprocess spawning).
- `processor` and `db` operations in `kitchn_lib` emit standard `log::debug!` events, which are captured by `LogTracer` and forwarded to the debug socket.

## Verbose/Debug Logging
- **Enabled via**: `kitchn --debug` (global flag).
- **Output Format**: `TIME [LEVEL] [SCOPE] Message`
    - **TIME**: `HH:MM:SS` (Dimmed)
    - **LEVEL**: Colored (ERROR=Red, WARN=Yellow, INFO=Green, DEBUG=Blue, TRACE=Magenta)
    - **SCOPE**: Optional component scope (e.g., `[BAKE]`)
- **Hook Visibility**: Standard out/err from hooks is captured and logged as `DEBUG` events, ensuring full visibility of failures.
