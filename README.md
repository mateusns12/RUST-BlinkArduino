# Blink code for Arduino using RUST



[!https://img.shields.io/badge/Language-RUST-critical?style=for-the-badge&logo=rust]
[!https://img.shields.io/badge/System-Arch_WLS2-A100FF?style=for-the-badge&logo=windows]
[!https://img.shields.io/badge/Board-Arduino_Leonardo-06B6D4?style=for-the-badge&logo=arduino]

Simple blink program, using the arduino_hal crate, by [Rahix](https://github.com/Rahix/avr-hal). Developed in WSL2.

# Notes

- Using nightly-20-12-08.

- Versions above nightly-2021-01-07 won't work. See this [Issue](https://github.com/rust-lang/rust/issues/88252).

- WSL2 doesn't have native support for USB devices, so [usbip-win](https://docs.microsoft.com/en-us/windows/wsl/connect-usb) was used.