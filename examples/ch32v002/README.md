# ch32v002-examples

- MCU: CH32V002J4M6 (Should work with other CH32V002x4x6s)
- Tested with WCH-Link v2.20(v40)
- Ubuntu 24.04

## Running Examples

### using rust-nightly for building

call `rustup install nightly` and `rustup override set nightly` to build this project with rust nightly,
which allows to compilation for riscv32ec using the [json-file](riscv32ec-unknown-none-elf.json) provided.

You might need to install the sources for core with `rustup component add rust-src`

### Connection

| WCH-Link | CH32V002J4M6 |
|---|---|
| 5V | PIN4(Vdd) |
| GND | PIN2(Vss) |
| SWIO | PIN8(SWIO) |

#### adc/blinky/embassy_blinky

- Connect LED to PD6(PIN1) with 470ohm that depends on your LED

#### pwm

- pwm output is PC4(PIN7).

#### uart_tx

- Connect your uart RX to TX_(PIN1).

### Using [wlink](https://github.com/ch32-rs/wlink)
- Install wlink using the installation instructions: https://github.com/ch32-rs/wlink?tab=readme-ov-file#install

- Edit the [`.cargo/config.toml`](.cargo/config.toml) file so the runner is `wlink`. This may already be the default runner.

- Build and run the [blinky](src/bin/blinky.rs) example with `cargo run --release --bin blinky`.

### Check the connection

If programming the chip fails, you can check the connection health with `wlink status` command.

```
$ wlink status
02:00:30 [INFO] Connected to WCH-Link v2.20(v40) (WCH-LinkE-CH32V305)
02:00:30 [INFO] Attached chip: CH32V007 [CH32V002J4M6] (ChipID: 0x00240620)
02:00:30 [INFO] Chip ESIG: FlashSize(16KB) UID(cd-ab-56-af-30-bc-53-17)
02:00:30 [INFO] Flash protected: false
02:00:30 [INFO] RISC-V ISA(misa): Some("RV32CEMX")
02:00:30 [INFO] RISC-V arch(marchid): Some("WCH-V2C")
```

It's ok the tool prints CH32V007 but CH32V002. You can see the CH32V002J4M6 follows.

### Using [probe-rs](https://probe.rs/)

- probe-rs does not work for CH32V002 at this time (Apr 2026). The following may work with the future release.

- Install probe-rs using the installation instructions: https://probe.rs/docs/getting-started/installation/
    - If you are on a Linux based system, you may have to add udev rules to allow probe-rs access to the WCH-Link debugger. https://probe.rs/docs/getting-started/probe-setup/#linux%3A-udev-rules

- Edit the [`.cargo/config.toml`](.cargo/config.toml) file so the runner is `probe-rs run --chip ch32v002`.

- Build and run the [blinky](src/bin/blinky.rs) example with `cargo run --release --bin blinky`.
