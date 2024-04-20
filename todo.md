#+TITLE: Todo

# General
    - [X] Brake out debug commands from main
    - [X] Give dissasmbler sane variable & function names
- [x] Add tests
- [X] Acutally set aux flag
- [X] Add Program counter to debugger
- [ ] Refactor code in general, lots of code could probably be broken out
- [ ] Pretify instruction matching in instructions.rs, it looks horrible..
- [ ] Improve set_instruction function, add cycles and more?
- [ ] Rename project, it is no longer about space invaders.. 
- [ ] Add functionality to dissasmbler to switch all numbers to hex
- [ ] Add more informations to instructions in dissasembler, e.g. RST should also display a nubmer, CPI should show the value that is compared
- [ ] Add cycle count to debugger
- [x] Add cycle count to each instruction
- [ ] Add possibility to run at certain HZ

# Maybe
- [ ] Rename low_byte & high_byte in instructions to Registers & give them an enum type?
- [ ] Handle interrupt -> requires injection of instruction (or skip?)

# Implement Op Codes
- [X] ALL
- [X] Arithmetic
    - [X] ADD
    - [X] SUB
    - [X] SBB
    - [X] ADC
    - [X] ANA
    - [X] ORA
    - [X] XRA
    - [X] CMP
    - [X] MOV
- [X] Immediate
    - [X] ADI
    - [X] ACI
    - [X] SBI
    - [X] SUI
    - [X] ANI
    - [X] ORI
    - [X] XRI
    - [X] CPI
    - [X] MVI
- [X] Misc
    - [X] LXI
    - [X] DCR
    - [X] DAD
    - [X] RRC
    - [X] INX
    - [X] LXI
    - [X] STA
    - [X] LDA
    - [X] JNZ
    - [X] JMP
    - [X] PUSH
    - [X] RET
    - [X] CALL
    - [X] POP
    - [X] OUT
    - [X] XCHG
    - [X] EI





* Guide
** How to implement new instruction:
1. Add enum for instruction type in _instructions.rs_
2. Match binary value of op code in function _byte_to_op_
3. set the following:
   + self.adress_mode
   + self.inst_type
   + self.low_byte (optionally)
   + self.high_byte (optionally)
   + self.name
4. create function _[name]_op_ in _i8080.rs_ which sets flags and executes OP function
5. In file _i8080.rs_ match the instruction type in function
   _execute_instruction_ and execute the newly created function for the OP
6. Remember to increment the PC correctly


## Dissasembler bindings

| Binding | name       | function                                   |
|---------|------------|--------------------------------------------|
| c       | continue   | run forever                                |
| s       | step       | stops the loaded program and take one step |
| b+      | breakpoint | Set a breakpoint and run                   |
| q       | quit       | Exit emulation                             |

+Due to a unfixed issue, type a space before typing the line number.

