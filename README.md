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

## How to run the emulator

When the emulator is ran in a terminal, a tui debugger/dissasembler will open up. 
It will not execute any instructions until either a _[s]tep_ or _[c]ontinue_ 
command is issued. 

It has support for breakpoints and reading any memory address.

```sh
 remus8080 -r <PATH_TO_ROM> -f <CPU_FREQUENCY> 
```

### Arguments

| flag     | short flag | Description                                   |
|----------|------------|-----------------------------------------------|
| --rom    | -r         |  path to rom to run on the emulator           |
| --freq   | -f         |  frequency to run the emulator, default 2 MHZ |
-------------------------------------------------------------------------

## Using the dissassembler/debugger

List of commands that can be issued to the debugger. The TUI will update during
stepping but not will the emulator is in "run" which occurs after a _[c]ontinue_ 
command is issued.

| Command    | Description                                        |
|----------  |----------------------------------------------------|
| s [N]      | Step - execute N instructions                      |
| q          | Quit                                               |
| b [N]      | set Breakpoint at instruction N                    |
| c          | Continue until breakpoint                          |
| m [N]      | update Memory field to start at adress N (decimal) |
-------------------------------------------------------------------





