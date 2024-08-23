# TockOS Configurator

![](docs/demo.gif)

#### How to use

Run the following commands:

```shell
$ cargo install --path configurator # install the configurator binary
$ tock-configurator # run the configuration menu TUI app
```

After configuring the platform, you should see a file named `.config.json`, similar
to this:

```json
{
  "TYPE": "Microbit",
  "CAPSULES": {
    "ALARM": {
      "timer": [
        "Rtc"
      ]
    },
    "CONSOLE": {
      "uart": [
        "Uart0"
      ],
      "baud_rate": 112500
    }
  },
  "SCHEDULER": "Cooperative",
  "PROCESS_COUNT": 0,
  "STACK_SIZE": 2304,
  "SYSCALL_FILTER": "None"
}
```

The crate `tock-generator` deals with parsing the configuration file into
`Rust` code. Add it as a dependency in your crate.

```rust
use tock_generator::{TockMain, Nrf52833};

let tock_main = TockMain::from_json(Nrf52833::default(), ".config.json")?;
tock_main.write_to_file("main.rs")?;
```

**Currently work in progress 🚧**
