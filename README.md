# herbolution

Herbolution is a 3D voxel game written in Rust.

## Installation

```bash
# Download the source code from GitHub.
git clone https://github.com/peterhenryd/herbolution

# Set working directory to the downloaded repository.
cd herbolution

# To build an executable at target/release/herbolution_client, run:
cargo build --release

# To run the project:
cargo run --release

```

If you don't have Cargo installed, [here](https://rustup.rs) is the official installer.

## Usage

If you decide to run Herbolution, please note that the client will create a `.herbolution` directory in your home directory.

Herbolution has the following (currently hard-coded) controls:

- Use the `W`, `A`, `S` and `D` keys to move forward, left, backward and right.
- Use the space-bar and left-shift key to move up (jump) and down.
- Scroll to change the player's speed.
- Hold left-lick to destroy cubes, and right-click (or left-control left-click) to place stone cubes.
- Press the backtick/`~` key to toggle the debug information display.
- Click on the window with your cursor to focus it, and use the escape key to unfocus it.

## License

Herbolution's source code is licensed under [the MIT license](LICENSE). Herbolution's assets are licensed under
[the CC BY-NC-SA 4.0 license](assets/LICENSE) (this may change in the future).
