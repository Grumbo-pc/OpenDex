# OpenDex

OpenDex is a Windows desktop client for using an Android phone in a desktop-style window over ADB and scrcpy. It can request Samsung-style desktop mode when the device firmware supports it, while retaining normal scrcpy mirroring as a fallback.

## Features

- USB Android device discovery through ADB
- Samsung desktop-mode request with a configurable virtual display
- Mouse and keyboard forwarding through scrcpy
- Drag-and-drop file transfer to `/sdcard/Download/`
- Configurable resolution, frame rate, bitrate, and executable paths
- Local-only operation with no account, analytics, or cloud service

Samsung desktop mode is device- and firmware-dependent. OpenDex is not affiliated with, sponsored by, or endorsed by Samsung Electronics. The Samsung and DeX names are used only to describe compatibility.

## Requirements

- Windows 10 or Windows 11
- An Android device with Developer options and USB debugging enabled
- A USB connection and an approved Android debugging authorization

The Windows build includes scrcpy 4.1 and compatible ADB files. Do not install or use the bundled tools separately unless you understand their licenses and security implications.

## Development

```powershell
npm install
npm run tauri dev
```

Build the frontend and native application:

```powershell
npm run build
npm run tauri build -- --bundles nsis
```

The NSIS installer is a Win32 distribution artifact. Microsoft Store publication additionally requires a Partner Center submission, publisher identity, signing, Store metadata, privacy URL, and Microsoft certification. This repository does not contain publisher certificates or signing keys.

## Privacy and support

OpenDex stores connection preferences locally and communicates directly with the Android device selected by the user. See the [Privacy Policy](PRIVACY.md). Questions and issue reports belong in the [GitHub issue tracker](https://github.com/Grumbo-pc/OpenDex/issues).

## Third-party software

OpenDex bundles scrcpy and Android platform tools. License and attribution information is available in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) and the bundled `tools/scrcpy/LICENSE.txt` file.

## License

OpenDex source code is distributed by its repository owner. The bundled third-party tools remain under their respective licenses; see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
