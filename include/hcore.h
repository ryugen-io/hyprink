#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Opaque context pointer (wraps HyprConfig with error storage)
struct HCoreContext;

extern "C" {

HCoreContext *hcore_context_new();

void hcore_context_free(HCoreContext *ctx);

/// Copies the last error message into the provided buffer.
/// Returns the number of bytes written (excluding null terminator),
/// or -1 if the buffer was too small or no error exists.
int hcore_get_last_error(HCoreContext *ctx, char *buffer, uintptr_t len);

void hcore_log(HCoreContext *ctx, const char *level, const char *scope, const char *msg);

int hcore_pack(HCoreContext *ctx, const char *src_dir, const char *out_file);

int hcore_unpack(HCoreContext *ctx, const char *pkg_file, const char *target_dir);

int hcore_install(HCoreContext *ctx, const char *path);

}  // extern "C"
