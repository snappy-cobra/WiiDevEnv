# WiiDevEnv

Simple docker setup with a file watch to allow for quick and easy Wii development.

## Requirements

You need to have Docker and Docker-compose installed.

## Usage

Simply call `docker-compose up` in the root folder of this repo.
Changes in the source folder will automatically trigger a new build.
Exit by signalling an interupt to docker-compose (CTRL+C).
