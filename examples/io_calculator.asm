MOV #first_number_buffer &number_buffer;
:get_input
MOV #1 &12;
MOV #number_buffer &string_location;
MOV #read_input &callback_location;
JMP #print;
:read_input
MOV #2 &12;
MOV #input_buffer r3;
:read_input_loop
YIELD;
JEZ r0 #parse;
JLT r0 #30 #read_input;
JGT r0 #39 #read_input;
MOVPTR r0 r3;
ADD #1 r3;
JMP #read_input_loop;
:parse
MOV #1 r1;
SUB #1 r3;
:parse_loop
DEREF r3 r2;
JEZ r2 &second_input;
JLT r2 #30 #parse_loop;
JGT r2 #39 #end;
SUB #30 r2;
MUL r1 r2;
ADD r2 r0;
MUL #A r1;
SUB #1 r3;
JMP #parse_loop;
:second_input
MOV r0 rF;
MOV #math &second_input;
MOV #second_number_buffer &number_buffer;
JMP #get_input;
:math
MOV r0 rE;
MOV #1 &12;
MOV #operaton_buffer &string_location;
MOV #math_loop &callback_location;
JMP #print
:math_loop
MOV #2 &12;
YIELD;
JEZ r0 #end;
JEQ r0 #2B #add;
JEQ r0 #2D #sub;
JEQ r0 #2A #mul;
JMP #math_loop;
:add
ADD rE rF;
JMP #after_math;
:sub
SUB rE rF;
JMP #after_math;
:mul
MUL rE rF;
:after_math
YIELD;
JNZ r0 #after_math;
:print_output
MOV #1 &12;
MOV rF r2;
MOV #pow_10 &pow_10;
:print_output_loop
MOV &pow_10 r1;
ADD #1 &pow_1;
MOV #30 r0;
:digit_loop
JLT r2 r1 #after_digit;
SUB r1 r2
ADD #1 r0
JLE r1 r2 #digit_loop
:after_digit
YIELD;
JGT r1 #1 #print_output_loop;
MOV #0 r0;
YIELD;
MOV rF r0;
JMP #second_input;
:end
YIELD;
MOV #0 r0;
JMP #end;
:print
MOV #1 &12;
:print_loop
DEREF &string_location r0;
ADD #1 &string_location;
YIELD;
JNZ r0 #print_loop;
JMP &callback_location;
:string_location
MOV r0 r0;
:callback_location
MOV r0 r0;
:second_input
MOV r0 r0;
:number_buffer
MOV r0 r0;
:input_buffer
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
:first_number_buffer
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
:second_number_buffer
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
:operation_buffer
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;
MOV r0 r0;