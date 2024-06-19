.export Main
.segment "CODE"

.proc Main

LDX #$45 ; immediate
STX $00 ; STX ZP 
STX $00,Y ; STX ZPY 
STX $1000 ; STX ABS

; time to test addressing modes
LDX #0
LDX $00
LDX #0
LDX $00,Y
LDX #0
LDX $1000
LDX #0
LDX $1000,Y
  rts
.endproc