<p align="center"><img src="images/dark.png" alt="FIIS logo">

---
### What is it?
**FIIS** (FFmpeg If It Sucked) is a Rust-based command-line tool for simple digital signal processing. It supports audio effects like delay, gain, softclipping, etc.

## Usage
```bash
fiis [OPTIONS] <FILE_PATH> [EFFECTS]...
```
Effects are written as `"name:arg1=a:arg2=b..."`. a and b are numerical. The order of the arguments doesn't matter.

**Note:** The tool will complain if an effect is missing arguments, but it will not say anything if you include *extra* arguments.

### Examples
```bash
fiis path/to/file.wav "softclip:db=10" "delay:mix=0.85:feedback=0.6:time=200" "normalize" -o output.wav
```
Effects are applied sequentially, so **order matters**.

```bash
fiis path/to/file.wav "gain:db=10" --overwrite
```
This will edit the original file.

```bash
fiis path/to/file.wav "delay:mix=1:feedback=1.1:time=50" -t 0
```
Some effects may generate extra audio (like delay and reverb). You can specify the length of these tails in seconds with the `-t` option.

### Build from source
Make sure you have `cargo` and `git` installed.
```bash
git clone https://github.com/Polarice-d/fiis
cd fiis
cargo build --release
