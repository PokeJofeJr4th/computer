# Assembly

### Move

usage:

```
0x0000 MOV &SRC, &DEST
0x0001 MOV $LIT, &DEST
```

load the value at `&SRC` or the literal value `$LIT` into `&DEST`

### Jump

usage:

```
0x0002 JMP &SRC
0x0003 JMP $LIT
```

move the instruction pointer to the value at `&SRC` or literal value `$LIT`

### Add

usage:
```
0x0004 ADD &SRC, &DEST
0x0005 ADD $LIT, &DEST
0x0006 ADD &SRCA, &SRC, &DEST
0x0007 ADD $LIT, &SRC, &DEST
```

add `&SRC`/`$LIT` to `&DEST`; or, store the sum of `&SRCA`/`$LIT` and `&SRC` in `&DEST`
