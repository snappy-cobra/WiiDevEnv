# WiiDevEnv

Docker setup with a file watch auto-build to allow for quick and easy Wii development.

Programming is done in Rust. The `src` folder of this project shows a small demo application.
The Rust environment is pre-configured to get you creating games as soon as possible.

## Requirements

You need to have Docker and Docker-compose installed.

## Usage

Simply call `docker-compose up`.
Changes in the source folder will automatically trigger a new build.
Exit by signalling an interrupt to docker-compose (CTRL+C).

### Demo

The source currently contains a small demo of bouncing (overlapping) cubes. Press `1` in the emulator to shake them up!

## Tests

### Unit tests

The unit test suite can be run by going to the `app/lib` directory and running

```
cargo +nightly test --target=x86_64-unknown-linux-gnu
```

or

```
cargo +nightly test --target=aarch64-apple-darwin
```

depending on your native computer architecture.

This will run all (unit) tests in the `./app/lib` subcrate.
In this subcrate, all testing features of Rust are available.
The limitation is that not all functionality of `ogc_rs`/`grrrlib` is.

### On-target Integration Tests

The build process will create a `boot.elf` and a `boot-test.elf`.
The latter is a binary that will run the integration test suite on the target console.

The advantage of these tests is that everything that we can use everything that is available on a real Wii (like rendering, the clock, calls to Grrlib, etc.).
The disadvantage is that we can use nothing else; the normal Rust test flow cannot be used. (It requires `std` and `panic = unwind`.)


Hypothetically these tests might be run a real Wii (if standard output could be read),
but more likely this is done using the Dolphin emulator.

To run the target tests on the Dolphin emulator, the following command can be used:

```bash
timeout 5s path/to/dolphin --batch --exec=./bin/boot-test.elf 2>&1 | grep "OSREPORT_HLE"
```

On success, the exit code will be `0` and the output will look like:

```
54:40:704 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b08->80022764| Running the target test suite...
54:40:708 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b08->8001ffc8| Running tests...
54:40:720 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b08->80021a3c| Trivial test ...
54:40:720 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b08->8000d95c| Trivial test ... ok
54:40:721 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b08->80021b60| Test run successful!
```

On failure, the exit code will be nonzero and the output will look like:

```
53:47:386 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b74->800227d0| Running the target test suite...
53:47:390 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b74->80020034| Running tests...
54:40:402 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b08->80021a3c| Trivial test ...
54:40:402 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b08->8000d95c| Trivial test ... ok
53:47:402 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b74->80013608| Problematic test ...
53:47:403 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b74->80020670| #######################################
53:47:404 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b74->800216b0| # <[ PANIC ]> panicked at 'assertion failed: 1 == 0', src/target_tests/mod.rs:17:9
53:47:405 Core/HLE/HLE_OS.cpp:82 N[OSREPORT_HLE]: 80020b74->800206f4| #######################################
```

_Dolphin hangs on panic. The `timeout` command is used to transform this in a non-zero exit. If the test suite becomes large/slow, the timeout might need to be made longer._


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
Finally, a big thanks to [DevkitPro](https://github.com/devkitPro), which makes all the homebrew possible in the first place.

