# Assembly

`&XXXX` refers to a pointer; the value in machine code is the memory address. `$XXXX` refers to a literal; the value in machine code is an actual number being used.

### Move

```
HEX   ASM              DESCRIPTION
0000  MOV &SRC, &DEST  &DEST = &SRC
0001  MOV $LIT, &DEST  &DEST = $LT
```

### Jump

```
HEX   ASM
0002  JMP &SRC
0003  JMP $LIT
```

### Add

```
HEX   ASM                     DESCRIPTION
0010  ADD &SRC, &DEST         &DEST += &SRC
0011  ADD $LIT, &DEST         &DEST += $LIT
0012  ADD &SRCA, &SRC, &DEST  &DEST = &SRCA + &SRC
0013  ADD $LIT, &SRC, &DEST   &DEST = &SRC + $LIT

0018  INC &PTR                &PTR++
```

### Subtract

```
HEX   ASM                     DESCRIPTION
0020  SUB &SRC, &DEST         &DEST -= &SRC
0021  SUB $LIT, &DEST         &DEST -= $LIT
0022  SUB &SRCA, &SRC, &DEST  &DEST = &SRC - &SRCA
0023  SUB $LIT, &SRC, &DEST   &DEST = &SRC - $LIT
0024  SUB &SRC, $LIT, &DEST   &DEST = $LIT - &SRC
```

### Multiply

```
HEX   ASM                     DESCRIPTION
0030  MUL &SRC, &DEST         &DEST *= &SRC
0031  MUL $LIT, &DEST         &DEST *= $LIT
0032  MUL &SRCA, &SRC, &DEST  &DEST = &SRCA * &SRC
0033  MUL $LIT, &SRC, &DEST   &DEST = $LIT * &SRC
```
