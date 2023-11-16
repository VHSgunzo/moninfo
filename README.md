# moninfo
Show info about monitors output in Rust

## To get started:
* **Download the latest revision**
```
git clone https://github.com/VHSgunzo/moninfo.git && cd moninfo
```
* **Compile a binary**
```
rustup default nightly
rustup target add x86_64-unknown-linux-gnu
rustup component add rust-src --toolchain nightly
cargo build --release
```
* Or take an already precompiled binary file from the [releases](https://github.com/VHSgunzo/moninfo/releases)

* **Usage**
```
./moninfo <arg>

no arg            Show all info about monitors output
-m|--mhfsize      Show MangoHud font size for primary monitor
-x|--xrandr       Show monitors config for xrandr
-h|--help         Show this usage info
```
