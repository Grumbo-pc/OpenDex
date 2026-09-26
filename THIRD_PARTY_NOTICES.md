OpenDex bundles **scrcpy 4.1** and its user-mode Windows runtime dependencies, including SDL, FFmpeg libraries, and libusb. scrcpy is provided by Genymobile under the Apache License 2.0. The exact scrcpy license is retained at `tools/scrcpy/LICENSE.txt` and must ship with every distribution.
OpenDex does not bundle Android Platform Tools, Android USB drivers, or ADB USB transport DLLs. Users must obtain and maintain ADB and any device-specific USB driver through their normal Android device support channel.
# Third-party notices

OpenDex release installers do not bundle ADB, scrcpy, Android USB drivers, or their runtime libraries. Users download and maintain those components separately from the official Android Platform-Tools and Genymobile scrcpy release pages.

The `tools/scrcpy` directory is retained for local development only and is not included in release installers.

