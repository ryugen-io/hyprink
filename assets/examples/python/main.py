import ctypes
import os
import sys

# Load the shared library
# In a real scenario, this would be in /usr/lib or similar
lib_path = os.path.abspath("../../../target/release/libhi_ffi.so")
try:
    hyprink = ctypes.CDLL(lib_path)
except OSError as e:
    print(f"Failed to load library at {lib_path}: {e}")
    sys.exit(1)

# Define opaque pointer type for HyprinkContext
class HyprinkContext(ctypes.Structure):
    pass

HyprinkContext_p = ctypes.POINTER(HyprinkContext)

# Define function signatures
hyprink.hyprink_context_new.restype = HyprinkContext_p
hyprink.hyprink_context_free.argtypes = [HyprinkContext_p]

# Set App Name
hyprink.hyprink_context_set_app_name.argtypes = [HyprinkContext_p, ctypes.c_char_p]
hyprink.hyprink_context_set_app_name.restype = None

hyprink.hyprink_log.argtypes = [HyprinkContext_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p]
hyprink.hyprink_log.restype = None

hyprink.hyprink_log_preset.argtypes = [HyprinkContext_p, ctypes.c_char_p, ctypes.c_char_p]
hyprink.hyprink_log_preset.restype = ctypes.c_int

hyprink.hyprink_pack.argtypes = [HyprinkContext_p, ctypes.c_char_p, ctypes.c_char_p]
hyprink.hyprink_pack.restype = ctypes.c_int

hyprink.hyprink_get_last_error.argtypes = [HyprinkContext_p, ctypes.c_char_p, ctypes.c_size_t]
hyprink.hyprink_get_last_error.restype = ctypes.c_int

def main():
    print("Initializing hyprink Context (Python)...")

    # 1. Create Context
    ctx = hyprink.hyprink_context_new()
    if not ctx:
        print("Failed to create context!")
        sys.exit(1)

    try:
        # Set App Name
        hyprink.hyprink_context_set_app_name(ctx, b"PythonExample")

        # 2. Logging Example
        print("Sending logs from Python...")
        # strings must be bytes in ctypes
        hyprink.hyprink_log(ctx, b"info", b"python_example", b"Hello from Python via FFI!")
        hyprink.hyprink_log(ctx, b"success", b"python_example", b"Bindings are working!")

        print("Testing Presets...")
        hyprink.hyprink_log_preset(ctx, b"test_pass", None)
        hyprink.hyprink_log_preset(ctx, b"info", b"Python preset override!")

        # 3. Error Handling Example
        print("\nAttempting to pack a non-existent directory...")
        result = hyprink.hyprink_pack(ctx, b"/path/to/nothing", b"output.pkg")

        if result != 0:
            error_buffer = ctypes.create_string_buffer(1024)
            hyprink.hyprink_get_last_error(ctx, error_buffer, 1024)
            print(f"Caught expected error: {error_buffer.value.decode('utf-8')}")

    finally:
        # 4. Cleanup
        hyprink.hyprink_context_free(ctx)
        print("\nContext freed. Exiting.")

if __name__ == "__main__":
    main()
