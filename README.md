# fenestra-template

A [cargo-generate](https://github.com/cargo-generate/cargo-generate)
starter for [fenestra](https://github.com/richer-richard/fenestra) apps:
a themed counter + input + dark-mode toggle, a headless UI test that
drives the app with synthetic input, and CI that runs it — no display
server required.

```sh
cargo install cargo-generate
cargo generate richer-richard/fenestra-template
cd your-app
cargo run    # the window
cargo test   # the UI, verified headlessly
```

Read [AGENTS.md upstream](https://github.com/richer-richard/fenestra/blob/main/AGENTS.md)
for the build → render → look → verify loop, and the
[book](https://richer-richard.github.io/fenestra/book/) for the guided
tour.
