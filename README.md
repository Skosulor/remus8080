# REMUS 8080

_Rust Emulator for Space 8080_, an emulator for the Intel 8080 processor
written in rust. Originally for emulating space invaders but the goal shifted
to implementing all the instructions instead and making a fun 
dissassembler/debugger.


It is pretty much complete, there are a few things left on the TODO list but 
all the instructions are implemented. 

## Building

requirements: [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html "cargo")

```sh 
cargo build
```

## How to use

```sh
 remus8080 -r <PATH_TO_ROM> -f <CPU_FREQUENCY> 
```

## Arguments

| flag     | short flag | Description                                   |
|----------|------------|-----------------------------------------------|
| --rom    | -r         |  path to rom to run on the emulator           |
| --freq   | -f         |  frequency to run the emulator, default 2 MHZ |
-------------------------------------------------------------------------




