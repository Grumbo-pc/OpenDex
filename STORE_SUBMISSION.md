# Microsoft Store submission checklist

OpenDex is designed as a desktop (Win32) application bundled through Tauri. Before submission:

- Build a signed Windows package with a Partner Center-reserved identity and verified publisher certificate.
- Provide `PRIVACY.md` as the public privacy-policy URL/content and a real support contact.
- Explain that ADB and scrcpy are external prerequisites and that the installer does not include Android USB drivers or non-Microsoft driver components.
- Describe the USB/ADB use clearly in the Store listing: a connected Android device and user-approved USB debugging are required.
- Do not claim Samsung DeX works on every device; it is OEM and firmware dependent.
- Test install, launch, uninstall, and update on a clean Windows 10/11 machine.
- Submit accurate screenshots, age rating, category, and accessibility declarations in Partner Center.

OpenDex does not request Windows capability declarations or use cloud data collection. Code signing and Partner Center review are publisher-controlled steps that cannot be completed from the source tree.
