# CHIP-8 Emulator
[![Build](https://github.com/oreisacher/chip8-emulator/actions/workflows/rust.yml/badge.svg)](https://github.com/oreisacher/chip8-emulator/actions/workflows/rust.yml)

Small CHIP-8 Emulator written in Rust. 
The emulator currently implements the original CHIP-8 instruction set and provides keyboard input, sound and display rendering for running ROMs.

Keyboard input is mapped to the original CHIP-8 keypad layout as follows:

| CHIP-8 |   |   |   |     | Keyboard |   |   |   |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| `1` | `2` | `3` | `C` | → | `1` | `2` | `3` | `4` |
| `4` | `5` | `6` | `D` | → | `Q` | `W` | `E` | `R` |
| `7` | `8` | `9` | `E` | → | `A` | `S` | `D` | `F` |
| `A` | `0` | `B` | `F` | → | `Y` | `X` | `C` | `V` |

## Usage
Run a CHIP-8 ROM:

```bash
./chip8 <path/to/rom>
```

## Configuration
On first launch, a default config file is generated. It allows customization of the emulator, including:
- **Display colors** — on/off pixel colors (RGB)
- **CPU speed** — instructions executed per second
- **Quirks** — toggle behavior for CHIP-8 instruction edge cases that vary between original interpreters and later implementations (e.g. `SHIFT`, `LOAD/STORE`, `JUMP`)

## Build from source

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain, includes Cargo)

### System dependencies

**Linux (Debian/Ubuntu):**
```bash
sudo apt install build-essential cmake libglfw3-dev libgl1-mesa-dev pkg-config libasound2-dev libwayland-dev libxkbcommon-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev
```

**Linux (Arch):**
```bash
sudo pacman -S pkg-config alsa-lib cmake base-devel wayland wayland-protocols libxkbcommon libxrandr libxinerama libxcursor libxi
```

### Build
```bash
git clone https://github.com/oreisacher/chip8-emulator.git
cd chip8-emulator
cargo build --release
```
The compiled binary will be at `target/release/chip8`.

## Image Gallery
<img width="1415" height="715" alt="Chip8Gallery" src="https://github.com/user-attachments/assets/dee8b234-63eb-4425-8b7a-666be6dfc8bb" />
