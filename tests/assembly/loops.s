.export Main
.segment "CODE"

.proc Main


  lda #0
  ldx #0
  MUL3:
    ADC #10
    INX
    CPX #3
    BNE MUL3
  RTS
  ; Initialize each monster's HP
  lda #100
  ldx #7
initialize_hp_loop:
  sta $0300, x
  dex
  bpl initialize_hp_loop

  ; Make a couple changes to test the logic
  lda #0
  sta $0301
  lda #15
  sta $0306

  ; Perform the multi-attack
  ldx #0
multi_attack_loop:
  lda $0300, x
  sec
  sbc #50
  bpl store_hp
  lda #0
store_hp:
  sta $0300, x
  inx
  cpx #8
  bne multi_attack_loop

  ; Loop compete, return from subroutine
  rts

FUNCTION:


.endproc