# Assembly

`&XXXX` refers to a pointer; the value in machine code is the memory address. `#XXXX` refers to a literal; the value in machine code is an actual number being used.

first nibble: operation

- 0: MOV/JMP (remaining nibbles special)
- 1: ADD
- 2: SUB
- 3: MUL
- 4: EQ
- 5: NE
- 6: LT
- 7: LE
- 8: GT
- 9: GE
- A: NOT
- B: AND
- C: OR
- D: XOR
- E: SHL
- F: SHR

second nibble: mode

- 0: &SRC, &DEST (&DEST += &SRC)
- 0 (CMP): J__ &SRC, &SRCA, &DST (if &SRC > &SRCA)
- 1: #LIT, &DEST (&DEST += #LIT)
- 1 (CMP): J__ #LIT, &SRC, &DST (if #LIT > &SRC)
- 2: &SRCA, &SRC, &DEST (&DEST = &SRCA + &SRC)
- 3: #LIT, &SRC, &DEST (&DEST = &SRC + #LIT)
- 4: &SRC, #LIT, &DEST (&DEST = #LIT + &SRC; only when order matters)
- 5 (CMP): J__ &SRC, #LIT, &DST (if &SRC > #LIT)
- 6: ?
- 7: ?
- 8: ?
- 9: ?
- A: ?
- B: ?
- C: third nibble is mode, fourth nibble is first arg
- D: third nibble is mode, fourth nibble is second arg
- E: third nibble is mode, fourth nibble is third arg
- F: third nibble is mode, fourth nibble unused

third nibble: first arg / fourth nibble: second arg

- literal from 0-15
- one of 16 predefined registers (r0-rF/r15)

### MOV / JMP

second nibble: mode

- 0: MOV &SRC, &DEST
- 1: MOV #LIT, &DEST
- 2: SWP &SRC, &DEST
- 3: JMP &SRC
- 4: JMP #LIT
- 5: JEZ &CND &SRC
- 6: JEZ &CND #LIT
- 7: JNZ &CND &SRC
- 8: JNZ &CND #LIT
- 9: ?
- A: ?
- B: ?
- C: ?
- D: third nibble is mode, fourth nibble is first arg
- E: third nibble is mode, fourth nibble is second arg
- F: third nibble is mode, fourth nibble unused

third nibble: first arg / fourth nibble: second arg
