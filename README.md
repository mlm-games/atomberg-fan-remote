# Atomberg Remote

<div align="center">

**IR Remote Control for Atomberg BLDC Fans**

[![GitHub Release](https://img.shields.io/github/v/release/mlm-games/atomberg-remote?label=GitHub&color=181717)](https://github.com/mlm-games/atomberg-fan-remote/releases/latest)

<!-- Future badges
[![F-Droid](https://img.shields.io/f-droid/v/dev.mlm.atombergremote?label=F-Droid&color=1976d2)](https://f-droid.org/packages/dev.mlm.atombergremote)
[![IzzyOnDroid](https://img.shields.io/endpoint?url=https://apt.izzysoft.de/fdroid/api/v1/shield/dev.mlm.atombergremote&label=IzzyOnDroid)](https://apt.izzysoft.de/fdroid/index/apk/dev.mlm.atombergremote)
-->

</div>

## Features

- **Power & Speed Control** - Full control over fan speed (1-5) and boost mode
- **Timer Functions** - Cycle through timer settings (1h → 2h → 4h → 8h → Off)
- **Sleep Mode** - Activate sleep mode for quiet operation
- **Material You Design** - Slint UI
- **Custom Profiles** - Support for custom IR codes via JSON configuration

## Requirements

⚠️ **Your device must have an IR blaster** to use this app

- Android 6.0+ (API 26)
- IR transmitter hardware (consumerIR)
- Compatible with Atomberg BLDC fans (Renesa+ and similar models)

## Installation

### GitHub Releases

Download the APK for your device architecture:
- `arm64-v8a` - Most modern phones
- `armeabi-v7a` - Older 32-bit devices (might add if requested)
- `x86_64` / `x86` - Emulators and some tablets (similar to above)

[**Download Latest Release**](https://github.com/mlm-games/atomberg-remote/releases/latest)

<!-- Future installation methods
### F-Droid

<a href="https://f-droid.org/packages/dev.mlm.atombergremote">
  <img alt="Get it on F-Droid" height="80" src="https://fdroid.gitlab.io/artwork/badge/get-it-on.png"/>
</a>

### IzzyOnDroid

Add the [IzzyOnDroid repository](https://apt.izzysoft.de/fdroid/) to your F-Droid client or download directly from [IzzyOnDroid](https://apt.izzysoft.de/fdroid/index/apk/dev.mlm.atombergremote)
-->

## Usage

1. Point your phone's IR blaster towards the fan
2. Use the power button to turn on/off
3. Select speed levels 1-5 or boost mode
4. Access timer and sleep functions as needed

## Custom Profiles (Todo/Forking would be better)

Place a `profile.json` file in the app's internal storage to customize IR codes:

```json
{
  "sleep": {
    "type": "nec",
    "address": 61184,
    "command": 23
  },
  "timer_cycle": {
    "type": "raw",
    "pattern": [8880, 4420, 630, ...]
  }
}
```

## Building from Source

```bash
# Prerequisites
cargo install cargo-apk

# Clone and build
git clone https://github.com/mlm-games/atomberg-remote
cd atomberg-remote
cargo apk build --release
```

## Technical Details

- **Framework**: Rust with Slint UI
- **IR Protocol**: NEC 38kHz
- **Supported Commands**: Power, Speed 1-5, Boost, Sleep, Timer

## License

GPL-3.0

---

*Not affiliated with Atomberg Technologies. This is an independent open-source project.*
