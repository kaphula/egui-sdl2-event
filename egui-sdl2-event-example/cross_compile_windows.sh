# #!/bin/bash
# Edit this to match the SDL2 mingw development files you downloaded and extracted somewhere:
MINGW_SDL2_LIB_DIR=" /home/${USER}/lib/SDL2-2.30.5/x86_64-w64-mingw32/lib/"

# Edit these to match your system's mingw cross-compiler locations:
MINGW_GCC_PATH="/usr/bin/x86_64-w64-mingw32-gcc"
MINGW_AR_PATH="/usr/x86_64-w64-mingw32/bin/ar"
RUSTFLAGS="-L${MINGW_SDL2_LIB_DIR}" CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${MINGW_GCC_PATH}" AR="${MINGW_AR_PATH}" cargo build --release --target x86_64-pc-windows-gnu
