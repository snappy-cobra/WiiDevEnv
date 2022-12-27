# WiiDevEnv

Docker setup with a file watch auto-build to allow for quick and easy Wii development.

Programming is done in Rust. The `src` folder of this project shows a small demo application.
The Rust environment is pre-configured to get you creating games as soon as possible.

## Requirements

You need to have Docker and [Earthly](https://earthly.dev/get-earthly) installed.

## Usage

### Single build

`earthly +build`

This will create a `boot.elf` in the `bin/` directory.
This file can opened with the Dolphin emulator or put on an SDcard to run on a real (homebrew-enabled) Wii.

### Build watcher

`earthly +build-watch`

Changes in the source folder will automatically trigger a new build.
Exit by signalling an interrupt (Ctrl+C).

### Run tests

#### All tests:
`earthly +test`

#### Unit tests:
`earthly +unit-test`

This will run all (unit) tests in the `./app/lib` subcrate.
In this subcrate, all testing features of Rust are available.
The limitation is that not all functionality of `ogc_rs`/`grrrlib` is.
(Specifically: Only those features for which `ogc_rs` has a drop-in replacement for `std` are.)

#### Integration tests: (This will run on a containerized simulated Wii)
`earthly +integration-test`

These tests are slower, as they will run on a containerized emulated Wii.
(using the Dolphin emulator).

The advantage of these tests is that everything that we can use everything that is available on a real Wii (like rendering, the clock, calls to Grrlib, etc.).
The disadvantage is that we can use nothing else; the normal Rust test flow cannot be used. (The normal test runner requires `std` and `panic = unwind`.)

`earthly +integration-test` will create a `build-test.elf` in the `bin/` directory.
This file can be copied to a real Wii. Tests could be run there, but output can only be inspected if we can read the log output on a real Wii.

### Demo

The source currently contains a small demo of bouncing (overlapping) cubes. Press `1` in the emulator to shake them up!

### Continuous Integration and building details

Earthly is used to make builds and tests more manageable.

#### Re-build base images

The `rust-wii-dev-env` and `dolphin-emu` images are 'base' images which will change very rarely, but take very long to build. (`rust-wii-dev-env` takes ~20min on a M1 mac, `dolphin-emu` takes ~60min on a M1 mac)
As such, rather than keeping them in the normal Earthly flow (which would somethimes trigger a rebuild caused by a burst cache), they are published separately to Docker hub, and those published images are used later on.

The images are 'multi platform' images, meaning that they can be used natively both on amd64 machines (Intel and adjacent) and on arm64 (M1 and adjacent) hardware.

To change/republish them, use `earthly --push +build-env-all-platforms` and `earthly --push +dolphin-all-platforms` respectively.
**This will only work if you are logged in (using `docker login`) and have access to the repository location!**
As such, if you clone the project and want/need to tinker with this, you probably need to replace the mentions of `qqwy/` with `yourownusername/` in the `Earthfile`.

Side note: The final step of the publishing can easily take 15+ minutes in which it seems like all output of earthly/buildkit/docker has frozen. Be patient.

#### Cargo chef

Rust's build tool `cargo` by itself does not support building dependencies separately from the main project code.
In Docker we really want this separation because it can significantly speed up repeated builds.
The plugin [cargo chef](https://github.com/LukeMathWalker/cargo-chef) is used to allow this.
However, the current process is not perfect for nested crates (which we have, such as usage of `app/lib` inside `app/`).

## How to contribute

1. Never push directly to `main`.
2. Please branch from `develop` with a branch name like `<your_name>/feature`.
3. Finish your work, be sure it is working and clean.
4. Make a pull-request to merge your branch into `develop`. Somebody needs to approve it.
5. Once merged, delete your branch.

# License

This project is MIT licensed.

## Credits

Thanks to the [rust-wii](https://github.com/rust-wii) project, without their work this repo wouldn't be possible.
Thanks to the [Rosalina](https://github.com/ProfElements/rosalina) project, which was a great source of inspiration.
Thanks to [GRRLIB](https://github.com/GRRLIB/GRRLIB) which makes Wii development a bit more fun.
Thanks to [Earthly](https://github.com/earthly/earthly) which makes maintaining the complicated build and CI process (almost...) bearable.
Finally, a big thanks to [DevkitPro](https://github.com/devkitPro), which makes all the homebrew possible in the first place.


