# OpenDex setup

The verified Windows scrcpy 4.1 release and its compatible ADB executable are bundled under `tools/scrcpy`, so no separate Windows installation is needed. macOS and Linux builds use their platform-specific `adb` and `scrcpy` binaries from `PATH` unless paths are supplied through Settings.

On the phone, enable Developer options and USB debugging, connect by USB, and accept the RSA fingerprint prompt.

```powershell
npm install
npm run tauri dev
```

For a distributable build:

```powershell
npm run tauri build
```

The frontend is in `src/`; the cross-platform native process adapter is `src-tauri/src/main.rs`. Samsung desktop mode is firmware-dependent: OpenDex makes a best-effort request and always retains normal scrcpy mirroring as a fallback. It does not root, flash, or patch the phone.
