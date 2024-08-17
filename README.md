# rp235x-pac - PAC for Raspberry Pi RP235x microcontrollers

This is a [Peripheral Access Crate] for the Raspberry Pi [RP235x] dual-core
Cortex-M33 / RISC-V microcontroller.

[Peripheral Access Crate]: https://rust-embedded.github.io/book/start/registers.html
[RP235x]: https://datasheets.raspberrypi.org/rp2350/rp2350-datasheet.pdf

This crate has been built using [svd2rust] version 0.33.4 and [svdtools], using
the SVD file in the [pico-sdk v2.0.0].

[svd2rust]: https://github.com/rust-embedded/svd2rust
[svdtools]: https://github.com/stm32-rs/svdtools
[pico-sdk v2.0.0]: https://github.com/raspberrypi/pico-sdk/blob/2.0.0/src/rp2350/hardware_regs/RP2350.svd

## Licence

The contents of this crate are auto-generated and licensed under the same terms
as the underlying SVD file, which is licensed by Raspberry Pi (Trading)) Ltd
under a BSD-3-Clause licence.

## Changelog

See the [CHANGELOG.md file]

[CHANGELOG.md file]: ./CHANGELOG.md
