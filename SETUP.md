The verified Windows scrcpy 4.1 release is bundled under `tools/scrcpy`. The Windows release does not install or redistribute an Android USB driver or ADB USB transport DLLs. Install Android Platform Tools separately, add `adb.exe` to `PATH`, or supply its path in Settings. macOS and Linux builds use their platform-specific `adb` and `scrcpy` binaries from `PATH` unless paths are supplied through Settings.
# OpenDex setup

Windows releases do not bundle ADB or scrcpy. Download Android Platform-Tools from https://developer.android.com/tools/releases/platform-tools and the latest Windows scrcpy release from https://github.com/Genymobile/scrcpy/releases. Extract both ZIP files, then add their folders to `PATH`, or set the full `adb.exe` and `scrcpy.exe` paths in OpenDex Settings. Install any phone manufacturer's USB driver separately if Windows does not recognize the device.

On the phone, enable Developer options and USB debugging, connect by USB, and accept the RSA fingerprint prompt.

```powershell
npm install
npm run tauri dev
```

For a distributable build:

```powershell
npm run tauri build
```

The Windows NSIS installers support silent installation with `/S`. For example:

```powershell
OpenDex_0.1.0_x64-setup.exe /S
OpenDex_0.1.0_arm64-setup.exe /S
```

Use `/D=C:\Program Files\OpenDex` to choose a silent-install destination. Install Android Platform Tools separately before launching OpenDex; the app explains how to download and configure `adb.exe` in Settings.

The frontend is in `src/`; the cross-platform native process adapter is `src-tauri/src/main.rs`. Samsung desktop mode is firmware-dependent: OpenDex makes a best-effort request and always retains normal scrcpy mirroring as a fallback. It does not root, flash, or patch the phone.
