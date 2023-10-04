MOV #msg r1;
MOV #1 &12;
:print_loop
PTRREAD r1 r0;
ADD #1 r1;
YIELD;
JMP #print_loop;
:msg
"Hello, World!"