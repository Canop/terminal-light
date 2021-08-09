[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/terminal-light.svg
[l1]: https://crates.io/crates/terminal-light

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/terminal-light/badge.svg
[l3]: https://docs.rs/terminal-light/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

# terminal-light

This crate answers the question *"Is the terminal dark or light?"*.

It provides either

* the background color as RGB
* the background color's luma, which varies from 0 (black) to 1 (white)

A use case in a TUI is to determine what set of colors would be more suitable depending on the terminal's background:
```
let should_use_light_skin = terminal_light::luma()
    .map_or(false, |luma| luma > 0.6);
```


