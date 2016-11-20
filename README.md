# stm32f411e-rust-blinky

> Example blinky binary implemented with both the
> [stm32f411e-disco](https://github.com/ShaunKarran/stm32f411e-disco) lib and direct memory writes.

# Building and running

First you will need to install a set of tools. Follow these
[installation instructions](https://japaric.github.io/rbr2016/01-installation-instructions/README.html).

Now that you have all the tools, you will need 2 terminal windows. One to run openocd to connect to the board, and another to compile/run gdb.

Terminal 1:
- Run `openocd -f board/stm32f4discovery.cfg` and leave it running.

Terminal 2:

- Change to this project folder. `cd /path/to/project/stm32f411e-rust-blinky`
- Compile using xargo. `xargo build --target thumbv7em-none-eabihf`
- Run gdb. `arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/stm32f411e-rust-blinky`
The .gdbinit file will automatically run the commands to load the program onto the board, set a breakpoint at main and continue to that breakpoint.

The blinky program is now flashed to the board and you can debug.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
