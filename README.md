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
4. Make a pull-request to merge your branch into `main`. Somebody needs to approve it.
5. Once merged, delete your branch.
