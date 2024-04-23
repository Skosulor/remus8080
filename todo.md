# General
    - [X] Brake out debug commands from main
    - [X] Give dissasmbler sane variable & function names
- [x] Add tests
- [X] Acutally set aux flag
- [X] Add Program counter to debugger
- [/] Refactor code in general, lots of code could probably be broken out
- [X] Improve set_instruction function, add cycles and more?
- [X] Rename project, it is no longer about space invaders.. 
- [X] Add more informations to instructions in dissasembler, e.g. RST should also display a nubmer, CPI should show the value that is compared
- [X] Handle breakpoints set on immidiate value instead of instruction
- [X] Add cycle count to debugger
- [X] Add cycle count to each instruction
- [X] Add possibility to run at certain HZ

# Future TODO (i.e. probably never)
- [ ] Pretify instruction matching in instructions.rs, it looks horrible..
- [ ] Handle interrupt -> requires injection of instruction (or skip?)
- [ ] Use instruciton length to update program counter

# Implement Op Codes
- [X] ALL Instructions 

- Space Invaders Instructions
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

