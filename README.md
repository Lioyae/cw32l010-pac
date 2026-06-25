# cw32l010-pac

Peripheral access crate for the CW32L010 microcontroller, generated from the
CW32L010 SVD.

## Blinky example

The `blinky` example targets the CW32L010 Mini Board. The board LED is connected
to `PB0`.

The example:

- enables the GPIOB peripheral clock with the `SYSCTRL.AHBEN` key `0x5A5A`
- configures `PB0` as a digital GPIO
- configures `PB0` as push-pull output
- toggles `PB0` by writing `GPIOB.BSRR`

One important CW32L010 GPIO detail: `DIR = 0` means output, and `DIR = 1` means
input.

Build the example:

```powershell
cargo build --example blinky --features rt
```

Run it with `probe-rs`:

```powershell
cargo run --example blinky --features rt
```
