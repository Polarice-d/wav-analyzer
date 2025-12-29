<p align="center"><img src="assets/dark.png" alt="FIIS logo">

---
**FIIS** (FFmpeg If It Sucked) is a minimal Rust-based command-line tool for simple digital signal processing. It supports audio effects like delay, gain, softclipping, etc.

## Usage
```bash
fiis [OPTIONS] <FILE_PATH> [EFFECTS]...
```
Effects are written as `"name:arg1=a:arg2=b..."`. a and b are numerical. The order of the arguments doesn't matter.

### Examples

Effects are applied in sequence from left to right.
```bash
fiis path/to/file.wav "softclip:db=10" "peakingeq:db=-10:" "normalize" -o output.wav
```
This will edit the original file.
```bash
fiis path/to/file.wav "gain:db=10" --overwrite
```
Some effects may generate extra audio (like delay and reverb). You can specify the length of these tails in seconds with the `-t, --tail` option.
```bash
fiis path/to/file.wav "delay:wet=1:fb=1.1:time=50" -t 0 --overwrite
```
**Note:** the tool will encode the output file with the same spec as the input file (bit depth, sample format, sample rate).

#### Before
https://github.com/user-attachments/assets/09eb13b8-49a1-45b7-8d40-9fa9b4015f32

#### After `"delay:fb=0.3:time=425.87:wet=1"`
https://github.com/user-attachments/assets/91d9a0f6-9c37-41c0-9daa-7dac91d8d0df

### Supported Effects
| Name | Usage | Details |
| -    | -     | -           |
|**Gain**| `gain:db=x` | Scales the amplitude by `x` dB.|
|**Softclip**| `softclip:db=x`| Applies `x` dB of drive followed by standard `tanh` waveshaping. |
|**Normalize**| `normalize` | Performs peak normalization to 0 dB. Useful for preventing clipping.|
|**Delay**    | `delay:wet=w:fb=y:time=z` | Adds `x`% of wet signal. Feedback specifies the energy scaling `y` on each echo. Time specifies the time between echoes in `z` miliseconds. For feedback values >= 1, the `--tail` option is required. The default maximum tail length is 1 hour. If (for some reason) you want a longer tail you can do so with the `--tail` option. I'm not responsible for out-of-memory crashes.|
|**Peaking EQ** | `peakingeq:db=x:bw=y:freq=z` | Applies a peaking EQ filter with gain `x` across `y` octaves centered at frequency `z`.|
|**Low Shelf and High Shelf EQ** | `lshelfeq/hshelfeq:db=x:s=y:freq=z` | Applies a low/high shelf EQ filter with gain `x` with 'steepness' `y` centered at frequency `z`.|
|**Bandpass EQ** | `bandpasseq:q=x:freq=y` | Applies a bandpass EQ filter at center frequency `y` with 'precision' `x`. |

**These are in development:**
- Algorithmic reverb
- Convolution reverb
- Tools for editing audio (cut, combine, etc.)
- Options to change final sample rate, bit depth, and sample format

The tool is highly modular, so feel free to make your own effects!


## Build from source
Make sure you have `cargo` and `git` installed.
```bash
git clone https://github.com/Polarice-d/fiis
cd fiis
cargo build --release
```
The executable will be in `target/release/`

## Acknowledgments
- Thanks to Robert Bristow-Johnson for creating the Audio EQ Cookbook
- Thanks to the developers of the `Hound`, `CLAP`, `ringbuffer`, `colored`, and `indicativ` libraries! You made my life a lot easier 😅
