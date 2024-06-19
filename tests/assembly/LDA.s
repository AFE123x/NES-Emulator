.export Main
.segment "CODE"

.proc Main
 ;this program will test all the LDA instructions

 ; addressing modes
LDA #$45    ; Load the immediate value $45 into the accumulator (A)
STA $00     ; Store the value of A into memory address $00
EOR #$45    ; XOR A with the immediate value $45 (A ^ $45), result in A
LDA $00     ; Load the value from zero page address $00 into A
EOR #$45    ; XOR A with the immediate value $45 (A ^ $45), result in A
LDA $00,X   ; Load the value from zero page address $00 + X into A
EOR #$45    ; XOR A with the immediate value $45 (A ^ $45), result in A
LDA $1000   ; Load the value from absolute address $0000 into A
EOR #$45    ; XOR A with the immediate value $45 (A ^ $45), result in A
LDA $0000,X ; Load the value from absolute address $0000 + X into A
EOR #$45    ; XOR A with the immediate value $45 (A ^ $45), result in A
LDA $0000,Y ; Load the value from absolute address $0000 + Y into A
STA ($40,X) ; Store the value of A into the address computed as $40 + X
STA ($40),Y ; Store the value of A into the address computed as $40 + Y
EOR #$45    ; XOR A with the immediate value $45 (A ^ $45), result in A
LDA($40,X) ; Store the value of A into the address computed as $40 + X
EOR #$45    ; XOR A with the immediate value $45 (A ^ $45), result in A
LDA ($40),Y ; Store the value of A into the address computed as $40 + Y

  rts
.endproc