This is just a small proof of concept of LED blinking hello world running on
ATmega32U4/Pro Micro/Leonardo micro controller with as little distractions and
complexities as possible.

As AVR support hasn't been fully merged yet, you have to build LLVM
and Rust on your own to get this working. Other than taking a ton of time
to build, and about 17GB of diskspace, it's a fairly effortless exercise.

Documentation on how to prepare the environment can be found [here][1].

A big thanks to all the people have been active over at the
[avr-rust][2] project since last summer. Really fun
to finally taste the fruit of all the hours that have gone into this.

For more details on the ATmega32U4 board, [here are the datasheets][3].

[1]: https://github.com/gergoerdi/rust-avr-chip8-avr/blob/ad28b9676de82925e2d75232da374a866621ff9a/README.md
[2]: https://github.com/avr-rust
[3]: http://www.atmel.com/Images/Atmel-7766-8-bit-AVR-ATmega16U4-32U4_Datasheet.pdf