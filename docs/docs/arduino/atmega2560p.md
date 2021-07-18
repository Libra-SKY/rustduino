---
id: atmega2560p
slug: /atmega2560p
title: AtMega2560P
---


----

ATmega2560
([datasheet](http://ww1.microchip.com/downloads/en/DeviceDoc/ATmega4808-4809-Data-Sheet-DS40002173A.pdf))is
a microcontroller featuring the 8-bit AVR® processor with hardware multiplier -
running at up to 20 MHz and with up to 48 KB Flash, 6 KB SRAM and 256 bytes of
EEPROM in 48-pin packages. The series uses the latest Core Independent
Peripherals with low-power features, including an Event System, intelligent
analog, and advanced peripherals.

**Arduino Nano Every** is a tiny powerful board that is based on the
**ATMega2560 AVR** processor. The Arduino Nano Every is almost similar to the
Arduino Nano board with the addition of a more powerful processor like
Atmega4809. This board comes with more program memory compared to Arduino Uno
and RAM is 200% bigger, helping us create a lot of variables.

## Compiling and Linking

```bash
$ cargo +nightly build -Z build-std=core --release --target avr-atmega2560.json
$ cargo +nightly build --release
```

Then, to upload it to a device, assuming that you have avrdude installed, run:

```bash
$ avrdude -v -patmega2560p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega2560p/release/examples/serial.elf:e
```
