# Atomberg Remote

Simple Android IR remote for Atomberg BLDC ceiling fans.

<p align="center">
  <a href="https://github.com/mlm-games/atomberg-fan-remote/releases/latest">
    <img alt="Download on GitHub Releases" height="28"
         src="https://img.shields.io/badge/Download-GitHub%20Releases-24292e?logo=github&logoColor=white">
  </a>
  <!-- <a href="https://f-droid.org/packages/dev.mlm.atombergremote/">
    <img alt="Get it on F-Droid" height="60"
         src="https://fdroid.gitlab.io/artwork/badge/get-it-on.png">
  </a>
  <a href="https://apt.izzysoft.de/fdroid/index/apk/dev.mlm.atombergremote">
    <img alt="Get it on IzzyOnDroid" height="28"
         src="https://img.shields.io/badge/IzzyOnDroid-available-00a3d9?logo=android&logoColor=white&labelColor=1b1f23">
  </a>
  <a href="https://play.google.com/store/apps/details?id=dev.mlm.atombergremote">
    <img alt="Get it on Google Play" height="60"
         src="https://play.google.com/intl/en_us/badges/static/images/badges/en_badge_web_generic.png">
  </a> -->
</p>

<!-- add later; will render automatically when files exist
<p align="center">
  <img src="fastlane/metadata/android/en-US/images/phoneScreenshots/1.png" alt="Screenshot 1" width="24%">
  <img src="fastlane/metadata/android/en-US/images/phoneScreenshots/2.png" alt="Screenshot 2" width="24%">
  <img src="fastlane/metadata/android/en-US/images/phoneScreenshots/3.png" alt="Screenshot 3" width="24%">
  <img src="fastlane/metadata/android/en-US/images/phoneScreenshots/4.png" alt="Screenshot 4" width="24%">
</p>
-->

## Highlights
- Power toggle, Boost, speeds 1–5
- Timer cycle (1h → 2h → 4h → 8h → Off), optional Sleep/LED
- Works fully offline; sends standard NEC ~38 kHz IR patterns
- Clean, minimal UI (Slint)

## Requirements
- Android 8.0+ (API 26+) and a device with a Consumer IR blaster
- Permission: TRANSMIT_IR (no network access)

## Install
- Get the APK from GitHub Releases (linked above).
- F-Droid, IzzyOnDroid, and Google Play links are included for convenience and may become active later.

## Use
1) Point the phone’s IR blaster at the fan’s receiver.  
2) Tap Power or pick a speed 1–5 (Boost sets max speed temporarily).  
3) Timer cycles through preset durations; Sleep/LED appear if supported by your profile.

## Custom profile (advanced)
You can override certain actions with a JSON placed in the app’s files directory (internal storage). This requires adb “run-as” or root on release builds.

Example profile.json:
```json
{
  "sleep": { "type": "raw", "pattern": [8930,4420,580,570, ...] },
  "timer_cycle": { "type": "raw", "pattern": [8880,4420,630,520, ...] }
}
```
Supported keys: sleep, led, timer_1h, timer_2h, timer_4h, timer_cycle.  
Each entry: 
- type "raw" with "pattern": [ints], or 
- type "nec" with "address": u16, "command": u8.

## Build from source
- Prereqs: Rust (toolchain ~1.90), Android SDK/NDK, cargo-apk.
- Target: arm64 (aarch64-linux-android).
- Commands:
```bash
cargo install cargo-apk --locked
cargo apk build --release --target aarch64-linux-android
# APK will be under target/aarch64-linux-android/release/apk/
```

## Notes
- Unofficial project; not affiliated with Atomberg.
- If your device lacks an IR emitter, the app won’t transmit (no error dialogs).
- Ui is slightly skewed to the left in potrait mode due to slint's scroll bar (reserved space)
  
## Support
- Issues: https://github.com/mlm-games/mlm-games-atomberg-fan-remote/issues

## License
See LICENSE. (GPLv3.0-or-later)
