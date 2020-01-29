# Game Maker: Studio 2 Modding Tool

GM:S 2 data.win editor/library written in Rust.

## Usage (Rivals of Aether)

### Install

1. Download the zip from the releases
2. Unzip into your Rivals of Aether steam install

### Installing mods

1. Create a `mods` folder inside your Rivals install
2. Place any mods into the `mods/sprites` or `mods/audio` folders
3. Double click `_inject.bat`

### Extracting files

Just double click `_extract.bat`, all the files will now be in the `files` folder

NOTE: the `files` folder and the `mods` folder should have the same structure for your mods to install.

## Build from source

### Requirements
* Have cargo/rustc installed

### Install
```
cargo build --release
````
