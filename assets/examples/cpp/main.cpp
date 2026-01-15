#include "hyprink.h"
#include <iostream>
#include <vector>

int main() {
  std::cout << "Initializing hyprink Context..." << std::endl;

  // 1. Create Context (loads hyprink.conf)
  HyprinkContext *ctx = hyprink_context_new();
  if (!ctx) {
    std::cerr << "Failed to create context!" << std::endl;
    return 1;
  }

  // Set App Name
  hyprink_context_set_app_name(ctx, "CppExample");

  // 2. Logging Example
  // This will use the hyprink logic from Rust (colors, file logging, etc.)
  hyprink_log(ctx, "info", "cpp_example", "Hello from C++ via FFI!");
  hyprink_log(ctx, "warn", "cpp_example", "This uses the shared library!");

  std::cout << "Testing Presets..." << std::endl;
  hyprink_log_preset(ctx, "test_pass", nullptr);
  hyprink_log_preset(ctx, "info", "Overridden preset message from C++!");

  // 3. Error Handling Example (simulated failure)
  std::cout << "\nAttempting to pack a non-existent directory..." << std::endl;
  int result = hyprink_pack(ctx, "/path/to/nothing", "output.pkg");

  if (result != 0) {
    // Retrieve the error message from Rust
    char error_buffer[1024];
    hyprink_get_last_error(ctx, error_buffer, 1024);
    std::cerr << "Caught expected error: " << error_buffer << std::endl;
  }

  // 4. Cleanup
  hyprink_context_free(ctx);
  std::cout << "\nContext freed. Exiting." << std::endl;

  return 0;
}
