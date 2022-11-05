# WiiDevEnv

Simple docker setup with a file watch to allow for quick and easy Wii development.

## Requirements

You need to have Docker and Docker-compose installed.

## Usage

Simply call `docker-compose up` in the root folder of this repo.
Changes in the source folder will automatically trigger a new build.
Exit by signalling an interupt to docker-compose (CTRL+C).

## How to contribute

1. Never push directly to `main`.
2. Please branch from `develop` with a branch name like `<your_name>/feature`.
3. Finish your work, be sure it is working and clean.
4. Make a pull-request to merge your branch into `develop`. Somebody needs to approve it.
5. Once merged, delete your branch.

`develop` will be merged into `main` when deemed appropriate.

# License

This project is MIT licensed.

## Credits

Thanks to the [rust-wii](https://github.com/rust-wii) project, whithout their work this repo woudn't be possible.
Thanks to the [Rosalina](https://github.com/ProfElements/rosalina) project, which was a great source of inspiration.
Thanks to [GRRLIB](https://github.com/GRRLIB/GRRLIB) which makes Wii development a bit more fun.
Finally, a big thanks to [DevkitPro](https://github.com/devkitPro), which makes all the homebrew possible in the first place.