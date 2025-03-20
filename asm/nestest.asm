; PRG CRC32 checksum: 7c5060f0
; CHR CRC32 checksum: 6dd12df7
; Overall CRC32 checksum: 158b0388
; Code base address: $c000

.setcpu "6502x"
.segment "HEADER"

.byte "NES", $1a                 ; Magic string that always begins an iNES header
.byte $01                        ; Number of 16KB PRG-ROM banks
.byte $01                        ; Number of 8KB CHR-ROM banks
.byte $00                        ; Control bits 1
.byte $00                        ; Control bits 2
.byte $00                        ; Number of 8KB PRG-RAM banks
.byte $00                        ; Video format NTSC/PAL

APU_FRAME = $4017
APU_PL1_HI = $4003
APU_PL1_LO = $4002
APU_PL1_SWEEP = $4001
APU_PL1_VOL = $4000
APU_PL2_HI = $4007
APU_PL2_LO = $4006
APU_PL2_SWEEP = $4005
APU_PL2_VOL = $4004
APU_SND_CHN = $4015
JOYPAD1 = $4016
JOYPAD2 = $4017
PPU_ADDR = $2006
PPU_CTRL = $2000
PPU_DATA = $2007
PPU_MASK = $2001
PPU_SCROLL = $2005
PPU_STATUS = $2002


_var_0000_indexed = $0000
_var_0001 = $0001
_var_0005 = $0005
_var_0010_indexed = $0010
_var_0020 = $0020
_var_0033_indexed = $0033
_var_0034 = $0034
_var_0040_indexed = $0040
_var_0043_indexed = $0043
_var_0044 = $0044
_var_0045_indexed = $0045
_var_0046 = $0046
_var_0047 = $0047
_var_0048_indexed = $0048
_var_0049_indexed = $0049
_var_004a_indexed = $004A
_var_004f = $004F
_var_0050_indexed = $0050
_var_0056 = $0056
_var_0067 = $0067
_var_0068 = $0068
_var_0069_indexed = $0069
_var_0078 = $0078
_var_007f = $007F
_var_0080_indexed = $0080
_var_0082_indexed = $0082
_var_0085_indexed = $0085
_var_0089_indexed = $0089
_var_0097_indexed = $0097
_var_0098 = $0098
_var_00d0_indexed = $00D0
_var_00d1 = $00D1
_var_00d2 = $00D2
_var_00d3 = $00D3
_var_00d4 = $00D4
_var_00d5 = $00D5
_var_00d6 = $00D6
_var_00d7 = $00D7
_var_00d8 = $00D8
_var_00e6_indexed = $00E6
_var_00f9_indexed = $00F9
_var_00ff_indexed = $00FF
_var_0146_indexed = $0146
_var_0200 = $0200
_var_0201 = $0201
_var_0245 = $0245
_var_0300_indexed = $0300
_var_0303 = $0303
_var_0400_indexed = $0400
_var_0432 = $0432
_var_0489 = $0489
_var_0548_indexed = $0548
_var_0549 = $0549
_var_0556 = $0556
_var_0557_indexed = $0557
_var_0577 = $0577
_var_0578 = $0578
_var_0580_indexed = $0580
_var_0585_indexed = $0585
_var_05ff_indexed = $05FF
_var_0600_indexed = $0600
_var_0633_indexed = $0633
_var_0647 = $0647
_var_0678 = $0678
_var_0689 = $0689
_var_07ff = $07FF


.segment "CODE"

.byte $4c, $f5, $c5, $60         ; $C000

Reset:
  sei                            ; $C004  78
  cld                            ; $C005  D8
  ldx #$FF                       ; $C006  A2 FF
  txs                            ; $C008  9A

_label_c009:
  lda PPU_STATUS                 ; $C009  AD 02 20
  bpl _label_c009                ; $C00C  10 FB

_label_c00e:
  lda PPU_STATUS                 ; $C00E  AD 02 20
  bpl _label_c00e                ; $C011  10 FB
  lda #$00                       ; $C013  A9 00
  sta PPU_CTRL                   ; $C015  8D 00 20
  sta PPU_MASK                   ; $C018  8D 01 20
  sta PPU_SCROLL                 ; $C01B  8D 05 20
  sta PPU_SCROLL                 ; $C01E  8D 05 20
  lda PPU_STATUS                 ; $C021  AD 02 20
  ldx #$20                       ; $C024  A2 20
  stx PPU_ADDR                   ; $C026  8E 06 20
  ldx #$00                       ; $C029  A2 00
  stx PPU_ADDR                   ; $C02B  8E 06 20
  ldx #$00                       ; $C02E  A2 00
  ldy #$0F                       ; $C030  A0 0F
  lda #$00                       ; $C032  A9 00

_label_c034:
  sta PPU_DATA                   ; $C034  8D 07 20
  dex                            ; $C037  CA
  bne _label_c034                ; $C038  D0 FA
  dey                            ; $C03A  88
  bne _label_c034                ; $C03B  D0 F7
  lda #$3F                       ; $C03D  A9 3F
  sta PPU_ADDR                   ; $C03F  8D 06 20
  lda #$00                       ; $C042  A9 00
  sta PPU_ADDR                   ; $C044  8D 06 20
  ldx #$00                       ; $C047  A2 00

_label_c049:
  lda a:_data_ff78_indexed,X     ; $C049  BD 78 FF
  sta PPU_DATA                   ; $C04C  8D 07 20
  inx                            ; $C04F  E8
  cpx #$20                       ; $C050  E0 20
  bne _label_c049                ; $C052  D0 F5
  lda #$C0                       ; $C054  A9 C0
  sta APU_FRAME                  ; $C056  8D 17 40
  lda #$00                       ; $C059  A9 00
  sta APU_SND_CHN                ; $C05B  8D 15 40
  lda #$78                       ; $C05E  A9 78
  sta z:_var_00d0_indexed        ; $C060  85 D0
  lda #$FB                       ; $C062  A9 FB
  sta z:_var_00d1                ; $C064  85 D1
  lda #$7F                       ; $C066  A9 7F
  sta z:_var_00d3                ; $C068  85 D3
  ldy #$00                       ; $C06A  A0 00
  sty PPU_ADDR                   ; $C06C  8C 06 20
  sty PPU_ADDR                   ; $C06F  8C 06 20

_label_c072:
  lda #$00                       ; $C072  A9 00
  sta z:_var_00d7                ; $C074  85 D7
  lda #$07                       ; $C076  A9 07
  sta z:_var_00d0_indexed        ; $C078  85 D0
  lda #$C3                       ; $C07A  A9 C3
  sta z:_var_00d1                ; $C07C  85 D1
  jsr _func_c2a7                 ; $C07E  20 A7 C2

_label_c081:
  jsr _func_c28d                 ; $C081  20 8D C2
  ldx #$12                       ; $C084  A2 12
  jsr _func_c261                 ; $C086  20 61 C2
  lda z:_var_00d5                ; $C089  A5 D5
  lsr a                          ; $C08B  4A
  lsr a                          ; $C08C  4A
  lsr a                          ; $C08D  4A
  bcs _label_c0ac                ; $C08E  B0 1C
  lsr a                          ; $C090  4A
  bcs _label_c09f                ; $C091  B0 0C
  lsr a                          ; $C093  4A
  bcs _label_c0bd                ; $C094  B0 27
  lsr a                          ; $C096  4A
  bcs _label_c09c                ; $C097  B0 03
  jmp _label_c081                ; $C099  4C 81 C0

_label_c09c:
  jmp _label_c126                ; $C09C  4C 26 C1

_label_c09f:
  jsr _func_c66f                 ; $C09F  20 6F C6
  dec z:_var_00d7                ; $C0A2  C6 D7
  bpl _label_c081                ; $C0A4  10 DB
  lda #$0D                       ; $C0A6  A9 0D
  sta z:_var_00d7                ; $C0A8  85 D7
  bne _label_c081                ; $C0AA  D0 D5

_label_c0ac:
  jsr _func_c66f                 ; $C0AC  20 6F C6
  inc z:_var_00d7                ; $C0AF  E6 D7
  lda z:_var_00d7                ; $C0B1  A5 D7
  cmp #$0E                       ; $C0B3  C9 0E
  bcc _label_c081                ; $C0B5  90 CA
  lda #$00                       ; $C0B7  A9 00
  sta z:_var_00d7                ; $C0B9  85 D7
  beq _label_c081                ; $C0BB  F0 C4

_label_c0bd:
  jsr _func_c689                 ; $C0BD  20 89 C6
  lda z:_var_00d7                ; $C0C0  A5 D7
  beq _label_c0ca                ; $C0C2  F0 06
  jsr _jump_engine_c0ed          ; $C0C4  20 ED C0

.byte $4c, $81, $c0              ; $C0C7

_label_c0ca:
  lda #$00                       ; $C0CA  A9 00
  sta z:_var_00d8                ; $C0CC  85 D8
  inc z:_var_00d7                ; $C0CE  E6 D7
  jsr _jump_engine_c0ed          ; $C0D0  20 ED C0

  .word _label_d7e6              ; $C0D3  E6 D7
  .word _label_d7a5              ; $C0D5  A5 D7
.byte $c9, $0e, $d0, $f5, $a9, $00, $85, $d7, $a5, $d8, $f0, $02, $a9, $ff, $85, $00 ; $C0D7
.byte $20, $ed, $c1, $4c, $81, $c0 ; $C0E7

_jump_engine_c0ed:               ; jump engine detected
  lda z:_var_00d7                ; $C0ED  A5 D7
  asl a                          ; $C0EF  0A
  tax                            ; $C0F0  AA
  lda a:_jump_table_c10a,X       ; $C0F1  BD 0A C1
  sta a:_var_0200                ; $C0F4  8D 00 02
  lda a:_jump_table_c10a+1,X     ; $C0F7  BD 0B C1
  sta a:_var_0201                ; $C0FA  8D 01 02
  lda #$C1                       ; $C0FD  A9 C1
  pha                            ; $C0FF  48
  lda #$DE                       ; $C100  A9 DE
  pha                            ; $C102  48
  lda #$00                       ; $C103  A9 00
  sta z:_var_0000_indexed        ; $C105  85 00
  jmp (_var_0200)                ; $C107  6C 00 02

_jump_table_c10a:
  .word _label_c72d              ; $C10A  2D C7
  .word _label_c72d              ; $C10C  2D C7
  .word _label_c7db              ; $C10E  DB C7
  .word _label_c885              ; $C110  85 C8
  .word _label_cbde              ; $C112  DE CB
  .word _label_cdf8              ; $C114  F8 CD
  .word _label_ceee              ; $C116  EE CE
  .word _label_cfa2              ; $C118  A2 CF
  .word _label_d174              ; $C11A  74 D1
  .word _label_d4fb              ; $C11C  FB D4
  .word _label_c1d4              ; $C11E  D4 C1
  .word _label_df4a              ; $C120  4A DF
  .word _label_dbb8              ; $C122  B8 DB
  .word _label_e1aa              ; $C124  AA E1

_label_c126:
  lda #$00                       ; $C126  A9 00
  sta z:_var_00d7                ; $C128  85 D7
  lda #$92                       ; $C12A  A9 92
  sta z:_var_00d0_indexed        ; $C12C  85 D0
  lda #$C4                       ; $C12E  A9 C4
  sta z:_var_00d1                ; $C130  85 D1
  jsr _func_c2a7                 ; $C132  20 A7 C2

_label_c135:
  jsr _func_c28d                 ; $C135  20 8D C2
  ldx #$0F                       ; $C138  A2 0F
  jsr _func_c261                 ; $C13A  20 61 C2
  lda z:_var_00d5                ; $C13D  A5 D5
  lsr a                          ; $C13F  4A
  lsr a                          ; $C140  4A
  lsr a                          ; $C141  4A
  bcs _label_c160                ; $C142  B0 1C
  lsr a                          ; $C144  4A
  bcs _label_c153                ; $C145  B0 0C
  lsr a                          ; $C147  4A
  bcs _label_c171                ; $C148  B0 27
  lsr a                          ; $C14A  4A
  bcs _label_c150                ; $C14B  B0 03
  jmp _label_c135                ; $C14D  4C 35 C1

_label_c150:
  jmp _label_c072                ; $C150  4C 72 C0

_label_c153:
  jsr _func_c66f                 ; $C153  20 6F C6
  dec z:_var_00d7                ; $C156  C6 D7
  bpl _label_c135                ; $C158  10 DB
  lda #$0A                       ; $C15A  A9 0A
  sta z:_var_00d7                ; $C15C  85 D7
  bne _label_c135                ; $C15E  D0 D5

_label_c160:
  jsr _func_c66f                 ; $C160  20 6F C6
  inc z:_var_00d7                ; $C163  E6 D7
  lda z:_var_00d7                ; $C165  A5 D7
  cmp #$0B                       ; $C167  C9 0B
  bcc _label_c135                ; $C169  90 CA
  lda #$00                       ; $C16B  A9 00
  sta z:_var_00d7                ; $C16D  85 D7
  beq _label_c135                ; $C16F  F0 C4

_label_c171:
  jsr _func_c689                 ; $C171  20 89 C6
  lda z:_var_00d7                ; $C174  A5 D7
  beq _label_c17e                ; $C176  F0 06
  jsr _jump_engine_c1a1          ; $C178  20 A1 C1

.byte $4c, $35, $c1              ; $C17B

_label_c17e:
  lda #$00                       ; $C17E  A9 00
  sta z:_var_00d8                ; $C180  85 D8
  inc z:_var_00d7                ; $C182  E6 D7
  jsr _jump_engine_c1a1          ; $C184  20 A1 C1

  .word _label_d7e6              ; $C187  E6 D7
  .word _label_d7a5              ; $C189  A5 D7
.byte $c9, $0b, $d0, $f5, $a9, $00, $85, $d7, $a5, $d8, $f0, $02, $a9, $ff, $85, $00 ; $C18B
.byte $20, $ed, $c1, $4c, $35, $c1 ; $C19B

_jump_engine_c1a1:               ; jump engine detected
  lda z:_var_00d7                ; $C1A1  A5 D7
  asl a                          ; $C1A3  0A
  tax                            ; $C1A4  AA
  lda a:_jump_table_c1be,X       ; $C1A5  BD BE C1
  sta a:_var_0200                ; $C1A8  8D 00 02
  lda a:_jump_table_c1be+1,X     ; $C1AB  BD BF C1
  sta a:_var_0201                ; $C1AE  8D 01 02
  lda #$C1                       ; $C1B1  A9 C1
  pha                            ; $C1B3  48
  lda #$DE                       ; $C1B4  A9 DE
  pha                            ; $C1B6  48
  lda #$00                       ; $C1B7  A9 00
  sta z:_var_0000_indexed        ; $C1B9  85 00
  jmp (_var_0200)                ; $C1BB  6C 00 02

_jump_table_c1be:
  .word _label_c6a3              ; $C1BE  A3 C6
  .word _label_c6a3              ; $C1C0  A3 C6
  .word _label_e51e              ; $C1C2  1E E5
  .word _label_e73d              ; $C1C4  3D E7
  .word _label_e8d3              ; $C1C6  D3 E8
  .word _label_e916              ; $C1C8  16 E9
  .word _label_eb86              ; $C1CA  86 EB
  .word _label_edf6              ; $C1CC  F6 ED
  .word _label_f066              ; $C1CE  66 F0
  .word _label_f2d6              ; $C1D0  D6 F2
  .word _label_f546              ; $C1D2  46 F5

_label_c1d4:
  lda #$00                       ; $C1D4  A9 00
  sta z:_var_0000_indexed        ; $C1D6  85 00
  jsr _func_d900                 ; $C1D8  20 00 D9
  jsr _func_dae0                 ; $C1DB  20 E0 DA
  nop                            ; $C1DE  EA
  nop                            ; $C1DF  EA
  nop                            ; $C1E0  EA
  lda z:_var_0000_indexed        ; $C1E1  A5 00
  beq _label_c1e7                ; $C1E3  F0 02
  sta z:_var_00d8                ; $C1E5  85 D8

_label_c1e7:
  jmp _label_c1ed                ; $C1E7  4C ED C1

.byte $4c, $81, $c0              ; $C1EA

_label_c1ed:
  jsr _func_c28d                 ; $C1ED  20 8D C2
  lda #$00                       ; $C1F0  A9 00
  sta z:_var_00d3                ; $C1F2  85 D3
  lda z:_var_00d7                ; $C1F4  A5 D7
  clc                            ; $C1F6  18
  adc #$04                       ; $C1F7  69 04
  asl a                          ; $C1F9  0A
  rol z:_var_00d3                ; $C1FA  26 D3
  asl a                          ; $C1FC  0A
  rol z:_var_00d3                ; $C1FD  26 D3
  asl a                          ; $C1FF  0A
  rol z:_var_00d3                ; $C200  26 D3
  asl a                          ; $C202  0A
  rol z:_var_00d3                ; $C203  26 D3
  asl a                          ; $C205  0A
  rol z:_var_00d3                ; $C206  26 D3
  pha                            ; $C208  48
  lda z:_var_00d3                ; $C209  A5 D3
  ora #$20                       ; $C20B  09 20
  sta PPU_ADDR                   ; $C20D  8D 06 20
  pla                            ; $C210  68
  ora #$04                       ; $C211  09 04
  sta PPU_ADDR                   ; $C213  8D 06 20
  lda z:_var_0000_indexed        ; $C216  A5 00
  beq _label_c237                ; $C218  F0 1D
  cmp #$FF                       ; $C21A  C9 FF
  beq _label_c244                ; $C21C  F0 26
  lsr a                          ; $C21E  4A
  lsr a                          ; $C21F  4A
  lsr a                          ; $C220  4A
  lsr a                          ; $C221  4A
  tax                            ; $C222  AA
  lda a:_data_c251_indexed,X     ; $C223  BD 51 C2
  sta PPU_DATA                   ; $C226  8D 07 20
  lda z:_var_0000_indexed        ; $C229  A5 00
  and #$0F                       ; $C22B  29 0F
  tax                            ; $C22D  AA
  lda a:_data_c251_indexed,X     ; $C22E  BD 51 C2
  sta PPU_DATA                   ; $C231  8D 07 20
  jmp _label_c294                ; $C234  4C 94 C2

_label_c237:
  lda #$4F                       ; $C237  A9 4F
  sta PPU_DATA                   ; $C239  8D 07 20
  lda #$4B                       ; $C23C  A9 4B
  sta PPU_DATA                   ; $C23E  8D 07 20
  jmp _label_c294                ; $C241  4C 94 C2

_label_c244:
  lda #$45                       ; $C244  A9 45
  sta PPU_DATA                   ; $C246  8D 07 20
  lda #$72                       ; $C249  A9 72
  sta PPU_DATA                   ; $C24B  8D 07 20
  jmp _label_c294                ; $C24E  4C 94 C2

_data_c251_indexed:
.byte $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $41, $42, $43, $44, $45, $46 ; $C251

_func_c261:
  lda z:_var_00d7                ; $C261  A5 D7
  clc                            ; $C263  18
  adc #$04                       ; $C264  69 04
  tay                            ; $C266  A8
  lda #$84                       ; $C267  A9 84
  sta PPU_CTRL                   ; $C269  8D 00 20
  lda #$20                       ; $C26C  A9 20
  sta PPU_ADDR                   ; $C26E  8D 06 20
  lda #$02                       ; $C271  A9 02
  sta PPU_ADDR                   ; $C273  8D 06 20

_label_c276:
  lda #$20                       ; $C276  A9 20
  dey                            ; $C278  88
  iny                            ; $C279  C8
  bne _label_c27e                ; $C27A  D0 02
  lda #$2A                       ; $C27C  A9 2A

_label_c27e:
  sta PPU_DATA                   ; $C27E  8D 07 20
  dey                            ; $C281  88
  dex                            ; $C282  CA
  bne _label_c276                ; $C283  D0 F1
  lda #$80                       ; $C285  A9 80
  sta PPU_CTRL                   ; $C287  8D 00 20
  jmp _label_c294                ; $C28A  4C 94 C2

_func_c28d:
  lda z:_var_00d2                ; $C28D  A5 D2

_label_c28f:
  cmp z:_var_00d2                ; $C28F  C5 D2
  beq _label_c28f                ; $C291  F0 FC
  rts                            ; $C293  60

_label_c294:
  lda #$00                       ; $C294  A9 00
  sta PPU_SCROLL                 ; $C296  8D 05 20
  sta PPU_SCROLL                 ; $C299  8D 05 20
  lda #$00                       ; $C29C  A9 00
  sta PPU_ADDR                   ; $C29E  8D 06 20
  lda #$00                       ; $C2A1  A9 00
  sta PPU_ADDR                   ; $C2A3  8D 06 20
  rts                            ; $C2A6  60

_func_c2a7:
  lda #$00                       ; $C2A7  A9 00
  sta PPU_CTRL                   ; $C2A9  8D 00 20
  sta PPU_MASK                   ; $C2AC  8D 01 20
  jsr _func_c2ed                 ; $C2AF  20 ED C2
  lda #$20                       ; $C2B2  A9 20
  sta PPU_ADDR                   ; $C2B4  8D 06 20
  ldy #$00                       ; $C2B7  A0 00
  sty PPU_ADDR                   ; $C2B9  8C 06 20

_label_c2bc:
  ldx #$20                       ; $C2BC  A2 20

_label_c2be:
  lda (_var_00d0_indexed),Y      ; $C2BE  B1 D0
  beq _label_c2e2                ; $C2C0  F0 20
  cmp #$FF                       ; $C2C2  C9 FF
  beq _label_c2d3                ; $C2C4  F0 0D
  sta PPU_DATA                   ; $C2C6  8D 07 20
  iny                            ; $C2C9  C8
  bne _label_c2ce                ; $C2CA  D0 02
  inc z:_var_00d1                ; $C2CC  E6 D1

_label_c2ce:
  dex                            ; $C2CE  CA
  bne _label_c2be                ; $C2CF  D0 ED
  beq _label_c2bc                ; $C2D1  F0 E9

_label_c2d3:
  iny                            ; $C2D3  C8
  bne _label_c2d8                ; $C2D4  D0 02
  inc z:_var_00d1                ; $C2D6  E6 D1

_label_c2d8:
  lda #$20                       ; $C2D8  A9 20
  sta PPU_DATA                   ; $C2DA  8D 07 20
  dex                            ; $C2DD  CA
  bne _label_c2d8                ; $C2DE  D0 F8
  beq _label_c2bc                ; $C2E0  F0 DA

_label_c2e2:
  lda #$80                       ; $C2E2  A9 80
  sta PPU_CTRL                   ; $C2E4  8D 00 20
  lda #$0E                       ; $C2E7  A9 0E
  sta PPU_MASK                   ; $C2E9  8D 01 20
  rts                            ; $C2EC  60

_func_c2ed:
  lda #$20                       ; $C2ED  A9 20
  sta PPU_ADDR                   ; $C2EF  8D 06 20
  lda #$00                       ; $C2F2  A9 00
  sta PPU_ADDR                   ; $C2F4  8D 06 20
  ldx #$1E                       ; $C2F7  A2 1E
  lda #$20                       ; $C2F9  A9 20

_label_c2fb:
  ldy #$20                       ; $C2FB  A0 20

_label_c2fd:
  sta PPU_DATA                   ; $C2FD  8D 07 20
  dey                            ; $C300  88
  bne _label_c2fd                ; $C301  D0 FA
  dex                            ; $C303  CA
  bne _label_c2fb                ; $C304  D0 F5
  rts                            ; $C306  60

.byte $ff, $ff, $ff, $ff, $20, $20, $20, $20, $2d, $2d, $20, $52, $75, $6e, $20, $61 ; $C307
.byte $6c, $6c, $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20 ; $C317
.byte $42, $72, $61, $6e, $63, $68, $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20 ; $C327
.byte $20, $2d, $2d, $20, $46, $6c, $61, $67, $20, $74, $65, $73, $74, $73, $ff, $20 ; $C337
.byte $20, $20, $20, $2d, $2d, $20, $49, $6d, $6d, $65, $64, $69, $61, $74, $65, $20 ; $C347
.byte $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $49, $6d, $70 ; $C357
.byte $6c, $69, $65, $64, $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d ; $C367
.byte $2d, $20, $53, $74, $61, $63, $6b, $20, $74, $65, $73, $74, $73, $ff, $20, $20 ; $C377
.byte $20, $20, $2d, $2d, $20, $41, $63, $63, $75, $6d, $75, $6c, $61, $74, $6f, $72 ; $C387
.byte $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $28, $49 ; $C397
.byte $6e, $64, $69, $72, $65, $63, $74, $2c, $58, $29, $20, $74, $65, $73, $74, $73 ; $C3A7
.byte $ff, $20, $20, $20, $20, $2d, $2d, $20, $5a, $65, $72, $6f, $70, $61, $67, $65 ; $C3B7
.byte $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $41, $62 ; $C3C7
.byte $73, $6f, $6c, $75, $74, $65, $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20 ; $C3D7
.byte $20, $2d, $2d, $20, $28, $49, $6e, $64, $69, $72, $65, $63, $74, $29, $2c, $59 ; $C3E7
.byte $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $41, $62 ; $C3F7
.byte $73, $6f, $6c, $75, $74, $65, $2c, $59, $20, $74, $65, $73, $74, $73, $ff, $20 ; $C407
.byte $20, $20, $20, $2d, $2d, $20, $5a, $65, $72, $6f, $70, $61, $67, $65, $2c, $58 ; $C417
.byte $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $41, $62 ; $C427
.byte $73, $6f, $6c, $75, $74, $65, $2c, $58, $20, $74, $65, $73, $74, $73, $ff, $ff ; $C437
.byte $ff, $20, $20, $20, $20, $55, $70, $2f, $44, $6f, $77, $6e, $3a, $20, $73, $65 ; $C447
.byte $6c, $65, $63, $74, $20, $74, $65, $73, $74, $ff, $20, $20, $20, $20, $20, $20 ; $C457
.byte $53, $74, $61, $72, $74, $3a, $20, $72, $75, $6e, $20, $74, $65, $73, $74, $ff ; $C467
.byte $20, $20, $20, $20, $20, $53, $65, $6c, $65, $63, $74, $3a, $20, $49, $6e, $76 ; $C477
.byte $61, $6c, $69, $64, $20, $6f, $70, $73, $21, $ff, $00, $ff, $ff, $ff, $ff, $20 ; $C487
.byte $20, $20, $20, $2d, $2d, $20, $52, $75, $6e, $20, $61, $6c, $6c, $20, $74, $65 ; $C497
.byte $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $4e, $4f, $50, $20, $74 ; $C4A7
.byte $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $4c, $41, $58, $20 ; $C4B7
.byte $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $53, $41, $58 ; $C4C7
.byte $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $53, $42 ; $C4D7
.byte $43, $20, $74, $65, $73, $74, $20, $28, $6f, $70, $63, $6f, $64, $65, $20, $30 ; $C4E7
.byte $45, $42, $68, $29, $ff, $20, $20, $20, $20, $2d, $2d, $20, $44, $43, $50, $20 ; $C4F7
.byte $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $49, $53, $42 ; $C507
.byte $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $53, $4c ; $C517
.byte $4f, $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20, $52 ; $C527
.byte $4c, $41, $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d, $20 ; $C537
.byte $53, $52, $45, $20, $74, $65, $73, $74, $73, $ff, $20, $20, $20, $20, $2d, $2d ; $C547
.byte $20, $52, $52, $41, $20, $74, $65, $73, $74, $73, $ff, $ff, $ff, $ff, $ff, $ff ; $C557
.byte $20, $20, $20, $20, $55, $70, $2f, $44, $6f, $77, $6e, $3a, $20, $73, $65, $6c ; $C567
.byte $65, $63, $74, $20, $74, $65, $73, $74, $ff, $20, $20, $20, $20, $20, $20, $53 ; $C577
.byte $74, $61, $72, $74, $3a, $20, $72, $75, $6e, $20, $74, $65, $73, $74, $ff, $20 ; $C587
.byte $20, $20, $20, $20, $53, $65, $6c, $65, $63, $74, $3a, $20, $4e, $6f, $72, $6d ; $C597
.byte $61, $6c, $20, $6f, $70, $73, $ff, $00 ; $C5A7

NMI:
  pha                            ; $C5AF  48
  txa                            ; $C5B0  8A
  pha                            ; $C5B1  48
  lda PPU_STATUS                 ; $C5B2  AD 02 20
  lda #$20                       ; $C5B5  A9 20
  sta PPU_ADDR                   ; $C5B7  8D 06 20
  lda #$40                       ; $C5BA  A9 40
  sta PPU_ADDR                   ; $C5BC  8D 06 20
  inc z:_var_00d2                ; $C5BF  E6 D2
  lda #$00                       ; $C5C1  A9 00
  sta PPU_SCROLL                 ; $C5C3  8D 05 20
  sta PPU_SCROLL                 ; $C5C6  8D 05 20
  lda #$00                       ; $C5C9  A9 00
  sta PPU_ADDR                   ; $C5CB  8D 06 20
  lda #$00                       ; $C5CE  A9 00
  sta PPU_ADDR                   ; $C5D0  8D 06 20
  ldx #$09                       ; $C5D3  A2 09
  stx JOYPAD1                    ; $C5D5  8E 16 40
  dex                            ; $C5D8  CA
  stx JOYPAD1                    ; $C5D9  8E 16 40

_label_c5dc:
  lda JOYPAD1                    ; $C5DC  AD 16 40
  lsr a                          ; $C5DF  4A
  rol z:_var_00d4                ; $C5E0  26 D4
  dex                            ; $C5E2  CA
  bne _label_c5dc                ; $C5E3  D0 F7
  lda z:_var_00d4                ; $C5E5  A5 D4
  tax                            ; $C5E7  AA
  eor z:_var_00d6                ; $C5E8  45 D6
  and z:_var_00d4                ; $C5EA  25 D4
  sta z:_var_00d5                ; $C5EC  85 D5
  stx z:_var_00d6                ; $C5EE  86 D6
  pla                            ; $C5F0  68
  tax                            ; $C5F1  AA
  pla                            ; $C5F2  68
  rti                            ; $C5F3  40

IRQ:
  rti                            ; $C5F4  40

.byte $a2, $00, $86, $00, $86, $10, $86, $11, $20, $2d, $c7, $20, $db, $c7, $20, $85 ; $C5F5
.byte $c8, $20, $de, $cb, $20, $f8, $cd, $20, $ee, $ce, $20, $a2, $cf, $20, $74, $d1 ; $C605
.byte $20, $fb, $d4, $20, $00, $d9, $a5, $00, $85, $10, $a9, $00, $85, $00, $20, $e0 ; $C615
.byte $da, $20, $4a, $df, $20, $b8, $db, $20, $aa, $e1, $20, $a3, $c6, $20, $1e, $e5 ; $C625
.byte $20, $3d, $e7, $20, $d3, $e8, $20, $16, $e9, $20, $86, $eb, $20, $f6, $ed, $20 ; $C635
.byte $66, $f0, $20, $d6, $f2, $a5, $00, $85, $11, $a9, $00, $85, $00, $20, $46, $f5 ; $C645
.byte $a5, $00, $05, $10, $05, $11, $f0, $0e, $20, $6f, $c6, $a6, $00, $86, $02, $a6 ; $C655
.byte $10, $86, $03, $4c, $6e, $c6, $20, $89, $c6, $60 ; $C665

_func_c66f:
  lda #$03                       ; $C66F  A9 03
  sta APU_SND_CHN                ; $C671  8D 15 40
  lda #$87                       ; $C674  A9 87
  sta APU_PL1_VOL                ; $C676  8D 00 40
  lda #$89                       ; $C679  A9 89
  sta APU_PL1_SWEEP              ; $C67B  8D 01 40
  lda #$F0                       ; $C67E  A9 F0
  sta APU_PL1_LO                 ; $C680  8D 02 40
  lda #$00                       ; $C683  A9 00
  sta APU_PL1_HI                 ; $C685  8D 03 40
  rts                            ; $C688  60

_func_c689:
  lda #$02                       ; $C689  A9 02
  sta APU_SND_CHN                ; $C68B  8D 15 40
  lda #$3F                       ; $C68E  A9 3F
  sta APU_PL2_VOL                ; $C690  8D 04 40
  lda #$9A                       ; $C693  A9 9A
  sta APU_PL2_SWEEP              ; $C695  8D 05 40
  lda #$FF                       ; $C698  A9 FF
  sta APU_PL2_LO                 ; $C69A  8D 06 40
  lda #$00                       ; $C69D  A9 00
  sta APU_PL2_HI                 ; $C69F  8D 07 40
  rts                            ; $C6A2  60

_label_c6a3:
  ldy #$4E                       ; $C6A3  A0 4E
  lda #$FF                       ; $C6A5  A9 FF
  sta z:_var_0001                ; $C6A7  85 01
  jsr _func_c6b0                 ; $C6A9  20 B0 C6
  jsr _func_c6b7                 ; $C6AC  20 B7 C6
  rts                            ; $C6AF  60

_func_c6b0:
  lda #$FF                       ; $C6B0  A9 FF
  pha                            ; $C6B2  48
  lda #$AA                       ; $C6B3  A9 AA
  bne _label_c6bc                ; $C6B5  D0 05

_func_c6b7:
  lda #$34                       ; $C6B7  A9 34
  pha                            ; $C6B9  48
  lda #$55                       ; $C6BA  A9 55

_label_c6bc:
  plp                            ; $C6BC  28
.byte $04, $a9                   ; $C6BD  04 A9  disambiguous instruction: nop z:$A9
.byte $44, $a9                   ; $C6BF  44 A9  disambiguous instruction: nop z:$A9
.byte $64, $a9                   ; $C6C1  64 A9  disambiguous instruction: nop z:$A9
  nop                            ; $C6C3  EA
  nop                            ; $C6C4  EA
  nop                            ; $C6C5  EA
  nop                            ; $C6C6  EA
  php                            ; $C6C7  08
  pha                            ; $C6C8  48
.byte $0c, $a9, $a9              ; $C6C9  0C A9 A9  disambiguous instruction: nop a:$A9A9
  nop                            ; $C6CC  EA
  nop                            ; $C6CD  EA
  nop                            ; $C6CE  EA
  nop                            ; $C6CF  EA
  php                            ; $C6D0  08
  pha                            ; $C6D1  48
.byte $14, $a9                   ; $C6D2  14 A9  disambiguous instruction: nop z:$A9,X
.byte $34, $a9                   ; $C6D4  34 A9  disambiguous instruction: nop z:$A9,X
.byte $54, $a9                   ; $C6D6  54 A9  disambiguous instruction: nop z:$A9,X
.byte $74, $a9                   ; $C6D8  74 A9  disambiguous instruction: nop z:$A9,X
.byte $d4, $a9                   ; $C6DA  D4 A9  disambiguous instruction: nop z:$A9,X
.byte $f4, $a9                   ; $C6DC  F4 A9  disambiguous instruction: nop z:$A9,X
  nop                            ; $C6DE  EA
  nop                            ; $C6DF  EA
  nop                            ; $C6E0  EA
  nop                            ; $C6E1  EA
  php                            ; $C6E2  08
  pha                            ; $C6E3  48
.byte $1a                        ; $C6E4  1A  disambiguous instruction: nop
.byte $3a                        ; $C6E5  3A  disambiguous instruction: nop
.byte $5a                        ; $C6E6  5A  disambiguous instruction: nop
.byte $7a                        ; $C6E7  7A  disambiguous instruction: nop
.byte $da                        ; $C6E8  DA  disambiguous instruction: nop
.byte $fa                        ; $C6E9  FA  disambiguous instruction: nop
.byte $80, $89                   ; $C6EA  80 89  disambiguous instruction: nop #$89
  nop                            ; $C6EC  EA
  nop                            ; $C6ED  EA
  nop                            ; $C6EE  EA
  nop                            ; $C6EF  EA
  php                            ; $C6F0  08
  pha                            ; $C6F1  48
.byte $1c, $a9, $a9              ; $C6F2  1C A9 A9  disambiguous instruction: nop a:$A9A9,X
.byte $3c, $a9, $a9              ; $C6F5  3C A9 A9  disambiguous instruction: nop a:$A9A9,X
.byte $5c, $a9, $a9              ; $C6F8  5C A9 A9  disambiguous instruction: nop a:$A9A9,X
.byte $7c, $a9, $a9              ; $C6FB  7C A9 A9  disambiguous instruction: nop a:$A9A9,X
.byte $dc, $a9, $a9              ; $C6FE  DC A9 A9  disambiguous instruction: nop a:$A9A9,X
.byte $fc, $a9, $a9              ; $C701  FC A9 A9  disambiguous instruction: nop a:$A9A9,X
  nop                            ; $C704  EA
  nop                            ; $C705  EA
  nop                            ; $C706  EA
  nop                            ; $C707  EA
  php                            ; $C708  08
  pha                            ; $C709  48
  ldx #$05                       ; $C70A  A2 05

_label_c70c:
  pla                            ; $C70C  68
  cmp #$55                       ; $C70D  C9 55
  beq _label_c71b                ; $C70F  F0 0A
  cmp #$AA                       ; $C711  C9 AA
  beq _label_c71b                ; $C713  F0 06
  pla                            ; $C715  68
  sty z:_var_0000_indexed        ; $C716  84 00
  jmp _label_c728                ; $C718  4C 28 C7

_label_c71b:
  pla                            ; $C71B  68
  and #$CB                       ; $C71C  29 CB
  cmp #$00                       ; $C71E  C9 00
  beq _label_c728                ; $C720  F0 06
  cmp #$CB                       ; $C722  C9 CB
  beq _label_c728                ; $C724  F0 02
  sty z:_var_0000_indexed        ; $C726  84 00

_label_c728:
  iny                            ; $C728  C8
  dex                            ; $C729  CA
  bne _label_c70c                ; $C72A  D0 E0
  rts                            ; $C72C  60

_label_c72d:
  nop                            ; $C72D  EA
  sec                            ; $C72E  38
  bcs _label_c735                ; $C72F  B0 04
  ldx #$01                       ; $C731  A2 01
  stx z:_var_0000_indexed        ; $C733  86 00

_label_c735:
  nop                            ; $C735  EA
  clc                            ; $C736  18
  bcs _label_c73c                ; $C737  B0 03
  jmp _label_c740                ; $C739  4C 40 C7

_label_c73c:
  ldx #$02                       ; $C73C  A2 02
  stx z:_var_0000_indexed        ; $C73E  86 00

_label_c740:
  nop                            ; $C740  EA
  sec                            ; $C741  38
  bcc _label_c747                ; $C742  90 03
  jmp _label_c74b                ; $C744  4C 4B C7

_label_c747:
  ldx #$03                       ; $C747  A2 03
  stx z:_var_0000_indexed        ; $C749  86 00

_label_c74b:
  nop                            ; $C74B  EA
  clc                            ; $C74C  18
  bcc _label_c753                ; $C74D  90 04
  ldx #$04                       ; $C74F  A2 04
  stx z:_var_0000_indexed        ; $C751  86 00

_label_c753:
  nop                            ; $C753  EA
  lda #$00                       ; $C754  A9 00
  beq _label_c75c                ; $C756  F0 04
  ldx #$05                       ; $C758  A2 05
  stx z:_var_0000_indexed        ; $C75A  86 00

_label_c75c:
  nop                            ; $C75C  EA
  lda #$40                       ; $C75D  A9 40
  beq _label_c764                ; $C75F  F0 03
  jmp _label_c768                ; $C761  4C 68 C7

_label_c764:
  ldx #$06                       ; $C764  A2 06
  stx z:_var_0000_indexed        ; $C766  86 00

_label_c768:
  nop                            ; $C768  EA
  lda #$40                       ; $C769  A9 40
  bne _label_c771                ; $C76B  D0 04
  ldx #$07                       ; $C76D  A2 07
  stx z:_var_0000_indexed        ; $C76F  86 00

_label_c771:
  nop                            ; $C771  EA
  lda #$00                       ; $C772  A9 00
  bne _label_c779                ; $C774  D0 03
  jmp _label_c77d                ; $C776  4C 7D C7

_label_c779:
  ldx #$08                       ; $C779  A2 08
  stx z:_var_0000_indexed        ; $C77B  86 00

_label_c77d:
  nop                            ; $C77D  EA
  lda #$FF                       ; $C77E  A9 FF
  sta z:_var_0001                ; $C780  85 01
  bit z:_var_0001                ; $C782  24 01
  bvs _label_c78a                ; $C784  70 04
  ldx #$09                       ; $C786  A2 09
  stx z:_var_0000_indexed        ; $C788  86 00

_label_c78a:
  nop                            ; $C78A  EA
  bit z:_var_0001                ; $C78B  24 01
  bvc _label_c792                ; $C78D  50 03
  jmp _label_c796                ; $C78F  4C 96 C7

_label_c792:
  ldx #$0A                       ; $C792  A2 0A
  stx z:_var_0000_indexed        ; $C794  86 00

_label_c796:
  nop                            ; $C796  EA
  lda #$00                       ; $C797  A9 00
  sta z:_var_0001                ; $C799  85 01
  bit z:_var_0001                ; $C79B  24 01
  bvc _label_c7a3                ; $C79D  50 04
  ldx #$0B                       ; $C79F  A2 0B
  stx z:_var_0000_indexed        ; $C7A1  86 00

_label_c7a3:
  nop                            ; $C7A3  EA
  bit z:_var_0001                ; $C7A4  24 01
  bvs _label_c7ab                ; $C7A6  70 03
  jmp _label_c7af                ; $C7A8  4C AF C7

_label_c7ab:
  ldx #$0C                       ; $C7AB  A2 0C
  stx z:_var_0000_indexed        ; $C7AD  86 00

_label_c7af:
  nop                            ; $C7AF  EA
  lda #$00                       ; $C7B0  A9 00
  bpl _label_c7b8                ; $C7B2  10 04
  ldx #$0D                       ; $C7B4  A2 0D
  stx z:_var_0000_indexed        ; $C7B6  86 00

_label_c7b8:
  nop                            ; $C7B8  EA
  lda #$80                       ; $C7B9  A9 80
  bpl _label_c7c0                ; $C7BB  10 03
  jmp _label_c7d9                ; $C7BD  4C D9 C7

_label_c7c0:
  ldx #$0E                       ; $C7C0  A2 0E
  stx z:_var_0000_indexed        ; $C7C2  86 00
  nop                            ; $C7C4  EA
  lda #$80                       ; $C7C5  A9 80
  bmi _label_c7cd                ; $C7C7  30 04
  ldx #$0F                       ; $C7C9  A2 0F
  stx z:_var_0000_indexed        ; $C7CB  86 00

_label_c7cd:
  nop                            ; $C7CD  EA
  lda #$00                       ; $C7CE  A9 00
  bmi _label_c7d5                ; $C7D0  30 03
  jmp _label_c7d9                ; $C7D2  4C D9 C7

_label_c7d5:
  ldx #$10                       ; $C7D5  A2 10
  stx z:_var_0000_indexed        ; $C7D7  86 00

_label_c7d9:
  nop                            ; $C7D9  EA
  rts                            ; $C7DA  60

_label_c7db:
  nop                            ; $C7DB  EA
  lda #$FF                       ; $C7DC  A9 FF
  sta z:_var_0001                ; $C7DE  85 01
  bit z:_var_0001                ; $C7E0  24 01
  lda #$00                       ; $C7E2  A9 00
  sec                            ; $C7E4  38
  sei                            ; $C7E5  78
  sed                            ; $C7E6  F8
  php                            ; $C7E7  08
  pla                            ; $C7E8  68
  and #$EF                       ; $C7E9  29 EF
  cmp #$6F                       ; $C7EB  C9 6F
  beq _label_c7f3                ; $C7ED  F0 04
  ldx #$11                       ; $C7EF  A2 11
  stx z:_var_0000_indexed        ; $C7F1  86 00

_label_c7f3:
  nop                            ; $C7F3  EA
  lda #$40                       ; $C7F4  A9 40
  sta z:_var_0001                ; $C7F6  85 01
  bit z:_var_0001                ; $C7F8  24 01
  cld                            ; $C7FA  D8
  lda #$10                       ; $C7FB  A9 10
  clc                            ; $C7FD  18
  php                            ; $C7FE  08
  pla                            ; $C7FF  68
  and #$EF                       ; $C800  29 EF
  cmp #$64                       ; $C802  C9 64
  beq _label_c80a                ; $C804  F0 04
  ldx #$12                       ; $C806  A2 12
  stx z:_var_0000_indexed        ; $C808  86 00

_label_c80a:
  nop                            ; $C80A  EA
  lda #$80                       ; $C80B  A9 80
  sta z:_var_0001                ; $C80D  85 01
  bit z:_var_0001                ; $C80F  24 01
  sed                            ; $C811  F8
  lda #$00                       ; $C812  A9 00
  sec                            ; $C814  38
  php                            ; $C815  08
  pla                            ; $C816  68
  and #$EF                       ; $C817  29 EF
  cmp #$2F                       ; $C819  C9 2F
  beq _label_c821                ; $C81B  F0 04
  ldx #$13                       ; $C81D  A2 13
  stx z:_var_0000_indexed        ; $C81F  86 00

_label_c821:
  nop                            ; $C821  EA
  lda #$FF                       ; $C822  A9 FF
  pha                            ; $C824  48
  plp                            ; $C825  28
  bne _label_c831                ; $C826  D0 09
  bpl _label_c831                ; $C828  10 07
  bvc _label_c831                ; $C82A  50 05
  bcc _label_c831                ; $C82C  90 03
  jmp _label_c835                ; $C82E  4C 35 C8

_label_c831:
  ldx #$14                       ; $C831  A2 14
  stx z:_var_0000_indexed        ; $C833  86 00

_label_c835:
  nop                            ; $C835  EA
  lda #$04                       ; $C836  A9 04
  pha                            ; $C838  48
  plp                            ; $C839  28
  beq _label_c845                ; $C83A  F0 09
  bmi _label_c845                ; $C83C  30 07
  bvs _label_c845                ; $C83E  70 05
  bcs _label_c845                ; $C840  B0 03
  jmp _label_c849                ; $C842  4C 49 C8

_label_c845:
  ldx #$15                       ; $C845  A2 15
  stx z:_var_0000_indexed        ; $C847  86 00

_label_c849:
  nop                            ; $C849  EA
  sed                            ; $C84A  F8
  lda #$FF                       ; $C84B  A9 FF
  sta z:_var_0001                ; $C84D  85 01
  bit z:_var_0001                ; $C84F  24 01
  clc                            ; $C851  18
  lda #$00                       ; $C852  A9 00
  pha                            ; $C854  48
  lda #$FF                       ; $C855  A9 FF
  pla                            ; $C857  68
  bne _label_c863                ; $C858  D0 09
  bmi _label_c863                ; $C85A  30 07
  bvc _label_c863                ; $C85C  50 05
  bcs _label_c863                ; $C85E  B0 03
  jmp _label_c867                ; $C860  4C 67 C8

_label_c863:
  ldx #$16                       ; $C863  A2 16
  stx z:_var_0000_indexed        ; $C865  86 00

_label_c867:
  nop                            ; $C867  EA
  lda #$00                       ; $C868  A9 00
  sta z:_var_0001                ; $C86A  85 01
  bit z:_var_0001                ; $C86C  24 01
  sec                            ; $C86E  38
  lda #$FF                       ; $C86F  A9 FF
  pha                            ; $C871  48
  lda #$00                       ; $C872  A9 00
  pla                            ; $C874  68
  beq _label_c880                ; $C875  F0 09
  bpl _label_c880                ; $C877  10 07
  bvs _label_c880                ; $C879  70 05
  bcc _label_c880                ; $C87B  90 03
  jmp _label_c884                ; $C87D  4C 84 C8

_label_c880:
  ldx #$17                       ; $C880  A2 17
  stx z:_var_0000_indexed        ; $C882  86 00

_label_c884:
  rts                            ; $C884  60

_label_c885:
  nop                            ; $C885  EA
  clc                            ; $C886  18
  lda #$FF                       ; $C887  A9 FF
  sta z:_var_0001                ; $C889  85 01
  bit z:_var_0001                ; $C88B  24 01
  lda #$55                       ; $C88D  A9 55
  ora #$AA                       ; $C88F  09 AA
  bcs _label_c89e                ; $C891  B0 0B
  bpl _label_c89e                ; $C893  10 09
  cmp #$FF                       ; $C895  C9 FF
  bne _label_c89e                ; $C897  D0 05
  bvc _label_c89e                ; $C899  50 03
  jmp _label_c8a2                ; $C89B  4C A2 C8

_label_c89e:
  ldx #$18                       ; $C89E  A2 18
  stx z:_var_0000_indexed        ; $C8A0  86 00

_label_c8a2:
  nop                            ; $C8A2  EA
  sec                            ; $C8A3  38
  clv                            ; $C8A4  B8
  lda #$00                       ; $C8A5  A9 00
  ora #$00                       ; $C8A7  09 00
  bne _label_c8b4                ; $C8A9  D0 09
  bvs _label_c8b4                ; $C8AB  70 07
  bcc _label_c8b4                ; $C8AD  90 05
  bmi _label_c8b4                ; $C8AF  30 03
  jmp _label_c8b8                ; $C8B1  4C B8 C8

_label_c8b4:
  ldx #$19                       ; $C8B4  A2 19
  stx z:_var_0000_indexed        ; $C8B6  86 00

_label_c8b8:
  nop                            ; $C8B8  EA
  clc                            ; $C8B9  18
  bit z:_var_0001                ; $C8BA  24 01
  lda #$55                       ; $C8BC  A9 55
  and #$AA                       ; $C8BE  29 AA
  bne _label_c8cb                ; $C8C0  D0 09
  bvc _label_c8cb                ; $C8C2  50 07
  bcs _label_c8cb                ; $C8C4  B0 05
  bmi _label_c8cb                ; $C8C6  30 03
  jmp _label_c8cf                ; $C8C8  4C CF C8

_label_c8cb:
  ldx #$1A                       ; $C8CB  A2 1A
  stx z:_var_0000_indexed        ; $C8CD  86 00

_label_c8cf:
  nop                            ; $C8CF  EA
  sec                            ; $C8D0  38
  clv                            ; $C8D1  B8
  lda #$F8                       ; $C8D2  A9 F8
  and #$EF                       ; $C8D4  29 EF
  bcc _label_c8e3                ; $C8D6  90 0B
  bpl _label_c8e3                ; $C8D8  10 09
  cmp #$E8                       ; $C8DA  C9 E8
  bne _label_c8e3                ; $C8DC  D0 05
  bvs _label_c8e3                ; $C8DE  70 03
  jmp _label_c8e7                ; $C8E0  4C E7 C8

_label_c8e3:
  ldx #$1B                       ; $C8E3  A2 1B
  stx z:_var_0000_indexed        ; $C8E5  86 00

_label_c8e7:
  nop                            ; $C8E7  EA
  clc                            ; $C8E8  18
  bit z:_var_0001                ; $C8E9  24 01
  lda #$5F                       ; $C8EB  A9 5F
  eor #$AA                       ; $C8ED  49 AA
  bcs _label_c8fc                ; $C8EF  B0 0B
  bpl _label_c8fc                ; $C8F1  10 09
  cmp #$F5                       ; $C8F3  C9 F5
  bne _label_c8fc                ; $C8F5  D0 05
  bvc _label_c8fc                ; $C8F7  50 03
  jmp _label_c900                ; $C8F9  4C 00 C9

_label_c8fc:
  ldx #$1C                       ; $C8FC  A2 1C
  stx z:_var_0000_indexed        ; $C8FE  86 00

_label_c900:
  nop                            ; $C900  EA
  sec                            ; $C901  38
  clv                            ; $C902  B8
  lda #$70                       ; $C903  A9 70
  eor #$70                       ; $C905  49 70
  bne _label_c912                ; $C907  D0 09
  bvs _label_c912                ; $C909  70 07
  bcc _label_c912                ; $C90B  90 05
  bmi _label_c912                ; $C90D  30 03
  jmp _label_c916                ; $C90F  4C 16 C9

_label_c912:
  ldx #$1D                       ; $C912  A2 1D
  stx z:_var_0000_indexed        ; $C914  86 00

_label_c916:
  nop                            ; $C916  EA
  clc                            ; $C917  18
  bit z:_var_0001                ; $C918  24 01
  lda #$00                       ; $C91A  A9 00
  adc #$69                       ; $C91C  69 69
  bmi _label_c92b                ; $C91E  30 0B
  bcs _label_c92b                ; $C920  B0 09
  cmp #$69                       ; $C922  C9 69
  bne _label_c92b                ; $C924  D0 05
  bvs _label_c92b                ; $C926  70 03
  jmp _label_c92f                ; $C928  4C 2F C9

_label_c92b:
  ldx #$1E                       ; $C92B  A2 1E
  stx z:_var_0000_indexed        ; $C92D  86 00

_label_c92f:
  nop                            ; $C92F  EA
  sec                            ; $C930  38
  sed                            ; $C931  F8
  bit z:_var_0001                ; $C932  24 01
  lda #$01                       ; $C934  A9 01
  adc #$69                       ; $C936  69 69
  bmi _label_c945                ; $C938  30 0B
  bcs _label_c945                ; $C93A  B0 09
  cmp #$6B                       ; $C93C  C9 6B
  bne _label_c945                ; $C93E  D0 05
  bvs _label_c945                ; $C940  70 03
  jmp _label_c949                ; $C942  4C 49 C9

_label_c945:
  ldx #$1F                       ; $C945  A2 1F
  stx z:_var_0000_indexed        ; $C947  86 00

_label_c949:
  nop                            ; $C949  EA
  cld                            ; $C94A  D8
  sec                            ; $C94B  38
  clv                            ; $C94C  B8
  lda #$7F                       ; $C94D  A9 7F
  adc #$7F                       ; $C94F  69 7F
  bpl _label_c95e                ; $C951  10 0B
  bcs _label_c95e                ; $C953  B0 09
  cmp #$FF                       ; $C955  C9 FF
  bne _label_c95e                ; $C957  D0 05
  bvc _label_c95e                ; $C959  50 03
  jmp _label_c962                ; $C95B  4C 62 C9

_label_c95e:
  ldx #$20                       ; $C95E  A2 20
  stx z:_var_0000_indexed        ; $C960  86 00

_label_c962:
  nop                            ; $C962  EA
  clc                            ; $C963  18
  bit z:_var_0001                ; $C964  24 01
  lda #$7F                       ; $C966  A9 7F
  adc #$80                       ; $C968  69 80
  bpl _label_c977                ; $C96A  10 0B
  bcs _label_c977                ; $C96C  B0 09
  cmp #$FF                       ; $C96E  C9 FF
  bne _label_c977                ; $C970  D0 05
  bvs _label_c977                ; $C972  70 03
  jmp _label_c97b                ; $C974  4C 7B C9

_label_c977:
  ldx #$21                       ; $C977  A2 21
  stx z:_var_0000_indexed        ; $C979  86 00

_label_c97b:
  nop                            ; $C97B  EA
  sec                            ; $C97C  38
  clv                            ; $C97D  B8
  lda #$7F                       ; $C97E  A9 7F
  adc #$80                       ; $C980  69 80
  bne _label_c98d                ; $C982  D0 09
  bmi _label_c98d                ; $C984  30 07
  bvs _label_c98d                ; $C986  70 05
  bcc _label_c98d                ; $C988  90 03
  jmp _label_c991                ; $C98A  4C 91 C9

_label_c98d:
  ldx #$22                       ; $C98D  A2 22
  stx z:_var_0000_indexed        ; $C98F  86 00

_label_c991:
  nop                            ; $C991  EA
  sec                            ; $C992  38
  clv                            ; $C993  B8
  lda #$9F                       ; $C994  A9 9F
  beq _label_c9a1                ; $C996  F0 09
  bpl _label_c9a1                ; $C998  10 07
  bvs _label_c9a1                ; $C99A  70 05
  bcc _label_c9a1                ; $C99C  90 03
  jmp _label_c9a5                ; $C99E  4C A5 C9

_label_c9a1:
  ldx #$23                       ; $C9A1  A2 23
  stx z:_var_0000_indexed        ; $C9A3  86 00

_label_c9a5:
  nop                            ; $C9A5  EA
  clc                            ; $C9A6  18
  bit z:_var_0001                ; $C9A7  24 01
  lda #$00                       ; $C9A9  A9 00
  bne _label_c9b6                ; $C9AB  D0 09
  bmi _label_c9b6                ; $C9AD  30 07
  bvc _label_c9b6                ; $C9AF  50 05
  bcs _label_c9b6                ; $C9B1  B0 03
  jmp _label_c9ba                ; $C9B3  4C BA C9

_label_c9b6:
  ldx #$23                       ; $C9B6  A2 23
  stx z:_var_0000_indexed        ; $C9B8  86 00

_label_c9ba:
  nop                            ; $C9BA  EA
  bit z:_var_0001                ; $C9BB  24 01
  lda #$40                       ; $C9BD  A9 40
  cmp #$40                       ; $C9BF  C9 40
  bmi _label_c9cc                ; $C9C1  30 09
  bcc _label_c9cc                ; $C9C3  90 07
  bne _label_c9cc                ; $C9C5  D0 05
  bvc _label_c9cc                ; $C9C7  50 03
  jmp _label_c9d0                ; $C9C9  4C D0 C9

_label_c9cc:
  ldx #$24                       ; $C9CC  A2 24
  stx z:_var_0000_indexed        ; $C9CE  86 00

_label_c9d0:
  nop                            ; $C9D0  EA
  clv                            ; $C9D1  B8
  cmp #$3F                       ; $C9D2  C9 3F
  beq _label_c9df                ; $C9D4  F0 09
  bmi _label_c9df                ; $C9D6  30 07
  bcc _label_c9df                ; $C9D8  90 05
  bvs _label_c9df                ; $C9DA  70 03
  jmp _label_c9e3                ; $C9DC  4C E3 C9

_label_c9df:
  ldx #$25                       ; $C9DF  A2 25
  stx z:_var_0000_indexed        ; $C9E1  86 00

_label_c9e3:
  nop                            ; $C9E3  EA
  cmp #$41                       ; $C9E4  C9 41
  beq _label_c9ef                ; $C9E6  F0 07
  bpl _label_c9ef                ; $C9E8  10 05
  bpl _label_c9ef                ; $C9EA  10 03
  jmp _label_c9f3                ; $C9EC  4C F3 C9

_label_c9ef:
  ldx #$26                       ; $C9EF  A2 26
  stx z:_var_0000_indexed        ; $C9F1  86 00

_label_c9f3:
  nop                            ; $C9F3  EA
  lda #$80                       ; $C9F4  A9 80
  cmp #$00                       ; $C9F6  C9 00
  beq _label_ca01                ; $C9F8  F0 07
  bpl _label_ca01                ; $C9FA  10 05
  bcc _label_ca01                ; $C9FC  90 03
  jmp _label_ca05                ; $C9FE  4C 05 CA

_label_ca01:
  ldx #$27                       ; $CA01  A2 27
  stx z:_var_0000_indexed        ; $CA03  86 00

_label_ca05:
  nop                            ; $CA05  EA
  cmp #$80                       ; $CA06  C9 80
  bne _label_ca11                ; $CA08  D0 07
  bmi _label_ca11                ; $CA0A  30 05
  bcc _label_ca11                ; $CA0C  90 03
  jmp _label_ca15                ; $CA0E  4C 15 CA

_label_ca11:
  ldx #$28                       ; $CA11  A2 28
  stx z:_var_0000_indexed        ; $CA13  86 00

_label_ca15:
  nop                            ; $CA15  EA
  cmp #$81                       ; $CA16  C9 81
  bcs _label_ca21                ; $CA18  B0 07
  beq _label_ca21                ; $CA1A  F0 05
  bpl _label_ca21                ; $CA1C  10 03
  jmp _label_ca25                ; $CA1E  4C 25 CA

_label_ca21:
  ldx #$29                       ; $CA21  A2 29
  stx z:_var_0000_indexed        ; $CA23  86 00

_label_ca25:
  nop                            ; $CA25  EA
  cmp #$7F                       ; $CA26  C9 7F
  bcc _label_ca31                ; $CA28  90 07
  beq _label_ca31                ; $CA2A  F0 05
  bmi _label_ca31                ; $CA2C  30 03
  jmp _label_ca35                ; $CA2E  4C 35 CA

_label_ca31:
  ldx #$2A                       ; $CA31  A2 2A
  stx z:_var_0000_indexed        ; $CA33  86 00

_label_ca35:
  nop                            ; $CA35  EA
  bit z:_var_0001                ; $CA36  24 01
  ldy #$40                       ; $CA38  A0 40
  cpy #$40                       ; $CA3A  C0 40
  bne _label_ca47                ; $CA3C  D0 09
  bmi _label_ca47                ; $CA3E  30 07
  bcc _label_ca47                ; $CA40  90 05
  bvc _label_ca47                ; $CA42  50 03
  jmp _label_ca4b                ; $CA44  4C 4B CA

_label_ca47:
  ldx #$2B                       ; $CA47  A2 2B
  stx z:_var_0000_indexed        ; $CA49  86 00

_label_ca4b:
  nop                            ; $CA4B  EA
  clv                            ; $CA4C  B8
  cpy #$3F                       ; $CA4D  C0 3F
  beq _label_ca5a                ; $CA4F  F0 09
  bmi _label_ca5a                ; $CA51  30 07
  bcc _label_ca5a                ; $CA53  90 05
  bvs _label_ca5a                ; $CA55  70 03
  jmp _label_ca5e                ; $CA57  4C 5E CA

_label_ca5a:
  ldx #$2C                       ; $CA5A  A2 2C
  stx z:_var_0000_indexed        ; $CA5C  86 00

_label_ca5e:
  nop                            ; $CA5E  EA
  cpy #$41                       ; $CA5F  C0 41
  beq _label_ca6a                ; $CA61  F0 07
  bpl _label_ca6a                ; $CA63  10 05
  bpl _label_ca6a                ; $CA65  10 03
  jmp _label_ca6e                ; $CA67  4C 6E CA

_label_ca6a:
  ldx #$2D                       ; $CA6A  A2 2D
  stx z:_var_0000_indexed        ; $CA6C  86 00

_label_ca6e:
  nop                            ; $CA6E  EA
  ldy #$80                       ; $CA6F  A0 80
  cpy #$00                       ; $CA71  C0 00
  beq _label_ca7c                ; $CA73  F0 07
  bpl _label_ca7c                ; $CA75  10 05
  bcc _label_ca7c                ; $CA77  90 03
  jmp _label_ca80                ; $CA79  4C 80 CA

_label_ca7c:
  ldx #$2E                       ; $CA7C  A2 2E
  stx z:_var_0000_indexed        ; $CA7E  86 00

_label_ca80:
  nop                            ; $CA80  EA
  cpy #$80                       ; $CA81  C0 80
  bne _label_ca8c                ; $CA83  D0 07
  bmi _label_ca8c                ; $CA85  30 05
  bcc _label_ca8c                ; $CA87  90 03
  jmp _label_ca90                ; $CA89  4C 90 CA

_label_ca8c:
  ldx #$2F                       ; $CA8C  A2 2F
  stx z:_var_0000_indexed        ; $CA8E  86 00

_label_ca90:
  nop                            ; $CA90  EA
  cpy #$81                       ; $CA91  C0 81
  bcs _label_ca9c                ; $CA93  B0 07
  beq _label_ca9c                ; $CA95  F0 05
  bpl _label_ca9c                ; $CA97  10 03
  jmp _label_caa0                ; $CA99  4C A0 CA

_label_ca9c:
  ldx #$30                       ; $CA9C  A2 30
  stx z:_var_0000_indexed        ; $CA9E  86 00

_label_caa0:
  nop                            ; $CAA0  EA
  cpy #$7F                       ; $CAA1  C0 7F
  bcc _label_caac                ; $CAA3  90 07
  beq _label_caac                ; $CAA5  F0 05
  bmi _label_caac                ; $CAA7  30 03
  jmp _label_cab0                ; $CAA9  4C B0 CA

_label_caac:
  ldx #$31                       ; $CAAC  A2 31
  stx z:_var_0000_indexed        ; $CAAE  86 00

_label_cab0:
  nop                            ; $CAB0  EA
  bit z:_var_0001                ; $CAB1  24 01
  ldx #$40                       ; $CAB3  A2 40
  cpx #$40                       ; $CAB5  E0 40
  bne _label_cac2                ; $CAB7  D0 09
  bmi _label_cac2                ; $CAB9  30 07
  bcc _label_cac2                ; $CABB  90 05
  bvc _label_cac2                ; $CABD  50 03
  jmp _label_cac6                ; $CABF  4C C6 CA

_label_cac2:
  lda #$32                       ; $CAC2  A9 32
  sta z:_var_0000_indexed        ; $CAC4  85 00

_label_cac6:
  nop                            ; $CAC6  EA
  clv                            ; $CAC7  B8
  cpx #$3F                       ; $CAC8  E0 3F
  beq _label_cad5                ; $CACA  F0 09
  bmi _label_cad5                ; $CACC  30 07
  bcc _label_cad5                ; $CACE  90 05
  bvs _label_cad5                ; $CAD0  70 03
  jmp _label_cad9                ; $CAD2  4C D9 CA

_label_cad5:
  lda #$33                       ; $CAD5  A9 33
  sta z:_var_0000_indexed        ; $CAD7  85 00

_label_cad9:
  nop                            ; $CAD9  EA
  cpx #$41                       ; $CADA  E0 41
  beq _label_cae5                ; $CADC  F0 07
  bpl _label_cae5                ; $CADE  10 05
  bpl _label_cae5                ; $CAE0  10 03
  jmp _label_cae9                ; $CAE2  4C E9 CA

_label_cae5:
  lda #$34                       ; $CAE5  A9 34
  sta z:_var_0000_indexed        ; $CAE7  85 00

_label_cae9:
  nop                            ; $CAE9  EA
  ldx #$80                       ; $CAEA  A2 80
  cpx #$00                       ; $CAEC  E0 00
  beq _label_caf7                ; $CAEE  F0 07
  bpl _label_caf7                ; $CAF0  10 05
  bcc _label_caf7                ; $CAF2  90 03
  jmp _label_cafb                ; $CAF4  4C FB CA

_label_caf7:
  lda #$35                       ; $CAF7  A9 35
  sta z:_var_0000_indexed        ; $CAF9  85 00

_label_cafb:
  nop                            ; $CAFB  EA
  cpx #$80                       ; $CAFC  E0 80
  bne _label_cb07                ; $CAFE  D0 07
  bmi _label_cb07                ; $CB00  30 05
  bcc _label_cb07                ; $CB02  90 03
  jmp _label_cb0b                ; $CB04  4C 0B CB

_label_cb07:
  lda #$36                       ; $CB07  A9 36
  sta z:_var_0000_indexed        ; $CB09  85 00

_label_cb0b:
  nop                            ; $CB0B  EA
  cpx #$81                       ; $CB0C  E0 81
  bcs _label_cb17                ; $CB0E  B0 07
  beq _label_cb17                ; $CB10  F0 05
  bpl _label_cb17                ; $CB12  10 03
  jmp _label_cb1b                ; $CB14  4C 1B CB

_label_cb17:
  lda #$37                       ; $CB17  A9 37
  sta z:_var_0000_indexed        ; $CB19  85 00

_label_cb1b:
  nop                            ; $CB1B  EA
  cpx #$7F                       ; $CB1C  E0 7F
  bcc _label_cb27                ; $CB1E  90 07
  beq _label_cb27                ; $CB20  F0 05
  bmi _label_cb27                ; $CB22  30 03
  jmp _label_cb2b                ; $CB24  4C 2B CB

_label_cb27:
  lda #$38                       ; $CB27  A9 38
  sta z:_var_0000_indexed        ; $CB29  85 00

_label_cb2b:
  nop                            ; $CB2B  EA
  sec                            ; $CB2C  38
  clv                            ; $CB2D  B8
  ldx #$9F                       ; $CB2E  A2 9F
  beq _label_cb3b                ; $CB30  F0 09
  bpl _label_cb3b                ; $CB32  10 07
  bvs _label_cb3b                ; $CB34  70 05
  bcc _label_cb3b                ; $CB36  90 03
  jmp _label_cb3f                ; $CB38  4C 3F CB

_label_cb3b:
  ldx #$39                       ; $CB3B  A2 39
  stx z:_var_0000_indexed        ; $CB3D  86 00

_label_cb3f:
  nop                            ; $CB3F  EA
  clc                            ; $CB40  18
  bit z:_var_0001                ; $CB41  24 01
  ldx #$00                       ; $CB43  A2 00
  bne _label_cb50                ; $CB45  D0 09
  bmi _label_cb50                ; $CB47  30 07
  bvc _label_cb50                ; $CB49  50 05
  bcs _label_cb50                ; $CB4B  B0 03
  jmp _label_cb54                ; $CB4D  4C 54 CB

_label_cb50:
  ldx #$3A                       ; $CB50  A2 3A
  stx z:_var_0000_indexed        ; $CB52  86 00

_label_cb54:
  nop                            ; $CB54  EA
  sec                            ; $CB55  38
  clv                            ; $CB56  B8
  ldy #$9F                       ; $CB57  A0 9F
  beq _label_cb64                ; $CB59  F0 09
  bpl _label_cb64                ; $CB5B  10 07
  bvs _label_cb64                ; $CB5D  70 05
  bcc _label_cb64                ; $CB5F  90 03
  jmp _label_cb68                ; $CB61  4C 68 CB

_label_cb64:
  ldx #$3B                       ; $CB64  A2 3B
  stx z:_var_0000_indexed        ; $CB66  86 00

_label_cb68:
  nop                            ; $CB68  EA
  clc                            ; $CB69  18
  bit z:_var_0001                ; $CB6A  24 01
  ldy #$00                       ; $CB6C  A0 00
  bne _label_cb79                ; $CB6E  D0 09
  bmi _label_cb79                ; $CB70  30 07
  bvc _label_cb79                ; $CB72  50 05
  bcs _label_cb79                ; $CB74  B0 03
  jmp _label_cb7d                ; $CB76  4C 7D CB

_label_cb79:
  ldx #$3C                       ; $CB79  A2 3C
  stx z:_var_0000_indexed        ; $CB7B  86 00

_label_cb7d:
  nop                            ; $CB7D  EA
  lda #$55                       ; $CB7E  A9 55
  ldx #$AA                       ; $CB80  A2 AA
  ldy #$33                       ; $CB82  A0 33
  cmp #$55                       ; $CB84  C9 55
  bne _label_cbab                ; $CB86  D0 23
  cpx #$AA                       ; $CB88  E0 AA
  bne _label_cbab                ; $CB8A  D0 1F
  cpy #$33                       ; $CB8C  C0 33
  bne _label_cbab                ; $CB8E  D0 1B
  cmp #$55                       ; $CB90  C9 55
  bne _label_cbab                ; $CB92  D0 17
  cpx #$AA                       ; $CB94  E0 AA
  bne _label_cbab                ; $CB96  D0 13
  cpy #$33                       ; $CB98  C0 33
  bne _label_cbab                ; $CB9A  D0 0F
  cmp #$56                       ; $CB9C  C9 56
  beq _label_cbab                ; $CB9E  F0 0B
  cpx #$AB                       ; $CBA0  E0 AB
  beq _label_cbab                ; $CBA2  F0 07
  cpy #$34                       ; $CBA4  C0 34
  beq _label_cbab                ; $CBA6  F0 03
  jmp _label_cbaf                ; $CBA8  4C AF CB

_label_cbab:
  ldx #$3D                       ; $CBAB  A2 3D
  stx z:_var_0000_indexed        ; $CBAD  86 00

_label_cbaf:
  ldy #$71                       ; $CBAF  A0 71
  jsr _func_f931                 ; $CBB1  20 31 F9
  sbc #$40                       ; $CBB4  E9 40
  jsr _func_f937                 ; $CBB6  20 37 F9
  iny                            ; $CBB9  C8
  jsr _func_f947                 ; $CBBA  20 47 F9
  sbc #$3F                       ; $CBBD  E9 3F
  jsr _func_f94c                 ; $CBBF  20 4C F9
  iny                            ; $CBC2  C8
  jsr _func_f95c                 ; $CBC3  20 5C F9
  sbc #$41                       ; $CBC6  E9 41
  jsr _func_f962                 ; $CBC8  20 62 F9
  iny                            ; $CBCB  C8
  jsr _func_f972                 ; $CBCC  20 72 F9
  sbc #$00                       ; $CBCF  E9 00
  jsr _func_f976                 ; $CBD1  20 76 F9
  iny                            ; $CBD4  C8
  jsr _func_f980                 ; $CBD5  20 80 F9
  sbc #$7F                       ; $CBD8  E9 7F
  jsr _func_f984                 ; $CBDA  20 84 F9
  rts                            ; $CBDD  60

_label_cbde:
  nop                            ; $CBDE  EA
  lda #$FF                       ; $CBDF  A9 FF
  sta z:_var_0001                ; $CBE1  85 01
  lda #$44                       ; $CBE3  A9 44
  ldx #$55                       ; $CBE5  A2 55
  ldy #$66                       ; $CBE7  A0 66
  inx                            ; $CBE9  E8
  dey                            ; $CBEA  88
  cpx #$56                       ; $CBEB  E0 56
  bne _label_cc10                ; $CBED  D0 21
  cpy #$65                       ; $CBEF  C0 65
  bne _label_cc10                ; $CBF1  D0 1D
  inx                            ; $CBF3  E8
  inx                            ; $CBF4  E8
  dey                            ; $CBF5  88
  dey                            ; $CBF6  88
  cpx #$58                       ; $CBF7  E0 58
  bne _label_cc10                ; $CBF9  D0 15
  cpy #$63                       ; $CBFB  C0 63
  bne _label_cc10                ; $CBFD  D0 11
  dex                            ; $CBFF  CA
  iny                            ; $CC00  C8
  cpx #$57                       ; $CC01  E0 57
  bne _label_cc10                ; $CC03  D0 0B
  cpy #$64                       ; $CC05  C0 64
  bne _label_cc10                ; $CC07  D0 07
  cmp #$44                       ; $CC09  C9 44
  bne _label_cc10                ; $CC0B  D0 03
  jmp _label_cc14                ; $CC0D  4C 14 CC

_label_cc10:
  ldx #$3E                       ; $CC10  A2 3E
  stx z:_var_0000_indexed        ; $CC12  86 00

_label_cc14:
  nop                            ; $CC14  EA
  sec                            ; $CC15  38
  ldx #$69                       ; $CC16  A2 69
  lda #$96                       ; $CC18  A9 96
  bit z:_var_0001                ; $CC1A  24 01
  ldy #$FF                       ; $CC1C  A0 FF
  iny                            ; $CC1E  C8
  bne _label_cc5e                ; $CC1F  D0 3D
  bmi _label_cc5e                ; $CC21  30 3B
  bcc _label_cc5e                ; $CC23  90 39
  bvc _label_cc5e                ; $CC25  50 37
  cpy #$00                       ; $CC27  C0 00
  bne _label_cc5e                ; $CC29  D0 33
  iny                            ; $CC2B  C8
  beq _label_cc5e                ; $CC2C  F0 30
  bmi _label_cc5e                ; $CC2E  30 2E
  bcc _label_cc5e                ; $CC30  90 2C
  bvc _label_cc5e                ; $CC32  50 2A
  clc                            ; $CC34  18
  clv                            ; $CC35  B8
  ldy #$00                       ; $CC36  A0 00
  dey                            ; $CC38  88
  beq _label_cc5e                ; $CC39  F0 23
  bpl _label_cc5e                ; $CC3B  10 21
  bcs _label_cc5e                ; $CC3D  B0 1F
  bvs _label_cc5e                ; $CC3F  70 1D
  cpy #$FF                       ; $CC41  C0 FF
  bne _label_cc5e                ; $CC43  D0 19
  clc                            ; $CC45  18
  dey                            ; $CC46  88
  beq _label_cc5e                ; $CC47  F0 15
  bpl _label_cc5e                ; $CC49  10 13
  bcs _label_cc5e                ; $CC4B  B0 11
  bvs _label_cc5e                ; $CC4D  70 0F
  cpy #$FE                       ; $CC4F  C0 FE
  bne _label_cc5e                ; $CC51  D0 0B
  cmp #$96                       ; $CC53  C9 96
  bne _label_cc5e                ; $CC55  D0 07
  cpx #$69                       ; $CC57  E0 69
  bne _label_cc5e                ; $CC59  D0 03
  jmp _label_cc62                ; $CC5B  4C 62 CC

_label_cc5e:
  ldx #$3F                       ; $CC5E  A2 3F
  stx z:_var_0000_indexed        ; $CC60  86 00

_label_cc62:
  nop                            ; $CC62  EA
  sec                            ; $CC63  38
  ldy #$69                       ; $CC64  A0 69
  lda #$96                       ; $CC66  A9 96
  bit z:_var_0001                ; $CC68  24 01
  ldx #$FF                       ; $CC6A  A2 FF
  inx                            ; $CC6C  E8
  bne _label_ccac                ; $CC6D  D0 3D
  bmi _label_ccac                ; $CC6F  30 3B
  bcc _label_ccac                ; $CC71  90 39
  bvc _label_ccac                ; $CC73  50 37
  cpx #$00                       ; $CC75  E0 00
  bne _label_ccac                ; $CC77  D0 33
  inx                            ; $CC79  E8
  beq _label_ccac                ; $CC7A  F0 30
  bmi _label_ccac                ; $CC7C  30 2E
  bcc _label_ccac                ; $CC7E  90 2C
  bvc _label_ccac                ; $CC80  50 2A
  clc                            ; $CC82  18
  clv                            ; $CC83  B8
  ldx #$00                       ; $CC84  A2 00
  dex                            ; $CC86  CA
  beq _label_ccac                ; $CC87  F0 23
  bpl _label_ccac                ; $CC89  10 21
  bcs _label_ccac                ; $CC8B  B0 1F
  bvs _label_ccac                ; $CC8D  70 1D
  cpx #$FF                       ; $CC8F  E0 FF
  bne _label_ccac                ; $CC91  D0 19
  clc                            ; $CC93  18
  dex                            ; $CC94  CA
  beq _label_ccac                ; $CC95  F0 15
  bpl _label_ccac                ; $CC97  10 13
  bcs _label_ccac                ; $CC99  B0 11
  bvs _label_ccac                ; $CC9B  70 0F
  cpx #$FE                       ; $CC9D  E0 FE
  bne _label_ccac                ; $CC9F  D0 0B
  cmp #$96                       ; $CCA1  C9 96
  bne _label_ccac                ; $CCA3  D0 07
  cpy #$69                       ; $CCA5  C0 69
  bne _label_ccac                ; $CCA7  D0 03
  jmp _label_ccb0                ; $CCA9  4C B0 CC

_label_ccac:
  ldx #$40                       ; $CCAC  A2 40
  stx z:_var_0000_indexed        ; $CCAE  86 00

_label_ccb0:
  nop                            ; $CCB0  EA
  lda #$85                       ; $CCB1  A9 85
  ldx #$34                       ; $CCB3  A2 34
  ldy #$99                       ; $CCB5  A0 99
  clc                            ; $CCB7  18
  bit z:_var_0001                ; $CCB8  24 01
  tay                            ; $CCBA  A8
  beq _label_cceb                ; $CCBB  F0 2E
  bcs _label_cceb                ; $CCBD  B0 2C
  bvc _label_cceb                ; $CCBF  50 2A
  bpl _label_cceb                ; $CCC1  10 28
  cmp #$85                       ; $CCC3  C9 85
  bne _label_cceb                ; $CCC5  D0 24
  cpx #$34                       ; $CCC7  E0 34
  bne _label_cceb                ; $CCC9  D0 20
  cpy #$85                       ; $CCCB  C0 85
  bne _label_cceb                ; $CCCD  D0 1C
  lda #$00                       ; $CCCF  A9 00
  sec                            ; $CCD1  38
  clv                            ; $CCD2  B8
  tay                            ; $CCD3  A8
  bne _label_cceb                ; $CCD4  D0 15
  bcc _label_cceb                ; $CCD6  90 13
  bvs _label_cceb                ; $CCD8  70 11
  bmi _label_cceb                ; $CCDA  30 0F
  cmp #$00                       ; $CCDC  C9 00
  bne _label_cceb                ; $CCDE  D0 0B
  cpx #$34                       ; $CCE0  E0 34
  bne _label_cceb                ; $CCE2  D0 07
  cpy #$00                       ; $CCE4  C0 00
  bne _label_cceb                ; $CCE6  D0 03
  jmp _label_ccef                ; $CCE8  4C EF CC

_label_cceb:
  ldx #$41                       ; $CCEB  A2 41
  stx z:_var_0000_indexed        ; $CCED  86 00

_label_ccef:
  nop                            ; $CCEF  EA
  lda #$85                       ; $CCF0  A9 85
  ldx #$34                       ; $CCF2  A2 34
  ldy #$99                       ; $CCF4  A0 99
  clc                            ; $CCF6  18
  bit z:_var_0001                ; $CCF7  24 01
  tax                            ; $CCF9  AA
  beq _label_cd2a                ; $CCFA  F0 2E
  bcs _label_cd2a                ; $CCFC  B0 2C
  bvc _label_cd2a                ; $CCFE  50 2A
  bpl _label_cd2a                ; $CD00  10 28
  cmp #$85                       ; $CD02  C9 85
  bne _label_cd2a                ; $CD04  D0 24
  cpx #$85                       ; $CD06  E0 85
  bne _label_cd2a                ; $CD08  D0 20
  cpy #$99                       ; $CD0A  C0 99
  bne _label_cd2a                ; $CD0C  D0 1C
  lda #$00                       ; $CD0E  A9 00
  sec                            ; $CD10  38
  clv                            ; $CD11  B8
  tax                            ; $CD12  AA
  bne _label_cd2a                ; $CD13  D0 15
  bcc _label_cd2a                ; $CD15  90 13
  bvs _label_cd2a                ; $CD17  70 11
  bmi _label_cd2a                ; $CD19  30 0F
  cmp #$00                       ; $CD1B  C9 00
  bne _label_cd2a                ; $CD1D  D0 0B
  cpx #$00                       ; $CD1F  E0 00
  bne _label_cd2a                ; $CD21  D0 07
  cpy #$99                       ; $CD23  C0 99
  bne _label_cd2a                ; $CD25  D0 03
  jmp _label_cd2e                ; $CD27  4C 2E CD

_label_cd2a:
  ldx #$42                       ; $CD2A  A2 42
  stx z:_var_0000_indexed        ; $CD2C  86 00

_label_cd2e:
  nop                            ; $CD2E  EA
  lda #$85                       ; $CD2F  A9 85
  ldx #$34                       ; $CD31  A2 34
  ldy #$99                       ; $CD33  A0 99
  clc                            ; $CD35  18
  bit z:_var_0001                ; $CD36  24 01
  tya                            ; $CD38  98
  beq _label_cd69                ; $CD39  F0 2E
  bcs _label_cd69                ; $CD3B  B0 2C
  bvc _label_cd69                ; $CD3D  50 2A
  bpl _label_cd69                ; $CD3F  10 28
  cmp #$99                       ; $CD41  C9 99
  bne _label_cd69                ; $CD43  D0 24
  cpx #$34                       ; $CD45  E0 34
  bne _label_cd69                ; $CD47  D0 20
  cpy #$99                       ; $CD49  C0 99
  bne _label_cd69                ; $CD4B  D0 1C
  ldy #$00                       ; $CD4D  A0 00
  sec                            ; $CD4F  38
  clv                            ; $CD50  B8
  tya                            ; $CD51  98
  bne _label_cd69                ; $CD52  D0 15
  bcc _label_cd69                ; $CD54  90 13
  bvs _label_cd69                ; $CD56  70 11
  bmi _label_cd69                ; $CD58  30 0F
  cmp #$00                       ; $CD5A  C9 00
  bne _label_cd69                ; $CD5C  D0 0B
  cpx #$34                       ; $CD5E  E0 34
  bne _label_cd69                ; $CD60  D0 07
  cpy #$00                       ; $CD62  C0 00
  bne _label_cd69                ; $CD64  D0 03
  jmp _label_cd6d                ; $CD66  4C 6D CD

_label_cd69:
  ldx #$43                       ; $CD69  A2 43
  stx z:_var_0000_indexed        ; $CD6B  86 00

_label_cd6d:
  nop                            ; $CD6D  EA
  lda #$85                       ; $CD6E  A9 85
  ldx #$34                       ; $CD70  A2 34
  ldy #$99                       ; $CD72  A0 99
  clc                            ; $CD74  18
  bit z:_var_0001                ; $CD75  24 01
  txa                            ; $CD77  8A
  beq _label_cda8                ; $CD78  F0 2E
  bcs _label_cda8                ; $CD7A  B0 2C
  bvc _label_cda8                ; $CD7C  50 2A
  bmi _label_cda8                ; $CD7E  30 28
  cmp #$34                       ; $CD80  C9 34
  bne _label_cda8                ; $CD82  D0 24
  cpx #$34                       ; $CD84  E0 34
  bne _label_cda8                ; $CD86  D0 20
  cpy #$99                       ; $CD88  C0 99
  bne _label_cda8                ; $CD8A  D0 1C
  ldx #$00                       ; $CD8C  A2 00
  sec                            ; $CD8E  38
  clv                            ; $CD8F  B8
  txa                            ; $CD90  8A
  bne _label_cda8                ; $CD91  D0 15
  bcc _label_cda8                ; $CD93  90 13
  bvs _label_cda8                ; $CD95  70 11
  bmi _label_cda8                ; $CD97  30 0F
  cmp #$00                       ; $CD99  C9 00
  bne _label_cda8                ; $CD9B  D0 0B
  cpx #$00                       ; $CD9D  E0 00
  bne _label_cda8                ; $CD9F  D0 07
  cpy #$99                       ; $CDA1  C0 99
  bne _label_cda8                ; $CDA3  D0 03
  jmp _label_cdac                ; $CDA5  4C AC CD

_label_cda8:
  ldx #$44                       ; $CDA8  A2 44
  stx z:_var_0000_indexed        ; $CDAA  86 00

_label_cdac:
  nop                            ; $CDAC  EA
  tsx                            ; $CDAD  BA
  stx a:_var_07ff                ; $CDAE  8E FF 07
  ldy #$33                       ; $CDB1  A0 33
  ldx #$69                       ; $CDB3  A2 69
  lda #$84                       ; $CDB5  A9 84
  clc                            ; $CDB7  18
  bit z:_var_0001                ; $CDB8  24 01
  txs                            ; $CDBA  9A
  beq _label_cdef                ; $CDBB  F0 32
  bpl _label_cdef                ; $CDBD  10 30
  bcs _label_cdef                ; $CDBF  B0 2E
  bvc _label_cdef                ; $CDC1  50 2C
  cmp #$84                       ; $CDC3  C9 84
  bne _label_cdef                ; $CDC5  D0 28
  cpx #$69                       ; $CDC7  E0 69
  bne _label_cdef                ; $CDC9  D0 24
  cpy #$33                       ; $CDCB  C0 33
  bne _label_cdef                ; $CDCD  D0 20
  ldy #$01                       ; $CDCF  A0 01
  lda #$04                       ; $CDD1  A9 04
  sec                            ; $CDD3  38
  clv                            ; $CDD4  B8
  ldx #$00                       ; $CDD5  A2 00
  tsx                            ; $CDD7  BA
  beq _label_cdef                ; $CDD8  F0 15
  bmi _label_cdef                ; $CDDA  30 13
  bcc _label_cdef                ; $CDDC  90 11
  bvs _label_cdef                ; $CDDE  70 0F
  cpx #$69                       ; $CDE0  E0 69
  bne _label_cdef                ; $CDE2  D0 0B
  cmp #$04                       ; $CDE4  C9 04
  bne _label_cdef                ; $CDE6  D0 07
  cpy #$01                       ; $CDE8  C0 01
  bne _label_cdef                ; $CDEA  D0 03
  jmp _label_cdf3                ; $CDEC  4C F3 CD

_label_cdef:
  ldx #$45                       ; $CDEF  A2 45
  stx z:_var_0000_indexed        ; $CDF1  86 00

_label_cdf3:
  ldx a:_var_07ff                ; $CDF3  AE FF 07
  txs                            ; $CDF6  9A
  rts                            ; $CDF7  60

_label_cdf8:
  lda #$FF                       ; $CDF8  A9 FF
  sta z:_var_0001                ; $CDFA  85 01
  tsx                            ; $CDFC  BA
  stx a:_var_07ff                ; $CDFD  8E FF 07
  nop                            ; $CE00  EA
  ldx #$80                       ; $CE01  A2 80
  txs                            ; $CE03  9A
  lda #$33                       ; $CE04  A9 33
  pha                            ; $CE06  48
  lda #$69                       ; $CE07  A9 69
  pha                            ; $CE09  48
  tsx                            ; $CE0A  BA
  cpx #$7E                       ; $CE0B  E0 7E
  bne _label_ce2f                ; $CE0D  D0 20
  pla                            ; $CE0F  68
  cmp #$69                       ; $CE10  C9 69
  bne _label_ce2f                ; $CE12  D0 1B
  pla                            ; $CE14  68
  cmp #$33                       ; $CE15  C9 33
  bne _label_ce2f                ; $CE17  D0 16
  tsx                            ; $CE19  BA
  cpx #$80                       ; $CE1A  E0 80
  bne _label_ce2f                ; $CE1C  D0 11
  lda a:$0180                    ; $CE1E  AD 80 01
  cmp #$33                       ; $CE21  C9 33
  bne _label_ce2f                ; $CE23  D0 0A
  lda a:$017F                    ; $CE25  AD 7F 01
  cmp #$69                       ; $CE28  C9 69
  bne _label_ce2f                ; $CE2A  D0 03
  jmp _label_ce33                ; $CE2C  4C 33 CE

_label_ce2f:
  ldx #$46                       ; $CE2F  A2 46
  stx z:_var_0000_indexed        ; $CE31  86 00

_label_ce33:
  nop                            ; $CE33  EA
  ldx #$80                       ; $CE34  A2 80
  txs                            ; $CE36  9A
  jsr _func_ce3d                 ; $CE37  20 3D CE
  jmp _label_ce5b                ; $CE3A  4C 5B CE

_func_ce3d:
  tsx                            ; $CE3D  BA
  cpx #$7E                       ; $CE3E  E0 7E
  bne _label_ce5b                ; $CE40  D0 19
  pla                            ; $CE42  68
  pla                            ; $CE43  68
  tsx                            ; $CE44  BA
  cpx #$80                       ; $CE45  E0 80
  bne _label_ce5b                ; $CE47  D0 12
  lda #$00                       ; $CE49  A9 00
  jsr _func_ce4e                 ; $CE4B  20 4E CE

_func_ce4e:
  pla                            ; $CE4E  68
  cmp #$4D                       ; $CE4F  C9 4D
  bne _label_ce5b                ; $CE51  D0 08
  pla                            ; $CE53  68
  cmp #$CE                       ; $CE54  C9 CE
  bne _label_ce5b                ; $CE56  D0 03
  jmp _label_ce5f                ; $CE58  4C 5F CE

_label_ce5b:
  ldx #$47                       ; $CE5B  A2 47
  stx z:_var_0000_indexed        ; $CE5D  86 00

_label_ce5f:
  nop                            ; $CE5F  EA
  lda #$CE                       ; $CE60  A9 CE
  pha                            ; $CE62  48
  lda #$66                       ; $CE63  A9 66
  pha                            ; $CE65  48
  rts                            ; $CE66  60

.byte $a2, $77, $a0, $69, $18, $24, $01, $a9, $83, $20, $66, $ce, $f0, $24, $10, $22 ; $CE67
.byte $b0, $20, $50, $1e, $c9, $83, $d0, $1a, $c0, $69, $d0, $16, $e0, $77, $d0, $12 ; $CE77
.byte $38, $b8, $a9, $00, $20, $66, $ce, $d0, $09, $30, $07, $90, $05, $70, $03, $4c ; $CE87
.byte $9d, $ce, $a2, $48, $86, $00, $ea, $a9, $ce, $48, $a9, $ae, $48, $a9, $65, $48 ; $CE97
.byte $a9, $55, $a0, $88, $a2, $99, $40, $30, $35, $50, $33, $f0, $31, $90, $2f, $c9 ; $CEA7
.byte $55, $d0, $2b, $c0, $88, $d0, $27, $e0, $99, $d0, $23, $a9, $ce, $48, $a9, $ce ; $CEB7
.byte $48, $a9, $87, $48, $a9, $55, $40, $10, $15, $70, $13, $d0, $11, $90, $0f, $c9 ; $CEC7
.byte $55, $d0, $0b, $c0, $88, $d0, $07, $e0, $99, $d0, $03, $4c, $e9, $ce, $a2, $49 ; $CED7
.byte $86, $00, $ae, $ff, $07, $9a, $60 ; $CEE7

_label_ceee:
  ldx #$55                       ; $CEEE  A2 55
  ldy #$69                       ; $CEF0  A0 69
  lda #$FF                       ; $CEF2  A9 FF
  sta z:_var_0001                ; $CEF4  85 01
  nop                            ; $CEF6  EA
  bit z:_var_0001                ; $CEF7  24 01
  sec                            ; $CEF9  38
  lda #$01                       ; $CEFA  A9 01
  lsr a                          ; $CEFC  4A
  bcc _label_cf1c                ; $CEFD  90 1D
  bne _label_cf1c                ; $CEFF  D0 1B
  bmi _label_cf1c                ; $CF01  30 19
  bvc _label_cf1c                ; $CF03  50 17
  cmp #$00                       ; $CF05  C9 00
  bne _label_cf1c                ; $CF07  D0 13
  clv                            ; $CF09  B8
  lda #$AA                       ; $CF0A  A9 AA
  lsr a                          ; $CF0C  4A
  bcs _label_cf1c                ; $CF0D  B0 0D
  beq _label_cf1c                ; $CF0F  F0 0B
  bmi _label_cf1c                ; $CF11  30 09
  bvs _label_cf1c                ; $CF13  70 07
  cmp #$55                       ; $CF15  C9 55
  bne _label_cf1c                ; $CF17  D0 03
  jmp _label_cf20                ; $CF19  4C 20 CF

_label_cf1c:
  ldx #$4A                       ; $CF1C  A2 4A
  stx z:_var_0000_indexed        ; $CF1E  86 00

_label_cf20:
  nop                            ; $CF20  EA
  bit z:_var_0001                ; $CF21  24 01
  sec                            ; $CF23  38
  lda #$80                       ; $CF24  A9 80
  asl a                          ; $CF26  0A
  bcc _label_cf47                ; $CF27  90 1E
  bne _label_cf47                ; $CF29  D0 1C
  bmi _label_cf47                ; $CF2B  30 1A
  bvc _label_cf47                ; $CF2D  50 18
  cmp #$00                       ; $CF2F  C9 00
  bne _label_cf47                ; $CF31  D0 14
  clv                            ; $CF33  B8
  sec                            ; $CF34  38
  lda #$55                       ; $CF35  A9 55
  asl a                          ; $CF37  0A
  bcs _label_cf47                ; $CF38  B0 0D
  beq _label_cf47                ; $CF3A  F0 0B
  bpl _label_cf47                ; $CF3C  10 09
  bvs _label_cf47                ; $CF3E  70 07
  cmp #$AA                       ; $CF40  C9 AA
  bne _label_cf47                ; $CF42  D0 03
  jmp _label_cf4b                ; $CF44  4C 4B CF

_label_cf47:
  ldx #$4B                       ; $CF47  A2 4B
  stx z:_var_0000_indexed        ; $CF49  86 00

_label_cf4b:
  nop                            ; $CF4B  EA
  bit z:_var_0001                ; $CF4C  24 01
  sec                            ; $CF4E  38
  lda #$01                       ; $CF4F  A9 01
  ror a                          ; $CF51  6A
  bcc _label_cf72                ; $CF52  90 1E
  beq _label_cf72                ; $CF54  F0 1C
  bpl _label_cf72                ; $CF56  10 1A
  bvc _label_cf72                ; $CF58  50 18
  cmp #$80                       ; $CF5A  C9 80
  bne _label_cf72                ; $CF5C  D0 14
  clv                            ; $CF5E  B8
  clc                            ; $CF5F  18
  lda #$55                       ; $CF60  A9 55
  ror a                          ; $CF62  6A
  bcc _label_cf72                ; $CF63  90 0D
  beq _label_cf72                ; $CF65  F0 0B
  bmi _label_cf72                ; $CF67  30 09
  bvs _label_cf72                ; $CF69  70 07
  cmp #$2A                       ; $CF6B  C9 2A
  bne _label_cf72                ; $CF6D  D0 03
  jmp _label_cf76                ; $CF6F  4C 76 CF

_label_cf72:
  ldx #$4C                       ; $CF72  A2 4C
  stx z:_var_0000_indexed        ; $CF74  86 00

_label_cf76:
  nop                            ; $CF76  EA
  bit z:_var_0001                ; $CF77  24 01
  sec                            ; $CF79  38
  lda #$80                       ; $CF7A  A9 80
  rol a                          ; $CF7C  2A
  bcc _label_cf9d                ; $CF7D  90 1E
  beq _label_cf9d                ; $CF7F  F0 1C
  bmi _label_cf9d                ; $CF81  30 1A
  bvc _label_cf9d                ; $CF83  50 18
  cmp #$01                       ; $CF85  C9 01
  bne _label_cf9d                ; $CF87  D0 14
  clv                            ; $CF89  B8
  clc                            ; $CF8A  18
  lda #$55                       ; $CF8B  A9 55
  rol a                          ; $CF8D  2A
  bcs _label_cf9d                ; $CF8E  B0 0D
  beq _label_cf9d                ; $CF90  F0 0B
  bpl _label_cf9d                ; $CF92  10 09
  bvs _label_cf9d                ; $CF94  70 07
  cmp #$AA                       ; $CF96  C9 AA
  bne _label_cf9d                ; $CF98  D0 03
  jmp _label_cfa1                ; $CF9A  4C A1 CF

_label_cf9d:
  ldx #$4D                       ; $CF9D  A2 4D
  stx z:_var_0000_indexed        ; $CF9F  86 00

_label_cfa1:
  rts                            ; $CFA1  60

_label_cfa2:
  lda z:_var_0000_indexed        ; $CFA2  A5 00
  sta a:_var_07ff                ; $CFA4  8D FF 07
  lda #$00                       ; $CFA7  A9 00
  sta z:_var_0080_indexed        ; $CFA9  85 80
  lda #$02                       ; $CFAB  A9 02
  sta z:$81                      ; $CFAD  85 81
  lda #$FF                       ; $CFAF  A9 FF
  sta z:_var_0001                ; $CFB1  85 01
  lda #$00                       ; $CFB3  A9 00
  sta z:_var_0082_indexed        ; $CFB5  85 82
  lda #$03                       ; $CFB7  A9 03
  sta z:$83                      ; $CFB9  85 83
  sta z:$84                      ; $CFBB  85 84
  lda #$00                       ; $CFBD  A9 00
  sta z:_var_00ff_indexed        ; $CFBF  85 FF
  lda #$04                       ; $CFC1  A9 04
  sta z:_var_0000_indexed        ; $CFC3  85 00
  lda #$5A                       ; $CFC5  A9 5A
  sta a:_var_0200                ; $CFC7  8D 00 02
  lda #$5B                       ; $CFCA  A9 5B
  sta a:_var_0300_indexed        ; $CFCC  8D 00 03
  lda #$5C                       ; $CFCF  A9 5C
  sta a:_var_0303                ; $CFD1  8D 03 03
  lda #$5D                       ; $CFD4  A9 5D
  sta a:_var_0400_indexed        ; $CFD6  8D 00 04
  ldx #$00                       ; $CFD9  A2 00
  lda (_var_0080_indexed,X)      ; $CFDB  A1 80
  cmp #$5A                       ; $CFDD  C9 5A
  bne _label_d000                ; $CFDF  D0 1F
  inx                            ; $CFE1  E8
  inx                            ; $CFE2  E8
  lda (_var_0080_indexed,X)      ; $CFE3  A1 80
  cmp #$5B                       ; $CFE5  C9 5B
  bne _label_d000                ; $CFE7  D0 17
  inx                            ; $CFE9  E8
  lda (_var_0080_indexed,X)      ; $CFEA  A1 80
  cmp #$5C                       ; $CFEC  C9 5C
  bne _label_d000                ; $CFEE  D0 10
  ldx #$00                       ; $CFF0  A2 00
  lda (_var_00ff_indexed,X)      ; $CFF2  A1 FF
  cmp #$5D                       ; $CFF4  C9 5D
  bne _label_d000                ; $CFF6  D0 08
  ldx #$81                       ; $CFF8  A2 81
  lda (_var_00ff_indexed,X)      ; $CFFA  A1 FF
  cmp #$5A                       ; $CFFC  C9 5A
  beq _label_d005                ; $CFFE  F0 05

_label_d000:
  lda #$58                       ; $D000  A9 58
  sta a:_var_07ff                ; $D002  8D FF 07

_label_d005:
  lda #$AA                       ; $D005  A9 AA
  ldx #$00                       ; $D007  A2 00
  sta (_var_0080_indexed,X)      ; $D009  81 80
  inx                            ; $D00B  E8
  inx                            ; $D00C  E8
  lda #$AB                       ; $D00D  A9 AB
  sta (_var_0080_indexed,X)      ; $D00F  81 80
  inx                            ; $D011  E8
  lda #$AC                       ; $D012  A9 AC
  sta (_var_0080_indexed,X)      ; $D014  81 80
  ldx #$00                       ; $D016  A2 00
  lda #$AD                       ; $D018  A9 AD
  sta (_var_00ff_indexed,X)      ; $D01A  81 FF
  lda a:_var_0200                ; $D01C  AD 00 02
  cmp #$AA                       ; $D01F  C9 AA
  bne _label_d038                ; $D021  D0 15
  lda a:_var_0300_indexed        ; $D023  AD 00 03
  cmp #$AB                       ; $D026  C9 AB
  bne _label_d038                ; $D028  D0 0E
  lda a:_var_0303                ; $D02A  AD 03 03
  cmp #$AC                       ; $D02D  C9 AC
  bne _label_d038                ; $D02F  D0 07
  lda a:_var_0400_indexed        ; $D031  AD 00 04
  cmp #$AD                       ; $D034  C9 AD
  beq _label_d03d                ; $D036  F0 05

_label_d038:
  lda #$59                       ; $D038  A9 59
  sta a:_var_07ff                ; $D03A  8D FF 07

_label_d03d:
  lda a:_var_07ff                ; $D03D  AD FF 07
  sta z:_var_0000_indexed        ; $D040  85 00
  lda #$00                       ; $D042  A9 00
  sta a:_var_0300_indexed        ; $D044  8D 00 03
  lda #$AA                       ; $D047  A9 AA
  sta a:_var_0200                ; $D049  8D 00 02
  ldx #$00                       ; $D04C  A2 00
  ldy #$5A                       ; $D04E  A0 5A
  jsr _func_f7b6                 ; $D050  20 B6 F7
  ora (_var_0080_indexed,X)      ; $D053  01 80
  jsr _func_f7c0                 ; $D055  20 C0 F7
  iny                            ; $D058  C8
  jsr _func_f7ce                 ; $D059  20 CE F7
  ora (_var_0082_indexed,X)      ; $D05C  01 82
  jsr _func_f7d3                 ; $D05E  20 D3 F7
  iny                            ; $D061  C8
  jsr _func_f7df                 ; $D062  20 DF F7
  and (_var_0080_indexed,X)      ; $D065  21 80
  jsr _func_f7e5                 ; $D067  20 E5 F7
  iny                            ; $D06A  C8
  lda #$EF                       ; $D06B  A9 EF
  sta a:_var_0300_indexed        ; $D06D  8D 00 03
  jsr _func_f7f1                 ; $D070  20 F1 F7
  and (_var_0082_indexed,X)      ; $D073  21 82
  jsr _func_f7f6                 ; $D075  20 F6 F7
  iny                            ; $D078  C8
  jsr _func_f804                 ; $D079  20 04 F8
  eor (_var_0080_indexed,X)      ; $D07C  41 80
  jsr _func_f80a                 ; $D07E  20 0A F8
  iny                            ; $D081  C8
  lda #$70                       ; $D082  A9 70
  sta a:_var_0300_indexed        ; $D084  8D 00 03
  jsr _func_f818                 ; $D087  20 18 F8
  eor (_var_0082_indexed,X)      ; $D08A  41 82
  jsr _func_f81d                 ; $D08C  20 1D F8
  iny                            ; $D08F  C8
  lda #$69                       ; $D090  A9 69
  sta a:_var_0200                ; $D092  8D 00 02
  jsr _func_f829                 ; $D095  20 29 F8
  adc (_var_0080_indexed,X)      ; $D098  61 80
  jsr _func_f82f                 ; $D09A  20 2F F8
  iny                            ; $D09D  C8
  jsr _func_f83d                 ; $D09E  20 3D F8
  adc (_var_0080_indexed,X)      ; $D0A1  61 80
  jsr _func_f843                 ; $D0A3  20 43 F8
  iny                            ; $D0A6  C8
  lda #$7F                       ; $D0A7  A9 7F
  sta a:_var_0200                ; $D0A9  8D 00 02
  jsr _func_f851                 ; $D0AC  20 51 F8
  adc (_var_0080_indexed,X)      ; $D0AF  61 80
  jsr _func_f856                 ; $D0B1  20 56 F8
  iny                            ; $D0B4  C8
  lda #$80                       ; $D0B5  A9 80
  sta a:_var_0200                ; $D0B7  8D 00 02
  jsr _func_f864                 ; $D0BA  20 64 F8
  adc (_var_0080_indexed,X)      ; $D0BD  61 80
  jsr _func_f86a                 ; $D0BF  20 6A F8
  iny                            ; $D0C2  C8
  jsr _func_f878                 ; $D0C3  20 78 F8
  adc (_var_0080_indexed,X)      ; $D0C6  61 80
  jsr _func_f87d                 ; $D0C8  20 7D F8
  iny                            ; $D0CB  C8
  lda #$40                       ; $D0CC  A9 40
  sta a:_var_0200                ; $D0CE  8D 00 02
  jsr _func_f889                 ; $D0D1  20 89 F8
  cmp (_var_0080_indexed,X)      ; $D0D4  C1 80
  jsr _func_f88e                 ; $D0D6  20 8E F8
  iny                            ; $D0D9  C8
  pha                            ; $D0DA  48
  lda #$3F                       ; $D0DB  A9 3F
  sta a:_var_0200                ; $D0DD  8D 00 02
  pla                            ; $D0E0  68
  jsr _func_f89a                 ; $D0E1  20 9A F8
  cmp (_var_0080_indexed,X)      ; $D0E4  C1 80
  jsr _func_f89c                 ; $D0E6  20 9C F8
  iny                            ; $D0E9  C8
  pha                            ; $D0EA  48
  lda #$41                       ; $D0EB  A9 41
  sta a:_var_0200                ; $D0ED  8D 00 02
  pla                            ; $D0F0  68
  cmp (_var_0080_indexed,X)      ; $D0F1  C1 80
  jsr _func_f8a8                 ; $D0F3  20 A8 F8
  iny                            ; $D0F6  C8
  pha                            ; $D0F7  48
  lda #$00                       ; $D0F8  A9 00
  sta a:_var_0200                ; $D0FA  8D 00 02
  pla                            ; $D0FD  68
  jsr _func_f8b2                 ; $D0FE  20 B2 F8
  cmp (_var_0080_indexed,X)      ; $D101  C1 80
  jsr _func_f8b5                 ; $D103  20 B5 F8
  iny                            ; $D106  C8
  pha                            ; $D107  48
  lda #$80                       ; $D108  A9 80
  sta a:_var_0200                ; $D10A  8D 00 02
  pla                            ; $D10D  68
  cmp (_var_0080_indexed,X)      ; $D10E  C1 80
  jsr _func_f8bf                 ; $D110  20 BF F8
  iny                            ; $D113  C8
  pha                            ; $D114  48
  lda #$81                       ; $D115  A9 81
  sta a:_var_0200                ; $D117  8D 00 02
  pla                            ; $D11A  68
  cmp (_var_0080_indexed,X)      ; $D11B  C1 80
  jsr _func_f8c9                 ; $D11D  20 C9 F8
  iny                            ; $D120  C8
  pha                            ; $D121  48
  lda #$7F                       ; $D122  A9 7F
  sta a:_var_0200                ; $D124  8D 00 02
  pla                            ; $D127  68
  cmp (_var_0080_indexed,X)      ; $D128  C1 80
  jsr _func_f8d3                 ; $D12A  20 D3 F8
  iny                            ; $D12D  C8
  lda #$40                       ; $D12E  A9 40
  sta a:_var_0200                ; $D130  8D 00 02
  jsr _func_f931                 ; $D133  20 31 F9
  sbc (_var_0080_indexed,X)      ; $D136  E1 80
  jsr _func_f937                 ; $D138  20 37 F9
  iny                            ; $D13B  C8
  lda #$3F                       ; $D13C  A9 3F
  sta a:_var_0200                ; $D13E  8D 00 02
  jsr _func_f947                 ; $D141  20 47 F9
  sbc (_var_0080_indexed,X)      ; $D144  E1 80
  jsr _func_f94c                 ; $D146  20 4C F9
  iny                            ; $D149  C8
  lda #$41                       ; $D14A  A9 41
  sta a:_var_0200                ; $D14C  8D 00 02
  jsr _func_f95c                 ; $D14F  20 5C F9
  sbc (_var_0080_indexed,X)      ; $D152  E1 80
  jsr _func_f962                 ; $D154  20 62 F9
  iny                            ; $D157  C8
  lda #$00                       ; $D158  A9 00
  sta a:_var_0200                ; $D15A  8D 00 02
  jsr _func_f972                 ; $D15D  20 72 F9
  sbc (_var_0080_indexed,X)      ; $D160  E1 80
  jsr _func_f976                 ; $D162  20 76 F9
  iny                            ; $D165  C8
  lda #$7F                       ; $D166  A9 7F
  sta a:_var_0200                ; $D168  8D 00 02
  jsr _func_f980                 ; $D16B  20 80 F9
  sbc (_var_0080_indexed,X)      ; $D16E  E1 80
  jsr _func_f984                 ; $D170  20 84 F9
  rts                            ; $D173  60

_label_d174:
  lda #$55                       ; $D174  A9 55
  sta z:_var_0078                ; $D176  85 78
  lda #$FF                       ; $D178  A9 FF
  sta z:_var_0001                ; $D17A  85 01
  bit z:_var_0001                ; $D17C  24 01
  ldy #$11                       ; $D17E  A0 11
  ldx #$23                       ; $D180  A2 23
  lda #$00                       ; $D182  A9 00
  lda z:_var_0078                ; $D184  A5 78
  beq _label_d198                ; $D186  F0 10
  bmi _label_d198                ; $D188  30 0E
  cmp #$55                       ; $D18A  C9 55
  bne _label_d198                ; $D18C  D0 0A
  cpy #$11                       ; $D18E  C0 11
  bne _label_d198                ; $D190  D0 06
  cpx #$23                       ; $D192  E0 23
  bvc _label_d198                ; $D194  50 02
  beq _label_d19c                ; $D196  F0 04

_label_d198:
  lda #$76                       ; $D198  A9 76
  sta z:_var_0000_indexed        ; $D19A  85 00

_label_d19c:
  lda #$46                       ; $D19C  A9 46
  bit z:_var_0001                ; $D19E  24 01
  sta z:_var_0078                ; $D1A0  85 78
  beq _label_d1ae                ; $D1A2  F0 0A
  bpl _label_d1ae                ; $D1A4  10 08
  bvc _label_d1ae                ; $D1A6  50 06
  lda z:_var_0078                ; $D1A8  A5 78
  cmp #$46                       ; $D1AA  C9 46
  beq _label_d1b2                ; $D1AC  F0 04

_label_d1ae:
  lda #$77                       ; $D1AE  A9 77
  sta z:_var_0000_indexed        ; $D1B0  85 00

_label_d1b2:
  lda #$55                       ; $D1B2  A9 55
  sta z:_var_0078                ; $D1B4  85 78
  bit z:_var_0001                ; $D1B6  24 01
  lda #$11                       ; $D1B8  A9 11
  ldx #$23                       ; $D1BA  A2 23
  ldy #$00                       ; $D1BC  A0 00
  ldy z:_var_0078                ; $D1BE  A4 78
  beq _label_d1d2                ; $D1C0  F0 10
  bmi _label_d1d2                ; $D1C2  30 0E
  cpy #$55                       ; $D1C4  C0 55
  bne _label_d1d2                ; $D1C6  D0 0A
  cmp #$11                       ; $D1C8  C9 11
  bne _label_d1d2                ; $D1CA  D0 06
  cpx #$23                       ; $D1CC  E0 23
  bvc _label_d1d2                ; $D1CE  50 02
  beq _label_d1d6                ; $D1D0  F0 04

_label_d1d2:
  lda #$78                       ; $D1D2  A9 78
  sta z:_var_0000_indexed        ; $D1D4  85 00

_label_d1d6:
  ldy #$46                       ; $D1D6  A0 46
  bit z:_var_0001                ; $D1D8  24 01
  sty z:_var_0078                ; $D1DA  84 78
  beq _label_d1e8                ; $D1DC  F0 0A
  bpl _label_d1e8                ; $D1DE  10 08
  bvc _label_d1e8                ; $D1E0  50 06
  ldy z:_var_0078                ; $D1E2  A4 78
  cpy #$46                       ; $D1E4  C0 46
  beq _label_d1ec                ; $D1E6  F0 04

_label_d1e8:
  lda #$79                       ; $D1E8  A9 79
  sta z:_var_0000_indexed        ; $D1EA  85 00

_label_d1ec:
  bit z:_var_0001                ; $D1EC  24 01
  lda #$55                       ; $D1EE  A9 55
  sta z:_var_0078                ; $D1F0  85 78
  ldy #$11                       ; $D1F2  A0 11
  lda #$23                       ; $D1F4  A9 23
  ldx #$00                       ; $D1F6  A2 00
  ldx z:_var_0078                ; $D1F8  A6 78
  beq _label_d20c                ; $D1FA  F0 10
  bmi _label_d20c                ; $D1FC  30 0E
  cpx #$55                       ; $D1FE  E0 55
  bne _label_d20c                ; $D200  D0 0A
  cpy #$11                       ; $D202  C0 11
  bne _label_d20c                ; $D204  D0 06
  cmp #$23                       ; $D206  C9 23
  bvc _label_d20c                ; $D208  50 02
  beq _label_d210                ; $D20A  F0 04

_label_d20c:
  lda #$7A                       ; $D20C  A9 7A
  sta z:_var_0000_indexed        ; $D20E  85 00

_label_d210:
  ldx #$46                       ; $D210  A2 46
  bit z:_var_0001                ; $D212  24 01
  stx z:_var_0078                ; $D214  86 78
  beq _label_d222                ; $D216  F0 0A
  bpl _label_d222                ; $D218  10 08
  bvc _label_d222                ; $D21A  50 06
  ldx z:_var_0078                ; $D21C  A6 78
  cpx #$46                       ; $D21E  E0 46
  beq _label_d226                ; $D220  F0 04

_label_d222:
  lda #$7B                       ; $D222  A9 7B
  sta z:_var_0000_indexed        ; $D224  85 00

_label_d226:
  lda #$C0                       ; $D226  A9 C0
  sta z:_var_0078                ; $D228  85 78
  ldx #$33                       ; $D22A  A2 33
  ldy #$88                       ; $D22C  A0 88
  lda #$05                       ; $D22E  A9 05
  bit z:_var_0078                ; $D230  24 78
  bpl _label_d244                ; $D232  10 10
  bvc _label_d244                ; $D234  50 0E
  bne _label_d244                ; $D236  D0 0C
  cmp #$05                       ; $D238  C9 05
  bne _label_d244                ; $D23A  D0 08
  cpx #$33                       ; $D23C  E0 33
  bne _label_d244                ; $D23E  D0 04
  cpy #$88                       ; $D240  C0 88
  beq _label_d248                ; $D242  F0 04

_label_d244:
  lda #$7C                       ; $D244  A9 7C
  sta z:_var_0000_indexed        ; $D246  85 00

_label_d248:
  lda #$03                       ; $D248  A9 03
  sta z:_var_0078                ; $D24A  85 78
  lda #$01                       ; $D24C  A9 01
  bit z:_var_0078                ; $D24E  24 78
  bmi _label_d25a                ; $D250  30 08
  bvs _label_d25a                ; $D252  70 06
  beq _label_d25a                ; $D254  F0 04
  cmp #$01                       ; $D256  C9 01
  beq _label_d25e                ; $D258  F0 04

_label_d25a:
  lda #$7D                       ; $D25A  A9 7D
  sta z:_var_0000_indexed        ; $D25C  85 00

_label_d25e:
  ldy #$7E                       ; $D25E  A0 7E
  lda #$AA                       ; $D260  A9 AA
  sta z:_var_0078                ; $D262  85 78
  jsr _func_f7b6                 ; $D264  20 B6 F7
  ora z:_var_0078                ; $D267  05 78
  jsr _func_f7c0                 ; $D269  20 C0 F7
  iny                            ; $D26C  C8
  lda #$00                       ; $D26D  A9 00
  sta z:_var_0078                ; $D26F  85 78
  jsr _func_f7ce                 ; $D271  20 CE F7
  ora z:_var_0078                ; $D274  05 78
  jsr _func_f7d3                 ; $D276  20 D3 F7
  iny                            ; $D279  C8
  lda #$AA                       ; $D27A  A9 AA
  sta z:_var_0078                ; $D27C  85 78
  jsr _func_f7df                 ; $D27E  20 DF F7
  and z:_var_0078                ; $D281  25 78
  jsr _func_f7e5                 ; $D283  20 E5 F7
  iny                            ; $D286  C8
  lda #$EF                       ; $D287  A9 EF
  sta z:_var_0078                ; $D289  85 78
  jsr _func_f7f1                 ; $D28B  20 F1 F7
  and z:_var_0078                ; $D28E  25 78
  jsr _func_f7f6                 ; $D290  20 F6 F7
  iny                            ; $D293  C8
  lda #$AA                       ; $D294  A9 AA
  sta z:_var_0078                ; $D296  85 78
  jsr _func_f804                 ; $D298  20 04 F8
  eor z:_var_0078                ; $D29B  45 78
  jsr _func_f80a                 ; $D29D  20 0A F8
  iny                            ; $D2A0  C8
  lda #$70                       ; $D2A1  A9 70
  sta z:_var_0078                ; $D2A3  85 78
  jsr _func_f818                 ; $D2A5  20 18 F8
  eor z:_var_0078                ; $D2A8  45 78
  jsr _func_f81d                 ; $D2AA  20 1D F8
  iny                            ; $D2AD  C8
  lda #$69                       ; $D2AE  A9 69
  sta z:_var_0078                ; $D2B0  85 78
  jsr _func_f829                 ; $D2B2  20 29 F8
  adc z:_var_0078                ; $D2B5  65 78
  jsr _func_f82f                 ; $D2B7  20 2F F8
  iny                            ; $D2BA  C8
  jsr _func_f83d                 ; $D2BB  20 3D F8
  adc z:_var_0078                ; $D2BE  65 78
  jsr _func_f843                 ; $D2C0  20 43 F8
  iny                            ; $D2C3  C8
  lda #$7F                       ; $D2C4  A9 7F
  sta z:_var_0078                ; $D2C6  85 78
  jsr _func_f851                 ; $D2C8  20 51 F8
  adc z:_var_0078                ; $D2CB  65 78
  jsr _func_f856                 ; $D2CD  20 56 F8
  iny                            ; $D2D0  C8
  lda #$80                       ; $D2D1  A9 80
  sta z:_var_0078                ; $D2D3  85 78
  jsr _func_f864                 ; $D2D5  20 64 F8
  adc z:_var_0078                ; $D2D8  65 78
  jsr _func_f86a                 ; $D2DA  20 6A F8
  iny                            ; $D2DD  C8
  jsr _func_f878                 ; $D2DE  20 78 F8
  adc z:_var_0078                ; $D2E1  65 78
  jsr _func_f87d                 ; $D2E3  20 7D F8
  iny                            ; $D2E6  C8
  lda #$40                       ; $D2E7  A9 40
  sta z:_var_0078                ; $D2E9  85 78
  jsr _func_f889                 ; $D2EB  20 89 F8
  cmp z:_var_0078                ; $D2EE  C5 78
  jsr _func_f88e                 ; $D2F0  20 8E F8
  iny                            ; $D2F3  C8
  pha                            ; $D2F4  48
  lda #$3F                       ; $D2F5  A9 3F
  sta z:_var_0078                ; $D2F7  85 78
  pla                            ; $D2F9  68
  jsr _func_f89a                 ; $D2FA  20 9A F8
  cmp z:_var_0078                ; $D2FD  C5 78
  jsr _func_f89c                 ; $D2FF  20 9C F8
  iny                            ; $D302  C8
  pha                            ; $D303  48
  lda #$41                       ; $D304  A9 41
  sta z:_var_0078                ; $D306  85 78
  pla                            ; $D308  68
  cmp z:_var_0078                ; $D309  C5 78
  jsr _func_f8a8                 ; $D30B  20 A8 F8
  iny                            ; $D30E  C8
  pha                            ; $D30F  48
  lda #$00                       ; $D310  A9 00
  sta z:_var_0078                ; $D312  85 78
  pla                            ; $D314  68
  jsr _func_f8b2                 ; $D315  20 B2 F8
  cmp z:_var_0078                ; $D318  C5 78
  jsr _func_f8b5                 ; $D31A  20 B5 F8
  iny                            ; $D31D  C8
  pha                            ; $D31E  48
  lda #$80                       ; $D31F  A9 80
  sta z:_var_0078                ; $D321  85 78
  pla                            ; $D323  68
  cmp z:_var_0078                ; $D324  C5 78
  jsr _func_f8bf                 ; $D326  20 BF F8
  iny                            ; $D329  C8
  pha                            ; $D32A  48
  lda #$81                       ; $D32B  A9 81
  sta z:_var_0078                ; $D32D  85 78
  pla                            ; $D32F  68
  cmp z:_var_0078                ; $D330  C5 78
  jsr _func_f8c9                 ; $D332  20 C9 F8
  iny                            ; $D335  C8
  pha                            ; $D336  48
  lda #$7F                       ; $D337  A9 7F
  sta z:_var_0078                ; $D339  85 78
  pla                            ; $D33B  68
  cmp z:_var_0078                ; $D33C  C5 78
  jsr _func_f8d3                 ; $D33E  20 D3 F8
  iny                            ; $D341  C8
  lda #$40                       ; $D342  A9 40
  sta z:_var_0078                ; $D344  85 78
  jsr _func_f931                 ; $D346  20 31 F9
  sbc z:_var_0078                ; $D349  E5 78
  jsr _func_f937                 ; $D34B  20 37 F9
  iny                            ; $D34E  C8
  lda #$3F                       ; $D34F  A9 3F
  sta z:_var_0078                ; $D351  85 78
  jsr _func_f947                 ; $D353  20 47 F9
  sbc z:_var_0078                ; $D356  E5 78
  jsr _func_f94c                 ; $D358  20 4C F9
  iny                            ; $D35B  C8
  lda #$41                       ; $D35C  A9 41
  sta z:_var_0078                ; $D35E  85 78
  jsr _func_f95c                 ; $D360  20 5C F9
  sbc z:_var_0078                ; $D363  E5 78
  jsr _func_f962                 ; $D365  20 62 F9
  iny                            ; $D368  C8
  lda #$00                       ; $D369  A9 00
  sta z:_var_0078                ; $D36B  85 78
  jsr _func_f972                 ; $D36D  20 72 F9
  sbc z:_var_0078                ; $D370  E5 78
  jsr _func_f976                 ; $D372  20 76 F9
  iny                            ; $D375  C8
  lda #$7F                       ; $D376  A9 7F
  sta z:_var_0078                ; $D378  85 78
  jsr _func_f980                 ; $D37A  20 80 F9
  sbc z:_var_0078                ; $D37D  E5 78
  jsr _func_f984                 ; $D37F  20 84 F9
  iny                            ; $D382  C8
  lda #$40                       ; $D383  A9 40
  sta z:_var_0078                ; $D385  85 78
  jsr _func_f889                 ; $D387  20 89 F8
  tax                            ; $D38A  AA
  cpx z:_var_0078                ; $D38B  E4 78
  jsr _func_f88e                 ; $D38D  20 8E F8
  iny                            ; $D390  C8
  lda #$3F                       ; $D391  A9 3F
  sta z:_var_0078                ; $D393  85 78
  jsr _func_f89a                 ; $D395  20 9A F8
  cpx z:_var_0078                ; $D398  E4 78
  jsr _func_f89c                 ; $D39A  20 9C F8
  iny                            ; $D39D  C8
  lda #$41                       ; $D39E  A9 41
  sta z:_var_0078                ; $D3A0  85 78
  cpx z:_var_0078                ; $D3A2  E4 78
  jsr _func_f8a8                 ; $D3A4  20 A8 F8
  iny                            ; $D3A7  C8
  lda #$00                       ; $D3A8  A9 00
  sta z:_var_0078                ; $D3AA  85 78
  jsr _func_f8b2                 ; $D3AC  20 B2 F8
  tax                            ; $D3AF  AA
  cpx z:_var_0078                ; $D3B0  E4 78
  jsr _func_f8b5                 ; $D3B2  20 B5 F8
  iny                            ; $D3B5  C8
  lda #$80                       ; $D3B6  A9 80
  sta z:_var_0078                ; $D3B8  85 78
  cpx z:_var_0078                ; $D3BA  E4 78
  jsr _func_f8bf                 ; $D3BC  20 BF F8
  iny                            ; $D3BF  C8
  lda #$81                       ; $D3C0  A9 81
  sta z:_var_0078                ; $D3C2  85 78
  cpx z:_var_0078                ; $D3C4  E4 78
  jsr _func_f8c9                 ; $D3C6  20 C9 F8
  iny                            ; $D3C9  C8
  lda #$7F                       ; $D3CA  A9 7F
  sta z:_var_0078                ; $D3CC  85 78
  cpx z:_var_0078                ; $D3CE  E4 78
  jsr _func_f8d3                 ; $D3D0  20 D3 F8
  iny                            ; $D3D3  C8
  tya                            ; $D3D4  98
  tax                            ; $D3D5  AA
  lda #$40                       ; $D3D6  A9 40
  sta z:_var_0078                ; $D3D8  85 78
  jsr _func_f8dd                 ; $D3DA  20 DD F8
  cpy z:_var_0078                ; $D3DD  C4 78
  jsr _func_f8e2                 ; $D3DF  20 E2 F8
  inx                            ; $D3E2  E8
  lda #$3F                       ; $D3E3  A9 3F
  sta z:_var_0078                ; $D3E5  85 78
  jsr _func_f8ee                 ; $D3E7  20 EE F8
  cpy z:_var_0078                ; $D3EA  C4 78
  jsr _func_f8f0                 ; $D3EC  20 F0 F8
  inx                            ; $D3EF  E8
  lda #$41                       ; $D3F0  A9 41
  sta z:_var_0078                ; $D3F2  85 78
  cpy z:_var_0078                ; $D3F4  C4 78
  jsr _func_f8fc                 ; $D3F6  20 FC F8
  inx                            ; $D3F9  E8
  lda #$00                       ; $D3FA  A9 00
  sta z:_var_0078                ; $D3FC  85 78
  jsr _func_f906                 ; $D3FE  20 06 F9
  cpy z:_var_0078                ; $D401  C4 78
  jsr _func_f909                 ; $D403  20 09 F9
  inx                            ; $D406  E8
  lda #$80                       ; $D407  A9 80
  sta z:_var_0078                ; $D409  85 78
  cpy z:_var_0078                ; $D40B  C4 78
  jsr _func_f913                 ; $D40D  20 13 F9
  inx                            ; $D410  E8
  lda #$81                       ; $D411  A9 81
  sta z:_var_0078                ; $D413  85 78
  cpy z:_var_0078                ; $D415  C4 78
  jsr _func_f91d                 ; $D417  20 1D F9
  inx                            ; $D41A  E8
  lda #$7F                       ; $D41B  A9 7F
  sta z:_var_0078                ; $D41D  85 78
  cpy z:_var_0078                ; $D41F  C4 78
  jsr _func_f927                 ; $D421  20 27 F9
  inx                            ; $D424  E8
  txa                            ; $D425  8A
  tay                            ; $D426  A8
  jsr _func_f990                 ; $D427  20 90 F9
  sta z:_var_0078                ; $D42A  85 78
  lsr z:_var_0078                ; $D42C  46 78
  lda z:_var_0078                ; $D42E  A5 78
  jsr _func_f99d                 ; $D430  20 9D F9
  iny                            ; $D433  C8
  sta z:_var_0078                ; $D434  85 78
  lsr z:_var_0078                ; $D436  46 78
  lda z:_var_0078                ; $D438  A5 78
  jsr _func_f9ad                 ; $D43A  20 AD F9
  iny                            ; $D43D  C8
  jsr _func_f9bd                 ; $D43E  20 BD F9
  sta z:_var_0078                ; $D441  85 78
  asl z:_var_0078                ; $D443  06 78
  lda z:_var_0078                ; $D445  A5 78
  jsr _func_f9c3                 ; $D447  20 C3 F9
  iny                            ; $D44A  C8
  sta z:_var_0078                ; $D44B  85 78
  asl z:_var_0078                ; $D44D  06 78
  lda z:_var_0078                ; $D44F  A5 78
  jsr _func_f9d4                 ; $D451  20 D4 F9
  iny                            ; $D454  C8
  jsr _func_f9e4                 ; $D455  20 E4 F9
  sta z:_var_0078                ; $D458  85 78
  ror z:_var_0078                ; $D45A  66 78
  lda z:_var_0078                ; $D45C  A5 78
  jsr _func_f9ea                 ; $D45E  20 EA F9
  iny                            ; $D461  C8
  sta z:_var_0078                ; $D462  85 78
  ror z:_var_0078                ; $D464  66 78
  lda z:_var_0078                ; $D466  A5 78
  jsr _func_f9fb                 ; $D468  20 FB F9
  iny                            ; $D46B  C8
  jsr _func_fa0a                 ; $D46C  20 0A FA
  sta z:_var_0078                ; $D46F  85 78
  rol z:_var_0078                ; $D471  26 78
  lda z:_var_0078                ; $D473  A5 78
  jsr _func_fa10                 ; $D475  20 10 FA
  iny                            ; $D478  C8
  sta z:_var_0078                ; $D479  85 78
  rol z:_var_0078                ; $D47B  26 78
  lda z:_var_0078                ; $D47D  A5 78
  jsr _func_fa21                 ; $D47F  20 21 FA
  lda #$FF                       ; $D482  A9 FF
  sta z:_var_0078                ; $D484  85 78
  sta z:_var_0001                ; $D486  85 01
  bit z:_var_0001                ; $D488  24 01
  sec                            ; $D48A  38
  inc z:_var_0078                ; $D48B  E6 78
  bne _label_d49b                ; $D48D  D0 0C
  bmi _label_d49b                ; $D48F  30 0A
  bvc _label_d49b                ; $D491  50 08
  bcc _label_d49b                ; $D493  90 06
  lda z:_var_0078                ; $D495  A5 78
  cmp #$00                       ; $D497  C9 00
  beq _label_d49f                ; $D499  F0 04

_label_d49b:
  lda #$AB                       ; $D49B  A9 AB
  sta z:_var_0000_indexed        ; $D49D  85 00

_label_d49f:
  lda #$7F                       ; $D49F  A9 7F
  sta z:_var_0078                ; $D4A1  85 78
  clv                            ; $D4A3  B8
  clc                            ; $D4A4  18
  inc z:_var_0078                ; $D4A5  E6 78
  beq _label_d4b5                ; $D4A7  F0 0C
  bpl _label_d4b5                ; $D4A9  10 0A
  bvs _label_d4b5                ; $D4AB  70 08
  bcs _label_d4b5                ; $D4AD  B0 06
  lda z:_var_0078                ; $D4AF  A5 78
  cmp #$80                       ; $D4B1  C9 80
  beq _label_d4b9                ; $D4B3  F0 04

_label_d4b5:
  lda #$AC                       ; $D4B5  A9 AC
  sta z:_var_0000_indexed        ; $D4B7  85 00

_label_d4b9:
  lda #$00                       ; $D4B9  A9 00
  sta z:_var_0078                ; $D4BB  85 78
  bit z:_var_0001                ; $D4BD  24 01
  sec                            ; $D4BF  38
  dec z:_var_0078                ; $D4C0  C6 78
  beq _label_d4d0                ; $D4C2  F0 0C
  bpl _label_d4d0                ; $D4C4  10 0A
  bvc _label_d4d0                ; $D4C6  50 08
  bcc _label_d4d0                ; $D4C8  90 06
  lda z:_var_0078                ; $D4CA  A5 78
  cmp #$FF                       ; $D4CC  C9 FF
  beq _label_d4d4                ; $D4CE  F0 04

_label_d4d0:
  lda #$AD                       ; $D4D0  A9 AD
  sta z:_var_0000_indexed        ; $D4D2  85 00

_label_d4d4:
  lda #$80                       ; $D4D4  A9 80
  sta z:_var_0078                ; $D4D6  85 78
  clv                            ; $D4D8  B8
  clc                            ; $D4D9  18
  dec z:_var_0078                ; $D4DA  C6 78
  beq _label_d4ea                ; $D4DC  F0 0C
  bmi _label_d4ea                ; $D4DE  30 0A
  bvs _label_d4ea                ; $D4E0  70 08
  bcs _label_d4ea                ; $D4E2  B0 06
  lda z:_var_0078                ; $D4E4  A5 78
  cmp #$7F                       ; $D4E6  C9 7F
  beq _label_d4ee                ; $D4E8  F0 04

_label_d4ea:
  lda #$AE                       ; $D4EA  A9 AE
  sta z:_var_0000_indexed        ; $D4EC  85 00

_label_d4ee:
  lda #$01                       ; $D4EE  A9 01
  sta z:_var_0078                ; $D4F0  85 78
  dec z:_var_0078                ; $D4F2  C6 78
  beq _label_d4fa                ; $D4F4  F0 04
  lda #$AF                       ; $D4F6  A9 AF
  sta z:_var_0000_indexed        ; $D4F8  85 00

_label_d4fa:
  rts                            ; $D4FA  60

_label_d4fb:
  lda #$55                       ; $D4FB  A9 55
  sta a:_var_0678                ; $D4FD  8D 78 06
  lda #$FF                       ; $D500  A9 FF
  sta z:_var_0001                ; $D502  85 01
  bit z:_var_0001                ; $D504  24 01
  ldy #$11                       ; $D506  A0 11
  ldx #$23                       ; $D508  A2 23
  lda #$00                       ; $D50A  A9 00
  lda a:_var_0678                ; $D50C  AD 78 06
  beq _label_d521                ; $D50F  F0 10
  bmi _label_d521                ; $D511  30 0E
  cmp #$55                       ; $D513  C9 55
  bne _label_d521                ; $D515  D0 0A
  cpy #$11                       ; $D517  C0 11
  bne _label_d521                ; $D519  D0 06
  cpx #$23                       ; $D51B  E0 23
  bvc _label_d521                ; $D51D  50 02
  beq _label_d525                ; $D51F  F0 04

_label_d521:
  lda #$B0                       ; $D521  A9 B0
  sta z:_var_0000_indexed        ; $D523  85 00

_label_d525:
  lda #$46                       ; $D525  A9 46
  bit z:_var_0001                ; $D527  24 01
  sta a:_var_0678                ; $D529  8D 78 06
  beq _label_d539                ; $D52C  F0 0B
  bpl _label_d539                ; $D52E  10 09
  bvc _label_d539                ; $D530  50 07
  lda a:_var_0678                ; $D532  AD 78 06
  cmp #$46                       ; $D535  C9 46
  beq _label_d53d                ; $D537  F0 04

_label_d539:
  lda #$B1                       ; $D539  A9 B1
  sta z:_var_0000_indexed        ; $D53B  85 00

_label_d53d:
  lda #$55                       ; $D53D  A9 55
  sta a:_var_0678                ; $D53F  8D 78 06
  bit z:_var_0001                ; $D542  24 01
  lda #$11                       ; $D544  A9 11
  ldx #$23                       ; $D546  A2 23
  ldy #$00                       ; $D548  A0 00
  ldy a:_var_0678                ; $D54A  AC 78 06
  beq _label_d55f                ; $D54D  F0 10
  bmi _label_d55f                ; $D54F  30 0E
  cpy #$55                       ; $D551  C0 55
  bne _label_d55f                ; $D553  D0 0A
  cmp #$11                       ; $D555  C9 11
  bne _label_d55f                ; $D557  D0 06
  cpx #$23                       ; $D559  E0 23
  bvc _label_d55f                ; $D55B  50 02
  beq _label_d563                ; $D55D  F0 04

_label_d55f:
  lda #$B2                       ; $D55F  A9 B2
  sta z:_var_0000_indexed        ; $D561  85 00

_label_d563:
  ldy #$46                       ; $D563  A0 46
  bit z:_var_0001                ; $D565  24 01
  sty a:_var_0678                ; $D567  8C 78 06
  beq _label_d577                ; $D56A  F0 0B
  bpl _label_d577                ; $D56C  10 09
  bvc _label_d577                ; $D56E  50 07
  ldy a:_var_0678                ; $D570  AC 78 06
  cpy #$46                       ; $D573  C0 46
  beq _label_d57b                ; $D575  F0 04

_label_d577:
  lda #$B3                       ; $D577  A9 B3
  sta z:_var_0000_indexed        ; $D579  85 00

_label_d57b:
  bit z:_var_0001                ; $D57B  24 01
  lda #$55                       ; $D57D  A9 55
  sta a:_var_0678                ; $D57F  8D 78 06
  ldy #$11                       ; $D582  A0 11
  lda #$23                       ; $D584  A9 23
  ldx #$00                       ; $D586  A2 00
  ldx a:_var_0678                ; $D588  AE 78 06
  beq _label_d59d                ; $D58B  F0 10
  bmi _label_d59d                ; $D58D  30 0E
  cpx #$55                       ; $D58F  E0 55
  bne _label_d59d                ; $D591  D0 0A
  cpy #$11                       ; $D593  C0 11
  bne _label_d59d                ; $D595  D0 06
  cmp #$23                       ; $D597  C9 23
  bvc _label_d59d                ; $D599  50 02
  beq _label_d5a1                ; $D59B  F0 04

_label_d59d:
  lda #$B4                       ; $D59D  A9 B4
  sta z:_var_0000_indexed        ; $D59F  85 00

_label_d5a1:
  ldx #$46                       ; $D5A1  A2 46
  bit z:_var_0001                ; $D5A3  24 01
  stx a:_var_0678                ; $D5A5  8E 78 06
  beq _label_d5b5                ; $D5A8  F0 0B
  bpl _label_d5b5                ; $D5AA  10 09
  bvc _label_d5b5                ; $D5AC  50 07
  ldx a:_var_0678                ; $D5AE  AE 78 06
  cpx #$46                       ; $D5B1  E0 46
  beq _label_d5b9                ; $D5B3  F0 04

_label_d5b5:
  lda #$B5                       ; $D5B5  A9 B5
  sta z:_var_0000_indexed        ; $D5B7  85 00

_label_d5b9:
  lda #$C0                       ; $D5B9  A9 C0
  sta a:_var_0678                ; $D5BB  8D 78 06
  ldx #$33                       ; $D5BE  A2 33
  ldy #$88                       ; $D5C0  A0 88
  lda #$05                       ; $D5C2  A9 05
  bit a:_var_0678                ; $D5C4  2C 78 06
  bpl _label_d5d9                ; $D5C7  10 10
  bvc _label_d5d9                ; $D5C9  50 0E
  bne _label_d5d9                ; $D5CB  D0 0C
  cmp #$05                       ; $D5CD  C9 05
  bne _label_d5d9                ; $D5CF  D0 08
  cpx #$33                       ; $D5D1  E0 33
  bne _label_d5d9                ; $D5D3  D0 04
  cpy #$88                       ; $D5D5  C0 88
  beq _label_d5dd                ; $D5D7  F0 04

_label_d5d9:
  lda #$B6                       ; $D5D9  A9 B6
  sta z:_var_0000_indexed        ; $D5DB  85 00

_label_d5dd:
  lda #$03                       ; $D5DD  A9 03
  sta a:_var_0678                ; $D5DF  8D 78 06
  lda #$01                       ; $D5E2  A9 01
  bit a:_var_0678                ; $D5E4  2C 78 06
  bmi _label_d5f1                ; $D5E7  30 08
  bvs _label_d5f1                ; $D5E9  70 06
  beq _label_d5f1                ; $D5EB  F0 04
  cmp #$01                       ; $D5ED  C9 01
  beq _label_d5f5                ; $D5EF  F0 04

_label_d5f1:
  lda #$B7                       ; $D5F1  A9 B7
  sta z:_var_0000_indexed        ; $D5F3  85 00

_label_d5f5:
  ldy #$B8                       ; $D5F5  A0 B8
  lda #$AA                       ; $D5F7  A9 AA
  sta a:_var_0678                ; $D5F9  8D 78 06
  jsr _func_f7b6                 ; $D5FC  20 B6 F7
  ora a:_var_0678                ; $D5FF  0D 78 06
  jsr _func_f7c0                 ; $D602  20 C0 F7
  iny                            ; $D605  C8
  lda #$00                       ; $D606  A9 00
  sta a:_var_0678                ; $D608  8D 78 06
  jsr _func_f7ce                 ; $D60B  20 CE F7
  ora a:_var_0678                ; $D60E  0D 78 06
  jsr _func_f7d3                 ; $D611  20 D3 F7
  iny                            ; $D614  C8
  lda #$AA                       ; $D615  A9 AA
  sta a:_var_0678                ; $D617  8D 78 06
  jsr _func_f7df                 ; $D61A  20 DF F7
  and a:_var_0678                ; $D61D  2D 78 06
  jsr _func_f7e5                 ; $D620  20 E5 F7
  iny                            ; $D623  C8
  lda #$EF                       ; $D624  A9 EF
  sta a:_var_0678                ; $D626  8D 78 06
  jsr _func_f7f1                 ; $D629  20 F1 F7
  and a:_var_0678                ; $D62C  2D 78 06
  jsr _func_f7f6                 ; $D62F  20 F6 F7
  iny                            ; $D632  C8
  lda #$AA                       ; $D633  A9 AA
  sta a:_var_0678                ; $D635  8D 78 06
  jsr _func_f804                 ; $D638  20 04 F8
  eor a:_var_0678                ; $D63B  4D 78 06
  jsr _func_f80a                 ; $D63E  20 0A F8
  iny                            ; $D641  C8
  lda #$70                       ; $D642  A9 70
  sta a:_var_0678                ; $D644  8D 78 06
  jsr _func_f818                 ; $D647  20 18 F8
  eor a:_var_0678                ; $D64A  4D 78 06
  jsr _func_f81d                 ; $D64D  20 1D F8
  iny                            ; $D650  C8
  lda #$69                       ; $D651  A9 69
  sta a:_var_0678                ; $D653  8D 78 06
  jsr _func_f829                 ; $D656  20 29 F8
  adc a:_var_0678                ; $D659  6D 78 06
  jsr _func_f82f                 ; $D65C  20 2F F8
  iny                            ; $D65F  C8
  jsr _func_f83d                 ; $D660  20 3D F8
  adc a:_var_0678                ; $D663  6D 78 06
  jsr _func_f843                 ; $D666  20 43 F8
  iny                            ; $D669  C8
  lda #$7F                       ; $D66A  A9 7F
  sta a:_var_0678                ; $D66C  8D 78 06
  jsr _func_f851                 ; $D66F  20 51 F8
  adc a:_var_0678                ; $D672  6D 78 06
  jsr _func_f856                 ; $D675  20 56 F8
  iny                            ; $D678  C8
  lda #$80                       ; $D679  A9 80
  sta a:_var_0678                ; $D67B  8D 78 06
  jsr _func_f864                 ; $D67E  20 64 F8
  adc a:_var_0678                ; $D681  6D 78 06
  jsr _func_f86a                 ; $D684  20 6A F8
  iny                            ; $D687  C8
  jsr _func_f878                 ; $D688  20 78 F8
  adc a:_var_0678                ; $D68B  6D 78 06
  jsr _func_f87d                 ; $D68E  20 7D F8
  iny                            ; $D691  C8
  lda #$40                       ; $D692  A9 40
  sta a:_var_0678                ; $D694  8D 78 06
  jsr _func_f889                 ; $D697  20 89 F8
  cmp a:_var_0678                ; $D69A  CD 78 06
  jsr _func_f88e                 ; $D69D  20 8E F8
  iny                            ; $D6A0  C8
  pha                            ; $D6A1  48
  lda #$3F                       ; $D6A2  A9 3F
  sta a:_var_0678                ; $D6A4  8D 78 06
  pla                            ; $D6A7  68
  jsr _func_f89a                 ; $D6A8  20 9A F8
  cmp a:_var_0678                ; $D6AB  CD 78 06
  jsr _func_f89c                 ; $D6AE  20 9C F8
  iny                            ; $D6B1  C8
  pha                            ; $D6B2  48
  lda #$41                       ; $D6B3  A9 41
  sta a:_var_0678                ; $D6B5  8D 78 06
  pla                            ; $D6B8  68
  cmp a:_var_0678                ; $D6B9  CD 78 06
  jsr _func_f8a8                 ; $D6BC  20 A8 F8
  iny                            ; $D6BF  C8
  pha                            ; $D6C0  48
  lda #$00                       ; $D6C1  A9 00
  sta a:_var_0678                ; $D6C3  8D 78 06
  pla                            ; $D6C6  68
  jsr _func_f8b2                 ; $D6C7  20 B2 F8
  cmp a:_var_0678                ; $D6CA  CD 78 06
  jsr _func_f8b5                 ; $D6CD  20 B5 F8
  iny                            ; $D6D0  C8
  pha                            ; $D6D1  48
  lda #$80                       ; $D6D2  A9 80
  sta a:_var_0678                ; $D6D4  8D 78 06
  pla                            ; $D6D7  68
  cmp a:_var_0678                ; $D6D8  CD 78 06
  jsr _func_f8bf                 ; $D6DB  20 BF F8
  iny                            ; $D6DE  C8
  pha                            ; $D6DF  48
  lda #$81                       ; $D6E0  A9 81
  sta a:_var_0678                ; $D6E2  8D 78 06
  pla                            ; $D6E5  68
  cmp a:_var_0678                ; $D6E6  CD 78 06
  jsr _func_f8c9                 ; $D6E9  20 C9 F8
  iny                            ; $D6EC  C8
  pha                            ; $D6ED  48
  lda #$7F                       ; $D6EE  A9 7F
  sta a:_var_0678                ; $D6F0  8D 78 06
  pla                            ; $D6F3  68
  cmp a:_var_0678                ; $D6F4  CD 78 06
  jsr _func_f8d3                 ; $D6F7  20 D3 F8
  iny                            ; $D6FA  C8
  lda #$40                       ; $D6FB  A9 40
  sta a:_var_0678                ; $D6FD  8D 78 06
  jsr _func_f931                 ; $D700  20 31 F9
  sbc a:_var_0678                ; $D703  ED 78 06
  jsr _func_f937                 ; $D706  20 37 F9
  iny                            ; $D709  C8
  lda #$3F                       ; $D70A  A9 3F
  sta a:_var_0678                ; $D70C  8D 78 06
  jsr _func_f947                 ; $D70F  20 47 F9
  sbc a:_var_0678                ; $D712  ED 78 06
  jsr _func_f94c                 ; $D715  20 4C F9
  iny                            ; $D718  C8
  lda #$41                       ; $D719  A9 41
  sta a:_var_0678                ; $D71B  8D 78 06
  jsr _func_f95c                 ; $D71E  20 5C F9
  sbc a:_var_0678                ; $D721  ED 78 06
  jsr _func_f962                 ; $D724  20 62 F9
  iny                            ; $D727  C8
  lda #$00                       ; $D728  A9 00
  sta a:_var_0678                ; $D72A  8D 78 06
  jsr _func_f972                 ; $D72D  20 72 F9
  sbc a:_var_0678                ; $D730  ED 78 06
  jsr _func_f976                 ; $D733  20 76 F9
  iny                            ; $D736  C8
  lda #$7F                       ; $D737  A9 7F
  sta a:_var_0678                ; $D739  8D 78 06
  jsr _func_f980                 ; $D73C  20 80 F9
  sbc a:_var_0678                ; $D73F  ED 78 06
  jsr _func_f984                 ; $D742  20 84 F9
  iny                            ; $D745  C8
  lda #$40                       ; $D746  A9 40
  sta a:_var_0678                ; $D748  8D 78 06
  jsr _func_f889                 ; $D74B  20 89 F8
  tax                            ; $D74E  AA
  cpx a:_var_0678                ; $D74F  EC 78 06
  jsr _func_f88e                 ; $D752  20 8E F8
  iny                            ; $D755  C8
  lda #$3F                       ; $D756  A9 3F
  sta a:_var_0678                ; $D758  8D 78 06
  jsr _func_f89a                 ; $D75B  20 9A F8
  cpx a:_var_0678                ; $D75E  EC 78 06
  jsr _func_f89c                 ; $D761  20 9C F8
  iny                            ; $D764  C8
  lda #$41                       ; $D765  A9 41
  sta a:_var_0678                ; $D767  8D 78 06
  cpx a:_var_0678                ; $D76A  EC 78 06
  jsr _func_f8a8                 ; $D76D  20 A8 F8
  iny                            ; $D770  C8
  lda #$00                       ; $D771  A9 00
  sta a:_var_0678                ; $D773  8D 78 06
  jsr _func_f8b2                 ; $D776  20 B2 F8
  tax                            ; $D779  AA
  cpx a:_var_0678                ; $D77A  EC 78 06
  jsr _func_f8b5                 ; $D77D  20 B5 F8
  iny                            ; $D780  C8
  lda #$80                       ; $D781  A9 80
  sta a:_var_0678                ; $D783  8D 78 06
  cpx a:_var_0678                ; $D786  EC 78 06
  jsr _func_f8bf                 ; $D789  20 BF F8
  iny                            ; $D78C  C8
  lda #$81                       ; $D78D  A9 81
  sta a:_var_0678                ; $D78F  8D 78 06
  cpx a:_var_0678                ; $D792  EC 78 06
  jsr _func_f8c9                 ; $D795  20 C9 F8
  iny                            ; $D798  C8
  lda #$7F                       ; $D799  A9 7F
  sta a:_var_0678                ; $D79B  8D 78 06
  cpx a:_var_0678                ; $D79E  EC 78 06
  jsr _func_f8d3                 ; $D7A1  20 D3 F8
  iny                            ; $D7A4  C8

_label_d7a5:
  tya                            ; $D7A5  98
  tax                            ; $D7A6  AA
  lda #$40                       ; $D7A7  A9 40
  sta a:_var_0678                ; $D7A9  8D 78 06
  jsr _func_f8dd                 ; $D7AC  20 DD F8
  cpy a:_var_0678                ; $D7AF  CC 78 06
  jsr _func_f8e2                 ; $D7B2  20 E2 F8
  inx                            ; $D7B5  E8
  lda #$3F                       ; $D7B6  A9 3F
  sta a:_var_0678                ; $D7B8  8D 78 06
  jsr _func_f8ee                 ; $D7BB  20 EE F8
  cpy a:_var_0678                ; $D7BE  CC 78 06
  jsr _func_f8f0                 ; $D7C1  20 F0 F8
  inx                            ; $D7C4  E8
  lda #$41                       ; $D7C5  A9 41
  sta a:_var_0678                ; $D7C7  8D 78 06
  cpy a:_var_0678                ; $D7CA  CC 78 06
  jsr _func_f8fc                 ; $D7CD  20 FC F8
  inx                            ; $D7D0  E8
  lda #$00                       ; $D7D1  A9 00
  sta a:_var_0678                ; $D7D3  8D 78 06
  jsr _func_f906                 ; $D7D6  20 06 F9
  cpy a:_var_0678                ; $D7D9  CC 78 06
  jsr _func_f909                 ; $D7DC  20 09 F9
  inx                            ; $D7DF  E8
  lda #$80                       ; $D7E0  A9 80
  sta a:_var_0678                ; $D7E2  8D 78 06
.byte $cc                        ; $D7E5  CC  cpy a:$0678

_label_d7e6:
  sei                            ; $D7E6  78  branch into instruction detected
  asl z:_var_0020                ; $D7E7  06 20
  slo (_var_00f9_indexed),Y      ; $D7E9  13 F9
  inx                            ; $D7EB  E8
  lda #$81                       ; $D7EC  A9 81
  sta a:_var_0678                ; $D7EE  8D 78 06
  cpy a:_var_0678                ; $D7F1  CC 78 06
  jsr _func_f91d                 ; $D7F4  20 1D F9
  inx                            ; $D7F7  E8
  lda #$7F                       ; $D7F8  A9 7F
  sta a:_var_0678                ; $D7FA  8D 78 06
  cpy a:_var_0678                ; $D7FD  CC 78 06
  jsr _func_f927                 ; $D800  20 27 F9
  inx                            ; $D803  E8
  txa                            ; $D804  8A
  tay                            ; $D805  A8
  jsr _func_f990                 ; $D806  20 90 F9
  sta a:_var_0678                ; $D809  8D 78 06
  lsr a:_var_0678                ; $D80C  4E 78 06
  lda a:_var_0678                ; $D80F  AD 78 06
  jsr _func_f99d                 ; $D812  20 9D F9
  iny                            ; $D815  C8
  sta a:_var_0678                ; $D816  8D 78 06
  lsr a:_var_0678                ; $D819  4E 78 06
  lda a:_var_0678                ; $D81C  AD 78 06
  jsr _func_f9ad                 ; $D81F  20 AD F9
  iny                            ; $D822  C8
  jsr _func_f9bd                 ; $D823  20 BD F9
  sta a:_var_0678                ; $D826  8D 78 06
  asl a:_var_0678                ; $D829  0E 78 06
  lda a:_var_0678                ; $D82C  AD 78 06
  jsr _func_f9c3                 ; $D82F  20 C3 F9
  iny                            ; $D832  C8
  sta a:_var_0678                ; $D833  8D 78 06
  asl a:_var_0678                ; $D836  0E 78 06
  lda a:_var_0678                ; $D839  AD 78 06
  jsr _func_f9d4                 ; $D83C  20 D4 F9
  iny                            ; $D83F  C8
  jsr _func_f9e4                 ; $D840  20 E4 F9
  sta a:_var_0678                ; $D843  8D 78 06
  ror a:_var_0678                ; $D846  6E 78 06
  lda a:_var_0678                ; $D849  AD 78 06
  jsr _func_f9ea                 ; $D84C  20 EA F9
  iny                            ; $D84F  C8
  sta a:_var_0678                ; $D850  8D 78 06
  ror a:_var_0678                ; $D853  6E 78 06
  lda a:_var_0678                ; $D856  AD 78 06
  jsr _func_f9fb                 ; $D859  20 FB F9
  iny                            ; $D85C  C8
  jsr _func_fa0a                 ; $D85D  20 0A FA
  sta a:_var_0678                ; $D860  8D 78 06
  rol a:_var_0678                ; $D863  2E 78 06
  lda a:_var_0678                ; $D866  AD 78 06
  jsr _func_fa10                 ; $D869  20 10 FA
  iny                            ; $D86C  C8
  sta a:_var_0678                ; $D86D  8D 78 06
  rol a:_var_0678                ; $D870  2E 78 06
  lda a:_var_0678                ; $D873  AD 78 06
  jsr _func_fa21                 ; $D876  20 21 FA
  lda #$FF                       ; $D879  A9 FF
  sta a:_var_0678                ; $D87B  8D 78 06
  sta z:_var_0001                ; $D87E  85 01
  bit z:_var_0001                ; $D880  24 01
  sec                            ; $D882  38
  inc a:_var_0678                ; $D883  EE 78 06
  bne _label_d895                ; $D886  D0 0D
  bmi _label_d895                ; $D888  30 0B
  bvc _label_d895                ; $D88A  50 09
  bcc _label_d895                ; $D88C  90 07
  lda a:_var_0678                ; $D88E  AD 78 06
  cmp #$00                       ; $D891  C9 00
  beq _label_d899                ; $D893  F0 04

_label_d895:
  lda #$E5                       ; $D895  A9 E5
  sta z:_var_0000_indexed        ; $D897  85 00

_label_d899:
  lda #$7F                       ; $D899  A9 7F
  sta a:_var_0678                ; $D89B  8D 78 06
  clv                            ; $D89E  B8
  clc                            ; $D89F  18
  inc a:_var_0678                ; $D8A0  EE 78 06
  beq _label_d8b2                ; $D8A3  F0 0D
  bpl _label_d8b2                ; $D8A5  10 0B
  bvs _label_d8b2                ; $D8A7  70 09
  bcs _label_d8b2                ; $D8A9  B0 07
  lda a:_var_0678                ; $D8AB  AD 78 06
  cmp #$80                       ; $D8AE  C9 80
  beq _label_d8b6                ; $D8B0  F0 04

_label_d8b2:
  lda #$E6                       ; $D8B2  A9 E6
  sta z:_var_0000_indexed        ; $D8B4  85 00

_label_d8b6:
  lda #$00                       ; $D8B6  A9 00
  sta a:_var_0678                ; $D8B8  8D 78 06
  bit z:_var_0001                ; $D8BB  24 01
  sec                            ; $D8BD  38
  dec a:_var_0678                ; $D8BE  CE 78 06
  beq _label_d8d0                ; $D8C1  F0 0D
  bpl _label_d8d0                ; $D8C3  10 0B
  bvc _label_d8d0                ; $D8C5  50 09
  bcc _label_d8d0                ; $D8C7  90 07
  lda a:_var_0678                ; $D8C9  AD 78 06
  cmp #$FF                       ; $D8CC  C9 FF
  beq _label_d8d4                ; $D8CE  F0 04

_label_d8d0:
  lda #$E7                       ; $D8D0  A9 E7
  sta z:_var_0000_indexed        ; $D8D2  85 00

_label_d8d4:
  lda #$80                       ; $D8D4  A9 80
  sta a:_var_0678                ; $D8D6  8D 78 06
  clv                            ; $D8D9  B8
  clc                            ; $D8DA  18
  dec a:_var_0678                ; $D8DB  CE 78 06
  beq _label_d8ed                ; $D8DE  F0 0D
  bmi _label_d8ed                ; $D8E0  30 0B
  bvs _label_d8ed                ; $D8E2  70 09
  bcs _label_d8ed                ; $D8E4  B0 07
  lda a:_var_0678                ; $D8E6  AD 78 06
  cmp #$7F                       ; $D8E9  C9 7F
  beq _label_d8f1                ; $D8EB  F0 04

_label_d8ed:
  lda #$E8                       ; $D8ED  A9 E8
  sta z:_var_0000_indexed        ; $D8EF  85 00

_label_d8f1:
  lda #$01                       ; $D8F1  A9 01
  sta a:_var_0678                ; $D8F3  8D 78 06
  dec a:_var_0678                ; $D8F6  CE 78 06
  beq _label_d8ff                ; $D8F9  F0 04
  lda #$E9                       ; $D8FB  A9 E9
  sta z:_var_0000_indexed        ; $D8FD  85 00

_label_d8ff:
  rts                            ; $D8FF  60

_func_d900:
  lda #$A3                       ; $D900  A9 A3
  sta z:_var_0033_indexed        ; $D902  85 33
  lda #$89                       ; $D904  A9 89
  sta a:_var_0300_indexed        ; $D906  8D 00 03
  lda #$12                       ; $D909  A9 12
  sta a:_var_0245                ; $D90B  8D 45 02
  lda #$FF                       ; $D90E  A9 FF
  sta z:_var_0001                ; $D910  85 01
  ldx #$65                       ; $D912  A2 65
  lda #$00                       ; $D914  A9 00
  sta z:_var_0089_indexed        ; $D916  85 89
  lda #$03                       ; $D918  A9 03
  sta z:$8A                      ; $D91A  85 8A
  ldy #$00                       ; $D91C  A0 00
  sec                            ; $D91E  38
  lda #$00                       ; $D91F  A9 00
  clv                            ; $D921  B8
  lda (_var_0089_indexed),Y      ; $D922  B1 89
  beq _label_d932                ; $D924  F0 0C
  bcc _label_d932                ; $D926  90 0A
  bvs _label_d932                ; $D928  70 08
  cmp #$89                       ; $D92A  C9 89
  bne _label_d932                ; $D92C  D0 04
  cpx #$65                       ; $D92E  E0 65
  beq _label_d936                ; $D930  F0 04

_label_d932:
  lda #$EA                       ; $D932  A9 EA
  sta z:_var_0000_indexed        ; $D934  85 00

_label_d936:
  lda #$FF                       ; $D936  A9 FF
  sta z:_var_0097_indexed        ; $D938  85 97
  sta z:_var_0098                ; $D93A  85 98
  bit z:_var_0098                ; $D93C  24 98
  ldy #$34                       ; $D93E  A0 34
  lda (_var_0097_indexed),Y      ; $D940  B1 97
  cmp #$A3                       ; $D942  C9 A3
  bne _label_d948                ; $D944  D0 02
  bcs _label_d94c                ; $D946  B0 04

_label_d948:
  lda #$EB                       ; $D948  A9 EB
  sta z:_var_0000_indexed        ; $D94A  85 00

_label_d94c:
  lda z:_var_0000_indexed        ; $D94C  A5 00
  pha                            ; $D94E  48
  lda #$46                       ; $D94F  A9 46
  sta z:_var_00ff_indexed        ; $D951  85 FF
  lda #$01                       ; $D953  A9 01
  sta z:_var_0000_indexed        ; $D955  85 00
  ldy #$FF                       ; $D957  A0 FF
  lda (_var_00ff_indexed),Y      ; $D959  B1 FF
  cmp #$12                       ; $D95B  C9 12
  beq _label_d963                ; $D95D  F0 04
  pla                            ; $D95F  68
  lda #$EC                       ; $D960  A9 EC
  pha                            ; $D962  48

_label_d963:
  pla                            ; $D963  68
  sta z:_var_0000_indexed        ; $D964  85 00
  ldx #$ED                       ; $D966  A2 ED
  lda #$00                       ; $D968  A9 00
  sta z:_var_0033_indexed        ; $D96A  85 33
  lda #$04                       ; $D96C  A9 04
  sta z:_var_0034                ; $D96E  85 34
  ldy #$00                       ; $D970  A0 00
  clc                            ; $D972  18
  lda #$FF                       ; $D973  A9 FF
  sta z:_var_0001                ; $D975  85 01
  bit z:_var_0001                ; $D977  24 01
  lda #$AA                       ; $D979  A9 AA
  sta a:_var_0400_indexed        ; $D97B  8D 00 04
  lda #$55                       ; $D97E  A9 55
  ora (_var_0033_indexed),Y      ; $D980  11 33
  bcs _label_d98c                ; $D982  B0 08
  bpl _label_d98c                ; $D984  10 06
  cmp #$FF                       ; $D986  C9 FF
  bne _label_d98c                ; $D988  D0 02
  bvs _label_d98e                ; $D98A  70 02

_label_d98c:
  stx z:_var_0000_indexed        ; $D98C  86 00

_label_d98e:
  inx                            ; $D98E  E8
  sec                            ; $D98F  38
  clv                            ; $D990  B8
  lda #$00                       ; $D991  A9 00
  ora (_var_0033_indexed),Y      ; $D993  11 33
  beq _label_d99d                ; $D995  F0 06
  bvs _label_d99d                ; $D997  70 04
  bcc _label_d99d                ; $D999  90 02
  bmi _label_d99f                ; $D99B  30 02

_label_d99d:
  stx z:_var_0000_indexed        ; $D99D  86 00

_label_d99f:
  inx                            ; $D99F  E8
  clc                            ; $D9A0  18
  bit z:_var_0001                ; $D9A1  24 01
  lda #$55                       ; $D9A3  A9 55
  and (_var_0033_indexed),Y      ; $D9A5  31 33
  bne _label_d9af                ; $D9A7  D0 06
  bvc _label_d9af                ; $D9A9  50 04
  bcs _label_d9af                ; $D9AB  B0 02
  bpl _label_d9b1                ; $D9AD  10 02

_label_d9af:
  stx z:_var_0000_indexed        ; $D9AF  86 00

_label_d9b1:
  inx                            ; $D9B1  E8
  sec                            ; $D9B2  38
  clv                            ; $D9B3  B8
  lda #$EF                       ; $D9B4  A9 EF
  sta a:_var_0400_indexed        ; $D9B6  8D 00 04
  lda #$F8                       ; $D9B9  A9 F8
  and (_var_0033_indexed),Y      ; $D9BB  31 33
  bcc _label_d9c7                ; $D9BD  90 08
  bpl _label_d9c7                ; $D9BF  10 06
  cmp #$E8                       ; $D9C1  C9 E8
  bne _label_d9c7                ; $D9C3  D0 02
  bvc _label_d9c9                ; $D9C5  50 02

_label_d9c7:
  stx z:_var_0000_indexed        ; $D9C7  86 00

_label_d9c9:
  inx                            ; $D9C9  E8
  clc                            ; $D9CA  18
  bit z:_var_0001                ; $D9CB  24 01
  lda #$AA                       ; $D9CD  A9 AA
  sta a:_var_0400_indexed        ; $D9CF  8D 00 04
  lda #$5F                       ; $D9D2  A9 5F
  eor (_var_0033_indexed),Y      ; $D9D4  51 33
  bcs _label_d9e0                ; $D9D6  B0 08
  bpl _label_d9e0                ; $D9D8  10 06
  cmp #$F5                       ; $D9DA  C9 F5
  bne _label_d9e0                ; $D9DC  D0 02
  bvs _label_d9e2                ; $D9DE  70 02

_label_d9e0:
  stx z:_var_0000_indexed        ; $D9E0  86 00

_label_d9e2:
  inx                            ; $D9E2  E8
  sec                            ; $D9E3  38
  clv                            ; $D9E4  B8
  lda #$70                       ; $D9E5  A9 70
  sta a:_var_0400_indexed        ; $D9E7  8D 00 04
  eor (_var_0033_indexed),Y      ; $D9EA  51 33
  bne _label_d9f4                ; $D9EC  D0 06
  bvs _label_d9f4                ; $D9EE  70 04
  bcc _label_d9f4                ; $D9F0  90 02
  bpl _label_d9f6                ; $D9F2  10 02

_label_d9f4:
  stx z:_var_0000_indexed        ; $D9F4  86 00

_label_d9f6:
  inx                            ; $D9F6  E8
  clc                            ; $D9F7  18
  bit z:_var_0001                ; $D9F8  24 01
  lda #$69                       ; $D9FA  A9 69
  sta a:_var_0400_indexed        ; $D9FC  8D 00 04
  lda #$00                       ; $D9FF  A9 00
  adc (_var_0033_indexed),Y      ; $DA01  71 33
  bmi _label_da0d                ; $DA03  30 08
  bcs _label_da0d                ; $DA05  B0 06
  cmp #$69                       ; $DA07  C9 69
  bne _label_da0d                ; $DA09  D0 02
  bvc _label_da0f                ; $DA0B  50 02

_label_da0d:
  stx z:_var_0000_indexed        ; $DA0D  86 00

_label_da0f:
  inx                            ; $DA0F  E8
  sec                            ; $DA10  38
  bit z:_var_0001                ; $DA11  24 01
  lda #$00                       ; $DA13  A9 00
  adc (_var_0033_indexed),Y      ; $DA15  71 33
  bmi _label_da21                ; $DA17  30 08
  bcs _label_da21                ; $DA19  B0 06
  cmp #$6A                       ; $DA1B  C9 6A
  bne _label_da21                ; $DA1D  D0 02
  bvc _label_da23                ; $DA1F  50 02

_label_da21:
  stx z:_var_0000_indexed        ; $DA21  86 00

_label_da23:
  inx                            ; $DA23  E8
  sec                            ; $DA24  38
  clv                            ; $DA25  B8
  lda #$7F                       ; $DA26  A9 7F
  sta a:_var_0400_indexed        ; $DA28  8D 00 04
  adc (_var_0033_indexed),Y      ; $DA2B  71 33
  bpl _label_da37                ; $DA2D  10 08
  bcs _label_da37                ; $DA2F  B0 06
  cmp #$FF                       ; $DA31  C9 FF
  bne _label_da37                ; $DA33  D0 02
  bvs _label_da39                ; $DA35  70 02

_label_da37:
  stx z:_var_0000_indexed        ; $DA37  86 00

_label_da39:
  inx                            ; $DA39  E8
  clc                            ; $DA3A  18
  bit z:_var_0001                ; $DA3B  24 01
  lda #$80                       ; $DA3D  A9 80
  sta a:_var_0400_indexed        ; $DA3F  8D 00 04
  lda #$7F                       ; $DA42  A9 7F
  adc (_var_0033_indexed),Y      ; $DA44  71 33
  bpl _label_da50                ; $DA46  10 08
  bcs _label_da50                ; $DA48  B0 06
  cmp #$FF                       ; $DA4A  C9 FF
  bne _label_da50                ; $DA4C  D0 02
  bvc _label_da52                ; $DA4E  50 02

_label_da50:
  stx z:_var_0000_indexed        ; $DA50  86 00

_label_da52:
  inx                            ; $DA52  E8
  sec                            ; $DA53  38
  clv                            ; $DA54  B8
  lda #$80                       ; $DA55  A9 80
  sta a:_var_0400_indexed        ; $DA57  8D 00 04
  lda #$7F                       ; $DA5A  A9 7F
  adc (_var_0033_indexed),Y      ; $DA5C  71 33
  bne _label_da66                ; $DA5E  D0 06
  bmi _label_da66                ; $DA60  30 04
  bvs _label_da66                ; $DA62  70 02
  bcs _label_da68                ; $DA64  B0 02

_label_da66:
  stx z:_var_0000_indexed        ; $DA66  86 00

_label_da68:
  inx                            ; $DA68  E8
  bit z:_var_0001                ; $DA69  24 01
  lda #$40                       ; $DA6B  A9 40
  sta a:_var_0400_indexed        ; $DA6D  8D 00 04
  cmp (_var_0033_indexed),Y      ; $DA70  D1 33
  bmi _label_da7a                ; $DA72  30 06
  bcc _label_da7a                ; $DA74  90 04
  bne _label_da7a                ; $DA76  D0 02
  bvs _label_da7c                ; $DA78  70 02

_label_da7a:
  stx z:_var_0000_indexed        ; $DA7A  86 00

_label_da7c:
  inx                            ; $DA7C  E8
  clv                            ; $DA7D  B8
  dec a:_var_0400_indexed        ; $DA7E  CE 00 04
  cmp (_var_0033_indexed),Y      ; $DA81  D1 33
  beq _label_da8b                ; $DA83  F0 06
  bmi _label_da8b                ; $DA85  30 04
  bcc _label_da8b                ; $DA87  90 02
  bvc _label_da8d                ; $DA89  50 02

_label_da8b:
  stx z:_var_0000_indexed        ; $DA8B  86 00

_label_da8d:
  inx                            ; $DA8D  E8
  inc a:_var_0400_indexed        ; $DA8E  EE 00 04
  inc a:_var_0400_indexed        ; $DA91  EE 00 04
  cmp (_var_0033_indexed),Y      ; $DA94  D1 33
  beq _label_da9a                ; $DA96  F0 02
  bmi _label_da9c                ; $DA98  30 02

_label_da9a:
  stx z:_var_0000_indexed        ; $DA9A  86 00

_label_da9c:
  inx                            ; $DA9C  E8
  lda #$00                       ; $DA9D  A9 00
  sta a:_var_0400_indexed        ; $DA9F  8D 00 04
  lda #$80                       ; $DAA2  A9 80
  cmp (_var_0033_indexed),Y      ; $DAA4  D1 33
  beq _label_daac                ; $DAA6  F0 04
  bpl _label_daac                ; $DAA8  10 02
  bcs _label_daae                ; $DAAA  B0 02

_label_daac:
  stx z:_var_0000_indexed        ; $DAAC  86 00

_label_daae:
  inx                            ; $DAAE  E8
  ldy #$80                       ; $DAAF  A0 80
  sty a:_var_0400_indexed        ; $DAB1  8C 00 04
  ldy #$00                       ; $DAB4  A0 00
  cmp (_var_0033_indexed),Y      ; $DAB6  D1 33
  bne _label_dabe                ; $DAB8  D0 04
  bmi _label_dabe                ; $DABA  30 02
  bcs _label_dac0                ; $DABC  B0 02

_label_dabe:
  stx z:_var_0000_indexed        ; $DABE  86 00

_label_dac0:
  inx                            ; $DAC0  E8
  inc a:_var_0400_indexed        ; $DAC1  EE 00 04
  cmp (_var_0033_indexed),Y      ; $DAC4  D1 33
  bcs _label_dacc                ; $DAC6  B0 04
  beq _label_dacc                ; $DAC8  F0 02
  bmi _label_dace                ; $DACA  30 02

_label_dacc:
  stx z:_var_0000_indexed        ; $DACC  86 00

_label_dace:
  inx                            ; $DACE  E8
  dec a:_var_0400_indexed        ; $DACF  CE 00 04
  dec a:_var_0400_indexed        ; $DAD2  CE 00 04
  cmp (_var_0033_indexed),Y      ; $DAD5  D1 33
  bcc _label_dadd                ; $DAD7  90 04
  beq _label_dadd                ; $DAD9  F0 02
  bpl _label_dadf                ; $DADB  10 02

_label_dadd:
  stx z:_var_0000_indexed        ; $DADD  86 00

_label_dadf:
  rts                            ; $DADF  60

_func_dae0:
  lda #$00                       ; $DAE0  A9 00
  sta z:_var_0033_indexed        ; $DAE2  85 33
  lda #$04                       ; $DAE4  A9 04
  sta z:_var_0034                ; $DAE6  85 34
  ldy #$00                       ; $DAE8  A0 00
  ldx #$01                       ; $DAEA  A2 01
  bit z:_var_0001                ; $DAEC  24 01
  lda #$40                       ; $DAEE  A9 40
  sta a:_var_0400_indexed        ; $DAF0  8D 00 04
  sec                            ; $DAF3  38
  sbc (_var_0033_indexed),Y      ; $DAF4  F1 33
  bmi _label_db02                ; $DAF6  30 0A
  bcc _label_db02                ; $DAF8  90 08
  bne _label_db02                ; $DAFA  D0 06
  bvs _label_db02                ; $DAFC  70 04
  cmp #$00                       ; $DAFE  C9 00
  beq _label_db04                ; $DB00  F0 02

_label_db02:
  stx z:_var_0000_indexed        ; $DB02  86 00

_label_db04:
  inx                            ; $DB04  E8
  clv                            ; $DB05  B8
  sec                            ; $DB06  38
  lda #$40                       ; $DB07  A9 40
  dec a:_var_0400_indexed        ; $DB09  CE 00 04
  sbc (_var_0033_indexed),Y      ; $DB0C  F1 33
  beq _label_db1a                ; $DB0E  F0 0A
  bmi _label_db1a                ; $DB10  30 08
  bcc _label_db1a                ; $DB12  90 06
  bvs _label_db1a                ; $DB14  70 04
  cmp #$01                       ; $DB16  C9 01
  beq _label_db1c                ; $DB18  F0 02

_label_db1a:
  stx z:_var_0000_indexed        ; $DB1A  86 00

_label_db1c:
  inx                            ; $DB1C  E8
  lda #$40                       ; $DB1D  A9 40
  sec                            ; $DB1F  38
  bit z:_var_0001                ; $DB20  24 01
  inc a:_var_0400_indexed        ; $DB22  EE 00 04
  inc a:_var_0400_indexed        ; $DB25  EE 00 04
  sbc (_var_0033_indexed),Y      ; $DB28  F1 33
  bcs _label_db36                ; $DB2A  B0 0A
  beq _label_db36                ; $DB2C  F0 08
  bpl _label_db36                ; $DB2E  10 06
  bvs _label_db36                ; $DB30  70 04
  cmp #$FF                       ; $DB32  C9 FF
  beq _label_db38                ; $DB34  F0 02

_label_db36:
  stx z:_var_0000_indexed        ; $DB36  86 00

_label_db38:
  inx                            ; $DB38  E8
  clc                            ; $DB39  18
  lda #$00                       ; $DB3A  A9 00
  sta a:_var_0400_indexed        ; $DB3C  8D 00 04
  lda #$80                       ; $DB3F  A9 80
  sbc (_var_0033_indexed),Y      ; $DB41  F1 33
  bcc _label_db49                ; $DB43  90 04
  cmp #$7F                       ; $DB45  C9 7F
  beq _label_db4b                ; $DB47  F0 02

_label_db49:
  stx z:_var_0000_indexed        ; $DB49  86 00

_label_db4b:
  inx                            ; $DB4B  E8
  sec                            ; $DB4C  38
  lda #$7F                       ; $DB4D  A9 7F
  sta a:_var_0400_indexed        ; $DB4F  8D 00 04
  lda #$81                       ; $DB52  A9 81
  sbc (_var_0033_indexed),Y      ; $DB54  F1 33
  bvc _label_db5e                ; $DB56  50 06
  bcc _label_db5e                ; $DB58  90 04
  cmp #$02                       ; $DB5A  C9 02
  beq _label_db60                ; $DB5C  F0 02

_label_db5e:
  stx z:_var_0000_indexed        ; $DB5E  86 00

_label_db60:
  inx                            ; $DB60  E8
  lda #$00                       ; $DB61  A9 00
  lda #$87                       ; $DB63  A9 87
  sta (_var_0033_indexed),Y      ; $DB65  91 33
  lda a:_var_0400_indexed        ; $DB67  AD 00 04
  cmp #$87                       ; $DB6A  C9 87
  beq _label_db70                ; $DB6C  F0 02
  stx z:_var_0000_indexed        ; $DB6E  86 00

_label_db70:
  inx                            ; $DB70  E8
  lda #$7E                       ; $DB71  A9 7E
  sta a:_var_0200                ; $DB73  8D 00 02
  lda #$DB                       ; $DB76  A9 DB
  sta a:_var_0201                ; $DB78  8D 01 02
  jmp (_var_0200)                ; $DB7B  6C 00 02  jump engine detected

.byte $a9, $00, $8d, $ff, $02, $a9, $01, $8d, $00, $03, $a9, $03, $8d, $00, $02, $a9 ; $DB7E
.byte $a9, $8d, $00, $01, $a9, $55, $8d, $01, $01, $a9, $60, $8d, $02, $01, $a9, $a9 ; $DB8E
.byte $8d, $00, $03, $a9, $aa, $8d, $01, $03, $a9, $60, $8d, $02, $03, $20, $b5, $db ; $DB9E
.byte $c9, $aa, $f0, $02, $86, $00, $60, $6c, $ff, $02 ; $DBAE

_label_dbb8:
  lda #$FF                       ; $DBB8  A9 FF
  sta z:_var_0001                ; $DBBA  85 01
  lda #$AA                       ; $DBBC  A9 AA
  sta z:_var_0033_indexed        ; $DBBE  85 33
  lda #$BB                       ; $DBC0  A9 BB
  sta z:_var_0089_indexed        ; $DBC2  85 89
  ldx #$00                       ; $DBC4  A2 00
  lda #$66                       ; $DBC6  A9 66
  bit z:_var_0001                ; $DBC8  24 01
  sec                            ; $DBCA  38
  ldy #$00                       ; $DBCB  A0 00
  ldy z:_var_0033_indexed,X      ; $DBCD  B4 33
  bpl _label_dbe3                ; $DBCF  10 12
  beq _label_dbe3                ; $DBD1  F0 10
  bvc _label_dbe3                ; $DBD3  50 0E
  bcc _label_dbe3                ; $DBD5  90 0C
  cmp #$66                       ; $DBD7  C9 66
  bne _label_dbe3                ; $DBD9  D0 08
  cpx #$00                       ; $DBDB  E0 00
  bne _label_dbe3                ; $DBDD  D0 04
  cpy #$AA                       ; $DBDF  C0 AA
  beq _label_dbe7                ; $DBE1  F0 04

_label_dbe3:
  lda #$08                       ; $DBE3  A9 08
  sta z:_var_0000_indexed        ; $DBE5  85 00

_label_dbe7:
  ldx #$8A                       ; $DBE7  A2 8A
  lda #$66                       ; $DBE9  A9 66
  clv                            ; $DBEB  B8
  clc                            ; $DBEC  18
  ldy #$00                       ; $DBED  A0 00
  ldy z:_var_00ff_indexed,X      ; $DBEF  B4 FF
  bpl _label_dc05                ; $DBF1  10 12
  beq _label_dc05                ; $DBF3  F0 10
  bvs _label_dc05                ; $DBF5  70 0E
  bcs _label_dc05                ; $DBF7  B0 0C
  cpy #$BB                       ; $DBF9  C0 BB
  bne _label_dc05                ; $DBFB  D0 08
  cmp #$66                       ; $DBFD  C9 66
  bne _label_dc05                ; $DBFF  D0 04
  cpx #$8A                       ; $DC01  E0 8A
  beq _label_dc09                ; $DC03  F0 04

_label_dc05:
  lda #$09                       ; $DC05  A9 09
  sta z:_var_0000_indexed        ; $DC07  85 00

_label_dc09:
  bit z:_var_0001                ; $DC09  24 01
  sec                            ; $DC0B  38
  ldy #$44                       ; $DC0C  A0 44
  ldx #$00                       ; $DC0E  A2 00
  sty z:_var_0033_indexed,X      ; $DC10  94 33
  lda z:_var_0033_indexed        ; $DC12  A5 33
  bcc _label_dc2e                ; $DC14  90 18
  cmp #$44                       ; $DC16  C9 44
  bne _label_dc2e                ; $DC18  D0 14
  bvc _label_dc2e                ; $DC1A  50 12
  clc                            ; $DC1C  18
  clv                            ; $DC1D  B8
  ldy #$99                       ; $DC1E  A0 99
  ldx #$80                       ; $DC20  A2 80
  sty z:_var_0085_indexed,X      ; $DC22  94 85
  lda z:_var_0005                ; $DC24  A5 05
  bcs _label_dc2e                ; $DC26  B0 06
  cmp #$99                       ; $DC28  C9 99
  bne _label_dc2e                ; $DC2A  D0 02
  bvc _label_dc32                ; $DC2C  50 04

_label_dc2e:
  lda #$0A                       ; $DC2E  A9 0A
  sta z:_var_0000_indexed        ; $DC30  85 00

_label_dc32:
  ldy #$0B                       ; $DC32  A0 0B
  lda #$AA                       ; $DC34  A9 AA
  ldx #$78                       ; $DC36  A2 78
  sta z:_var_0078                ; $DC38  85 78
  jsr _func_f7b6                 ; $DC3A  20 B6 F7
  ora z:_var_0000_indexed,X      ; $DC3D  15 00
  jsr _func_f7c0                 ; $DC3F  20 C0 F7
  iny                            ; $DC42  C8
  lda #$00                       ; $DC43  A9 00
  sta z:_var_0078                ; $DC45  85 78
  jsr _func_f7ce                 ; $DC47  20 CE F7
  ora z:_var_0000_indexed,X      ; $DC4A  15 00
  jsr _func_f7d3                 ; $DC4C  20 D3 F7
  iny                            ; $DC4F  C8
  lda #$AA                       ; $DC50  A9 AA
  sta z:_var_0078                ; $DC52  85 78
  jsr _func_f7df                 ; $DC54  20 DF F7
  and z:_var_0000_indexed,X      ; $DC57  35 00
  jsr _func_f7e5                 ; $DC59  20 E5 F7
  iny                            ; $DC5C  C8
  lda #$EF                       ; $DC5D  A9 EF
  sta z:_var_0078                ; $DC5F  85 78
  jsr _func_f7f1                 ; $DC61  20 F1 F7
  and z:_var_0000_indexed,X      ; $DC64  35 00
  jsr _func_f7f6                 ; $DC66  20 F6 F7
  iny                            ; $DC69  C8
  lda #$AA                       ; $DC6A  A9 AA
  sta z:_var_0078                ; $DC6C  85 78
  jsr _func_f804                 ; $DC6E  20 04 F8
  eor z:_var_0000_indexed,X      ; $DC71  55 00
  jsr _func_f80a                 ; $DC73  20 0A F8
  iny                            ; $DC76  C8
  lda #$70                       ; $DC77  A9 70
  sta z:_var_0078                ; $DC79  85 78
  jsr _func_f818                 ; $DC7B  20 18 F8
  eor z:_var_0000_indexed,X      ; $DC7E  55 00
  jsr _func_f81d                 ; $DC80  20 1D F8
  iny                            ; $DC83  C8
  lda #$69                       ; $DC84  A9 69
  sta z:_var_0078                ; $DC86  85 78
  jsr _func_f829                 ; $DC88  20 29 F8
  adc z:_var_0000_indexed,X      ; $DC8B  75 00
  jsr _func_f82f                 ; $DC8D  20 2F F8
  iny                            ; $DC90  C8
  jsr _func_f83d                 ; $DC91  20 3D F8
  adc z:_var_0000_indexed,X      ; $DC94  75 00
  jsr _func_f843                 ; $DC96  20 43 F8
  iny                            ; $DC99  C8
  lda #$7F                       ; $DC9A  A9 7F
  sta z:_var_0078                ; $DC9C  85 78
  jsr _func_f851                 ; $DC9E  20 51 F8
  adc z:_var_0000_indexed,X      ; $DCA1  75 00
  jsr _func_f856                 ; $DCA3  20 56 F8
  iny                            ; $DCA6  C8
  lda #$80                       ; $DCA7  A9 80
  sta z:_var_0078                ; $DCA9  85 78
  jsr _func_f864                 ; $DCAB  20 64 F8
  adc z:_var_0000_indexed,X      ; $DCAE  75 00
  jsr _func_f86a                 ; $DCB0  20 6A F8
  iny                            ; $DCB3  C8
  jsr _func_f878                 ; $DCB4  20 78 F8
  adc z:_var_0000_indexed,X      ; $DCB7  75 00
  jsr _func_f87d                 ; $DCB9  20 7D F8
  iny                            ; $DCBC  C8
  lda #$40                       ; $DCBD  A9 40
  sta z:_var_0078                ; $DCBF  85 78
  jsr _func_f889                 ; $DCC1  20 89 F8
  cmp z:_var_0000_indexed,X      ; $DCC4  D5 00
  jsr _func_f88e                 ; $DCC6  20 8E F8
  iny                            ; $DCC9  C8
  pha                            ; $DCCA  48
  lda #$3F                       ; $DCCB  A9 3F
  sta z:_var_0078                ; $DCCD  85 78
  pla                            ; $DCCF  68
  jsr _func_f89a                 ; $DCD0  20 9A F8
  cmp z:_var_0000_indexed,X      ; $DCD3  D5 00
  jsr _func_f89c                 ; $DCD5  20 9C F8
  iny                            ; $DCD8  C8
  pha                            ; $DCD9  48
  lda #$41                       ; $DCDA  A9 41
  sta z:_var_0078                ; $DCDC  85 78
  pla                            ; $DCDE  68
  cmp z:_var_0000_indexed,X      ; $DCDF  D5 00
  jsr _func_f8a8                 ; $DCE1  20 A8 F8
  iny                            ; $DCE4  C8
  pha                            ; $DCE5  48
  lda #$00                       ; $DCE6  A9 00
  sta z:_var_0078                ; $DCE8  85 78
  pla                            ; $DCEA  68
  jsr _func_f8b2                 ; $DCEB  20 B2 F8
  cmp z:_var_0000_indexed,X      ; $DCEE  D5 00
  jsr _func_f8b5                 ; $DCF0  20 B5 F8
  iny                            ; $DCF3  C8
  pha                            ; $DCF4  48
  lda #$80                       ; $DCF5  A9 80
  sta z:_var_0078                ; $DCF7  85 78
  pla                            ; $DCF9  68
  cmp z:_var_0000_indexed,X      ; $DCFA  D5 00
  jsr _func_f8bf                 ; $DCFC  20 BF F8
  iny                            ; $DCFF  C8
  pha                            ; $DD00  48
  lda #$81                       ; $DD01  A9 81
  sta z:_var_0078                ; $DD03  85 78
  pla                            ; $DD05  68
  cmp z:_var_0000_indexed,X      ; $DD06  D5 00
  jsr _func_f8c9                 ; $DD08  20 C9 F8
  iny                            ; $DD0B  C8
  pha                            ; $DD0C  48
  lda #$7F                       ; $DD0D  A9 7F
  sta z:_var_0078                ; $DD0F  85 78
  pla                            ; $DD11  68
  cmp z:_var_0000_indexed,X      ; $DD12  D5 00
  jsr _func_f8d3                 ; $DD14  20 D3 F8
  iny                            ; $DD17  C8
  lda #$40                       ; $DD18  A9 40
  sta z:_var_0078                ; $DD1A  85 78
  jsr _func_f931                 ; $DD1C  20 31 F9
  sbc z:_var_0000_indexed,X      ; $DD1F  F5 00
  jsr _func_f937                 ; $DD21  20 37 F9
  iny                            ; $DD24  C8
  lda #$3F                       ; $DD25  A9 3F
  sta z:_var_0078                ; $DD27  85 78
  jsr _func_f947                 ; $DD29  20 47 F9
  sbc z:_var_0000_indexed,X      ; $DD2C  F5 00
  jsr _func_f94c                 ; $DD2E  20 4C F9
  iny                            ; $DD31  C8
  lda #$41                       ; $DD32  A9 41
  sta z:_var_0078                ; $DD34  85 78
  jsr _func_f95c                 ; $DD36  20 5C F9
  sbc z:_var_0000_indexed,X      ; $DD39  F5 00
  jsr _func_f962                 ; $DD3B  20 62 F9
  iny                            ; $DD3E  C8
  lda #$00                       ; $DD3F  A9 00
  sta z:_var_0078                ; $DD41  85 78
  jsr _func_f972                 ; $DD43  20 72 F9
  sbc z:_var_0000_indexed,X      ; $DD46  F5 00
  jsr _func_f976                 ; $DD48  20 76 F9
  iny                            ; $DD4B  C8
  lda #$7F                       ; $DD4C  A9 7F
  sta z:_var_0078                ; $DD4E  85 78
  jsr _func_f980                 ; $DD50  20 80 F9
  sbc z:_var_0000_indexed,X      ; $DD53  F5 00
  jsr _func_f984                 ; $DD55  20 84 F9
  lda #$AA                       ; $DD58  A9 AA
  sta z:_var_0033_indexed        ; $DD5A  85 33
  lda #$BB                       ; $DD5C  A9 BB
  sta z:_var_0089_indexed        ; $DD5E  85 89
  ldx #$00                       ; $DD60  A2 00
  ldy #$66                       ; $DD62  A0 66
  bit z:_var_0001                ; $DD64  24 01
  sec                            ; $DD66  38
  lda #$00                       ; $DD67  A9 00
  lda z:_var_0033_indexed,X      ; $DD69  B5 33
  bpl _label_dd7f                ; $DD6B  10 12
  beq _label_dd7f                ; $DD6D  F0 10
  bvc _label_dd7f                ; $DD6F  50 0E
  bcc _label_dd7f                ; $DD71  90 0C
  cpy #$66                       ; $DD73  C0 66
  bne _label_dd7f                ; $DD75  D0 08
  cpx #$00                       ; $DD77  E0 00
  bne _label_dd7f                ; $DD79  D0 04
  cmp #$AA                       ; $DD7B  C9 AA
  beq _label_dd83                ; $DD7D  F0 04

_label_dd7f:
  lda #$22                       ; $DD7F  A9 22
  sta z:_var_0000_indexed        ; $DD81  85 00

_label_dd83:
  ldx #$8A                       ; $DD83  A2 8A
  ldy #$66                       ; $DD85  A0 66
  clv                            ; $DD87  B8
  clc                            ; $DD88  18
  lda #$00                       ; $DD89  A9 00
  lda z:_var_00ff_indexed,X      ; $DD8B  B5 FF
  bpl _label_dda1                ; $DD8D  10 12
  beq _label_dda1                ; $DD8F  F0 10
  bvs _label_dda1                ; $DD91  70 0E
  bcs _label_dda1                ; $DD93  B0 0C
  cmp #$BB                       ; $DD95  C9 BB
  bne _label_dda1                ; $DD97  D0 08
  cpy #$66                       ; $DD99  C0 66
  bne _label_dda1                ; $DD9B  D0 04
  cpx #$8A                       ; $DD9D  E0 8A
  beq _label_dda5                ; $DD9F  F0 04

_label_dda1:
  lda #$23                       ; $DDA1  A9 23
  sta z:_var_0000_indexed        ; $DDA3  85 00

_label_dda5:
  bit z:_var_0001                ; $DDA5  24 01
  sec                            ; $DDA7  38
  lda #$44                       ; $DDA8  A9 44
  ldx #$00                       ; $DDAA  A2 00
  sta z:_var_0033_indexed,X      ; $DDAC  95 33
  lda z:_var_0033_indexed        ; $DDAE  A5 33
  bcc _label_ddca                ; $DDB0  90 18
  cmp #$44                       ; $DDB2  C9 44
  bne _label_ddca                ; $DDB4  D0 14
  bvc _label_ddca                ; $DDB6  50 12
  clc                            ; $DDB8  18
  clv                            ; $DDB9  B8
  lda #$99                       ; $DDBA  A9 99
  ldx #$80                       ; $DDBC  A2 80
  sta z:_var_0085_indexed,X      ; $DDBE  95 85
  lda z:_var_0005                ; $DDC0  A5 05
  bcs _label_ddca                ; $DDC2  B0 06
  cmp #$99                       ; $DDC4  C9 99
  bne _label_ddca                ; $DDC6  D0 02
  bvc _label_ddce                ; $DDC8  50 04

_label_ddca:
  lda #$24                       ; $DDCA  A9 24
  sta z:_var_0000_indexed        ; $DDCC  85 00

_label_ddce:
  ldy #$25                       ; $DDCE  A0 25
  ldx #$78                       ; $DDD0  A2 78
  jsr _func_f990                 ; $DDD2  20 90 F9
  sta z:_var_0000_indexed,X      ; $DDD5  95 00
  lsr z:_var_0000_indexed,X      ; $DDD7  56 00
  lda z:_var_0000_indexed,X      ; $DDD9  B5 00
  jsr _func_f99d                 ; $DDDB  20 9D F9
  iny                            ; $DDDE  C8
  sta z:_var_0000_indexed,X      ; $DDDF  95 00
  lsr z:_var_0000_indexed,X      ; $DDE1  56 00
  lda z:_var_0000_indexed,X      ; $DDE3  B5 00
  jsr _func_f9ad                 ; $DDE5  20 AD F9
  iny                            ; $DDE8  C8
  jsr _func_f9bd                 ; $DDE9  20 BD F9
  sta z:_var_0000_indexed,X      ; $DDEC  95 00
  asl z:_var_0000_indexed,X      ; $DDEE  16 00
  lda z:_var_0000_indexed,X      ; $DDF0  B5 00
  jsr _func_f9c3                 ; $DDF2  20 C3 F9
  iny                            ; $DDF5  C8
  sta z:_var_0000_indexed,X      ; $DDF6  95 00
  asl z:_var_0000_indexed,X      ; $DDF8  16 00
  lda z:_var_0000_indexed,X      ; $DDFA  B5 00
  jsr _func_f9d4                 ; $DDFC  20 D4 F9
  iny                            ; $DDFF  C8
  jsr _func_f9e4                 ; $DE00  20 E4 F9
  sta z:_var_0000_indexed,X      ; $DE03  95 00
  ror z:_var_0000_indexed,X      ; $DE05  76 00
  lda z:_var_0000_indexed,X      ; $DE07  B5 00
  jsr _func_f9ea                 ; $DE09  20 EA F9
  iny                            ; $DE0C  C8
  sta z:_var_0000_indexed,X      ; $DE0D  95 00
  ror z:_var_0000_indexed,X      ; $DE0F  76 00
  lda z:_var_0000_indexed,X      ; $DE11  B5 00
  jsr _func_f9fb                 ; $DE13  20 FB F9
  iny                            ; $DE16  C8
  jsr _func_fa0a                 ; $DE17  20 0A FA
  sta z:_var_0000_indexed,X      ; $DE1A  95 00
  rol z:_var_0000_indexed,X      ; $DE1C  36 00
  lda z:_var_0000_indexed,X      ; $DE1E  B5 00
  jsr _func_fa10                 ; $DE20  20 10 FA
  iny                            ; $DE23  C8
  sta z:_var_0000_indexed,X      ; $DE24  95 00
  rol z:_var_0000_indexed,X      ; $DE26  36 00
  lda z:_var_0000_indexed,X      ; $DE28  B5 00
  jsr _func_fa21                 ; $DE2A  20 21 FA
  lda #$FF                       ; $DE2D  A9 FF
  sta z:_var_0000_indexed,X      ; $DE2F  95 00
  sta z:_var_0001                ; $DE31  85 01
  bit z:_var_0001                ; $DE33  24 01
  sec                            ; $DE35  38
  inc z:_var_0000_indexed,X      ; $DE36  F6 00
  bne _label_de46                ; $DE38  D0 0C
  bmi _label_de46                ; $DE3A  30 0A
  bvc _label_de46                ; $DE3C  50 08
  bcc _label_de46                ; $DE3E  90 06
  lda z:_var_0000_indexed,X      ; $DE40  B5 00
  cmp #$00                       ; $DE42  C9 00
  beq _label_de4a                ; $DE44  F0 04

_label_de46:
  lda #$2D                       ; $DE46  A9 2D
  sta z:_var_0000_indexed        ; $DE48  85 00

_label_de4a:
  lda #$7F                       ; $DE4A  A9 7F
  sta z:_var_0000_indexed,X      ; $DE4C  95 00
  clv                            ; $DE4E  B8
  clc                            ; $DE4F  18
  inc z:_var_0000_indexed,X      ; $DE50  F6 00
  beq _label_de60                ; $DE52  F0 0C
  bpl _label_de60                ; $DE54  10 0A
  bvs _label_de60                ; $DE56  70 08
  bcs _label_de60                ; $DE58  B0 06
  lda z:_var_0000_indexed,X      ; $DE5A  B5 00
  cmp #$80                       ; $DE5C  C9 80
  beq _label_de64                ; $DE5E  F0 04

_label_de60:
  lda #$2E                       ; $DE60  A9 2E
  sta z:_var_0000_indexed        ; $DE62  85 00

_label_de64:
  lda #$00                       ; $DE64  A9 00
  sta z:_var_0000_indexed,X      ; $DE66  95 00
  bit z:_var_0001                ; $DE68  24 01
  sec                            ; $DE6A  38
  dec z:_var_0000_indexed,X      ; $DE6B  D6 00
  beq _label_de7b                ; $DE6D  F0 0C
  bpl _label_de7b                ; $DE6F  10 0A
  bvc _label_de7b                ; $DE71  50 08
  bcc _label_de7b                ; $DE73  90 06
  lda z:_var_0000_indexed,X      ; $DE75  B5 00
  cmp #$FF                       ; $DE77  C9 FF
  beq _label_de7f                ; $DE79  F0 04

_label_de7b:
  lda #$2F                       ; $DE7B  A9 2F
  sta z:_var_0000_indexed        ; $DE7D  85 00

_label_de7f:
  lda #$80                       ; $DE7F  A9 80
  sta z:_var_0000_indexed,X      ; $DE81  95 00
  clv                            ; $DE83  B8
  clc                            ; $DE84  18
  dec z:_var_0000_indexed,X      ; $DE85  D6 00
  beq _label_de95                ; $DE87  F0 0C
  bmi _label_de95                ; $DE89  30 0A
  bvs _label_de95                ; $DE8B  70 08
  bcs _label_de95                ; $DE8D  B0 06
  lda z:_var_0000_indexed,X      ; $DE8F  B5 00
  cmp #$7F                       ; $DE91  C9 7F
  beq _label_de99                ; $DE93  F0 04

_label_de95:
  lda #$30                       ; $DE95  A9 30
  sta z:_var_0000_indexed        ; $DE97  85 00

_label_de99:
  lda #$01                       ; $DE99  A9 01
  sta z:_var_0000_indexed,X      ; $DE9B  95 00
  dec z:_var_0000_indexed,X      ; $DE9D  D6 00
  beq _label_dea5                ; $DE9F  F0 04
  lda #$31                       ; $DEA1  A9 31
  sta z:_var_0000_indexed        ; $DEA3  85 00

_label_dea5:
  lda #$33                       ; $DEA5  A9 33
  sta z:_var_0078                ; $DEA7  85 78
  lda #$44                       ; $DEA9  A9 44
  ldy #$78                       ; $DEAB  A0 78
  ldx #$00                       ; $DEAD  A2 00
  sec                            ; $DEAF  38
  bit z:_var_0001                ; $DEB0  24 01
  ldx z:_var_0000_indexed,Y      ; $DEB2  B6 00
  bcc _label_dec8                ; $DEB4  90 12
  bvc _label_dec8                ; $DEB6  50 10
  bmi _label_dec8                ; $DEB8  30 0E
  beq _label_dec8                ; $DEBA  F0 0C
  cpx #$33                       ; $DEBC  E0 33
  bne _label_dec8                ; $DEBE  D0 08
  cpy #$78                       ; $DEC0  C0 78
  bne _label_dec8                ; $DEC2  D0 04
  cmp #$44                       ; $DEC4  C9 44
  beq _label_decc                ; $DEC6  F0 04

_label_dec8:
  lda #$32                       ; $DEC8  A9 32
  sta z:_var_0000_indexed        ; $DECA  85 00

_label_decc:
  lda #$97                       ; $DECC  A9 97
  sta z:_var_007f                ; $DECE  85 7F
  lda #$47                       ; $DED0  A9 47
  ldy #$FF                       ; $DED2  A0 FF
  ldx #$00                       ; $DED4  A2 00
  clc                            ; $DED6  18
  clv                            ; $DED7  B8
  ldx z:_var_0080_indexed,Y      ; $DED8  B6 80
  bcs _label_deee                ; $DEDA  B0 12
  bvs _label_deee                ; $DEDC  70 10
  bpl _label_deee                ; $DEDE  10 0E
  beq _label_deee                ; $DEE0  F0 0C
  cpx #$97                       ; $DEE2  E0 97
  bne _label_deee                ; $DEE4  D0 08
  cpy #$FF                       ; $DEE6  C0 FF
  bne _label_deee                ; $DEE8  D0 04
  cmp #$47                       ; $DEEA  C9 47
  beq _label_def2                ; $DEEC  F0 04

_label_deee:
  lda #$33                       ; $DEEE  A9 33
  sta z:_var_0000_indexed        ; $DEF0  85 00

_label_def2:
  lda #$00                       ; $DEF2  A9 00
  sta z:_var_007f                ; $DEF4  85 7F
  lda #$47                       ; $DEF6  A9 47
  ldy #$FF                       ; $DEF8  A0 FF
  ldx #$69                       ; $DEFA  A2 69
  clc                            ; $DEFC  18
  clv                            ; $DEFD  B8
  stx z:_var_0080_indexed,Y      ; $DEFE  96 80
  bcs _label_df1a                ; $DF00  B0 18
  bvs _label_df1a                ; $DF02  70 16
  bmi _label_df1a                ; $DF04  30 14
  beq _label_df1a                ; $DF06  F0 12
  cpx #$69                       ; $DF08  E0 69
  bne _label_df1a                ; $DF0A  D0 0E
  cpy #$FF                       ; $DF0C  C0 FF
  bne _label_df1a                ; $DF0E  D0 0A
  cmp #$47                       ; $DF10  C9 47
  bne _label_df1a                ; $DF12  D0 06
  lda z:_var_007f                ; $DF14  A5 7F
  cmp #$69                       ; $DF16  C9 69
  beq _label_df1e                ; $DF18  F0 04

_label_df1a:
  lda #$34                       ; $DF1A  A9 34
  sta z:_var_0000_indexed        ; $DF1C  85 00

_label_df1e:
  lda #$F5                       ; $DF1E  A9 F5
  sta z:_var_004f                ; $DF20  85 4F
  lda #$47                       ; $DF22  A9 47
  ldy #$4F                       ; $DF24  A0 4F
  bit z:_var_0001                ; $DF26  24 01
  ldx #$00                       ; $DF28  A2 00
  sec                            ; $DF2A  38
  stx z:_var_0000_indexed,Y      ; $DF2B  96 00
  bcc _label_df45                ; $DF2D  90 16
  bvc _label_df45                ; $DF2F  50 14
  bmi _label_df45                ; $DF31  30 12
  bne _label_df45                ; $DF33  D0 10
  cpx #$00                       ; $DF35  E0 00
  bne _label_df45                ; $DF37  D0 0C
  cpy #$4F                       ; $DF39  C0 4F
  bne _label_df45                ; $DF3B  D0 08
  cmp #$47                       ; $DF3D  C9 47
  bne _label_df45                ; $DF3F  D0 04
  lda z:_var_004f                ; $DF41  A5 4F
  beq _label_df49                ; $DF43  F0 04

_label_df45:
  lda #$35                       ; $DF45  A9 35
  sta z:_var_0000_indexed        ; $DF47  85 00

_label_df49:
  rts                            ; $DF49  60

_label_df4a:
  lda #$89                       ; $DF4A  A9 89
  sta a:_var_0300_indexed        ; $DF4C  8D 00 03
  lda #$A3                       ; $DF4F  A9 A3
  sta z:_var_0033_indexed        ; $DF51  85 33
  lda #$12                       ; $DF53  A9 12
  sta a:_var_0245                ; $DF55  8D 45 02
  ldx #$65                       ; $DF58  A2 65
  ldy #$00                       ; $DF5A  A0 00
  sec                            ; $DF5C  38
  lda #$00                       ; $DF5D  A9 00
  clv                            ; $DF5F  B8
  lda a:_var_0300_indexed,Y      ; $DF60  B9 00 03
  beq _label_df71                ; $DF63  F0 0C
  bcc _label_df71                ; $DF65  90 0A
  bvs _label_df71                ; $DF67  70 08
  cmp #$89                       ; $DF69  C9 89
  bne _label_df71                ; $DF6B  D0 04
  cpx #$65                       ; $DF6D  E0 65
  beq _label_df75                ; $DF6F  F0 04

_label_df71:
  lda #$36                       ; $DF71  A9 36
  sta z:_var_0000_indexed        ; $DF73  85 00

_label_df75:
  lda #$FF                       ; $DF75  A9 FF
  sta z:_var_0001                ; $DF77  85 01
  bit z:_var_0001                ; $DF79  24 01
  ldy #$34                       ; $DF7B  A0 34
  lda a:$FFFF,Y                  ; $DF7D  B9 FF FF
  cmp #$A3                       ; $DF80  C9 A3
  bne _label_df86                ; $DF82  D0 02
  bcs _label_df8a                ; $DF84  B0 04

_label_df86:
  lda #$37                       ; $DF86  A9 37
  sta z:_var_0000_indexed        ; $DF88  85 00

_label_df8a:
  lda #$46                       ; $DF8A  A9 46
  sta z:_var_00ff_indexed        ; $DF8C  85 FF
  ldy #$FF                       ; $DF8E  A0 FF
  lda a:_var_0146_indexed,Y      ; $DF90  B9 46 01
  cmp #$12                       ; $DF93  C9 12
  beq _label_df9b                ; $DF95  F0 04
  lda #$38                       ; $DF97  A9 38
  sta z:_var_0000_indexed        ; $DF99  85 00

_label_df9b:
  ldx #$39                       ; $DF9B  A2 39
  clc                            ; $DF9D  18
  lda #$FF                       ; $DF9E  A9 FF
  sta z:_var_0001                ; $DFA0  85 01
  bit z:_var_0001                ; $DFA2  24 01
  lda #$AA                       ; $DFA4  A9 AA
  sta a:_var_0400_indexed        ; $DFA6  8D 00 04
  lda #$55                       ; $DFA9  A9 55
  ldy #$00                       ; $DFAB  A0 00
  ora a:_var_0400_indexed,Y      ; $DFAD  19 00 04
  bcs _label_dfba                ; $DFB0  B0 08
  bpl _label_dfba                ; $DFB2  10 06
  cmp #$FF                       ; $DFB4  C9 FF
  bne _label_dfba                ; $DFB6  D0 02
  bvs _label_dfbc                ; $DFB8  70 02

_label_dfba:
  stx z:_var_0000_indexed        ; $DFBA  86 00

_label_dfbc:
  inx                            ; $DFBC  E8
  sec                            ; $DFBD  38
  clv                            ; $DFBE  B8
  lda #$00                       ; $DFBF  A9 00
  ora a:_var_0400_indexed,Y      ; $DFC1  19 00 04
  beq _label_dfcc                ; $DFC4  F0 06
  bvs _label_dfcc                ; $DFC6  70 04
  bcc _label_dfcc                ; $DFC8  90 02
  bmi _label_dfce                ; $DFCA  30 02

_label_dfcc:
  stx z:_var_0000_indexed        ; $DFCC  86 00

_label_dfce:
  inx                            ; $DFCE  E8
  clc                            ; $DFCF  18
  bit z:_var_0001                ; $DFD0  24 01
  lda #$55                       ; $DFD2  A9 55
  and a:_var_0400_indexed,Y      ; $DFD4  39 00 04
  bne _label_dfdf                ; $DFD7  D0 06
  bvc _label_dfdf                ; $DFD9  50 04
  bcs _label_dfdf                ; $DFDB  B0 02
  bpl _label_dfe1                ; $DFDD  10 02

_label_dfdf:
  stx z:_var_0000_indexed        ; $DFDF  86 00

_label_dfe1:
  inx                            ; $DFE1  E8
  sec                            ; $DFE2  38
  clv                            ; $DFE3  B8
  lda #$EF                       ; $DFE4  A9 EF
  sta a:_var_0400_indexed        ; $DFE6  8D 00 04
  lda #$F8                       ; $DFE9  A9 F8
  and a:_var_0400_indexed,Y      ; $DFEB  39 00 04
  bcc _label_dff8                ; $DFEE  90 08
  bpl _label_dff8                ; $DFF0  10 06
  cmp #$E8                       ; $DFF2  C9 E8
  bne _label_dff8                ; $DFF4  D0 02
  bvc _label_dffa                ; $DFF6  50 02

_label_dff8:
  stx z:_var_0000_indexed        ; $DFF8  86 00

_label_dffa:
  inx                            ; $DFFA  E8
  clc                            ; $DFFB  18
  bit z:_var_0001                ; $DFFC  24 01
  lda #$AA                       ; $DFFE  A9 AA
  sta a:_var_0400_indexed        ; $E000  8D 00 04
  lda #$5F                       ; $E003  A9 5F
  eor a:_var_0400_indexed,Y      ; $E005  59 00 04
  bcs _label_e012                ; $E008  B0 08
  bpl _label_e012                ; $E00A  10 06
  cmp #$F5                       ; $E00C  C9 F5
  bne _label_e012                ; $E00E  D0 02
  bvs _label_e014                ; $E010  70 02

_label_e012:
  stx z:_var_0000_indexed        ; $E012  86 00

_label_e014:
  inx                            ; $E014  E8
  sec                            ; $E015  38
  clv                            ; $E016  B8
  lda #$70                       ; $E017  A9 70
  sta a:_var_0400_indexed        ; $E019  8D 00 04
  eor a:_var_0400_indexed,Y      ; $E01C  59 00 04
  bne _label_e027                ; $E01F  D0 06
  bvs _label_e027                ; $E021  70 04
  bcc _label_e027                ; $E023  90 02
  bpl _label_e029                ; $E025  10 02

_label_e027:
  stx z:_var_0000_indexed        ; $E027  86 00

_label_e029:
  inx                            ; $E029  E8
  clc                            ; $E02A  18
  bit z:_var_0001                ; $E02B  24 01
  lda #$69                       ; $E02D  A9 69
  sta a:_var_0400_indexed        ; $E02F  8D 00 04
  lda #$00                       ; $E032  A9 00
  adc a:_var_0400_indexed,Y      ; $E034  79 00 04
  bmi _label_e041                ; $E037  30 08
  bcs _label_e041                ; $E039  B0 06
  cmp #$69                       ; $E03B  C9 69
  bne _label_e041                ; $E03D  D0 02
  bvc _label_e043                ; $E03F  50 02

_label_e041:
  stx z:_var_0000_indexed        ; $E041  86 00

_label_e043:
  inx                            ; $E043  E8
  sec                            ; $E044  38
  bit z:_var_0001                ; $E045  24 01
  lda #$00                       ; $E047  A9 00
  adc a:_var_0400_indexed,Y      ; $E049  79 00 04
  bmi _label_e056                ; $E04C  30 08
  bcs _label_e056                ; $E04E  B0 06
  cmp #$6A                       ; $E050  C9 6A
  bne _label_e056                ; $E052  D0 02
  bvc _label_e058                ; $E054  50 02

_label_e056:
  stx z:_var_0000_indexed        ; $E056  86 00

_label_e058:
  inx                            ; $E058  E8
  sec                            ; $E059  38
  clv                            ; $E05A  B8
  lda #$7F                       ; $E05B  A9 7F
  sta a:_var_0400_indexed        ; $E05D  8D 00 04
  adc a:_var_0400_indexed,Y      ; $E060  79 00 04
  bpl _label_e06d                ; $E063  10 08
  bcs _label_e06d                ; $E065  B0 06
  cmp #$FF                       ; $E067  C9 FF
  bne _label_e06d                ; $E069  D0 02
  bvs _label_e06f                ; $E06B  70 02

_label_e06d:
  stx z:_var_0000_indexed        ; $E06D  86 00

_label_e06f:
  inx                            ; $E06F  E8
  clc                            ; $E070  18
  bit z:_var_0001                ; $E071  24 01
  lda #$80                       ; $E073  A9 80
  sta a:_var_0400_indexed        ; $E075  8D 00 04
  lda #$7F                       ; $E078  A9 7F
  adc a:_var_0400_indexed,Y      ; $E07A  79 00 04
  bpl _label_e087                ; $E07D  10 08
  bcs _label_e087                ; $E07F  B0 06
  cmp #$FF                       ; $E081  C9 FF
  bne _label_e087                ; $E083  D0 02
  bvc _label_e089                ; $E085  50 02

_label_e087:
  stx z:_var_0000_indexed        ; $E087  86 00

_label_e089:
  inx                            ; $E089  E8
  sec                            ; $E08A  38
  clv                            ; $E08B  B8
  lda #$80                       ; $E08C  A9 80
  sta a:_var_0400_indexed        ; $E08E  8D 00 04
  lda #$7F                       ; $E091  A9 7F
  adc a:_var_0400_indexed,Y      ; $E093  79 00 04
  bne _label_e09e                ; $E096  D0 06
  bmi _label_e09e                ; $E098  30 04
  bvs _label_e09e                ; $E09A  70 02
  bcs _label_e0a0                ; $E09C  B0 02

_label_e09e:
  stx z:_var_0000_indexed        ; $E09E  86 00

_label_e0a0:
  inx                            ; $E0A0  E8
  bit z:_var_0001                ; $E0A1  24 01
  lda #$40                       ; $E0A3  A9 40
  sta a:_var_0400_indexed        ; $E0A5  8D 00 04
  cmp a:_var_0400_indexed,Y      ; $E0A8  D9 00 04
  bmi _label_e0b3                ; $E0AB  30 06
  bcc _label_e0b3                ; $E0AD  90 04
  bne _label_e0b3                ; $E0AF  D0 02
  bvs _label_e0b5                ; $E0B1  70 02

_label_e0b3:
  stx z:_var_0000_indexed        ; $E0B3  86 00

_label_e0b5:
  inx                            ; $E0B5  E8
  clv                            ; $E0B6  B8
  dec a:_var_0400_indexed        ; $E0B7  CE 00 04
  cmp a:_var_0400_indexed,Y      ; $E0BA  D9 00 04
  beq _label_e0c5                ; $E0BD  F0 06
  bmi _label_e0c5                ; $E0BF  30 04
  bcc _label_e0c5                ; $E0C1  90 02
  bvc _label_e0c7                ; $E0C3  50 02

_label_e0c5:
  stx z:_var_0000_indexed        ; $E0C5  86 00

_label_e0c7:
  inx                            ; $E0C7  E8
  inc a:_var_0400_indexed        ; $E0C8  EE 00 04
  inc a:_var_0400_indexed        ; $E0CB  EE 00 04
  cmp a:_var_0400_indexed,Y      ; $E0CE  D9 00 04
  beq _label_e0d5                ; $E0D1  F0 02
  bmi _label_e0d7                ; $E0D3  30 02

_label_e0d5:
  stx z:_var_0000_indexed        ; $E0D5  86 00

_label_e0d7:
  inx                            ; $E0D7  E8
  lda #$00                       ; $E0D8  A9 00
  sta a:_var_0400_indexed        ; $E0DA  8D 00 04
  lda #$80                       ; $E0DD  A9 80
  cmp a:_var_0400_indexed,Y      ; $E0DF  D9 00 04
  beq _label_e0e8                ; $E0E2  F0 04
  bpl _label_e0e8                ; $E0E4  10 02
  bcs _label_e0ea                ; $E0E6  B0 02

_label_e0e8:
  stx z:_var_0000_indexed        ; $E0E8  86 00

_label_e0ea:
  inx                            ; $E0EA  E8
  ldy #$80                       ; $E0EB  A0 80
  sty a:_var_0400_indexed        ; $E0ED  8C 00 04
  ldy #$00                       ; $E0F0  A0 00
  cmp a:_var_0400_indexed,Y      ; $E0F2  D9 00 04
  bne _label_e0fb                ; $E0F5  D0 04
  bmi _label_e0fb                ; $E0F7  30 02
  bcs _label_e0fd                ; $E0F9  B0 02

_label_e0fb:
  stx z:_var_0000_indexed        ; $E0FB  86 00

_label_e0fd:
  inx                            ; $E0FD  E8
  inc a:_var_0400_indexed        ; $E0FE  EE 00 04
  cmp a:_var_0400_indexed,Y      ; $E101  D9 00 04
  bcs _label_e10a                ; $E104  B0 04
  beq _label_e10a                ; $E106  F0 02
  bmi _label_e10c                ; $E108  30 02

_label_e10a:
  stx z:_var_0000_indexed        ; $E10A  86 00

_label_e10c:
  inx                            ; $E10C  E8
  dec a:_var_0400_indexed        ; $E10D  CE 00 04
  dec a:_var_0400_indexed        ; $E110  CE 00 04
  cmp a:_var_0400_indexed,Y      ; $E113  D9 00 04
  bcc _label_e11c                ; $E116  90 04
  beq _label_e11c                ; $E118  F0 02
  bpl _label_e11e                ; $E11A  10 02

_label_e11c:
  stx z:_var_0000_indexed        ; $E11C  86 00

_label_e11e:
  inx                            ; $E11E  E8
  bit z:_var_0001                ; $E11F  24 01
  lda #$40                       ; $E121  A9 40
  sta a:_var_0400_indexed        ; $E123  8D 00 04
  sec                            ; $E126  38
  sbc a:_var_0400_indexed,Y      ; $E127  F9 00 04
  bmi _label_e136                ; $E12A  30 0A
  bcc _label_e136                ; $E12C  90 08
  bne _label_e136                ; $E12E  D0 06
  bvs _label_e136                ; $E130  70 04
  cmp #$00                       ; $E132  C9 00
  beq _label_e138                ; $E134  F0 02

_label_e136:
  stx z:_var_0000_indexed        ; $E136  86 00

_label_e138:
  inx                            ; $E138  E8
  clv                            ; $E139  B8
  sec                            ; $E13A  38
  lda #$40                       ; $E13B  A9 40
  dec a:_var_0400_indexed        ; $E13D  CE 00 04
  sbc a:_var_0400_indexed,Y      ; $E140  F9 00 04
  beq _label_e14f                ; $E143  F0 0A
  bmi _label_e14f                ; $E145  30 08
  bcc _label_e14f                ; $E147  90 06
  bvs _label_e14f                ; $E149  70 04
  cmp #$01                       ; $E14B  C9 01
  beq _label_e151                ; $E14D  F0 02

_label_e14f:
  stx z:_var_0000_indexed        ; $E14F  86 00

_label_e151:
  inx                            ; $E151  E8
  lda #$40                       ; $E152  A9 40
  sec                            ; $E154  38
  bit z:_var_0001                ; $E155  24 01
  inc a:_var_0400_indexed        ; $E157  EE 00 04
  inc a:_var_0400_indexed        ; $E15A  EE 00 04
  sbc a:_var_0400_indexed,Y      ; $E15D  F9 00 04
  bcs _label_e16c                ; $E160  B0 0A
  beq _label_e16c                ; $E162  F0 08
  bpl _label_e16c                ; $E164  10 06
  bvs _label_e16c                ; $E166  70 04
  cmp #$FF                       ; $E168  C9 FF
  beq _label_e16e                ; $E16A  F0 02

_label_e16c:
  stx z:_var_0000_indexed        ; $E16C  86 00

_label_e16e:
  inx                            ; $E16E  E8
  clc                            ; $E16F  18
  lda #$00                       ; $E170  A9 00
  sta a:_var_0400_indexed        ; $E172  8D 00 04
  lda #$80                       ; $E175  A9 80
  sbc a:_var_0400_indexed,Y      ; $E177  F9 00 04
  bcc _label_e180                ; $E17A  90 04
  cmp #$7F                       ; $E17C  C9 7F
  beq _label_e182                ; $E17E  F0 02

_label_e180:
  stx z:_var_0000_indexed        ; $E180  86 00

_label_e182:
  inx                            ; $E182  E8
  sec                            ; $E183  38
  lda #$7F                       ; $E184  A9 7F
  sta a:_var_0400_indexed        ; $E186  8D 00 04
  lda #$81                       ; $E189  A9 81
  sbc a:_var_0400_indexed,Y      ; $E18B  F9 00 04
  bvc _label_e196                ; $E18E  50 06
  bcc _label_e196                ; $E190  90 04
  cmp #$02                       ; $E192  C9 02
  beq _label_e198                ; $E194  F0 02

_label_e196:
  stx z:_var_0000_indexed        ; $E196  86 00

_label_e198:
  inx                            ; $E198  E8
  lda #$00                       ; $E199  A9 00
  lda #$87                       ; $E19B  A9 87
  sta a:_var_0400_indexed,Y      ; $E19D  99 00 04
  lda a:_var_0400_indexed        ; $E1A0  AD 00 04
  cmp #$87                       ; $E1A3  C9 87
  beq _label_e1a9                ; $E1A5  F0 02
  stx z:_var_0000_indexed        ; $E1A7  86 00

_label_e1a9:
  rts                            ; $E1A9  60

_label_e1aa:
  lda #$FF                       ; $E1AA  A9 FF
  sta z:_var_0001                ; $E1AC  85 01
  lda #$AA                       ; $E1AE  A9 AA
  sta a:_var_0633_indexed        ; $E1B0  8D 33 06
  lda #$BB                       ; $E1B3  A9 BB
  sta a:_var_0689                ; $E1B5  8D 89 06
  ldx #$00                       ; $E1B8  A2 00
  lda #$66                       ; $E1BA  A9 66
  bit z:_var_0001                ; $E1BC  24 01
  sec                            ; $E1BE  38
  ldy #$00                       ; $E1BF  A0 00
  ldy a:_var_0633_indexed,X      ; $E1C1  BC 33 06
  bpl _label_e1d8                ; $E1C4  10 12
  beq _label_e1d8                ; $E1C6  F0 10
  bvc _label_e1d8                ; $E1C8  50 0E
  bcc _label_e1d8                ; $E1CA  90 0C
  cmp #$66                       ; $E1CC  C9 66
  bne _label_e1d8                ; $E1CE  D0 08
  cpx #$00                       ; $E1D0  E0 00
  bne _label_e1d8                ; $E1D2  D0 04
  cpy #$AA                       ; $E1D4  C0 AA
  beq _label_e1dc                ; $E1D6  F0 04

_label_e1d8:
  lda #$51                       ; $E1D8  A9 51
  sta z:_var_0000_indexed        ; $E1DA  85 00

_label_e1dc:
  ldx #$8A                       ; $E1DC  A2 8A
  lda #$66                       ; $E1DE  A9 66
  clv                            ; $E1E0  B8
  clc                            ; $E1E1  18
  ldy #$00                       ; $E1E2  A0 00
  ldy a:_var_05ff_indexed,X      ; $E1E4  BC FF 05
  bpl _label_e1fb                ; $E1E7  10 12
  beq _label_e1fb                ; $E1E9  F0 10
  bvs _label_e1fb                ; $E1EB  70 0E
  bcs _label_e1fb                ; $E1ED  B0 0C
  cpy #$BB                       ; $E1EF  C0 BB
  bne _label_e1fb                ; $E1F1  D0 08
  cmp #$66                       ; $E1F3  C9 66
  bne _label_e1fb                ; $E1F5  D0 04
  cpx #$8A                       ; $E1F7  E0 8A
  beq _label_e1ff                ; $E1F9  F0 04

_label_e1fb:
  lda #$52                       ; $E1FB  A9 52
  sta z:_var_0000_indexed        ; $E1FD  85 00

_label_e1ff:
  ldy #$53                       ; $E1FF  A0 53
  lda #$AA                       ; $E201  A9 AA
  ldx #$78                       ; $E203  A2 78
  sta a:_var_0678                ; $E205  8D 78 06
  jsr _func_f7b6                 ; $E208  20 B6 F7
  ora a:_var_0600_indexed,X      ; $E20B  1D 00 06
  jsr _func_f7c0                 ; $E20E  20 C0 F7
  iny                            ; $E211  C8
  lda #$00                       ; $E212  A9 00
  sta a:_var_0678                ; $E214  8D 78 06
  jsr _func_f7ce                 ; $E217  20 CE F7
  ora a:_var_0600_indexed,X      ; $E21A  1D 00 06
  jsr _func_f7d3                 ; $E21D  20 D3 F7
  iny                            ; $E220  C8
  lda #$AA                       ; $E221  A9 AA
  sta a:_var_0678                ; $E223  8D 78 06
  jsr _func_f7df                 ; $E226  20 DF F7
  and a:_var_0600_indexed,X      ; $E229  3D 00 06
  jsr _func_f7e5                 ; $E22C  20 E5 F7
  iny                            ; $E22F  C8
  lda #$EF                       ; $E230  A9 EF
  sta a:_var_0678                ; $E232  8D 78 06
  jsr _func_f7f1                 ; $E235  20 F1 F7
  and a:_var_0600_indexed,X      ; $E238  3D 00 06
  jsr _func_f7f6                 ; $E23B  20 F6 F7
  iny                            ; $E23E  C8
  lda #$AA                       ; $E23F  A9 AA
  sta a:_var_0678                ; $E241  8D 78 06
  jsr _func_f804                 ; $E244  20 04 F8
  eor a:_var_0600_indexed,X      ; $E247  5D 00 06
  jsr _func_f80a                 ; $E24A  20 0A F8
  iny                            ; $E24D  C8
  lda #$70                       ; $E24E  A9 70
  sta a:_var_0678                ; $E250  8D 78 06
  jsr _func_f818                 ; $E253  20 18 F8
  eor a:_var_0600_indexed,X      ; $E256  5D 00 06
  jsr _func_f81d                 ; $E259  20 1D F8
  iny                            ; $E25C  C8
  lda #$69                       ; $E25D  A9 69
  sta a:_var_0678                ; $E25F  8D 78 06
  jsr _func_f829                 ; $E262  20 29 F8
  adc a:_var_0600_indexed,X      ; $E265  7D 00 06
  jsr _func_f82f                 ; $E268  20 2F F8
  iny                            ; $E26B  C8
  jsr _func_f83d                 ; $E26C  20 3D F8
  adc a:_var_0600_indexed,X      ; $E26F  7D 00 06
  jsr _func_f843                 ; $E272  20 43 F8
  iny                            ; $E275  C8
  lda #$7F                       ; $E276  A9 7F
  sta a:_var_0678                ; $E278  8D 78 06
  jsr _func_f851                 ; $E27B  20 51 F8
  adc a:_var_0600_indexed,X      ; $E27E  7D 00 06
  jsr _func_f856                 ; $E281  20 56 F8
  iny                            ; $E284  C8
  lda #$80                       ; $E285  A9 80
  sta a:_var_0678                ; $E287  8D 78 06
  jsr _func_f864                 ; $E28A  20 64 F8
  adc a:_var_0600_indexed,X      ; $E28D  7D 00 06
  jsr _func_f86a                 ; $E290  20 6A F8
  iny                            ; $E293  C8
  jsr _func_f878                 ; $E294  20 78 F8
  adc a:_var_0600_indexed,X      ; $E297  7D 00 06
  jsr _func_f87d                 ; $E29A  20 7D F8
  iny                            ; $E29D  C8
  lda #$40                       ; $E29E  A9 40
  sta a:_var_0678                ; $E2A0  8D 78 06
  jsr _func_f889                 ; $E2A3  20 89 F8
  cmp a:_var_0600_indexed,X      ; $E2A6  DD 00 06
  jsr _func_f88e                 ; $E2A9  20 8E F8
  iny                            ; $E2AC  C8
  pha                            ; $E2AD  48
  lda #$3F                       ; $E2AE  A9 3F
  sta a:_var_0678                ; $E2B0  8D 78 06
  pla                            ; $E2B3  68
  jsr _func_f89a                 ; $E2B4  20 9A F8
  cmp a:_var_0600_indexed,X      ; $E2B7  DD 00 06
  jsr _func_f89c                 ; $E2BA  20 9C F8
  iny                            ; $E2BD  C8
  pha                            ; $E2BE  48
  lda #$41                       ; $E2BF  A9 41
  sta a:_var_0678                ; $E2C1  8D 78 06
  pla                            ; $E2C4  68
  cmp a:_var_0600_indexed,X      ; $E2C5  DD 00 06
  jsr _func_f8a8                 ; $E2C8  20 A8 F8
  iny                            ; $E2CB  C8
  pha                            ; $E2CC  48
  lda #$00                       ; $E2CD  A9 00
  sta a:_var_0678                ; $E2CF  8D 78 06
  pla                            ; $E2D2  68
  jsr _func_f8b2                 ; $E2D3  20 B2 F8
  cmp a:_var_0600_indexed,X      ; $E2D6  DD 00 06
  jsr _func_f8b5                 ; $E2D9  20 B5 F8
  iny                            ; $E2DC  C8
  pha                            ; $E2DD  48
  lda #$80                       ; $E2DE  A9 80
  sta a:_var_0678                ; $E2E0  8D 78 06
  pla                            ; $E2E3  68
  cmp a:_var_0600_indexed,X      ; $E2E4  DD 00 06
  jsr _func_f8bf                 ; $E2E7  20 BF F8
  iny                            ; $E2EA  C8
  pha                            ; $E2EB  48
  lda #$81                       ; $E2EC  A9 81
  sta a:_var_0678                ; $E2EE  8D 78 06
  pla                            ; $E2F1  68
  cmp a:_var_0600_indexed,X      ; $E2F2  DD 00 06
  jsr _func_f8c9                 ; $E2F5  20 C9 F8
  iny                            ; $E2F8  C8
  pha                            ; $E2F9  48
  lda #$7F                       ; $E2FA  A9 7F
  sta a:_var_0678                ; $E2FC  8D 78 06
  pla                            ; $E2FF  68
  cmp a:_var_0600_indexed,X      ; $E300  DD 00 06
  jsr _func_f8d3                 ; $E303  20 D3 F8
  iny                            ; $E306  C8
  lda #$40                       ; $E307  A9 40
  sta a:_var_0678                ; $E309  8D 78 06
  jsr _func_f931                 ; $E30C  20 31 F9
  sbc a:_var_0600_indexed,X      ; $E30F  FD 00 06
  jsr _func_f937                 ; $E312  20 37 F9
  iny                            ; $E315  C8
  lda #$3F                       ; $E316  A9 3F
  sta a:_var_0678                ; $E318  8D 78 06
  jsr _func_f947                 ; $E31B  20 47 F9
  sbc a:_var_0600_indexed,X      ; $E31E  FD 00 06
  jsr _func_f94c                 ; $E321  20 4C F9
  iny                            ; $E324  C8
  lda #$41                       ; $E325  A9 41
  sta a:_var_0678                ; $E327  8D 78 06
  jsr _func_f95c                 ; $E32A  20 5C F9
  sbc a:_var_0600_indexed,X      ; $E32D  FD 00 06
  jsr _func_f962                 ; $E330  20 62 F9
  iny                            ; $E333  C8
  lda #$00                       ; $E334  A9 00
  sta a:_var_0678                ; $E336  8D 78 06
  jsr _func_f972                 ; $E339  20 72 F9
  sbc a:_var_0600_indexed,X      ; $E33C  FD 00 06
  jsr _func_f976                 ; $E33F  20 76 F9
  iny                            ; $E342  C8
  lda #$7F                       ; $E343  A9 7F
  sta a:_var_0678                ; $E345  8D 78 06
  jsr _func_f980                 ; $E348  20 80 F9
  sbc a:_var_0600_indexed,X      ; $E34B  FD 00 06
  jsr _func_f984                 ; $E34E  20 84 F9
  lda #$AA                       ; $E351  A9 AA
  sta a:_var_0633_indexed        ; $E353  8D 33 06
  lda #$BB                       ; $E356  A9 BB
  sta a:_var_0689                ; $E358  8D 89 06
  ldx #$00                       ; $E35B  A2 00
  ldy #$66                       ; $E35D  A0 66
  bit z:_var_0001                ; $E35F  24 01
  sec                            ; $E361  38
  lda #$00                       ; $E362  A9 00
  lda a:_var_0633_indexed,X      ; $E364  BD 33 06
  bpl _label_e37b                ; $E367  10 12
  beq _label_e37b                ; $E369  F0 10
  bvc _label_e37b                ; $E36B  50 0E
  bcc _label_e37b                ; $E36D  90 0C
  cpy #$66                       ; $E36F  C0 66
  bne _label_e37b                ; $E371  D0 08
  cpx #$00                       ; $E373  E0 00
  bne _label_e37b                ; $E375  D0 04
  cmp #$AA                       ; $E377  C9 AA
  beq _label_e37f                ; $E379  F0 04

_label_e37b:
  lda #$6A                       ; $E37B  A9 6A
  sta z:_var_0000_indexed        ; $E37D  85 00

_label_e37f:
  ldx #$8A                       ; $E37F  A2 8A
  ldy #$66                       ; $E381  A0 66
  clv                            ; $E383  B8
  clc                            ; $E384  18
  lda #$00                       ; $E385  A9 00
  lda a:_var_05ff_indexed,X      ; $E387  BD FF 05
  bpl _label_e39e                ; $E38A  10 12
  beq _label_e39e                ; $E38C  F0 10
  bvs _label_e39e                ; $E38E  70 0E
  bcs _label_e39e                ; $E390  B0 0C
  cmp #$BB                       ; $E392  C9 BB
  bne _label_e39e                ; $E394  D0 08
  cpy #$66                       ; $E396  C0 66
  bne _label_e39e                ; $E398  D0 04
  cpx #$8A                       ; $E39A  E0 8A
  beq _label_e3a2                ; $E39C  F0 04

_label_e39e:
  lda #$6B                       ; $E39E  A9 6B
  sta z:_var_0000_indexed        ; $E3A0  85 00

_label_e3a2:
  bit z:_var_0001                ; $E3A2  24 01
  sec                            ; $E3A4  38
  lda #$44                       ; $E3A5  A9 44
  ldx #$00                       ; $E3A7  A2 00
  sta a:_var_0633_indexed,X      ; $E3A9  9D 33 06
  lda a:_var_0633_indexed        ; $E3AC  AD 33 06
  bcc _label_e3cb                ; $E3AF  90 1A
  cmp #$44                       ; $E3B1  C9 44
  bne _label_e3cb                ; $E3B3  D0 16
  bvc _label_e3cb                ; $E3B5  50 14
  clc                            ; $E3B7  18
  clv                            ; $E3B8  B8
  lda #$99                       ; $E3B9  A9 99
  ldx #$80                       ; $E3BB  A2 80
  sta a:_var_0585_indexed,X      ; $E3BD  9D 85 05
  lda a:$0605                    ; $E3C0  AD 05 06
  bcs _label_e3cb                ; $E3C3  B0 06
  cmp #$99                       ; $E3C5  C9 99
  bne _label_e3cb                ; $E3C7  D0 02
  bvc _label_e3cf                ; $E3C9  50 04

_label_e3cb:
  lda #$6C                       ; $E3CB  A9 6C
  sta z:_var_0000_indexed        ; $E3CD  85 00

_label_e3cf:
  ldy #$6D                       ; $E3CF  A0 6D
  ldx #$6D                       ; $E3D1  A2 6D
  jsr _func_f990                 ; $E3D3  20 90 F9
  sta a:_var_0600_indexed,X      ; $E3D6  9D 00 06
  lsr a:_var_0600_indexed,X      ; $E3D9  5E 00 06
  lda a:_var_0600_indexed,X      ; $E3DC  BD 00 06
  jsr _func_f99d                 ; $E3DF  20 9D F9
  iny                            ; $E3E2  C8
  sta a:_var_0600_indexed,X      ; $E3E3  9D 00 06
  lsr a:_var_0600_indexed,X      ; $E3E6  5E 00 06
  lda a:_var_0600_indexed,X      ; $E3E9  BD 00 06
  jsr _func_f9ad                 ; $E3EC  20 AD F9
  iny                            ; $E3EF  C8
  jsr _func_f9bd                 ; $E3F0  20 BD F9
  sta a:_var_0600_indexed,X      ; $E3F3  9D 00 06
  asl a:_var_0600_indexed,X      ; $E3F6  1E 00 06
  lda a:_var_0600_indexed,X      ; $E3F9  BD 00 06
  jsr _func_f9c3                 ; $E3FC  20 C3 F9
  iny                            ; $E3FF  C8
  sta a:_var_0600_indexed,X      ; $E400  9D 00 06
  asl a:_var_0600_indexed,X      ; $E403  1E 00 06
  lda a:_var_0600_indexed,X      ; $E406  BD 00 06
  jsr _func_f9d4                 ; $E409  20 D4 F9
  iny                            ; $E40C  C8
  jsr _func_f9e4                 ; $E40D  20 E4 F9
  sta a:_var_0600_indexed,X      ; $E410  9D 00 06
  ror a:_var_0600_indexed,X      ; $E413  7E 00 06
  lda a:_var_0600_indexed,X      ; $E416  BD 00 06
  jsr _func_f9ea                 ; $E419  20 EA F9
  iny                            ; $E41C  C8
  sta a:_var_0600_indexed,X      ; $E41D  9D 00 06
  ror a:_var_0600_indexed,X      ; $E420  7E 00 06
  lda a:_var_0600_indexed,X      ; $E423  BD 00 06
  jsr _func_f9fb                 ; $E426  20 FB F9
  iny                            ; $E429  C8
  jsr _func_fa0a                 ; $E42A  20 0A FA
  sta a:_var_0600_indexed,X      ; $E42D  9D 00 06
  rol a:_var_0600_indexed,X      ; $E430  3E 00 06
  lda a:_var_0600_indexed,X      ; $E433  BD 00 06
  jsr _func_fa10                 ; $E436  20 10 FA
  iny                            ; $E439  C8
  sta a:_var_0600_indexed,X      ; $E43A  9D 00 06
  rol a:_var_0600_indexed,X      ; $E43D  3E 00 06
  lda a:_var_0600_indexed,X      ; $E440  BD 00 06
  jsr _func_fa21                 ; $E443  20 21 FA
  lda #$FF                       ; $E446  A9 FF
  sta a:_var_0600_indexed,X      ; $E448  9D 00 06
  sta z:_var_0001                ; $E44B  85 01
  bit z:_var_0001                ; $E44D  24 01
  sec                            ; $E44F  38
  inc a:_var_0600_indexed,X      ; $E450  FE 00 06
  bne _label_e462                ; $E453  D0 0D
  bmi _label_e462                ; $E455  30 0B
  bvc _label_e462                ; $E457  50 09
  bcc _label_e462                ; $E459  90 07
  lda a:_var_0600_indexed,X      ; $E45B  BD 00 06
  cmp #$00                       ; $E45E  C9 00
  beq _label_e466                ; $E460  F0 04

_label_e462:
  lda #$75                       ; $E462  A9 75
  sta z:_var_0000_indexed        ; $E464  85 00

_label_e466:
  lda #$7F                       ; $E466  A9 7F
  sta a:_var_0600_indexed,X      ; $E468  9D 00 06
  clv                            ; $E46B  B8
  clc                            ; $E46C  18
  inc a:_var_0600_indexed,X      ; $E46D  FE 00 06
  beq _label_e47f                ; $E470  F0 0D
  bpl _label_e47f                ; $E472  10 0B
  bvs _label_e47f                ; $E474  70 09
  bcs _label_e47f                ; $E476  B0 07
  lda a:_var_0600_indexed,X      ; $E478  BD 00 06
  cmp #$80                       ; $E47B  C9 80
  beq _label_e483                ; $E47D  F0 04

_label_e47f:
  lda #$76                       ; $E47F  A9 76
  sta z:_var_0000_indexed        ; $E481  85 00

_label_e483:
  lda #$00                       ; $E483  A9 00
  sta a:_var_0600_indexed,X      ; $E485  9D 00 06
  bit z:_var_0001                ; $E488  24 01
  sec                            ; $E48A  38
  dec a:_var_0600_indexed,X      ; $E48B  DE 00 06
  beq _label_e49d                ; $E48E  F0 0D
  bpl _label_e49d                ; $E490  10 0B
  bvc _label_e49d                ; $E492  50 09
  bcc _label_e49d                ; $E494  90 07
  lda a:_var_0600_indexed,X      ; $E496  BD 00 06
  cmp #$FF                       ; $E499  C9 FF
  beq _label_e4a1                ; $E49B  F0 04

_label_e49d:
  lda #$77                       ; $E49D  A9 77
  sta z:_var_0000_indexed        ; $E49F  85 00

_label_e4a1:
  lda #$80                       ; $E4A1  A9 80
  sta a:_var_0600_indexed,X      ; $E4A3  9D 00 06
  clv                            ; $E4A6  B8
  clc                            ; $E4A7  18
  dec a:_var_0600_indexed,X      ; $E4A8  DE 00 06
  beq _label_e4ba                ; $E4AB  F0 0D
  bmi _label_e4ba                ; $E4AD  30 0B
  bvs _label_e4ba                ; $E4AF  70 09
  bcs _label_e4ba                ; $E4B1  B0 07
  lda a:_var_0600_indexed,X      ; $E4B3  BD 00 06
  cmp #$7F                       ; $E4B6  C9 7F
  beq _label_e4be                ; $E4B8  F0 04

_label_e4ba:
  lda #$78                       ; $E4BA  A9 78
  sta z:_var_0000_indexed        ; $E4BC  85 00

_label_e4be:
  lda #$01                       ; $E4BE  A9 01
  sta a:_var_0600_indexed,X      ; $E4C0  9D 00 06
  dec a:_var_0600_indexed,X      ; $E4C3  DE 00 06
  beq _label_e4cc                ; $E4C6  F0 04
  lda #$79                       ; $E4C8  A9 79
  sta z:_var_0000_indexed        ; $E4CA  85 00

_label_e4cc:
  lda #$33                       ; $E4CC  A9 33
  sta a:_var_0678                ; $E4CE  8D 78 06
  lda #$44                       ; $E4D1  A9 44
  ldy #$78                       ; $E4D3  A0 78
  ldx #$00                       ; $E4D5  A2 00
  sec                            ; $E4D7  38
  bit z:_var_0001                ; $E4D8  24 01
  ldx a:_var_0600_indexed,Y      ; $E4DA  BE 00 06
  bcc _label_e4f1                ; $E4DD  90 12
  bvc _label_e4f1                ; $E4DF  50 10
  bmi _label_e4f1                ; $E4E1  30 0E
  beq _label_e4f1                ; $E4E3  F0 0C
  cpx #$33                       ; $E4E5  E0 33
  bne _label_e4f1                ; $E4E7  D0 08
  cpy #$78                       ; $E4E9  C0 78
  bne _label_e4f1                ; $E4EB  D0 04
  cmp #$44                       ; $E4ED  C9 44
  beq _label_e4f5                ; $E4EF  F0 04

_label_e4f1:
  lda #$7A                       ; $E4F1  A9 7A
  sta z:_var_0000_indexed        ; $E4F3  85 00

_label_e4f5:
  lda #$97                       ; $E4F5  A9 97
  sta a:$067F                    ; $E4F7  8D 7F 06
  lda #$47                       ; $E4FA  A9 47
  ldy #$FF                       ; $E4FC  A0 FF
  ldx #$00                       ; $E4FE  A2 00
  clc                            ; $E500  18
  clv                            ; $E501  B8
  ldx a:_var_0580_indexed,Y      ; $E502  BE 80 05
  bcs _label_e519                ; $E505  B0 12
  bvs _label_e519                ; $E507  70 10
  bpl _label_e519                ; $E509  10 0E
  beq _label_e519                ; $E50B  F0 0C
  cpx #$97                       ; $E50D  E0 97
  bne _label_e519                ; $E50F  D0 08
  cpy #$FF                       ; $E511  C0 FF
  bne _label_e519                ; $E513  D0 04
  cmp #$47                       ; $E515  C9 47
  beq _label_e51d                ; $E517  F0 04

_label_e519:
  lda #$7B                       ; $E519  A9 7B
  sta z:_var_0000_indexed        ; $E51B  85 00

_label_e51d:
  rts                            ; $E51D  60

_label_e51e:
  lda #$55                       ; $E51E  A9 55
  sta a:_var_0580_indexed        ; $E520  8D 80 05
  lda #$AA                       ; $E523  A9 AA
  sta a:_var_0432                ; $E525  8D 32 04
  lda #$80                       ; $E528  A9 80
  sta z:_var_0043_indexed        ; $E52A  85 43
  lda #$05                       ; $E52C  A9 05
  sta z:_var_0044                ; $E52E  85 44
  lda #$32                       ; $E530  A9 32
  sta z:_var_0045_indexed        ; $E532  85 45
  lda #$04                       ; $E534  A9 04
  sta z:_var_0046                ; $E536  85 46
  ldx #$03                       ; $E538  A2 03
  ldy #$77                       ; $E53A  A0 77
  lda #$FF                       ; $E53C  A9 FF
  sta z:_var_0001                ; $E53E  85 01
  bit z:_var_0001                ; $E540  24 01
  sec                            ; $E542  38
  lda #$00                       ; $E543  A9 00
  lax (_var_0040_indexed,X)      ; $E545  A3 40
  nop                            ; $E547  EA
  nop                            ; $E548  EA
  nop                            ; $E549  EA
  nop                            ; $E54A  EA
  beq _label_e55f                ; $E54B  F0 12
  bmi _label_e55f                ; $E54D  30 10
  bvc _label_e55f                ; $E54F  50 0E
  bcc _label_e55f                ; $E551  90 0C
  cmp #$55                       ; $E553  C9 55
  bne _label_e55f                ; $E555  D0 08
  cpx #$55                       ; $E557  E0 55
  bne _label_e55f                ; $E559  D0 04
  cpy #$77                       ; $E55B  C0 77
  beq _label_e563                ; $E55D  F0 04

_label_e55f:
  lda #$7C                       ; $E55F  A9 7C
  sta z:_var_0000_indexed        ; $E561  85 00

_label_e563:
  ldx #$05                       ; $E563  A2 05
  ldy #$33                       ; $E565  A0 33
  clv                            ; $E567  B8
  clc                            ; $E568  18
  lda #$00                       ; $E569  A9 00
  lax (_var_0040_indexed,X)      ; $E56B  A3 40
  nop                            ; $E56D  EA
  nop                            ; $E56E  EA
  nop                            ; $E56F  EA
  nop                            ; $E570  EA
  beq _label_e585                ; $E571  F0 12
  bpl _label_e585                ; $E573  10 10
  bvs _label_e585                ; $E575  70 0E
  bcs _label_e585                ; $E577  B0 0C
  cmp #$AA                       ; $E579  C9 AA
  bne _label_e585                ; $E57B  D0 08
  cpx #$AA                       ; $E57D  E0 AA
  bne _label_e585                ; $E57F  D0 04
  cpy #$33                       ; $E581  C0 33
  beq _label_e589                ; $E583  F0 04

_label_e585:
  lda #$7D                       ; $E585  A9 7D
  sta z:_var_0000_indexed        ; $E587  85 00

_label_e589:
  lda #$87                       ; $E589  A9 87
  sta z:_var_0067                ; $E58B  85 67
  lda #$32                       ; $E58D  A9 32
  sta z:_var_0068                ; $E58F  85 68
  ldy #$57                       ; $E591  A0 57
  bit z:_var_0001                ; $E593  24 01
  sec                            ; $E595  38
  lda #$00                       ; $E596  A9 00
  lax z:_var_0067                ; $E598  A7 67
  nop                            ; $E59A  EA
  nop                            ; $E59B  EA
  nop                            ; $E59C  EA
  nop                            ; $E59D  EA
  beq _label_e5b2                ; $E59E  F0 12
  bpl _label_e5b2                ; $E5A0  10 10
  bvc _label_e5b2                ; $E5A2  50 0E
  bcc _label_e5b2                ; $E5A4  90 0C
  cmp #$87                       ; $E5A6  C9 87
  bne _label_e5b2                ; $E5A8  D0 08
  cpx #$87                       ; $E5AA  E0 87
  bne _label_e5b2                ; $E5AC  D0 04
  cpy #$57                       ; $E5AE  C0 57
  beq _label_e5b6                ; $E5B0  F0 04

_label_e5b2:
  lda #$7E                       ; $E5B2  A9 7E
  sta z:_var_0000_indexed        ; $E5B4  85 00

_label_e5b6:
  ldy #$53                       ; $E5B6  A0 53
  clv                            ; $E5B8  B8
  clc                            ; $E5B9  18
  lda #$00                       ; $E5BA  A9 00
  lax z:_var_0068                ; $E5BC  A7 68
  nop                            ; $E5BE  EA
  nop                            ; $E5BF  EA
  nop                            ; $E5C0  EA
  nop                            ; $E5C1  EA
  beq _label_e5d6                ; $E5C2  F0 12
  bmi _label_e5d6                ; $E5C4  30 10
  bvs _label_e5d6                ; $E5C6  70 0E
  bcs _label_e5d6                ; $E5C8  B0 0C
  cmp #$32                       ; $E5CA  C9 32
  bne _label_e5d6                ; $E5CC  D0 08
  cpx #$32                       ; $E5CE  E0 32
  bne _label_e5d6                ; $E5D0  D0 04
  cpy #$53                       ; $E5D2  C0 53
  beq _label_e5da                ; $E5D4  F0 04

_label_e5d6:
  lda #$7F                       ; $E5D6  A9 7F
  sta z:_var_0000_indexed        ; $E5D8  85 00

_label_e5da:
  lda #$87                       ; $E5DA  A9 87
  sta a:_var_0577                ; $E5DC  8D 77 05
  lda #$32                       ; $E5DF  A9 32
  sta a:_var_0578                ; $E5E1  8D 78 05
  ldy #$57                       ; $E5E4  A0 57
  bit z:_var_0001                ; $E5E6  24 01
  sec                            ; $E5E8  38
  lda #$00                       ; $E5E9  A9 00
  lax a:_var_0577                ; $E5EB  AF 77 05
  nop                            ; $E5EE  EA
  nop                            ; $E5EF  EA
  nop                            ; $E5F0  EA
  nop                            ; $E5F1  EA
  beq _label_e606                ; $E5F2  F0 12
  bpl _label_e606                ; $E5F4  10 10
  bvc _label_e606                ; $E5F6  50 0E
  bcc _label_e606                ; $E5F8  90 0C
  cmp #$87                       ; $E5FA  C9 87
  bne _label_e606                ; $E5FC  D0 08
  cpx #$87                       ; $E5FE  E0 87
  bne _label_e606                ; $E600  D0 04
  cpy #$57                       ; $E602  C0 57
  beq _label_e60a                ; $E604  F0 04

_label_e606:
  lda #$80                       ; $E606  A9 80
  sta z:_var_0000_indexed        ; $E608  85 00

_label_e60a:
  ldy #$53                       ; $E60A  A0 53
  clv                            ; $E60C  B8
  clc                            ; $E60D  18
  lda #$00                       ; $E60E  A9 00
  lax a:_var_0578                ; $E610  AF 78 05
  nop                            ; $E613  EA
  nop                            ; $E614  EA
  nop                            ; $E615  EA
  nop                            ; $E616  EA
  beq _label_e62b                ; $E617  F0 12
  bmi _label_e62b                ; $E619  30 10
  bvs _label_e62b                ; $E61B  70 0E
  bcs _label_e62b                ; $E61D  B0 0C
  cmp #$32                       ; $E61F  C9 32
  bne _label_e62b                ; $E621  D0 08
  cpx #$32                       ; $E623  E0 32
  bne _label_e62b                ; $E625  D0 04
  cpy #$53                       ; $E627  C0 53
  beq _label_e62f                ; $E629  F0 04

_label_e62b:
  lda #$81                       ; $E62B  A9 81
  sta z:_var_0000_indexed        ; $E62D  85 00

_label_e62f:
  lda #$FF                       ; $E62F  A9 FF
  sta z:_var_0043_indexed        ; $E631  85 43
  lda #$04                       ; $E633  A9 04
  sta z:_var_0044                ; $E635  85 44
  lda #$32                       ; $E637  A9 32
  sta z:_var_0045_indexed        ; $E639  85 45
  lda #$04                       ; $E63B  A9 04
  sta z:_var_0046                ; $E63D  85 46
  lda #$55                       ; $E63F  A9 55
  sta a:_var_0580_indexed        ; $E641  8D 80 05
  lda #$AA                       ; $E644  A9 AA
  sta a:_var_0432                ; $E646  8D 32 04
  ldx #$03                       ; $E649  A2 03
  ldy #$81                       ; $E64B  A0 81
  bit z:_var_0001                ; $E64D  24 01
  sec                            ; $E64F  38
  lda #$00                       ; $E650  A9 00
  lax (_var_0043_indexed),Y      ; $E652  B3 43
  nop                            ; $E654  EA
  nop                            ; $E655  EA
  nop                            ; $E656  EA
  nop                            ; $E657  EA
  beq _label_e66c                ; $E658  F0 12
  bmi _label_e66c                ; $E65A  30 10
  bvc _label_e66c                ; $E65C  50 0E
  bcc _label_e66c                ; $E65E  90 0C
  cmp #$55                       ; $E660  C9 55
  bne _label_e66c                ; $E662  D0 08
  cpx #$55                       ; $E664  E0 55
  bne _label_e66c                ; $E666  D0 04
  cpy #$81                       ; $E668  C0 81
  beq _label_e670                ; $E66A  F0 04

_label_e66c:
  lda #$82                       ; $E66C  A9 82
  sta z:_var_0000_indexed        ; $E66E  85 00

_label_e670:
  ldx #$05                       ; $E670  A2 05
  ldy #$00                       ; $E672  A0 00
  clv                            ; $E674  B8
  clc                            ; $E675  18
  lda #$00                       ; $E676  A9 00
  lax (_var_0045_indexed),Y      ; $E678  B3 45
  nop                            ; $E67A  EA
  nop                            ; $E67B  EA
  nop                            ; $E67C  EA
  nop                            ; $E67D  EA
  beq _label_e692                ; $E67E  F0 12
  bpl _label_e692                ; $E680  10 10
  bvs _label_e692                ; $E682  70 0E
  bcs _label_e692                ; $E684  B0 0C
  cmp #$AA                       ; $E686  C9 AA
  bne _label_e692                ; $E688  D0 08
  cpx #$AA                       ; $E68A  E0 AA
  bne _label_e692                ; $E68C  D0 04
  cpy #$00                       ; $E68E  C0 00
  beq _label_e696                ; $E690  F0 04

_label_e692:
  lda #$83                       ; $E692  A9 83
  sta z:_var_0000_indexed        ; $E694  85 00

_label_e696:
  lda #$87                       ; $E696  A9 87
  sta z:_var_0067                ; $E698  85 67
  lda #$32                       ; $E69A  A9 32
  sta z:_var_0068                ; $E69C  85 68
  ldy #$57                       ; $E69E  A0 57
  bit z:_var_0001                ; $E6A0  24 01
  sec                            ; $E6A2  38
  lda #$00                       ; $E6A3  A9 00
  lax z:_var_0010_indexed,Y      ; $E6A5  B7 10
  nop                            ; $E6A7  EA
  nop                            ; $E6A8  EA
  nop                            ; $E6A9  EA
  nop                            ; $E6AA  EA
  beq _label_e6bf                ; $E6AB  F0 12
  bpl _label_e6bf                ; $E6AD  10 10
  bvc _label_e6bf                ; $E6AF  50 0E
  bcc _label_e6bf                ; $E6B1  90 0C
  cmp #$87                       ; $E6B3  C9 87
  bne _label_e6bf                ; $E6B5  D0 08
  cpx #$87                       ; $E6B7  E0 87
  bne _label_e6bf                ; $E6B9  D0 04
  cpy #$57                       ; $E6BB  C0 57
  beq _label_e6c3                ; $E6BD  F0 04

_label_e6bf:
  lda #$84                       ; $E6BF  A9 84
  sta z:_var_0000_indexed        ; $E6C1  85 00

_label_e6c3:
  ldy #$FF                       ; $E6C3  A0 FF
  clv                            ; $E6C5  B8
  clc                            ; $E6C6  18
  lda #$00                       ; $E6C7  A9 00
  lax z:_var_0069_indexed,Y      ; $E6C9  B7 69
  nop                            ; $E6CB  EA
  nop                            ; $E6CC  EA
  nop                            ; $E6CD  EA
  nop                            ; $E6CE  EA
  beq _label_e6e3                ; $E6CF  F0 12
  bmi _label_e6e3                ; $E6D1  30 10
  bvs _label_e6e3                ; $E6D3  70 0E
  bcs _label_e6e3                ; $E6D5  B0 0C
  cmp #$32                       ; $E6D7  C9 32
  bne _label_e6e3                ; $E6D9  D0 08
  cpx #$32                       ; $E6DB  E0 32
  bne _label_e6e3                ; $E6DD  D0 04
  cpy #$FF                       ; $E6DF  C0 FF
  beq _label_e6e7                ; $E6E1  F0 04

_label_e6e3:
  lda #$85                       ; $E6E3  A9 85
  sta z:_var_0000_indexed        ; $E6E5  85 00

_label_e6e7:
  lda #$87                       ; $E6E7  A9 87
  sta a:$0587                    ; $E6E9  8D 87 05
  lda #$32                       ; $E6EC  A9 32
  sta a:$0588                    ; $E6EE  8D 88 05
  ldy #$30                       ; $E6F1  A0 30
  bit z:_var_0001                ; $E6F3  24 01
  sec                            ; $E6F5  38
  lda #$00                       ; $E6F6  A9 00
  lax a:_var_0557_indexed,Y      ; $E6F8  BF 57 05
  nop                            ; $E6FB  EA
  nop                            ; $E6FC  EA
  nop                            ; $E6FD  EA
  nop                            ; $E6FE  EA
  beq _label_e713                ; $E6FF  F0 12
  bpl _label_e713                ; $E701  10 10
  bvc _label_e713                ; $E703  50 0E
  bcc _label_e713                ; $E705  90 0C
  cmp #$87                       ; $E707  C9 87
  bne _label_e713                ; $E709  D0 08
  cpx #$87                       ; $E70B  E0 87
  bne _label_e713                ; $E70D  D0 04
  cpy #$30                       ; $E70F  C0 30
  beq _label_e717                ; $E711  F0 04

_label_e713:
  lda #$86                       ; $E713  A9 86
  sta z:_var_0000_indexed        ; $E715  85 00

_label_e717:
  ldy #$40                       ; $E717  A0 40
  clv                            ; $E719  B8
  clc                            ; $E71A  18
  lda #$00                       ; $E71B  A9 00
  lax a:_var_0548_indexed,Y      ; $E71D  BF 48 05
  nop                            ; $E720  EA
  nop                            ; $E721  EA
  nop                            ; $E722  EA
  nop                            ; $E723  EA
  beq _label_e738                ; $E724  F0 12
  bmi _label_e738                ; $E726  30 10
  bvs _label_e738                ; $E728  70 0E
  bcs _label_e738                ; $E72A  B0 0C
  cmp #$32                       ; $E72C  C9 32
  bne _label_e738                ; $E72E  D0 08
  cpx #$32                       ; $E730  E0 32
  bne _label_e738                ; $E732  D0 04
  cpy #$40                       ; $E734  C0 40
  beq _label_e73c                ; $E736  F0 04

_label_e738:
  lda #$87                       ; $E738  A9 87
  sta z:_var_0000_indexed        ; $E73A  85 00

_label_e73c:
  rts                            ; $E73C  60

_label_e73d:
  lda #$C0                       ; $E73D  A9 C0
  sta z:_var_0001                ; $E73F  85 01
  lda #$00                       ; $E741  A9 00
  sta a:_var_0489                ; $E743  8D 89 04
  lda #$89                       ; $E746  A9 89
  sta z:$60                      ; $E748  85 60
  lda #$04                       ; $E74A  A9 04
  sta z:$61                      ; $E74C  85 61
  ldy #$44                       ; $E74E  A0 44
  ldx #$17                       ; $E750  A2 17
  lda #$3E                       ; $E752  A9 3E
  bit z:_var_0001                ; $E754  24 01
  clc                            ; $E756  18
  sax (_var_0049_indexed,X)      ; $E757  83 49
  nop                            ; $E759  EA
  nop                            ; $E75A  EA
  nop                            ; $E75B  EA
  nop                            ; $E75C  EA
  bne _label_e778                ; $E75D  D0 19
  bcs _label_e778                ; $E75F  B0 17
  bvc _label_e778                ; $E761  50 15
  bpl _label_e778                ; $E763  10 13
  cmp #$3E                       ; $E765  C9 3E
  bne _label_e778                ; $E767  D0 0F
  cpy #$44                       ; $E769  C0 44
  bne _label_e778                ; $E76B  D0 0B
  cpx #$17                       ; $E76D  E0 17
  bne _label_e778                ; $E76F  D0 07
  lda a:_var_0489                ; $E771  AD 89 04
  cmp #$16                       ; $E774  C9 16
  beq _label_e77c                ; $E776  F0 04

_label_e778:
  lda #$88                       ; $E778  A9 88
  sta z:_var_0000_indexed        ; $E77A  85 00

_label_e77c:
  ldy #$44                       ; $E77C  A0 44
  ldx #$7A                       ; $E77E  A2 7A
  lda #$66                       ; $E780  A9 66
  sec                            ; $E782  38
  clv                            ; $E783  B8
  sax (_var_00e6_indexed,X)      ; $E784  83 E6
  nop                            ; $E786  EA
  nop                            ; $E787  EA
  nop                            ; $E788  EA
  nop                            ; $E789  EA
  beq _label_e7a5                ; $E78A  F0 19
  bcc _label_e7a5                ; $E78C  90 17
  bvs _label_e7a5                ; $E78E  70 15
  bmi _label_e7a5                ; $E790  30 13
  cmp #$66                       ; $E792  C9 66
  bne _label_e7a5                ; $E794  D0 0F
  cpy #$44                       ; $E796  C0 44
  bne _label_e7a5                ; $E798  D0 0B
  cpx #$7A                       ; $E79A  E0 7A
  bne _label_e7a5                ; $E79C  D0 07
  lda a:_var_0489                ; $E79E  AD 89 04
  cmp #$62                       ; $E7A1  C9 62
  beq _label_e7a9                ; $E7A3  F0 04

_label_e7a5:
  lda #$89                       ; $E7A5  A9 89
  sta z:_var_0000_indexed        ; $E7A7  85 00

_label_e7a9:
  lda #$FF                       ; $E7A9  A9 FF
  sta z:_var_0049_indexed        ; $E7AB  85 49
  ldy #$44                       ; $E7AD  A0 44
  ldx #$AA                       ; $E7AF  A2 AA
  lda #$55                       ; $E7B1  A9 55
  bit z:_var_0001                ; $E7B3  24 01
  clc                            ; $E7B5  18
  sax z:_var_0049_indexed        ; $E7B6  87 49
  nop                            ; $E7B8  EA
  nop                            ; $E7B9  EA
  nop                            ; $E7BA  EA
  nop                            ; $E7BB  EA
  beq _label_e7d6                ; $E7BC  F0 18
  bcs _label_e7d6                ; $E7BE  B0 16
  bvc _label_e7d6                ; $E7C0  50 14
  bpl _label_e7d6                ; $E7C2  10 12
  cmp #$55                       ; $E7C4  C9 55
  bne _label_e7d6                ; $E7C6  D0 0E
  cpy #$44                       ; $E7C8  C0 44
  bne _label_e7d6                ; $E7CA  D0 0A
  cpx #$AA                       ; $E7CC  E0 AA
  bne _label_e7d6                ; $E7CE  D0 06
  lda z:_var_0049_indexed        ; $E7D0  A5 49
  cmp #$00                       ; $E7D2  C9 00
  beq _label_e7da                ; $E7D4  F0 04

_label_e7d6:
  lda #$8A                       ; $E7D6  A9 8A
  sta z:_var_0000_indexed        ; $E7D8  85 00

_label_e7da:
  lda #$00                       ; $E7DA  A9 00
  sta z:_var_0056                ; $E7DC  85 56
  ldy #$58                       ; $E7DE  A0 58
  ldx #$EF                       ; $E7E0  A2 EF
  lda #$66                       ; $E7E2  A9 66
  sec                            ; $E7E4  38
  clv                            ; $E7E5  B8
  sax z:_var_0056                ; $E7E6  87 56
  nop                            ; $E7E8  EA
  nop                            ; $E7E9  EA
  nop                            ; $E7EA  EA
  nop                            ; $E7EB  EA
  beq _label_e806                ; $E7EC  F0 18
  bcc _label_e806                ; $E7EE  90 16
  bvs _label_e806                ; $E7F0  70 14
  bmi _label_e806                ; $E7F2  30 12
  cmp #$66                       ; $E7F4  C9 66
  bne _label_e806                ; $E7F6  D0 0E
  cpy #$58                       ; $E7F8  C0 58
  bne _label_e806                ; $E7FA  D0 0A
  cpx #$EF                       ; $E7FC  E0 EF
  bne _label_e806                ; $E7FE  D0 06
  lda z:_var_0056                ; $E800  A5 56
  cmp #$66                       ; $E802  C9 66
  beq _label_e80a                ; $E804  F0 04

_label_e806:
  lda #$8B                       ; $E806  A9 8B
  sta z:_var_0000_indexed        ; $E808  85 00

_label_e80a:
  lda #$FF                       ; $E80A  A9 FF
  sta a:_var_0549                ; $E80C  8D 49 05
  ldy #$E5                       ; $E80F  A0 E5
  ldx #$AF                       ; $E811  A2 AF
  lda #$F5                       ; $E813  A9 F5
  bit z:_var_0001                ; $E815  24 01
  clc                            ; $E817  18
  sax a:_var_0549                ; $E818  8F 49 05
  nop                            ; $E81B  EA
  nop                            ; $E81C  EA
  nop                            ; $E81D  EA
  nop                            ; $E81E  EA
  beq _label_e83a                ; $E81F  F0 19
  bcs _label_e83a                ; $E821  B0 17
  bvc _label_e83a                ; $E823  50 15
  bpl _label_e83a                ; $E825  10 13
  cmp #$F5                       ; $E827  C9 F5
  bne _label_e83a                ; $E829  D0 0F
  cpy #$E5                       ; $E82B  C0 E5
  bne _label_e83a                ; $E82D  D0 0B
  cpx #$AF                       ; $E82F  E0 AF
  bne _label_e83a                ; $E831  D0 07
  lda a:_var_0549                ; $E833  AD 49 05
  cmp #$A5                       ; $E836  C9 A5
  beq _label_e83e                ; $E838  F0 04

_label_e83a:
  lda #$8C                       ; $E83A  A9 8C
  sta z:_var_0000_indexed        ; $E83C  85 00

_label_e83e:
  lda #$00                       ; $E83E  A9 00
  sta a:_var_0556                ; $E840  8D 56 05
  ldy #$58                       ; $E843  A0 58
  ldx #$B3                       ; $E845  A2 B3
  lda #$97                       ; $E847  A9 97
  sec                            ; $E849  38
  clv                            ; $E84A  B8
  sax a:_var_0556                ; $E84B  8F 56 05
  nop                            ; $E84E  EA
  nop                            ; $E84F  EA
  nop                            ; $E850  EA
  nop                            ; $E851  EA
  beq _label_e86d                ; $E852  F0 19
  bcc _label_e86d                ; $E854  90 17
  bvs _label_e86d                ; $E856  70 15
  bpl _label_e86d                ; $E858  10 13
  cmp #$97                       ; $E85A  C9 97
  bne _label_e86d                ; $E85C  D0 0F
  cpy #$58                       ; $E85E  C0 58
  bne _label_e86d                ; $E860  D0 0B
  cpx #$B3                       ; $E862  E0 B3
  bne _label_e86d                ; $E864  D0 07
  lda a:_var_0556                ; $E866  AD 56 05
  cmp #$93                       ; $E869  C9 93
  beq _label_e871                ; $E86B  F0 04

_label_e86d:
  lda #$8D                       ; $E86D  A9 8D
  sta z:_var_0000_indexed        ; $E86F  85 00

_label_e871:
  lda #$FF                       ; $E871  A9 FF
  sta z:_var_0049_indexed        ; $E873  85 49
  ldy #$FF                       ; $E875  A0 FF
  ldx #$AA                       ; $E877  A2 AA
  lda #$55                       ; $E879  A9 55
  bit z:_var_0001                ; $E87B  24 01
  clc                            ; $E87D  18
  sax z:_var_004a_indexed,Y      ; $E87E  97 4A
  nop                            ; $E880  EA
  nop                            ; $E881  EA
  nop                            ; $E882  EA
  nop                            ; $E883  EA
  beq _label_e89e                ; $E884  F0 18
  bcs _label_e89e                ; $E886  B0 16
  bvc _label_e89e                ; $E888  50 14
  bpl _label_e89e                ; $E88A  10 12
  cmp #$55                       ; $E88C  C9 55
  bne _label_e89e                ; $E88E  D0 0E
  cpy #$FF                       ; $E890  C0 FF
  bne _label_e89e                ; $E892  D0 0A
  cpx #$AA                       ; $E894  E0 AA
  bne _label_e89e                ; $E896  D0 06
  lda z:_var_0049_indexed        ; $E898  A5 49
  cmp #$00                       ; $E89A  C9 00
  beq _label_e8a2                ; $E89C  F0 04

_label_e89e:
  lda #$8E                       ; $E89E  A9 8E
  sta z:_var_0000_indexed        ; $E8A0  85 00

_label_e8a2:
  lda #$00                       ; $E8A2  A9 00
  sta z:_var_0056                ; $E8A4  85 56
  ldy #$06                       ; $E8A6  A0 06
  ldx #$EF                       ; $E8A8  A2 EF
  lda #$66                       ; $E8AA  A9 66
  sec                            ; $E8AC  38
  clv                            ; $E8AD  B8
  sax z:_var_0050_indexed,Y      ; $E8AE  97 50
  nop                            ; $E8B0  EA
  nop                            ; $E8B1  EA
  nop                            ; $E8B2  EA
  nop                            ; $E8B3  EA
  beq _label_e8ce                ; $E8B4  F0 18
  bcc _label_e8ce                ; $E8B6  90 16
  bvs _label_e8ce                ; $E8B8  70 14
  bmi _label_e8ce                ; $E8BA  30 12
  cmp #$66                       ; $E8BC  C9 66
  bne _label_e8ce                ; $E8BE  D0 0E
  cpy #$06                       ; $E8C0  C0 06
  bne _label_e8ce                ; $E8C2  D0 0A
  cpx #$EF                       ; $E8C4  E0 EF
  bne _label_e8ce                ; $E8C6  D0 06
  lda z:_var_0056                ; $E8C8  A5 56
  cmp #$66                       ; $E8CA  C9 66
  beq _label_e8d2                ; $E8CC  F0 04

_label_e8ce:
  lda #$8F                       ; $E8CE  A9 8F
  sta z:_var_0000_indexed        ; $E8D0  85 00

_label_e8d2:
  rts                            ; $E8D2  60

_label_e8d3:
  ldy #$90                       ; $E8D3  A0 90
  jsr _func_f931                 ; $E8D5  20 31 F9
.byte $eb, $40                   ; $E8D8  EB 40  disambiguous instruction: sbc #$40
  nop                            ; $E8DA  EA
  nop                            ; $E8DB  EA
  nop                            ; $E8DC  EA
  nop                            ; $E8DD  EA
  jsr _func_f937                 ; $E8DE  20 37 F9
  iny                            ; $E8E1  C8
  jsr _func_f947                 ; $E8E2  20 47 F9
.byte $eb, $3f                   ; $E8E5  EB 3F  disambiguous instruction: sbc #$3F
  nop                            ; $E8E7  EA
  nop                            ; $E8E8  EA
  nop                            ; $E8E9  EA
  nop                            ; $E8EA  EA
  jsr _func_f94c                 ; $E8EB  20 4C F9
  iny                            ; $E8EE  C8
  jsr _func_f95c                 ; $E8EF  20 5C F9
.byte $eb, $41                   ; $E8F2  EB 41  disambiguous instruction: sbc #$41
  nop                            ; $E8F4  EA
  nop                            ; $E8F5  EA
  nop                            ; $E8F6  EA
  nop                            ; $E8F7  EA
  jsr _func_f962                 ; $E8F8  20 62 F9
  iny                            ; $E8FB  C8
  jsr _func_f972                 ; $E8FC  20 72 F9
.byte $eb, $00                   ; $E8FF  EB 00  disambiguous instruction: sbc #$00
  nop                            ; $E901  EA
  nop                            ; $E902  EA
  nop                            ; $E903  EA
  nop                            ; $E904  EA
  jsr _func_f976                 ; $E905  20 76 F9
  iny                            ; $E908  C8
  jsr _func_f980                 ; $E909  20 80 F9
.byte $eb, $7f                   ; $E90C  EB 7F  disambiguous instruction: sbc #$7F
  nop                            ; $E90E  EA
  nop                            ; $E90F  EA
  nop                            ; $E910  EA
  nop                            ; $E911  EA
  jsr _func_f984                 ; $E912  20 84 F9
  rts                            ; $E915  60

_label_e916:
  lda #$FF                       ; $E916  A9 FF
  sta z:_var_0001                ; $E918  85 01
  ldy #$95                       ; $E91A  A0 95
  ldx #$02                       ; $E91C  A2 02
  lda #$47                       ; $E91E  A9 47
  sta z:_var_0047                ; $E920  85 47
  lda #$06                       ; $E922  A9 06
  sta z:_var_0048_indexed        ; $E924  85 48
  lda #$EB                       ; $E926  A9 EB
  sta a:_var_0647                ; $E928  8D 47 06
  jsr _func_fa31                 ; $E92B  20 31 FA
  dcp (_var_0045_indexed,X)      ; $E92E  C3 45
  nop                            ; $E930  EA
  nop                            ; $E931  EA
  nop                            ; $E932  EA
  nop                            ; $E933  EA
  jsr _func_fa37                 ; $E934  20 37 FA
  lda a:_var_0647                ; $E937  AD 47 06
  cmp #$EA                       ; $E93A  C9 EA
  beq _label_e940                ; $E93C  F0 02
  sty z:_var_0000_indexed        ; $E93E  84 00

_label_e940:
  iny                            ; $E940  C8
  lda #$00                       ; $E941  A9 00
  sta a:_var_0647                ; $E943  8D 47 06
  jsr _func_fa42                 ; $E946  20 42 FA
  dcp (_var_0045_indexed,X)      ; $E949  C3 45
  nop                            ; $E94B  EA
  nop                            ; $E94C  EA
  nop                            ; $E94D  EA
  nop                            ; $E94E  EA
  jsr _func_fa47                 ; $E94F  20 47 FA
  lda a:_var_0647                ; $E952  AD 47 06
  cmp #$FF                       ; $E955  C9 FF
  beq _label_e95b                ; $E957  F0 02
  sty z:_var_0000_indexed        ; $E959  84 00

_label_e95b:
  iny                            ; $E95B  C8
  lda #$37                       ; $E95C  A9 37
  sta a:_var_0647                ; $E95E  8D 47 06
  jsr _func_fa54                 ; $E961  20 54 FA
  dcp (_var_0045_indexed,X)      ; $E964  C3 45
  nop                            ; $E966  EA
  nop                            ; $E967  EA
  nop                            ; $E968  EA
  nop                            ; $E969  EA
  jsr _func_fa59                 ; $E96A  20 59 FA
  lda a:_var_0647                ; $E96D  AD 47 06
  cmp #$36                       ; $E970  C9 36
  beq _label_e976                ; $E972  F0 02
  sty z:_var_0000_indexed        ; $E974  84 00

_label_e976:
  iny                            ; $E976  C8
  lda #$EB                       ; $E977  A9 EB
  sta z:_var_0047                ; $E979  85 47
  jsr _func_fa31                 ; $E97B  20 31 FA
  dcp z:_var_0047                ; $E97E  C7 47
  nop                            ; $E980  EA
  nop                            ; $E981  EA
  nop                            ; $E982  EA
  nop                            ; $E983  EA
  jsr _func_fa37                 ; $E984  20 37 FA
  lda z:_var_0047                ; $E987  A5 47
  cmp #$EA                       ; $E989  C9 EA
  beq _label_e98f                ; $E98B  F0 02
  sty z:_var_0000_indexed        ; $E98D  84 00

_label_e98f:
  iny                            ; $E98F  C8
  lda #$00                       ; $E990  A9 00
  sta z:_var_0047                ; $E992  85 47
  jsr _func_fa42                 ; $E994  20 42 FA
  dcp z:_var_0047                ; $E997  C7 47
  nop                            ; $E999  EA
  nop                            ; $E99A  EA
  nop                            ; $E99B  EA
  nop                            ; $E99C  EA
  jsr _func_fa47                 ; $E99D  20 47 FA
  lda z:_var_0047                ; $E9A0  A5 47
  cmp #$FF                       ; $E9A2  C9 FF
  beq _label_e9a8                ; $E9A4  F0 02
  sty z:_var_0000_indexed        ; $E9A6  84 00

_label_e9a8:
  iny                            ; $E9A8  C8
  lda #$37                       ; $E9A9  A9 37
  sta z:_var_0047                ; $E9AB  85 47
  jsr _func_fa54                 ; $E9AD  20 54 FA
  dcp z:_var_0047                ; $E9B0  C7 47
  nop                            ; $E9B2  EA
  nop                            ; $E9B3  EA
  nop                            ; $E9B4  EA
  nop                            ; $E9B5  EA
  jsr _func_fa59                 ; $E9B6  20 59 FA
  lda z:_var_0047                ; $E9B9  A5 47
  cmp #$36                       ; $E9BB  C9 36
  beq _label_e9c1                ; $E9BD  F0 02
  sty z:_var_0000_indexed        ; $E9BF  84 00

_label_e9c1:
  iny                            ; $E9C1  C8
  lda #$EB                       ; $E9C2  A9 EB
  sta a:_var_0647                ; $E9C4  8D 47 06
  jsr _func_fa31                 ; $E9C7  20 31 FA
  dcp a:_var_0647                ; $E9CA  CF 47 06
  nop                            ; $E9CD  EA
  nop                            ; $E9CE  EA
  nop                            ; $E9CF  EA
  nop                            ; $E9D0  EA
  jsr _func_fa37                 ; $E9D1  20 37 FA
  lda a:_var_0647                ; $E9D4  AD 47 06
  cmp #$EA                       ; $E9D7  C9 EA
  beq _label_e9dd                ; $E9D9  F0 02
  sty z:_var_0000_indexed        ; $E9DB  84 00

_label_e9dd:
  iny                            ; $E9DD  C8
  lda #$00                       ; $E9DE  A9 00
  sta a:_var_0647                ; $E9E0  8D 47 06
  jsr _func_fa42                 ; $E9E3  20 42 FA
  dcp a:_var_0647                ; $E9E6  CF 47 06
  nop                            ; $E9E9  EA
  nop                            ; $E9EA  EA
  nop                            ; $E9EB  EA
  nop                            ; $E9EC  EA
  jsr _func_fa47                 ; $E9ED  20 47 FA
  lda a:_var_0647                ; $E9F0  AD 47 06
  cmp #$FF                       ; $E9F3  C9 FF
  beq _label_e9f9                ; $E9F5  F0 02
  sty z:_var_0000_indexed        ; $E9F7  84 00

_label_e9f9:
  iny                            ; $E9F9  C8
  lda #$37                       ; $E9FA  A9 37
  sta a:_var_0647                ; $E9FC  8D 47 06
  jsr _func_fa54                 ; $E9FF  20 54 FA
  dcp a:_var_0647                ; $EA02  CF 47 06
  nop                            ; $EA05  EA
  nop                            ; $EA06  EA
  nop                            ; $EA07  EA
  nop                            ; $EA08  EA
  jsr _func_fa59                 ; $EA09  20 59 FA
  lda a:_var_0647                ; $EA0C  AD 47 06
  cmp #$36                       ; $EA0F  C9 36
  beq _label_ea15                ; $EA11  F0 02
  sty z:_var_0000_indexed        ; $EA13  84 00

_label_ea15:
  lda #$EB                       ; $EA15  A9 EB
  sta a:_var_0647                ; $EA17  8D 47 06
  lda #$48                       ; $EA1A  A9 48
  sta z:_var_0045_indexed        ; $EA1C  85 45
  lda #$05                       ; $EA1E  A9 05
  sta z:_var_0046                ; $EA20  85 46
  ldy #$FF                       ; $EA22  A0 FF
  jsr _func_fa31                 ; $EA24  20 31 FA
  dcp (_var_0045_indexed),Y      ; $EA27  D3 45
  nop                            ; $EA29  EA
  nop                            ; $EA2A  EA
  php                            ; $EA2B  08
  pha                            ; $EA2C  48
  ldy #$9E                       ; $EA2D  A0 9E
  pla                            ; $EA2F  68
  plp                            ; $EA30  28
  jsr _func_fa37                 ; $EA31  20 37 FA
  lda a:_var_0647                ; $EA34  AD 47 06
  cmp #$EA                       ; $EA37  C9 EA
  beq _label_ea3d                ; $EA39  F0 02
  sty z:_var_0000_indexed        ; $EA3B  84 00

_label_ea3d:
  ldy #$FF                       ; $EA3D  A0 FF
  lda #$00                       ; $EA3F  A9 00
  sta a:_var_0647                ; $EA41  8D 47 06
  jsr _func_fa42                 ; $EA44  20 42 FA
  dcp (_var_0045_indexed),Y      ; $EA47  D3 45
  nop                            ; $EA49  EA
  nop                            ; $EA4A  EA
  php                            ; $EA4B  08
  pha                            ; $EA4C  48
  ldy #$9F                       ; $EA4D  A0 9F
  pla                            ; $EA4F  68
  plp                            ; $EA50  28
  jsr _func_fa47                 ; $EA51  20 47 FA
  lda a:_var_0647                ; $EA54  AD 47 06
  cmp #$FF                       ; $EA57  C9 FF
  beq _label_ea5d                ; $EA59  F0 02
  sty z:_var_0000_indexed        ; $EA5B  84 00

_label_ea5d:
  ldy #$FF                       ; $EA5D  A0 FF
  lda #$37                       ; $EA5F  A9 37
  sta a:_var_0647                ; $EA61  8D 47 06
  jsr _func_fa54                 ; $EA64  20 54 FA
  dcp (_var_0045_indexed),Y      ; $EA67  D3 45
  nop                            ; $EA69  EA
  nop                            ; $EA6A  EA
  php                            ; $EA6B  08
  pha                            ; $EA6C  48
  ldy #$A0                       ; $EA6D  A0 A0
  pla                            ; $EA6F  68
  plp                            ; $EA70  28
  jsr _func_fa59                 ; $EA71  20 59 FA
  lda a:_var_0647                ; $EA74  AD 47 06
  cmp #$36                       ; $EA77  C9 36
  beq _label_ea7d                ; $EA79  F0 02
  sty z:_var_0000_indexed        ; $EA7B  84 00

_label_ea7d:
  ldy #$A1                       ; $EA7D  A0 A1
  ldx #$FF                       ; $EA7F  A2 FF
  lda #$EB                       ; $EA81  A9 EB
  sta z:_var_0047                ; $EA83  85 47
  jsr _func_fa31                 ; $EA85  20 31 FA
  dcp z:_var_0048_indexed,X      ; $EA88  D7 48
  nop                            ; $EA8A  EA
  nop                            ; $EA8B  EA
  nop                            ; $EA8C  EA
  nop                            ; $EA8D  EA
  jsr _func_fa37                 ; $EA8E  20 37 FA
  lda z:_var_0047                ; $EA91  A5 47
  cmp #$EA                       ; $EA93  C9 EA
  beq _label_ea99                ; $EA95  F0 02
  sty z:_var_0000_indexed        ; $EA97  84 00

_label_ea99:
  iny                            ; $EA99  C8
  lda #$00                       ; $EA9A  A9 00
  sta z:_var_0047                ; $EA9C  85 47
  jsr _func_fa42                 ; $EA9E  20 42 FA
  dcp z:_var_0048_indexed,X      ; $EAA1  D7 48
  nop                            ; $EAA3  EA
  nop                            ; $EAA4  EA
  nop                            ; $EAA5  EA
  nop                            ; $EAA6  EA
  jsr _func_fa47                 ; $EAA7  20 47 FA
  lda z:_var_0047                ; $EAAA  A5 47
  cmp #$FF                       ; $EAAC  C9 FF
  beq _label_eab2                ; $EAAE  F0 02
  sty z:_var_0000_indexed        ; $EAB0  84 00

_label_eab2:
  iny                            ; $EAB2  C8
  lda #$37                       ; $EAB3  A9 37
  sta z:_var_0047                ; $EAB5  85 47
  jsr _func_fa54                 ; $EAB7  20 54 FA
  dcp z:_var_0048_indexed,X      ; $EABA  D7 48
  nop                            ; $EABC  EA
  nop                            ; $EABD  EA
  nop                            ; $EABE  EA
  nop                            ; $EABF  EA
  jsr _func_fa59                 ; $EAC0  20 59 FA
  lda z:_var_0047                ; $EAC3  A5 47
  cmp #$36                       ; $EAC5  C9 36
  beq _label_eacb                ; $EAC7  F0 02
  sty z:_var_0000_indexed        ; $EAC9  84 00

_label_eacb:
  lda #$EB                       ; $EACB  A9 EB
  sta a:_var_0647                ; $EACD  8D 47 06
  ldy #$FF                       ; $EAD0  A0 FF
  jsr _func_fa31                 ; $EAD2  20 31 FA
  dcp a:_var_0548_indexed,Y      ; $EAD5  DB 48 05
  nop                            ; $EAD8  EA
  nop                            ; $EAD9  EA
  php                            ; $EADA  08
  pha                            ; $EADB  48
  ldy #$A4                       ; $EADC  A0 A4
  pla                            ; $EADE  68
  plp                            ; $EADF  28
  jsr _func_fa37                 ; $EAE0  20 37 FA
  lda a:_var_0647                ; $EAE3  AD 47 06
  cmp #$EA                       ; $EAE6  C9 EA
  beq _label_eaec                ; $EAE8  F0 02
  sty z:_var_0000_indexed        ; $EAEA  84 00

_label_eaec:
  ldy #$FF                       ; $EAEC  A0 FF
  lda #$00                       ; $EAEE  A9 00
  sta a:_var_0647                ; $EAF0  8D 47 06
  jsr _func_fa42                 ; $EAF3  20 42 FA
  dcp a:_var_0548_indexed,Y      ; $EAF6  DB 48 05
  nop                            ; $EAF9  EA
  nop                            ; $EAFA  EA
  php                            ; $EAFB  08
  pha                            ; $EAFC  48
  ldy #$A5                       ; $EAFD  A0 A5
  pla                            ; $EAFF  68
  plp                            ; $EB00  28
  jsr _func_fa47                 ; $EB01  20 47 FA
  lda a:_var_0647                ; $EB04  AD 47 06
  cmp #$FF                       ; $EB07  C9 FF
  beq _label_eb0d                ; $EB09  F0 02
  sty z:_var_0000_indexed        ; $EB0B  84 00

_label_eb0d:
  ldy #$FF                       ; $EB0D  A0 FF
  lda #$37                       ; $EB0F  A9 37
  sta a:_var_0647                ; $EB11  8D 47 06
  jsr _func_fa54                 ; $EB14  20 54 FA
  dcp a:_var_0548_indexed,Y      ; $EB17  DB 48 05
  nop                            ; $EB1A  EA
  nop                            ; $EB1B  EA
  php                            ; $EB1C  08
  pha                            ; $EB1D  48
  ldy #$A6                       ; $EB1E  A0 A6
  pla                            ; $EB20  68
  plp                            ; $EB21  28
  jsr _func_fa59                 ; $EB22  20 59 FA
  lda a:_var_0647                ; $EB25  AD 47 06
  cmp #$36                       ; $EB28  C9 36
  beq _label_eb2e                ; $EB2A  F0 02
  sty z:_var_0000_indexed        ; $EB2C  84 00

_label_eb2e:
  ldy #$A7                       ; $EB2E  A0 A7
  ldx #$FF                       ; $EB30  A2 FF
  lda #$EB                       ; $EB32  A9 EB
  sta a:_var_0647                ; $EB34  8D 47 06
  jsr _func_fa31                 ; $EB37  20 31 FA
  dcp a:_var_0548_indexed,X      ; $EB3A  DF 48 05
  nop                            ; $EB3D  EA
  nop                            ; $EB3E  EA
  nop                            ; $EB3F  EA
  nop                            ; $EB40  EA
  jsr _func_fa37                 ; $EB41  20 37 FA
  lda a:_var_0647                ; $EB44  AD 47 06
  cmp #$EA                       ; $EB47  C9 EA
  beq _label_eb4d                ; $EB49  F0 02
  sty z:_var_0000_indexed        ; $EB4B  84 00

_label_eb4d:
  iny                            ; $EB4D  C8
  lda #$00                       ; $EB4E  A9 00
  sta a:_var_0647                ; $EB50  8D 47 06
  jsr _func_fa42                 ; $EB53  20 42 FA
  dcp a:_var_0548_indexed,X      ; $EB56  DF 48 05
  nop                            ; $EB59  EA
  nop                            ; $EB5A  EA
  nop                            ; $EB5B  EA
  nop                            ; $EB5C  EA
  jsr _func_fa47                 ; $EB5D  20 47 FA
  lda a:_var_0647                ; $EB60  AD 47 06
  cmp #$FF                       ; $EB63  C9 FF
  beq _label_eb69                ; $EB65  F0 02
  sty z:_var_0000_indexed        ; $EB67  84 00

_label_eb69:
  iny                            ; $EB69  C8
  lda #$37                       ; $EB6A  A9 37
  sta a:_var_0647                ; $EB6C  8D 47 06
  jsr _func_fa54                 ; $EB6F  20 54 FA
  dcp a:_var_0548_indexed,X      ; $EB72  DF 48 05
  nop                            ; $EB75  EA
  nop                            ; $EB76  EA
  nop                            ; $EB77  EA
  nop                            ; $EB78  EA
  jsr _func_fa59                 ; $EB79  20 59 FA
  lda a:_var_0647                ; $EB7C  AD 47 06
  cmp #$36                       ; $EB7F  C9 36
  beq _label_eb85                ; $EB81  F0 02
  sty z:_var_0000_indexed        ; $EB83  84 00

_label_eb85:
  rts                            ; $EB85  60

_label_eb86:
  lda #$FF                       ; $EB86  A9 FF
  sta z:_var_0001                ; $EB88  85 01
  ldy #$AA                       ; $EB8A  A0 AA
  ldx #$02                       ; $EB8C  A2 02
  lda #$47                       ; $EB8E  A9 47
  sta z:_var_0047                ; $EB90  85 47
  lda #$06                       ; $EB92  A9 06
  sta z:_var_0048_indexed        ; $EB94  85 48
  lda #$EB                       ; $EB96  A9 EB
  sta a:_var_0647                ; $EB98  8D 47 06
  jsr _func_fab1                 ; $EB9B  20 B1 FA
  isc (_var_0045_indexed,X)      ; $EB9E  E3 45
  nop                            ; $EBA0  EA
  nop                            ; $EBA1  EA
  nop                            ; $EBA2  EA
  nop                            ; $EBA3  EA
  jsr _func_fab7                 ; $EBA4  20 B7 FA
  lda a:_var_0647                ; $EBA7  AD 47 06
  cmp #$EC                       ; $EBAA  C9 EC
  beq _label_ebb0                ; $EBAC  F0 02
  sty z:_var_0000_indexed        ; $EBAE  84 00

_label_ebb0:
  iny                            ; $EBB0  C8
  lda #$FF                       ; $EBB1  A9 FF
  sta a:_var_0647                ; $EBB3  8D 47 06
  jsr _func_fac2                 ; $EBB6  20 C2 FA
  isc (_var_0045_indexed,X)      ; $EBB9  E3 45
  nop                            ; $EBBB  EA
  nop                            ; $EBBC  EA
  nop                            ; $EBBD  EA
  nop                            ; $EBBE  EA
  jsr _func_fac7                 ; $EBBF  20 C7 FA
  lda a:_var_0647                ; $EBC2  AD 47 06
  cmp #$00                       ; $EBC5  C9 00
  beq _label_ebcb                ; $EBC7  F0 02
  sty z:_var_0000_indexed        ; $EBC9  84 00

_label_ebcb:
  iny                            ; $EBCB  C8
  lda #$37                       ; $EBCC  A9 37
  sta a:_var_0647                ; $EBCE  8D 47 06
  jsr _func_fad4                 ; $EBD1  20 D4 FA
  isc (_var_0045_indexed,X)      ; $EBD4  E3 45
  nop                            ; $EBD6  EA
  nop                            ; $EBD7  EA
  nop                            ; $EBD8  EA
  nop                            ; $EBD9  EA
  jsr _func_fada                 ; $EBDA  20 DA FA
  lda a:_var_0647                ; $EBDD  AD 47 06
  cmp #$38                       ; $EBE0  C9 38
  beq _label_ebe6                ; $EBE2  F0 02
  sty z:_var_0000_indexed        ; $EBE4  84 00

_label_ebe6:
  iny                            ; $EBE6  C8
  lda #$EB                       ; $EBE7  A9 EB
  sta z:_var_0047                ; $EBE9  85 47
  jsr _func_fab1                 ; $EBEB  20 B1 FA
  isc z:_var_0047                ; $EBEE  E7 47
  nop                            ; $EBF0  EA
  nop                            ; $EBF1  EA
  nop                            ; $EBF2  EA
  nop                            ; $EBF3  EA
  jsr _func_fab7                 ; $EBF4  20 B7 FA
  lda z:_var_0047                ; $EBF7  A5 47
  cmp #$EC                       ; $EBF9  C9 EC
  beq _label_ebff                ; $EBFB  F0 02
  sty z:_var_0000_indexed        ; $EBFD  84 00

_label_ebff:
  iny                            ; $EBFF  C8
  lda #$FF                       ; $EC00  A9 FF
  sta z:_var_0047                ; $EC02  85 47
  jsr _func_fac2                 ; $EC04  20 C2 FA
  isc z:_var_0047                ; $EC07  E7 47
  nop                            ; $EC09  EA
  nop                            ; $EC0A  EA
  nop                            ; $EC0B  EA
  nop                            ; $EC0C  EA
  jsr _func_fac7                 ; $EC0D  20 C7 FA
  lda z:_var_0047                ; $EC10  A5 47
  cmp #$00                       ; $EC12  C9 00
  beq _label_ec18                ; $EC14  F0 02
  sty z:_var_0000_indexed        ; $EC16  84 00

_label_ec18:
  iny                            ; $EC18  C8
  lda #$37                       ; $EC19  A9 37
  sta z:_var_0047                ; $EC1B  85 47
  jsr _func_fad4                 ; $EC1D  20 D4 FA
  isc z:_var_0047                ; $EC20  E7 47
  nop                            ; $EC22  EA
  nop                            ; $EC23  EA
  nop                            ; $EC24  EA
  nop                            ; $EC25  EA
  jsr _func_fada                 ; $EC26  20 DA FA
  lda z:_var_0047                ; $EC29  A5 47
  cmp #$38                       ; $EC2B  C9 38
  beq _label_ec31                ; $EC2D  F0 02
  sty z:_var_0000_indexed        ; $EC2F  84 00

_label_ec31:
  iny                            ; $EC31  C8
  lda #$EB                       ; $EC32  A9 EB
  sta a:_var_0647                ; $EC34  8D 47 06
  jsr _func_fab1                 ; $EC37  20 B1 FA
  isc a:_var_0647                ; $EC3A  EF 47 06
  nop                            ; $EC3D  EA
  nop                            ; $EC3E  EA
  nop                            ; $EC3F  EA
  nop                            ; $EC40  EA
  jsr _func_fab7                 ; $EC41  20 B7 FA
  lda a:_var_0647                ; $EC44  AD 47 06
  cmp #$EC                       ; $EC47  C9 EC
  beq _label_ec4d                ; $EC49  F0 02
  sty z:_var_0000_indexed        ; $EC4B  84 00

_label_ec4d:
  iny                            ; $EC4D  C8
  lda #$FF                       ; $EC4E  A9 FF
  sta a:_var_0647                ; $EC50  8D 47 06
  jsr _func_fac2                 ; $EC53  20 C2 FA
  isc a:_var_0647                ; $EC56  EF 47 06
  nop                            ; $EC59  EA
  nop                            ; $EC5A  EA
  nop                            ; $EC5B  EA
  nop                            ; $EC5C  EA
  jsr _func_fac7                 ; $EC5D  20 C7 FA
  lda a:_var_0647                ; $EC60  AD 47 06
  cmp #$00                       ; $EC63  C9 00
  beq _label_ec69                ; $EC65  F0 02
  sty z:_var_0000_indexed        ; $EC67  84 00

_label_ec69:
  iny                            ; $EC69  C8
  lda #$37                       ; $EC6A  A9 37
  sta a:_var_0647                ; $EC6C  8D 47 06
  jsr _func_fad4                 ; $EC6F  20 D4 FA
  isc a:_var_0647                ; $EC72  EF 47 06
  nop                            ; $EC75  EA
  nop                            ; $EC76  EA
  nop                            ; $EC77  EA
  nop                            ; $EC78  EA
  jsr _func_fada                 ; $EC79  20 DA FA
  lda a:_var_0647                ; $EC7C  AD 47 06
  cmp #$38                       ; $EC7F  C9 38
  beq _label_ec85                ; $EC81  F0 02
  sty z:_var_0000_indexed        ; $EC83  84 00

_label_ec85:
  lda #$EB                       ; $EC85  A9 EB
  sta a:_var_0647                ; $EC87  8D 47 06
  lda #$48                       ; $EC8A  A9 48
  sta z:_var_0045_indexed        ; $EC8C  85 45
  lda #$05                       ; $EC8E  A9 05
  sta z:_var_0046                ; $EC90  85 46
  ldy #$FF                       ; $EC92  A0 FF
  jsr _func_fab1                 ; $EC94  20 B1 FA
  isc (_var_0045_indexed),Y      ; $EC97  F3 45
  nop                            ; $EC99  EA
  nop                            ; $EC9A  EA
  php                            ; $EC9B  08
  pha                            ; $EC9C  48
  ldy #$B3                       ; $EC9D  A0 B3
  pla                            ; $EC9F  68
  plp                            ; $ECA0  28
  jsr _func_fab7                 ; $ECA1  20 B7 FA
  lda a:_var_0647                ; $ECA4  AD 47 06
  cmp #$EC                       ; $ECA7  C9 EC
  beq _label_ecad                ; $ECA9  F0 02
  sty z:_var_0000_indexed        ; $ECAB  84 00

_label_ecad:
  ldy #$FF                       ; $ECAD  A0 FF
  lda #$FF                       ; $ECAF  A9 FF
  sta a:_var_0647                ; $ECB1  8D 47 06
  jsr _func_fac2                 ; $ECB4  20 C2 FA
  isc (_var_0045_indexed),Y      ; $ECB7  F3 45
  nop                            ; $ECB9  EA
  nop                            ; $ECBA  EA
  php                            ; $ECBB  08
  pha                            ; $ECBC  48
  ldy #$B4                       ; $ECBD  A0 B4
  pla                            ; $ECBF  68
  plp                            ; $ECC0  28
  jsr _func_fac7                 ; $ECC1  20 C7 FA
  lda a:_var_0647                ; $ECC4  AD 47 06
  cmp #$00                       ; $ECC7  C9 00
  beq _label_eccd                ; $ECC9  F0 02
  sty z:_var_0000_indexed        ; $ECCB  84 00

_label_eccd:
  ldy #$FF                       ; $ECCD  A0 FF
  lda #$37                       ; $ECCF  A9 37
  sta a:_var_0647                ; $ECD1  8D 47 06
  jsr _func_fad4                 ; $ECD4  20 D4 FA
  isc (_var_0045_indexed),Y      ; $ECD7  F3 45
  nop                            ; $ECD9  EA
  nop                            ; $ECDA  EA
  php                            ; $ECDB  08
  pha                            ; $ECDC  48
  ldy #$B5                       ; $ECDD  A0 B5
  pla                            ; $ECDF  68
  plp                            ; $ECE0  28
  jsr _func_fada                 ; $ECE1  20 DA FA
  lda a:_var_0647                ; $ECE4  AD 47 06
  cmp #$38                       ; $ECE7  C9 38
  beq _label_eced                ; $ECE9  F0 02
  sty z:_var_0000_indexed        ; $ECEB  84 00

_label_eced:
  ldy #$B6                       ; $ECED  A0 B6
  ldx #$FF                       ; $ECEF  A2 FF
  lda #$EB                       ; $ECF1  A9 EB
  sta z:_var_0047                ; $ECF3  85 47
  jsr _func_fab1                 ; $ECF5  20 B1 FA
  isc z:_var_0048_indexed,X      ; $ECF8  F7 48
  nop                            ; $ECFA  EA
  nop                            ; $ECFB  EA
  nop                            ; $ECFC  EA
  nop                            ; $ECFD  EA
  jsr _func_fab7                 ; $ECFE  20 B7 FA
  lda z:_var_0047                ; $ED01  A5 47
  cmp #$EC                       ; $ED03  C9 EC
  beq _label_ed09                ; $ED05  F0 02
  sty z:_var_0000_indexed        ; $ED07  84 00

_label_ed09:
  iny                            ; $ED09  C8
  lda #$FF                       ; $ED0A  A9 FF
  sta z:_var_0047                ; $ED0C  85 47
  jsr _func_fac2                 ; $ED0E  20 C2 FA
  isc z:_var_0048_indexed,X      ; $ED11  F7 48
  nop                            ; $ED13  EA
  nop                            ; $ED14  EA
  nop                            ; $ED15  EA
  nop                            ; $ED16  EA
  jsr _func_fac7                 ; $ED17  20 C7 FA
  lda z:_var_0047                ; $ED1A  A5 47
  cmp #$00                       ; $ED1C  C9 00
  beq _label_ed22                ; $ED1E  F0 02
  sty z:_var_0000_indexed        ; $ED20  84 00

_label_ed22:
  iny                            ; $ED22  C8
  lda #$37                       ; $ED23  A9 37
  sta z:_var_0047                ; $ED25  85 47
  jsr _func_fad4                 ; $ED27  20 D4 FA
  isc z:_var_0048_indexed,X      ; $ED2A  F7 48
  nop                            ; $ED2C  EA
  nop                            ; $ED2D  EA
  nop                            ; $ED2E  EA
  nop                            ; $ED2F  EA
  jsr _func_fada                 ; $ED30  20 DA FA
  lda z:_var_0047                ; $ED33  A5 47
  cmp #$38                       ; $ED35  C9 38
  beq _label_ed3b                ; $ED37  F0 02
  sty z:_var_0000_indexed        ; $ED39  84 00

_label_ed3b:
  lda #$EB                       ; $ED3B  A9 EB
  sta a:_var_0647                ; $ED3D  8D 47 06
  ldy #$FF                       ; $ED40  A0 FF
  jsr _func_fab1                 ; $ED42  20 B1 FA
  isc a:_var_0548_indexed,Y      ; $ED45  FB 48 05
  nop                            ; $ED48  EA
  nop                            ; $ED49  EA
  php                            ; $ED4A  08
  pha                            ; $ED4B  48
  ldy #$B9                       ; $ED4C  A0 B9
  pla                            ; $ED4E  68
  plp                            ; $ED4F  28
  jsr _func_fab7                 ; $ED50  20 B7 FA
  lda a:_var_0647                ; $ED53  AD 47 06
  cmp #$EC                       ; $ED56  C9 EC
  beq _label_ed5c                ; $ED58  F0 02
  sty z:_var_0000_indexed        ; $ED5A  84 00

_label_ed5c:
  ldy #$FF                       ; $ED5C  A0 FF
  lda #$FF                       ; $ED5E  A9 FF
  sta a:_var_0647                ; $ED60  8D 47 06
  jsr _func_fac2                 ; $ED63  20 C2 FA
  isc a:_var_0548_indexed,Y      ; $ED66  FB 48 05
  nop                            ; $ED69  EA
  nop                            ; $ED6A  EA
  php                            ; $ED6B  08
  pha                            ; $ED6C  48
  ldy #$BA                       ; $ED6D  A0 BA
  pla                            ; $ED6F  68
  plp                            ; $ED70  28
  jsr _func_fac7                 ; $ED71  20 C7 FA
  lda a:_var_0647                ; $ED74  AD 47 06
  cmp #$00                       ; $ED77  C9 00
  beq _label_ed7d                ; $ED79  F0 02
  sty z:_var_0000_indexed        ; $ED7B  84 00

_label_ed7d:
  ldy #$FF                       ; $ED7D  A0 FF
  lda #$37                       ; $ED7F  A9 37
  sta a:_var_0647                ; $ED81  8D 47 06
  jsr _func_fad4                 ; $ED84  20 D4 FA
  isc a:_var_0548_indexed,Y      ; $ED87  FB 48 05
  nop                            ; $ED8A  EA
  nop                            ; $ED8B  EA
  php                            ; $ED8C  08
  pha                            ; $ED8D  48
  ldy #$BB                       ; $ED8E  A0 BB
  pla                            ; $ED90  68
  plp                            ; $ED91  28
  jsr _func_fada                 ; $ED92  20 DA FA
  lda a:_var_0647                ; $ED95  AD 47 06
  cmp #$38                       ; $ED98  C9 38
  beq _label_ed9e                ; $ED9A  F0 02
  sty z:_var_0000_indexed        ; $ED9C  84 00

_label_ed9e:
  ldy #$BC                       ; $ED9E  A0 BC
  ldx #$FF                       ; $EDA0  A2 FF
  lda #$EB                       ; $EDA2  A9 EB
  sta a:_var_0647                ; $EDA4  8D 47 06
  jsr _func_fab1                 ; $EDA7  20 B1 FA
  isc a:_var_0548_indexed,X      ; $EDAA  FF 48 05
  nop                            ; $EDAD  EA
  nop                            ; $EDAE  EA
  nop                            ; $EDAF  EA
  nop                            ; $EDB0  EA
  jsr _func_fab7                 ; $EDB1  20 B7 FA
  lda a:_var_0647                ; $EDB4  AD 47 06
  cmp #$EC                       ; $EDB7  C9 EC
  beq _label_edbd                ; $EDB9  F0 02
  sty z:_var_0000_indexed        ; $EDBB  84 00

_label_edbd:
  iny                            ; $EDBD  C8
  lda #$FF                       ; $EDBE  A9 FF
  sta a:_var_0647                ; $EDC0  8D 47 06
  jsr _func_fac2                 ; $EDC3  20 C2 FA
  isc a:_var_0548_indexed,X      ; $EDC6  FF 48 05
  nop                            ; $EDC9  EA
  nop                            ; $EDCA  EA
  nop                            ; $EDCB  EA
  nop                            ; $EDCC  EA
  jsr _func_fac7                 ; $EDCD  20 C7 FA
  lda a:_var_0647                ; $EDD0  AD 47 06
  cmp #$00                       ; $EDD3  C9 00
  beq _label_edd9                ; $EDD5  F0 02
  sty z:_var_0000_indexed        ; $EDD7  84 00

_label_edd9:
  iny                            ; $EDD9  C8
  lda #$37                       ; $EDDA  A9 37
  sta a:_var_0647                ; $EDDC  8D 47 06
  jsr _func_fad4                 ; $EDDF  20 D4 FA
  isc a:_var_0548_indexed,X      ; $EDE2  FF 48 05
  nop                            ; $EDE5  EA
  nop                            ; $EDE6  EA
  nop                            ; $EDE7  EA
  nop                            ; $EDE8  EA
  jsr _func_fada                 ; $EDE9  20 DA FA
  lda a:_var_0647                ; $EDEC  AD 47 06
  cmp #$38                       ; $EDEF  C9 38
  beq _label_edf5                ; $EDF1  F0 02
  sty z:_var_0000_indexed        ; $EDF3  84 00

_label_edf5:
  rts                            ; $EDF5  60

_label_edf6:
  lda #$FF                       ; $EDF6  A9 FF
  sta z:_var_0001                ; $EDF8  85 01
  ldy #$BF                       ; $EDFA  A0 BF
  ldx #$02                       ; $EDFC  A2 02
  lda #$47                       ; $EDFE  A9 47
  sta z:_var_0047                ; $EE00  85 47
  lda #$06                       ; $EE02  A9 06
  sta z:_var_0048_indexed        ; $EE04  85 48
  lda #$A5                       ; $EE06  A9 A5
  sta a:_var_0647                ; $EE08  8D 47 06
  jsr _func_fa7b                 ; $EE0B  20 7B FA
  slo (_var_0045_indexed,X)      ; $EE0E  03 45
  nop                            ; $EE10  EA
  nop                            ; $EE11  EA
  nop                            ; $EE12  EA
  nop                            ; $EE13  EA
  jsr _func_fa81                 ; $EE14  20 81 FA
  lda a:_var_0647                ; $EE17  AD 47 06
  cmp #$4A                       ; $EE1A  C9 4A
  beq _label_ee20                ; $EE1C  F0 02
  sty z:_var_0000_indexed        ; $EE1E  84 00

_label_ee20:
  iny                            ; $EE20  C8
  lda #$29                       ; $EE21  A9 29
  sta a:_var_0647                ; $EE23  8D 47 06
  jsr _func_fa8c                 ; $EE26  20 8C FA
  slo (_var_0045_indexed,X)      ; $EE29  03 45
  nop                            ; $EE2B  EA
  nop                            ; $EE2C  EA
  nop                            ; $EE2D  EA
  nop                            ; $EE2E  EA
  jsr _func_fa91                 ; $EE2F  20 91 FA
  lda a:_var_0647                ; $EE32  AD 47 06
  cmp #$52                       ; $EE35  C9 52
  beq _label_ee3b                ; $EE37  F0 02
  sty z:_var_0000_indexed        ; $EE39  84 00

_label_ee3b:
  iny                            ; $EE3B  C8
  lda #$37                       ; $EE3C  A9 37
  sta a:_var_0647                ; $EE3E  8D 47 06
  jsr _func_fa9e                 ; $EE41  20 9E FA
  slo (_var_0045_indexed,X)      ; $EE44  03 45
  nop                            ; $EE46  EA
  nop                            ; $EE47  EA
  nop                            ; $EE48  EA
  nop                            ; $EE49  EA
  jsr _func_faa4                 ; $EE4A  20 A4 FA
  lda a:_var_0647                ; $EE4D  AD 47 06
  cmp #$6E                       ; $EE50  C9 6E
  beq _label_ee56                ; $EE52  F0 02
  sty z:_var_0000_indexed        ; $EE54  84 00

_label_ee56:
  iny                            ; $EE56  C8
  lda #$A5                       ; $EE57  A9 A5
  sta z:_var_0047                ; $EE59  85 47
  jsr _func_fa7b                 ; $EE5B  20 7B FA
  slo z:_var_0047                ; $EE5E  07 47
  nop                            ; $EE60  EA
  nop                            ; $EE61  EA
  nop                            ; $EE62  EA
  nop                            ; $EE63  EA
  jsr _func_fa81                 ; $EE64  20 81 FA
  lda z:_var_0047                ; $EE67  A5 47
  cmp #$4A                       ; $EE69  C9 4A
  beq _label_ee6f                ; $EE6B  F0 02
  sty z:_var_0000_indexed        ; $EE6D  84 00

_label_ee6f:
  iny                            ; $EE6F  C8
  lda #$29                       ; $EE70  A9 29
  sta z:_var_0047                ; $EE72  85 47
  jsr _func_fa8c                 ; $EE74  20 8C FA
  slo z:_var_0047                ; $EE77  07 47
  nop                            ; $EE79  EA
  nop                            ; $EE7A  EA
  nop                            ; $EE7B  EA
  nop                            ; $EE7C  EA
  jsr _func_fa91                 ; $EE7D  20 91 FA
  lda z:_var_0047                ; $EE80  A5 47
  cmp #$52                       ; $EE82  C9 52
  beq _label_ee88                ; $EE84  F0 02
  sty z:_var_0000_indexed        ; $EE86  84 00

_label_ee88:
  iny                            ; $EE88  C8
  lda #$37                       ; $EE89  A9 37
  sta z:_var_0047                ; $EE8B  85 47
  jsr _func_fa9e                 ; $EE8D  20 9E FA
  slo z:_var_0047                ; $EE90  07 47
  nop                            ; $EE92  EA
  nop                            ; $EE93  EA
  nop                            ; $EE94  EA
  nop                            ; $EE95  EA
  jsr _func_faa4                 ; $EE96  20 A4 FA
  lda z:_var_0047                ; $EE99  A5 47
  cmp #$6E                       ; $EE9B  C9 6E
  beq _label_eea1                ; $EE9D  F0 02
  sty z:_var_0000_indexed        ; $EE9F  84 00

_label_eea1:
  iny                            ; $EEA1  C8
  lda #$A5                       ; $EEA2  A9 A5
  sta a:_var_0647                ; $EEA4  8D 47 06
  jsr _func_fa7b                 ; $EEA7  20 7B FA
  slo a:_var_0647                ; $EEAA  0F 47 06
  nop                            ; $EEAD  EA
  nop                            ; $EEAE  EA
  nop                            ; $EEAF  EA
  nop                            ; $EEB0  EA
  jsr _func_fa81                 ; $EEB1  20 81 FA
  lda a:_var_0647                ; $EEB4  AD 47 06
  cmp #$4A                       ; $EEB7  C9 4A
  beq _label_eebd                ; $EEB9  F0 02
  sty z:_var_0000_indexed        ; $EEBB  84 00

_label_eebd:
  iny                            ; $EEBD  C8
  lda #$29                       ; $EEBE  A9 29
  sta a:_var_0647                ; $EEC0  8D 47 06
  jsr _func_fa8c                 ; $EEC3  20 8C FA
  slo a:_var_0647                ; $EEC6  0F 47 06
  nop                            ; $EEC9  EA
  nop                            ; $EECA  EA
  nop                            ; $EECB  EA
  nop                            ; $EECC  EA
  jsr _func_fa91                 ; $EECD  20 91 FA
  lda a:_var_0647                ; $EED0  AD 47 06
  cmp #$52                       ; $EED3  C9 52
  beq _label_eed9                ; $EED5  F0 02
  sty z:_var_0000_indexed        ; $EED7  84 00

_label_eed9:
  iny                            ; $EED9  C8
  lda #$37                       ; $EEDA  A9 37
  sta a:_var_0647                ; $EEDC  8D 47 06
  jsr _func_fa9e                 ; $EEDF  20 9E FA
  slo a:_var_0647                ; $EEE2  0F 47 06
  nop                            ; $EEE5  EA
  nop                            ; $EEE6  EA
  nop                            ; $EEE7  EA
  nop                            ; $EEE8  EA
  jsr _func_faa4                 ; $EEE9  20 A4 FA
  lda a:_var_0647                ; $EEEC  AD 47 06
  cmp #$6E                       ; $EEEF  C9 6E
  beq _label_eef5                ; $EEF1  F0 02
  sty z:_var_0000_indexed        ; $EEF3  84 00

_label_eef5:
  lda #$A5                       ; $EEF5  A9 A5
  sta a:_var_0647                ; $EEF7  8D 47 06
  lda #$48                       ; $EEFA  A9 48
  sta z:_var_0045_indexed        ; $EEFC  85 45
  lda #$05                       ; $EEFE  A9 05
  sta z:_var_0046                ; $EF00  85 46
  ldy #$FF                       ; $EF02  A0 FF
  jsr _func_fa7b                 ; $EF04  20 7B FA
  slo (_var_0045_indexed),Y      ; $EF07  13 45
  nop                            ; $EF09  EA
  nop                            ; $EF0A  EA
  php                            ; $EF0B  08
  pha                            ; $EF0C  48
  ldy #$C8                       ; $EF0D  A0 C8
  pla                            ; $EF0F  68
  plp                            ; $EF10  28
  jsr _func_fa81                 ; $EF11  20 81 FA
  lda a:_var_0647                ; $EF14  AD 47 06
  cmp #$4A                       ; $EF17  C9 4A
  beq _label_ef1d                ; $EF19  F0 02
  sty z:_var_0000_indexed        ; $EF1B  84 00

_label_ef1d:
  ldy #$FF                       ; $EF1D  A0 FF
  lda #$29                       ; $EF1F  A9 29
  sta a:_var_0647                ; $EF21  8D 47 06
  jsr _func_fa8c                 ; $EF24  20 8C FA
  slo (_var_0045_indexed),Y      ; $EF27  13 45
  nop                            ; $EF29  EA
  nop                            ; $EF2A  EA
  php                            ; $EF2B  08
  pha                            ; $EF2C  48
  ldy #$C9                       ; $EF2D  A0 C9
  pla                            ; $EF2F  68
  plp                            ; $EF30  28
  jsr _func_fa91                 ; $EF31  20 91 FA
  lda a:_var_0647                ; $EF34  AD 47 06
  cmp #$52                       ; $EF37  C9 52
  beq _label_ef3d                ; $EF39  F0 02
  sty z:_var_0000_indexed        ; $EF3B  84 00

_label_ef3d:
  ldy #$FF                       ; $EF3D  A0 FF
  lda #$37                       ; $EF3F  A9 37
  sta a:_var_0647                ; $EF41  8D 47 06
  jsr _func_fa9e                 ; $EF44  20 9E FA
  slo (_var_0045_indexed),Y      ; $EF47  13 45
  nop                            ; $EF49  EA
  nop                            ; $EF4A  EA
  php                            ; $EF4B  08
  pha                            ; $EF4C  48
  ldy #$CA                       ; $EF4D  A0 CA
  pla                            ; $EF4F  68
  plp                            ; $EF50  28
  jsr _func_faa4                 ; $EF51  20 A4 FA
  lda a:_var_0647                ; $EF54  AD 47 06
  cmp #$6E                       ; $EF57  C9 6E
  beq _label_ef5d                ; $EF59  F0 02
  sty z:_var_0000_indexed        ; $EF5B  84 00

_label_ef5d:
  ldy #$CB                       ; $EF5D  A0 CB
  ldx #$FF                       ; $EF5F  A2 FF
  lda #$A5                       ; $EF61  A9 A5
  sta z:_var_0047                ; $EF63  85 47
  jsr _func_fa7b                 ; $EF65  20 7B FA
  slo z:_var_0048_indexed,X      ; $EF68  17 48
  nop                            ; $EF6A  EA
  nop                            ; $EF6B  EA
  nop                            ; $EF6C  EA
  nop                            ; $EF6D  EA
  jsr _func_fa81                 ; $EF6E  20 81 FA
  lda z:_var_0047                ; $EF71  A5 47
  cmp #$4A                       ; $EF73  C9 4A
  beq _label_ef79                ; $EF75  F0 02
  sty z:_var_0000_indexed        ; $EF77  84 00

_label_ef79:
  iny                            ; $EF79  C8
  lda #$29                       ; $EF7A  A9 29
  sta z:_var_0047                ; $EF7C  85 47
  jsr _func_fa8c                 ; $EF7E  20 8C FA
  slo z:_var_0048_indexed,X      ; $EF81  17 48
  nop                            ; $EF83  EA
  nop                            ; $EF84  EA
  nop                            ; $EF85  EA
  nop                            ; $EF86  EA
  jsr _func_fa91                 ; $EF87  20 91 FA
  lda z:_var_0047                ; $EF8A  A5 47
  cmp #$52                       ; $EF8C  C9 52
  beq _label_ef92                ; $EF8E  F0 02
  sty z:_var_0000_indexed        ; $EF90  84 00

_label_ef92:
  iny                            ; $EF92  C8
  lda #$37                       ; $EF93  A9 37
  sta z:_var_0047                ; $EF95  85 47
  jsr _func_fa9e                 ; $EF97  20 9E FA
  slo z:_var_0048_indexed,X      ; $EF9A  17 48
  nop                            ; $EF9C  EA
  nop                            ; $EF9D  EA
  nop                            ; $EF9E  EA
  nop                            ; $EF9F  EA
  jsr _func_faa4                 ; $EFA0  20 A4 FA
  lda z:_var_0047                ; $EFA3  A5 47
  cmp #$6E                       ; $EFA5  C9 6E
  beq _label_efab                ; $EFA7  F0 02
  sty z:_var_0000_indexed        ; $EFA9  84 00

_label_efab:
  lda #$A5                       ; $EFAB  A9 A5
  sta a:_var_0647                ; $EFAD  8D 47 06
  ldy #$FF                       ; $EFB0  A0 FF
  jsr _func_fa7b                 ; $EFB2  20 7B FA
  slo a:_var_0548_indexed,Y      ; $EFB5  1B 48 05
  nop                            ; $EFB8  EA
  nop                            ; $EFB9  EA
  php                            ; $EFBA  08
  pha                            ; $EFBB  48
  ldy #$CE                       ; $EFBC  A0 CE
  pla                            ; $EFBE  68
  plp                            ; $EFBF  28
  jsr _func_fa81                 ; $EFC0  20 81 FA
  lda a:_var_0647                ; $EFC3  AD 47 06
  cmp #$4A                       ; $EFC6  C9 4A
  beq _label_efcc                ; $EFC8  F0 02
  sty z:_var_0000_indexed        ; $EFCA  84 00

_label_efcc:
  ldy #$FF                       ; $EFCC  A0 FF
  lda #$29                       ; $EFCE  A9 29
  sta a:_var_0647                ; $EFD0  8D 47 06
  jsr _func_fa8c                 ; $EFD3  20 8C FA
  slo a:_var_0548_indexed,Y      ; $EFD6  1B 48 05
  nop                            ; $EFD9  EA
  nop                            ; $EFDA  EA
  php                            ; $EFDB  08
  pha                            ; $EFDC  48
  ldy #$CF                       ; $EFDD  A0 CF
  pla                            ; $EFDF  68
  plp                            ; $EFE0  28
  jsr _func_fa91                 ; $EFE1  20 91 FA
  lda a:_var_0647                ; $EFE4  AD 47 06
  cmp #$52                       ; $EFE7  C9 52
  beq _label_efed                ; $EFE9  F0 02
  sty z:_var_0000_indexed        ; $EFEB  84 00

_label_efed:
  ldy #$FF                       ; $EFED  A0 FF
  lda #$37                       ; $EFEF  A9 37
  sta a:_var_0647                ; $EFF1  8D 47 06
  jsr _func_fa9e                 ; $EFF4  20 9E FA
  slo a:_var_0548_indexed,Y      ; $EFF7  1B 48 05
  nop                            ; $EFFA  EA
  nop                            ; $EFFB  EA
  php                            ; $EFFC  08
  pha                            ; $EFFD  48
  ldy #$D0                       ; $EFFE  A0 D0
  pla                            ; $F000  68
  plp                            ; $F001  28
  jsr _func_faa4                 ; $F002  20 A4 FA
  lda a:_var_0647                ; $F005  AD 47 06
  cmp #$6E                       ; $F008  C9 6E
  beq _label_f00e                ; $F00A  F0 02
  sty z:_var_0000_indexed        ; $F00C  84 00

_label_f00e:
  ldy #$D1                       ; $F00E  A0 D1
  ldx #$FF                       ; $F010  A2 FF
  lda #$A5                       ; $F012  A9 A5
  sta a:_var_0647                ; $F014  8D 47 06
  jsr _func_fa7b                 ; $F017  20 7B FA
  slo a:_var_0548_indexed,X      ; $F01A  1F 48 05
  nop                            ; $F01D  EA
  nop                            ; $F01E  EA
  nop                            ; $F01F  EA
  nop                            ; $F020  EA
  jsr _func_fa81                 ; $F021  20 81 FA
  lda a:_var_0647                ; $F024  AD 47 06
  cmp #$4A                       ; $F027  C9 4A
  beq _label_f02d                ; $F029  F0 02
  sty z:_var_0000_indexed        ; $F02B  84 00

_label_f02d:
  iny                            ; $F02D  C8
  lda #$29                       ; $F02E  A9 29
  sta a:_var_0647                ; $F030  8D 47 06
  jsr _func_fa8c                 ; $F033  20 8C FA
  slo a:_var_0548_indexed,X      ; $F036  1F 48 05
  nop                            ; $F039  EA
  nop                            ; $F03A  EA
  nop                            ; $F03B  EA
  nop                            ; $F03C  EA
  jsr _func_fa91                 ; $F03D  20 91 FA
  lda a:_var_0647                ; $F040  AD 47 06
  cmp #$52                       ; $F043  C9 52
  beq _label_f049                ; $F045  F0 02
  sty z:_var_0000_indexed        ; $F047  84 00

_label_f049:
  iny                            ; $F049  C8
  lda #$37                       ; $F04A  A9 37
  sta a:_var_0647                ; $F04C  8D 47 06
  jsr _func_fa9e                 ; $F04F  20 9E FA
  slo a:_var_0548_indexed,X      ; $F052  1F 48 05
  nop                            ; $F055  EA
  nop                            ; $F056  EA
  nop                            ; $F057  EA
  nop                            ; $F058  EA
  jsr _func_faa4                 ; $F059  20 A4 FA
  lda a:_var_0647                ; $F05C  AD 47 06
  cmp #$6E                       ; $F05F  C9 6E
  beq _label_f065                ; $F061  F0 02
  sty z:_var_0000_indexed        ; $F063  84 00

_label_f065:
  rts                            ; $F065  60

_label_f066:
  lda #$FF                       ; $F066  A9 FF
  sta z:_var_0001                ; $F068  85 01
  ldy #$D4                       ; $F06A  A0 D4
  ldx #$02                       ; $F06C  A2 02
  lda #$47                       ; $F06E  A9 47
  sta z:_var_0047                ; $F070  85 47
  lda #$06                       ; $F072  A9 06
  sta z:_var_0048_indexed        ; $F074  85 48
  lda #$A5                       ; $F076  A9 A5
  sta a:_var_0647                ; $F078  8D 47 06
  jsr _func_fb53                 ; $F07B  20 53 FB
  rla (_var_0045_indexed,X)      ; $F07E  23 45
  nop                            ; $F080  EA
  nop                            ; $F081  EA
  nop                            ; $F082  EA
  nop                            ; $F083  EA
  jsr _func_fb59                 ; $F084  20 59 FB
  lda a:_var_0647                ; $F087  AD 47 06
  cmp #$4A                       ; $F08A  C9 4A
  beq _label_f090                ; $F08C  F0 02
  sty z:_var_0000_indexed        ; $F08E  84 00

_label_f090:
  iny                            ; $F090  C8
  lda #$29                       ; $F091  A9 29
  sta a:_var_0647                ; $F093  8D 47 06
  jsr _func_fb64                 ; $F096  20 64 FB
  rla (_var_0045_indexed,X)      ; $F099  23 45
  nop                            ; $F09B  EA
  nop                            ; $F09C  EA
  nop                            ; $F09D  EA
  nop                            ; $F09E  EA
  jsr _func_fb69                 ; $F09F  20 69 FB
  lda a:_var_0647                ; $F0A2  AD 47 06
  cmp #$52                       ; $F0A5  C9 52
  beq _label_f0ab                ; $F0A7  F0 02
  sty z:_var_0000_indexed        ; $F0A9  84 00

_label_f0ab:
  iny                            ; $F0AB  C8
  lda #$37                       ; $F0AC  A9 37
  sta a:_var_0647                ; $F0AE  8D 47 06
  jsr _func_fa68                 ; $F0B1  20 68 FA
  rla (_var_0045_indexed,X)      ; $F0B4  23 45
  nop                            ; $F0B6  EA
  nop                            ; $F0B7  EA
  nop                            ; $F0B8  EA
  nop                            ; $F0B9  EA
  jsr _func_fa6e                 ; $F0BA  20 6E FA
  lda a:_var_0647                ; $F0BD  AD 47 06
  cmp #$6F                       ; $F0C0  C9 6F
  beq _label_f0c6                ; $F0C2  F0 02
  sty z:_var_0000_indexed        ; $F0C4  84 00

_label_f0c6:
  iny                            ; $F0C6  C8
  lda #$A5                       ; $F0C7  A9 A5
  sta z:_var_0047                ; $F0C9  85 47
  jsr _func_fb53                 ; $F0CB  20 53 FB
  rla z:_var_0047                ; $F0CE  27 47
  nop                            ; $F0D0  EA
  nop                            ; $F0D1  EA
  nop                            ; $F0D2  EA
  nop                            ; $F0D3  EA
  jsr _func_fb59                 ; $F0D4  20 59 FB
  lda z:_var_0047                ; $F0D7  A5 47
  cmp #$4A                       ; $F0D9  C9 4A
  beq _label_f0df                ; $F0DB  F0 02
  sty z:_var_0000_indexed        ; $F0DD  84 00

_label_f0df:
  iny                            ; $F0DF  C8
  lda #$29                       ; $F0E0  A9 29
  sta z:_var_0047                ; $F0E2  85 47
  jsr _func_fb64                 ; $F0E4  20 64 FB
  rla z:_var_0047                ; $F0E7  27 47
  nop                            ; $F0E9  EA
  nop                            ; $F0EA  EA
  nop                            ; $F0EB  EA
  nop                            ; $F0EC  EA
  jsr _func_fb69                 ; $F0ED  20 69 FB
  lda z:_var_0047                ; $F0F0  A5 47
  cmp #$52                       ; $F0F2  C9 52
  beq _label_f0f8                ; $F0F4  F0 02
  sty z:_var_0000_indexed        ; $F0F6  84 00

_label_f0f8:
  iny                            ; $F0F8  C8
  lda #$37                       ; $F0F9  A9 37
  sta z:_var_0047                ; $F0FB  85 47
  jsr _func_fa68                 ; $F0FD  20 68 FA
  rla z:_var_0047                ; $F100  27 47
  nop                            ; $F102  EA
  nop                            ; $F103  EA
  nop                            ; $F104  EA
  nop                            ; $F105  EA
  jsr _func_fa6e                 ; $F106  20 6E FA
  lda z:_var_0047                ; $F109  A5 47
  cmp #$6F                       ; $F10B  C9 6F
  beq _label_f111                ; $F10D  F0 02
  sty z:_var_0000_indexed        ; $F10F  84 00

_label_f111:
  iny                            ; $F111  C8
  lda #$A5                       ; $F112  A9 A5
  sta a:_var_0647                ; $F114  8D 47 06
  jsr _func_fb53                 ; $F117  20 53 FB
  rla a:_var_0647                ; $F11A  2F 47 06
  nop                            ; $F11D  EA
  nop                            ; $F11E  EA
  nop                            ; $F11F  EA
  nop                            ; $F120  EA
  jsr _func_fb59                 ; $F121  20 59 FB
  lda a:_var_0647                ; $F124  AD 47 06
  cmp #$4A                       ; $F127  C9 4A
  beq _label_f12d                ; $F129  F0 02
  sty z:_var_0000_indexed        ; $F12B  84 00

_label_f12d:
  iny                            ; $F12D  C8
  lda #$29                       ; $F12E  A9 29
  sta a:_var_0647                ; $F130  8D 47 06
  jsr _func_fb64                 ; $F133  20 64 FB
  rla a:_var_0647                ; $F136  2F 47 06
  nop                            ; $F139  EA
  nop                            ; $F13A  EA
  nop                            ; $F13B  EA
  nop                            ; $F13C  EA
  jsr _func_fb69                 ; $F13D  20 69 FB
  lda a:_var_0647                ; $F140  AD 47 06
  cmp #$52                       ; $F143  C9 52
  beq _label_f149                ; $F145  F0 02
  sty z:_var_0000_indexed        ; $F147  84 00

_label_f149:
  iny                            ; $F149  C8
  lda #$37                       ; $F14A  A9 37
  sta a:_var_0647                ; $F14C  8D 47 06
  jsr _func_fa68                 ; $F14F  20 68 FA
  rla a:_var_0647                ; $F152  2F 47 06
  nop                            ; $F155  EA
  nop                            ; $F156  EA
  nop                            ; $F157  EA
  nop                            ; $F158  EA
  jsr _func_fa6e                 ; $F159  20 6E FA
  lda a:_var_0647                ; $F15C  AD 47 06
  cmp #$6F                       ; $F15F  C9 6F
  beq _label_f165                ; $F161  F0 02
  sty z:_var_0000_indexed        ; $F163  84 00

_label_f165:
  lda #$A5                       ; $F165  A9 A5
  sta a:_var_0647                ; $F167  8D 47 06
  lda #$48                       ; $F16A  A9 48
  sta z:_var_0045_indexed        ; $F16C  85 45
  lda #$05                       ; $F16E  A9 05
  sta z:_var_0046                ; $F170  85 46
  ldy #$FF                       ; $F172  A0 FF
  jsr _func_fb53                 ; $F174  20 53 FB
  rla (_var_0045_indexed),Y      ; $F177  33 45
  nop                            ; $F179  EA
  nop                            ; $F17A  EA
  php                            ; $F17B  08
  pha                            ; $F17C  48
  ldy #$DD                       ; $F17D  A0 DD
  pla                            ; $F17F  68
  plp                            ; $F180  28
  jsr _func_fb59                 ; $F181  20 59 FB
  lda a:_var_0647                ; $F184  AD 47 06
  cmp #$4A                       ; $F187  C9 4A
  beq _label_f18d                ; $F189  F0 02
  sty z:_var_0000_indexed        ; $F18B  84 00

_label_f18d:
  ldy #$FF                       ; $F18D  A0 FF
  lda #$29                       ; $F18F  A9 29
  sta a:_var_0647                ; $F191  8D 47 06
  jsr _func_fb64                 ; $F194  20 64 FB
  rla (_var_0045_indexed),Y      ; $F197  33 45
  nop                            ; $F199  EA
  nop                            ; $F19A  EA
  php                            ; $F19B  08
  pha                            ; $F19C  48
  ldy #$DE                       ; $F19D  A0 DE
  pla                            ; $F19F  68
  plp                            ; $F1A0  28
  jsr _func_fb69                 ; $F1A1  20 69 FB
  lda a:_var_0647                ; $F1A4  AD 47 06
  cmp #$52                       ; $F1A7  C9 52
  beq _label_f1ad                ; $F1A9  F0 02
  sty z:_var_0000_indexed        ; $F1AB  84 00

_label_f1ad:
  ldy #$FF                       ; $F1AD  A0 FF
  lda #$37                       ; $F1AF  A9 37
  sta a:_var_0647                ; $F1B1  8D 47 06
  jsr _func_fa68                 ; $F1B4  20 68 FA
  rla (_var_0045_indexed),Y      ; $F1B7  33 45
  nop                            ; $F1B9  EA
  nop                            ; $F1BA  EA
  php                            ; $F1BB  08
  pha                            ; $F1BC  48
  ldy #$DF                       ; $F1BD  A0 DF
  pla                            ; $F1BF  68
  plp                            ; $F1C0  28
  jsr _func_fa6e                 ; $F1C1  20 6E FA
  lda a:_var_0647                ; $F1C4  AD 47 06
  cmp #$6F                       ; $F1C7  C9 6F
  beq _label_f1cd                ; $F1C9  F0 02
  sty z:_var_0000_indexed        ; $F1CB  84 00

_label_f1cd:
  ldy #$E0                       ; $F1CD  A0 E0
  ldx #$FF                       ; $F1CF  A2 FF
  lda #$A5                       ; $F1D1  A9 A5
  sta z:_var_0047                ; $F1D3  85 47
  jsr _func_fb53                 ; $F1D5  20 53 FB
  rla z:_var_0048_indexed,X      ; $F1D8  37 48
  nop                            ; $F1DA  EA
  nop                            ; $F1DB  EA
  nop                            ; $F1DC  EA
  nop                            ; $F1DD  EA
  jsr _func_fb59                 ; $F1DE  20 59 FB
  lda z:_var_0047                ; $F1E1  A5 47
  cmp #$4A                       ; $F1E3  C9 4A
  beq _label_f1e9                ; $F1E5  F0 02
  sty z:_var_0000_indexed        ; $F1E7  84 00

_label_f1e9:
  iny                            ; $F1E9  C8
  lda #$29                       ; $F1EA  A9 29
  sta z:_var_0047                ; $F1EC  85 47
  jsr _func_fb64                 ; $F1EE  20 64 FB
  rla z:_var_0048_indexed,X      ; $F1F1  37 48
  nop                            ; $F1F3  EA
  nop                            ; $F1F4  EA
  nop                            ; $F1F5  EA
  nop                            ; $F1F6  EA
  jsr _func_fb69                 ; $F1F7  20 69 FB
  lda z:_var_0047                ; $F1FA  A5 47
  cmp #$52                       ; $F1FC  C9 52
  beq _label_f202                ; $F1FE  F0 02
  sty z:_var_0000_indexed        ; $F200  84 00

_label_f202:
  iny                            ; $F202  C8
  lda #$37                       ; $F203  A9 37
  sta z:_var_0047                ; $F205  85 47
  jsr _func_fa68                 ; $F207  20 68 FA
  rla z:_var_0048_indexed,X      ; $F20A  37 48
  nop                            ; $F20C  EA
  nop                            ; $F20D  EA
  nop                            ; $F20E  EA
  nop                            ; $F20F  EA
  jsr _func_fa6e                 ; $F210  20 6E FA
  lda z:_var_0047                ; $F213  A5 47
  cmp #$6F                       ; $F215  C9 6F
  beq _label_f21b                ; $F217  F0 02
  sty z:_var_0000_indexed        ; $F219  84 00

_label_f21b:
  lda #$A5                       ; $F21B  A9 A5
  sta a:_var_0647                ; $F21D  8D 47 06
  ldy #$FF                       ; $F220  A0 FF
  jsr _func_fb53                 ; $F222  20 53 FB
  rla a:_var_0548_indexed,Y      ; $F225  3B 48 05
  nop                            ; $F228  EA
  nop                            ; $F229  EA
  php                            ; $F22A  08
  pha                            ; $F22B  48
  ldy #$E3                       ; $F22C  A0 E3
  pla                            ; $F22E  68
  plp                            ; $F22F  28
  jsr _func_fb59                 ; $F230  20 59 FB
  lda a:_var_0647                ; $F233  AD 47 06
  cmp #$4A                       ; $F236  C9 4A
  beq _label_f23c                ; $F238  F0 02
  sty z:_var_0000_indexed        ; $F23A  84 00

_label_f23c:
  ldy #$FF                       ; $F23C  A0 FF
  lda #$29                       ; $F23E  A9 29
  sta a:_var_0647                ; $F240  8D 47 06
  jsr _func_fb64                 ; $F243  20 64 FB
  rla a:_var_0548_indexed,Y      ; $F246  3B 48 05
  nop                            ; $F249  EA
  nop                            ; $F24A  EA
  php                            ; $F24B  08
  pha                            ; $F24C  48
  ldy #$E4                       ; $F24D  A0 E4
  pla                            ; $F24F  68
  plp                            ; $F250  28
  jsr _func_fb69                 ; $F251  20 69 FB
  lda a:_var_0647                ; $F254  AD 47 06
  cmp #$52                       ; $F257  C9 52
  beq _label_f25d                ; $F259  F0 02
  sty z:_var_0000_indexed        ; $F25B  84 00

_label_f25d:
  ldy #$FF                       ; $F25D  A0 FF
  lda #$37                       ; $F25F  A9 37
  sta a:_var_0647                ; $F261  8D 47 06
  jsr _func_fa68                 ; $F264  20 68 FA
  rla a:_var_0548_indexed,Y      ; $F267  3B 48 05
  nop                            ; $F26A  EA
  nop                            ; $F26B  EA
  php                            ; $F26C  08
  pha                            ; $F26D  48
  ldy #$E5                       ; $F26E  A0 E5
  pla                            ; $F270  68
  plp                            ; $F271  28
  jsr _func_fa6e                 ; $F272  20 6E FA
  lda a:_var_0647                ; $F275  AD 47 06
  cmp #$6F                       ; $F278  C9 6F
  beq _label_f27e                ; $F27A  F0 02
  sty z:_var_0000_indexed        ; $F27C  84 00

_label_f27e:
  ldy #$E6                       ; $F27E  A0 E6
  ldx #$FF                       ; $F280  A2 FF
  lda #$A5                       ; $F282  A9 A5
  sta a:_var_0647                ; $F284  8D 47 06
  jsr _func_fb53                 ; $F287  20 53 FB
  rla a:_var_0548_indexed,X      ; $F28A  3F 48 05
  nop                            ; $F28D  EA
  nop                            ; $F28E  EA
  nop                            ; $F28F  EA
  nop                            ; $F290  EA
  jsr _func_fb59                 ; $F291  20 59 FB
  lda a:_var_0647                ; $F294  AD 47 06
  cmp #$4A                       ; $F297  C9 4A
  beq _label_f29d                ; $F299  F0 02
  sty z:_var_0000_indexed        ; $F29B  84 00

_label_f29d:
  iny                            ; $F29D  C8
  lda #$29                       ; $F29E  A9 29
  sta a:_var_0647                ; $F2A0  8D 47 06
  jsr _func_fb64                 ; $F2A3  20 64 FB
  rla a:_var_0548_indexed,X      ; $F2A6  3F 48 05
  nop                            ; $F2A9  EA
  nop                            ; $F2AA  EA
  nop                            ; $F2AB  EA
  nop                            ; $F2AC  EA
  jsr _func_fb69                 ; $F2AD  20 69 FB
  lda a:_var_0647                ; $F2B0  AD 47 06
  cmp #$52                       ; $F2B3  C9 52
  beq _label_f2b9                ; $F2B5  F0 02
  sty z:_var_0000_indexed        ; $F2B7  84 00

_label_f2b9:
  iny                            ; $F2B9  C8
  lda #$37                       ; $F2BA  A9 37
  sta a:_var_0647                ; $F2BC  8D 47 06
  jsr _func_fa68                 ; $F2BF  20 68 FA
  rla a:_var_0548_indexed,X      ; $F2C2  3F 48 05
  nop                            ; $F2C5  EA
  nop                            ; $F2C6  EA
  nop                            ; $F2C7  EA
  nop                            ; $F2C8  EA
  jsr _func_fa6e                 ; $F2C9  20 6E FA
  lda a:_var_0647                ; $F2CC  AD 47 06
  cmp #$6F                       ; $F2CF  C9 6F
  beq _label_f2d5                ; $F2D1  F0 02
  sty z:_var_0000_indexed        ; $F2D3  84 00

_label_f2d5:
  rts                            ; $F2D5  60

_label_f2d6:
  lda #$FF                       ; $F2D6  A9 FF
  sta z:_var_0001                ; $F2D8  85 01
  ldy #$E9                       ; $F2DA  A0 E9
  ldx #$02                       ; $F2DC  A2 02
  lda #$47                       ; $F2DE  A9 47
  sta z:_var_0047                ; $F2E0  85 47
  lda #$06                       ; $F2E2  A9 06
  sta z:_var_0048_indexed        ; $F2E4  85 48
  lda #$A5                       ; $F2E6  A9 A5
  sta a:_var_0647                ; $F2E8  8D 47 06
  jsr _func_fb1d                 ; $F2EB  20 1D FB
  sre (_var_0045_indexed,X)      ; $F2EE  43 45
  nop                            ; $F2F0  EA
  nop                            ; $F2F1  EA
  nop                            ; $F2F2  EA
  nop                            ; $F2F3  EA
  jsr _func_fb23                 ; $F2F4  20 23 FB
  lda a:_var_0647                ; $F2F7  AD 47 06
  cmp #$52                       ; $F2FA  C9 52
  beq _label_f300                ; $F2FC  F0 02
  sty z:_var_0000_indexed        ; $F2FE  84 00

_label_f300:
  iny                            ; $F300  C8
  lda #$29                       ; $F301  A9 29
  sta a:_var_0647                ; $F303  8D 47 06
  jsr _func_fb2e                 ; $F306  20 2E FB
  sre (_var_0045_indexed,X)      ; $F309  43 45
  nop                            ; $F30B  EA
  nop                            ; $F30C  EA
  nop                            ; $F30D  EA
  nop                            ; $F30E  EA
  jsr _func_fb33                 ; $F30F  20 33 FB
  lda a:_var_0647                ; $F312  AD 47 06
  cmp #$14                       ; $F315  C9 14
  beq _label_f31b                ; $F317  F0 02
  sty z:_var_0000_indexed        ; $F319  84 00

_label_f31b:
  iny                            ; $F31B  C8
  lda #$37                       ; $F31C  A9 37
  sta a:_var_0647                ; $F31E  8D 47 06
  jsr _func_fb40                 ; $F321  20 40 FB
  sre (_var_0045_indexed,X)      ; $F324  43 45
  nop                            ; $F326  EA
  nop                            ; $F327  EA
  nop                            ; $F328  EA
  nop                            ; $F329  EA
  jsr _func_fb46                 ; $F32A  20 46 FB
  lda a:_var_0647                ; $F32D  AD 47 06
  cmp #$1B                       ; $F330  C9 1B
  beq _label_f336                ; $F332  F0 02
  sty z:_var_0000_indexed        ; $F334  84 00

_label_f336:
  iny                            ; $F336  C8
  lda #$A5                       ; $F337  A9 A5
  sta z:_var_0047                ; $F339  85 47
  jsr _func_fb1d                 ; $F33B  20 1D FB
  sre z:_var_0047                ; $F33E  47 47
  nop                            ; $F340  EA
  nop                            ; $F341  EA
  nop                            ; $F342  EA
  nop                            ; $F343  EA
  jsr _func_fb23                 ; $F344  20 23 FB
  lda z:_var_0047                ; $F347  A5 47
  cmp #$52                       ; $F349  C9 52
  beq _label_f34f                ; $F34B  F0 02
  sty z:_var_0000_indexed        ; $F34D  84 00

_label_f34f:
  iny                            ; $F34F  C8
  lda #$29                       ; $F350  A9 29
  sta z:_var_0047                ; $F352  85 47
  jsr _func_fb2e                 ; $F354  20 2E FB
  sre z:_var_0047                ; $F357  47 47
  nop                            ; $F359  EA
  nop                            ; $F35A  EA
  nop                            ; $F35B  EA
  nop                            ; $F35C  EA
  jsr _func_fb33                 ; $F35D  20 33 FB
  lda z:_var_0047                ; $F360  A5 47
  cmp #$14                       ; $F362  C9 14
  beq _label_f368                ; $F364  F0 02
  sty z:_var_0000_indexed        ; $F366  84 00

_label_f368:
  iny                            ; $F368  C8
  lda #$37                       ; $F369  A9 37
  sta z:_var_0047                ; $F36B  85 47
  jsr _func_fb40                 ; $F36D  20 40 FB
  sre z:_var_0047                ; $F370  47 47
  nop                            ; $F372  EA
  nop                            ; $F373  EA
  nop                            ; $F374  EA
  nop                            ; $F375  EA
  jsr _func_fb46                 ; $F376  20 46 FB
  lda z:_var_0047                ; $F379  A5 47
  cmp #$1B                       ; $F37B  C9 1B
  beq _label_f381                ; $F37D  F0 02
  sty z:_var_0000_indexed        ; $F37F  84 00

_label_f381:
  iny                            ; $F381  C8
  lda #$A5                       ; $F382  A9 A5
  sta a:_var_0647                ; $F384  8D 47 06
  jsr _func_fb1d                 ; $F387  20 1D FB
  sre a:_var_0647                ; $F38A  4F 47 06
  nop                            ; $F38D  EA
  nop                            ; $F38E  EA
  nop                            ; $F38F  EA
  nop                            ; $F390  EA
  jsr _func_fb23                 ; $F391  20 23 FB
  lda a:_var_0647                ; $F394  AD 47 06
  cmp #$52                       ; $F397  C9 52
  beq _label_f39d                ; $F399  F0 02
  sty z:_var_0000_indexed        ; $F39B  84 00

_label_f39d:
  iny                            ; $F39D  C8
  lda #$29                       ; $F39E  A9 29
  sta a:_var_0647                ; $F3A0  8D 47 06
  jsr _func_fb2e                 ; $F3A3  20 2E FB
  sre a:_var_0647                ; $F3A6  4F 47 06
  nop                            ; $F3A9  EA
  nop                            ; $F3AA  EA
  nop                            ; $F3AB  EA
  nop                            ; $F3AC  EA
  jsr _func_fb33                 ; $F3AD  20 33 FB
  lda a:_var_0647                ; $F3B0  AD 47 06
  cmp #$14                       ; $F3B3  C9 14
  beq _label_f3b9                ; $F3B5  F0 02
  sty z:_var_0000_indexed        ; $F3B7  84 00

_label_f3b9:
  iny                            ; $F3B9  C8
  lda #$37                       ; $F3BA  A9 37
  sta a:_var_0647                ; $F3BC  8D 47 06
  jsr _func_fb40                 ; $F3BF  20 40 FB
  sre a:_var_0647                ; $F3C2  4F 47 06
  nop                            ; $F3C5  EA
  nop                            ; $F3C6  EA
  nop                            ; $F3C7  EA
  nop                            ; $F3C8  EA
  jsr _func_fb46                 ; $F3C9  20 46 FB
  lda a:_var_0647                ; $F3CC  AD 47 06
  cmp #$1B                       ; $F3CF  C9 1B
  beq _label_f3d5                ; $F3D1  F0 02
  sty z:_var_0000_indexed        ; $F3D3  84 00

_label_f3d5:
  lda #$A5                       ; $F3D5  A9 A5
  sta a:_var_0647                ; $F3D7  8D 47 06
  lda #$48                       ; $F3DA  A9 48
  sta z:_var_0045_indexed        ; $F3DC  85 45
  lda #$05                       ; $F3DE  A9 05
  sta z:_var_0046                ; $F3E0  85 46
  ldy #$FF                       ; $F3E2  A0 FF
  jsr _func_fb1d                 ; $F3E4  20 1D FB
  sre (_var_0045_indexed),Y      ; $F3E7  53 45
  nop                            ; $F3E9  EA
  nop                            ; $F3EA  EA
  php                            ; $F3EB  08
  pha                            ; $F3EC  48
  ldy #$F2                       ; $F3ED  A0 F2
  pla                            ; $F3EF  68
  plp                            ; $F3F0  28
  jsr _func_fb23                 ; $F3F1  20 23 FB
  lda a:_var_0647                ; $F3F4  AD 47 06
  cmp #$52                       ; $F3F7  C9 52
  beq _label_f3fd                ; $F3F9  F0 02
  sty z:_var_0000_indexed        ; $F3FB  84 00

_label_f3fd:
  ldy #$FF                       ; $F3FD  A0 FF
  lda #$29                       ; $F3FF  A9 29
  sta a:_var_0647                ; $F401  8D 47 06
  jsr _func_fb2e                 ; $F404  20 2E FB
  sre (_var_0045_indexed),Y      ; $F407  53 45
  nop                            ; $F409  EA
  nop                            ; $F40A  EA
  php                            ; $F40B  08
  pha                            ; $F40C  48
  ldy #$F3                       ; $F40D  A0 F3
  pla                            ; $F40F  68
  plp                            ; $F410  28
  jsr _func_fb33                 ; $F411  20 33 FB
  lda a:_var_0647                ; $F414  AD 47 06
  cmp #$14                       ; $F417  C9 14
  beq _label_f41d                ; $F419  F0 02
  sty z:_var_0000_indexed        ; $F41B  84 00

_label_f41d:
  ldy #$FF                       ; $F41D  A0 FF
  lda #$37                       ; $F41F  A9 37
  sta a:_var_0647                ; $F421  8D 47 06
  jsr _func_fb40                 ; $F424  20 40 FB
  sre (_var_0045_indexed),Y      ; $F427  53 45
  nop                            ; $F429  EA
  nop                            ; $F42A  EA
  php                            ; $F42B  08
  pha                            ; $F42C  48
  ldy #$F4                       ; $F42D  A0 F4
  pla                            ; $F42F  68
  plp                            ; $F430  28
  jsr _func_fb46                 ; $F431  20 46 FB
  lda a:_var_0647                ; $F434  AD 47 06
  cmp #$1B                       ; $F437  C9 1B
  beq _label_f43d                ; $F439  F0 02
  sty z:_var_0000_indexed        ; $F43B  84 00

_label_f43d:
  ldy #$F5                       ; $F43D  A0 F5
  ldx #$FF                       ; $F43F  A2 FF
  lda #$A5                       ; $F441  A9 A5
  sta z:_var_0047                ; $F443  85 47
  jsr _func_fb1d                 ; $F445  20 1D FB
  sre z:_var_0048_indexed,X      ; $F448  57 48
  nop                            ; $F44A  EA
  nop                            ; $F44B  EA
  nop                            ; $F44C  EA
  nop                            ; $F44D  EA
  jsr _func_fb23                 ; $F44E  20 23 FB
  lda z:_var_0047                ; $F451  A5 47
  cmp #$52                       ; $F453  C9 52
  beq _label_f459                ; $F455  F0 02
  sty z:_var_0000_indexed        ; $F457  84 00

_label_f459:
  iny                            ; $F459  C8
  lda #$29                       ; $F45A  A9 29
  sta z:_var_0047                ; $F45C  85 47
  jsr _func_fb2e                 ; $F45E  20 2E FB
  sre z:_var_0048_indexed,X      ; $F461  57 48
  nop                            ; $F463  EA
  nop                            ; $F464  EA
  nop                            ; $F465  EA
  nop                            ; $F466  EA
  jsr _func_fb33                 ; $F467  20 33 FB
  lda z:_var_0047                ; $F46A  A5 47
  cmp #$14                       ; $F46C  C9 14
  beq _label_f472                ; $F46E  F0 02
  sty z:_var_0000_indexed        ; $F470  84 00

_label_f472:
  iny                            ; $F472  C8
  lda #$37                       ; $F473  A9 37
  sta z:_var_0047                ; $F475  85 47
  jsr _func_fb40                 ; $F477  20 40 FB
  sre z:_var_0048_indexed,X      ; $F47A  57 48
  nop                            ; $F47C  EA
  nop                            ; $F47D  EA
  nop                            ; $F47E  EA
  nop                            ; $F47F  EA
  jsr _func_fb46                 ; $F480  20 46 FB
  lda z:_var_0047                ; $F483  A5 47
  cmp #$1B                       ; $F485  C9 1B
  beq _label_f48b                ; $F487  F0 02
  sty z:_var_0000_indexed        ; $F489  84 00

_label_f48b:
  lda #$A5                       ; $F48B  A9 A5
  sta a:_var_0647                ; $F48D  8D 47 06
  ldy #$FF                       ; $F490  A0 FF
  jsr _func_fb1d                 ; $F492  20 1D FB
  sre a:_var_0548_indexed,Y      ; $F495  5B 48 05
  nop                            ; $F498  EA
  nop                            ; $F499  EA
  php                            ; $F49A  08
  pha                            ; $F49B  48
  ldy #$F8                       ; $F49C  A0 F8
  pla                            ; $F49E  68
  plp                            ; $F49F  28
  jsr _func_fb23                 ; $F4A0  20 23 FB
  lda a:_var_0647                ; $F4A3  AD 47 06
  cmp #$52                       ; $F4A6  C9 52
  beq _label_f4ac                ; $F4A8  F0 02
  sty z:_var_0000_indexed        ; $F4AA  84 00

_label_f4ac:
  ldy #$FF                       ; $F4AC  A0 FF
  lda #$29                       ; $F4AE  A9 29
  sta a:_var_0647                ; $F4B0  8D 47 06
  jsr _func_fb2e                 ; $F4B3  20 2E FB
  sre a:_var_0548_indexed,Y      ; $F4B6  5B 48 05
  nop                            ; $F4B9  EA
  nop                            ; $F4BA  EA
  php                            ; $F4BB  08
  pha                            ; $F4BC  48
  ldy #$F9                       ; $F4BD  A0 F9
  pla                            ; $F4BF  68
  plp                            ; $F4C0  28
  jsr _func_fb33                 ; $F4C1  20 33 FB
  lda a:_var_0647                ; $F4C4  AD 47 06
  cmp #$14                       ; $F4C7  C9 14
  beq _label_f4cd                ; $F4C9  F0 02
  sty z:_var_0000_indexed        ; $F4CB  84 00

_label_f4cd:
  ldy #$FF                       ; $F4CD  A0 FF
  lda #$37                       ; $F4CF  A9 37
  sta a:_var_0647                ; $F4D1  8D 47 06
  jsr _func_fb40                 ; $F4D4  20 40 FB
  sre a:_var_0548_indexed,Y      ; $F4D7  5B 48 05
  nop                            ; $F4DA  EA
  nop                            ; $F4DB  EA
  php                            ; $F4DC  08
  pha                            ; $F4DD  48
  ldy #$FA                       ; $F4DE  A0 FA
  pla                            ; $F4E0  68
  plp                            ; $F4E1  28
  jsr _func_fb46                 ; $F4E2  20 46 FB
  lda a:_var_0647                ; $F4E5  AD 47 06
  cmp #$1B                       ; $F4E8  C9 1B
  beq _label_f4ee                ; $F4EA  F0 02
  sty z:_var_0000_indexed        ; $F4EC  84 00

_label_f4ee:
  ldy #$FB                       ; $F4EE  A0 FB
  ldx #$FF                       ; $F4F0  A2 FF
  lda #$A5                       ; $F4F2  A9 A5
  sta a:_var_0647                ; $F4F4  8D 47 06
  jsr _func_fb1d                 ; $F4F7  20 1D FB
  sre a:_var_0548_indexed,X      ; $F4FA  5F 48 05
  nop                            ; $F4FD  EA
  nop                            ; $F4FE  EA
  nop                            ; $F4FF  EA
  nop                            ; $F500  EA
  jsr _func_fb23                 ; $F501  20 23 FB
  lda a:_var_0647                ; $F504  AD 47 06
  cmp #$52                       ; $F507  C9 52
  beq _label_f50d                ; $F509  F0 02
  sty z:_var_0000_indexed        ; $F50B  84 00

_label_f50d:
  iny                            ; $F50D  C8
  lda #$29                       ; $F50E  A9 29
  sta a:_var_0647                ; $F510  8D 47 06
  jsr _func_fb2e                 ; $F513  20 2E FB
  sre a:_var_0548_indexed,X      ; $F516  5F 48 05
  nop                            ; $F519  EA
  nop                            ; $F51A  EA
  nop                            ; $F51B  EA
  nop                            ; $F51C  EA
  jsr _func_fb33                 ; $F51D  20 33 FB
  lda a:_var_0647                ; $F520  AD 47 06
  cmp #$14                       ; $F523  C9 14
  beq _label_f529                ; $F525  F0 02
  sty z:_var_0000_indexed        ; $F527  84 00

_label_f529:
  iny                            ; $F529  C8
  lda #$37                       ; $F52A  A9 37
  sta a:_var_0647                ; $F52C  8D 47 06
  jsr _func_fb40                 ; $F52F  20 40 FB
  sre a:_var_0548_indexed,X      ; $F532  5F 48 05
  nop                            ; $F535  EA
  nop                            ; $F536  EA
  nop                            ; $F537  EA
  nop                            ; $F538  EA
  jsr _func_fb46                 ; $F539  20 46 FB
  lda a:_var_0647                ; $F53C  AD 47 06
  cmp #$1B                       ; $F53F  C9 1B
  beq _label_f545                ; $F541  F0 02
  sty z:_var_0000_indexed        ; $F543  84 00

_label_f545:
  rts                            ; $F545  60

_label_f546:
  lda #$FF                       ; $F546  A9 FF
  sta z:_var_0001                ; $F548  85 01
  ldy #$01                       ; $F54A  A0 01
  ldx #$02                       ; $F54C  A2 02
  lda #$47                       ; $F54E  A9 47
  sta z:_var_0047                ; $F550  85 47
  lda #$06                       ; $F552  A9 06
  sta z:_var_0048_indexed        ; $F554  85 48
  lda #$A5                       ; $F556  A9 A5
  sta a:_var_0647                ; $F558  8D 47 06
  jsr _func_fae9                 ; $F55B  20 E9 FA
  rra (_var_0045_indexed,X)      ; $F55E  63 45
  nop                            ; $F560  EA
  nop                            ; $F561  EA
  nop                            ; $F562  EA
  nop                            ; $F563  EA
  jsr _func_faef                 ; $F564  20 EF FA
  lda a:_var_0647                ; $F567  AD 47 06
  cmp #$52                       ; $F56A  C9 52
  beq _label_f570                ; $F56C  F0 02
  sty z:_var_0000_indexed        ; $F56E  84 00

_label_f570:
  iny                            ; $F570  C8
  lda #$29                       ; $F571  A9 29
  sta a:_var_0647                ; $F573  8D 47 06
  jsr _func_fafa                 ; $F576  20 FA FA
  rra (_var_0045_indexed,X)      ; $F579  63 45
  nop                            ; $F57B  EA
  nop                            ; $F57C  EA
  nop                            ; $F57D  EA
  nop                            ; $F57E  EA
  jsr _func_faff                 ; $F57F  20 FF FA
  lda a:_var_0647                ; $F582  AD 47 06
  cmp #$14                       ; $F585  C9 14
  beq _label_f58b                ; $F587  F0 02
  sty z:_var_0000_indexed        ; $F589  84 00

_label_f58b:
  iny                            ; $F58B  C8
  lda #$37                       ; $F58C  A9 37
  sta a:_var_0647                ; $F58E  8D 47 06
  jsr _func_fb0a                 ; $F591  20 0A FB
  rra (_var_0045_indexed,X)      ; $F594  63 45
  nop                            ; $F596  EA
  nop                            ; $F597  EA
  nop                            ; $F598  EA
  nop                            ; $F599  EA
  jsr _func_fb10                 ; $F59A  20 10 FB
  lda a:_var_0647                ; $F59D  AD 47 06
  cmp #$9B                       ; $F5A0  C9 9B
  beq _label_f5a6                ; $F5A2  F0 02
  sty z:_var_0000_indexed        ; $F5A4  84 00

_label_f5a6:
  iny                            ; $F5A6  C8
  lda #$A5                       ; $F5A7  A9 A5
  sta z:_var_0047                ; $F5A9  85 47
  jsr _func_fae9                 ; $F5AB  20 E9 FA
  rra z:_var_0047                ; $F5AE  67 47
  nop                            ; $F5B0  EA
  nop                            ; $F5B1  EA
  nop                            ; $F5B2  EA
  nop                            ; $F5B3  EA
  jsr _func_faef                 ; $F5B4  20 EF FA
  lda z:_var_0047                ; $F5B7  A5 47
  cmp #$52                       ; $F5B9  C9 52
  beq _label_f5bf                ; $F5BB  F0 02
  sty z:_var_0000_indexed        ; $F5BD  84 00

_label_f5bf:
  iny                            ; $F5BF  C8
  lda #$29                       ; $F5C0  A9 29
  sta z:_var_0047                ; $F5C2  85 47
  jsr _func_fafa                 ; $F5C4  20 FA FA
  rra z:_var_0047                ; $F5C7  67 47
  nop                            ; $F5C9  EA
  nop                            ; $F5CA  EA
  nop                            ; $F5CB  EA
  nop                            ; $F5CC  EA
  jsr _func_faff                 ; $F5CD  20 FF FA
  lda z:_var_0047                ; $F5D0  A5 47
  cmp #$14                       ; $F5D2  C9 14
  beq _label_f5d8                ; $F5D4  F0 02
  sty z:_var_0000_indexed        ; $F5D6  84 00

_label_f5d8:
  iny                            ; $F5D8  C8
  lda #$37                       ; $F5D9  A9 37
  sta z:_var_0047                ; $F5DB  85 47
  jsr _func_fb0a                 ; $F5DD  20 0A FB
  rra z:_var_0047                ; $F5E0  67 47
  nop                            ; $F5E2  EA
  nop                            ; $F5E3  EA
  nop                            ; $F5E4  EA
  nop                            ; $F5E5  EA
  jsr _func_fb10                 ; $F5E6  20 10 FB
  lda z:_var_0047                ; $F5E9  A5 47
  cmp #$9B                       ; $F5EB  C9 9B
  beq _label_f5f1                ; $F5ED  F0 02
  sty z:_var_0000_indexed        ; $F5EF  84 00

_label_f5f1:
  iny                            ; $F5F1  C8
  lda #$A5                       ; $F5F2  A9 A5
  sta a:_var_0647                ; $F5F4  8D 47 06
  jsr _func_fae9                 ; $F5F7  20 E9 FA
  rra a:_var_0647                ; $F5FA  6F 47 06
  nop                            ; $F5FD  EA
  nop                            ; $F5FE  EA
  nop                            ; $F5FF  EA
  nop                            ; $F600  EA
  jsr _func_faef                 ; $F601  20 EF FA
  lda a:_var_0647                ; $F604  AD 47 06
  cmp #$52                       ; $F607  C9 52
  beq _label_f60d                ; $F609  F0 02
  sty z:_var_0000_indexed        ; $F60B  84 00

_label_f60d:
  iny                            ; $F60D  C8
  lda #$29                       ; $F60E  A9 29
  sta a:_var_0647                ; $F610  8D 47 06
  jsr _func_fafa                 ; $F613  20 FA FA
  rra a:_var_0647                ; $F616  6F 47 06
  nop                            ; $F619  EA
  nop                            ; $F61A  EA
  nop                            ; $F61B  EA
  nop                            ; $F61C  EA
  jsr _func_faff                 ; $F61D  20 FF FA
  lda a:_var_0647                ; $F620  AD 47 06
  cmp #$14                       ; $F623  C9 14
  beq _label_f629                ; $F625  F0 02
  sty z:_var_0000_indexed        ; $F627  84 00

_label_f629:
  iny                            ; $F629  C8
  lda #$37                       ; $F62A  A9 37
  sta a:_var_0647                ; $F62C  8D 47 06
  jsr _func_fb0a                 ; $F62F  20 0A FB
  rra a:_var_0647                ; $F632  6F 47 06
  nop                            ; $F635  EA
  nop                            ; $F636  EA
  nop                            ; $F637  EA
  nop                            ; $F638  EA
  jsr _func_fb10                 ; $F639  20 10 FB
  lda a:_var_0647                ; $F63C  AD 47 06
  cmp #$9B                       ; $F63F  C9 9B
  beq _label_f645                ; $F641  F0 02
  sty z:_var_0000_indexed        ; $F643  84 00

_label_f645:
  lda #$A5                       ; $F645  A9 A5
  sta a:_var_0647                ; $F647  8D 47 06
  lda #$48                       ; $F64A  A9 48
  sta z:_var_0045_indexed        ; $F64C  85 45
  lda #$05                       ; $F64E  A9 05
  sta z:_var_0046                ; $F650  85 46
  ldy #$FF                       ; $F652  A0 FF
  jsr _func_fae9                 ; $F654  20 E9 FA
  rra (_var_0045_indexed),Y      ; $F657  73 45
  nop                            ; $F659  EA
  nop                            ; $F65A  EA
  php                            ; $F65B  08
  pha                            ; $F65C  48
  ldy #$0A                       ; $F65D  A0 0A
  pla                            ; $F65F  68
  plp                            ; $F660  28
  jsr _func_faef                 ; $F661  20 EF FA
  lda a:_var_0647                ; $F664  AD 47 06
  cmp #$52                       ; $F667  C9 52
  beq _label_f66d                ; $F669  F0 02
  sty z:_var_0000_indexed        ; $F66B  84 00

_label_f66d:
  ldy #$FF                       ; $F66D  A0 FF
  lda #$29                       ; $F66F  A9 29
  sta a:_var_0647                ; $F671  8D 47 06
  jsr _func_fafa                 ; $F674  20 FA FA
  rra (_var_0045_indexed),Y      ; $F677  73 45
  nop                            ; $F679  EA
  nop                            ; $F67A  EA
  php                            ; $F67B  08
  pha                            ; $F67C  48
  ldy #$0B                       ; $F67D  A0 0B
  pla                            ; $F67F  68
  plp                            ; $F680  28
  jsr _func_faff                 ; $F681  20 FF FA
  lda a:_var_0647                ; $F684  AD 47 06
  cmp #$14                       ; $F687  C9 14
  beq _label_f68d                ; $F689  F0 02
  sty z:_var_0000_indexed        ; $F68B  84 00

_label_f68d:
  ldy #$FF                       ; $F68D  A0 FF
  lda #$37                       ; $F68F  A9 37
  sta a:_var_0647                ; $F691  8D 47 06
  jsr _func_fb0a                 ; $F694  20 0A FB
  rra (_var_0045_indexed),Y      ; $F697  73 45
  nop                            ; $F699  EA
  nop                            ; $F69A  EA
  php                            ; $F69B  08
  pha                            ; $F69C  48
  ldy #$0C                       ; $F69D  A0 0C
  pla                            ; $F69F  68
  plp                            ; $F6A0  28
  jsr _func_fb10                 ; $F6A1  20 10 FB
  lda a:_var_0647                ; $F6A4  AD 47 06
  cmp #$9B                       ; $F6A7  C9 9B
  beq _label_f6ad                ; $F6A9  F0 02
  sty z:_var_0000_indexed        ; $F6AB  84 00

_label_f6ad:
  ldy #$0D                       ; $F6AD  A0 0D
  ldx #$FF                       ; $F6AF  A2 FF
  lda #$A5                       ; $F6B1  A9 A5
  sta z:_var_0047                ; $F6B3  85 47
  jsr _func_fae9                 ; $F6B5  20 E9 FA
  rra z:_var_0048_indexed,X      ; $F6B8  77 48
  nop                            ; $F6BA  EA
  nop                            ; $F6BB  EA
  nop                            ; $F6BC  EA
  nop                            ; $F6BD  EA
  jsr _func_faef                 ; $F6BE  20 EF FA
  lda z:_var_0047                ; $F6C1  A5 47
  cmp #$52                       ; $F6C3  C9 52
  beq _label_f6c9                ; $F6C5  F0 02
  sty z:_var_0000_indexed        ; $F6C7  84 00

_label_f6c9:
  iny                            ; $F6C9  C8
  lda #$29                       ; $F6CA  A9 29
  sta z:_var_0047                ; $F6CC  85 47
  jsr _func_fafa                 ; $F6CE  20 FA FA
  rra z:_var_0048_indexed,X      ; $F6D1  77 48
  nop                            ; $F6D3  EA
  nop                            ; $F6D4  EA
  nop                            ; $F6D5  EA
  nop                            ; $F6D6  EA
  jsr _func_faff                 ; $F6D7  20 FF FA
  lda z:_var_0047                ; $F6DA  A5 47
  cmp #$14                       ; $F6DC  C9 14
  beq _label_f6e2                ; $F6DE  F0 02
  sty z:_var_0000_indexed        ; $F6E0  84 00

_label_f6e2:
  iny                            ; $F6E2  C8
  lda #$37                       ; $F6E3  A9 37
  sta z:_var_0047                ; $F6E5  85 47
  jsr _func_fb0a                 ; $F6E7  20 0A FB
  rra z:_var_0048_indexed,X      ; $F6EA  77 48
  nop                            ; $F6EC  EA
  nop                            ; $F6ED  EA
  nop                            ; $F6EE  EA
  nop                            ; $F6EF  EA
  jsr _func_fb10                 ; $F6F0  20 10 FB
  lda z:_var_0047                ; $F6F3  A5 47
  cmp #$9B                       ; $F6F5  C9 9B
  beq _label_f6fb                ; $F6F7  F0 02
  sty z:_var_0000_indexed        ; $F6F9  84 00

_label_f6fb:
  lda #$A5                       ; $F6FB  A9 A5
  sta a:_var_0647                ; $F6FD  8D 47 06
  ldy #$FF                       ; $F700  A0 FF
  jsr _func_fae9                 ; $F702  20 E9 FA
  rra a:_var_0548_indexed,Y      ; $F705  7B 48 05
  nop                            ; $F708  EA
  nop                            ; $F709  EA
  php                            ; $F70A  08
  pha                            ; $F70B  48
  ldy #$10                       ; $F70C  A0 10
  pla                            ; $F70E  68
  plp                            ; $F70F  28
  jsr _func_faef                 ; $F710  20 EF FA
  lda a:_var_0647                ; $F713  AD 47 06
  cmp #$52                       ; $F716  C9 52
  beq _label_f71c                ; $F718  F0 02
  sty z:_var_0000_indexed        ; $F71A  84 00

_label_f71c:
  ldy #$FF                       ; $F71C  A0 FF
  lda #$29                       ; $F71E  A9 29
  sta a:_var_0647                ; $F720  8D 47 06
  jsr _func_fafa                 ; $F723  20 FA FA
  rra a:_var_0548_indexed,Y      ; $F726  7B 48 05
  nop                            ; $F729  EA
  nop                            ; $F72A  EA
  php                            ; $F72B  08
  pha                            ; $F72C  48
  ldy #$11                       ; $F72D  A0 11
  pla                            ; $F72F  68
  plp                            ; $F730  28
  jsr _func_faff                 ; $F731  20 FF FA
  lda a:_var_0647                ; $F734  AD 47 06
  cmp #$14                       ; $F737  C9 14
  beq _label_f73d                ; $F739  F0 02
  sty z:_var_0000_indexed        ; $F73B  84 00

_label_f73d:
  ldy #$FF                       ; $F73D  A0 FF
  lda #$37                       ; $F73F  A9 37
  sta a:_var_0647                ; $F741  8D 47 06
  jsr _func_fb0a                 ; $F744  20 0A FB
  rra a:_var_0548_indexed,Y      ; $F747  7B 48 05
  nop                            ; $F74A  EA
  nop                            ; $F74B  EA
  php                            ; $F74C  08
  pha                            ; $F74D  48
  ldy #$12                       ; $F74E  A0 12
  pla                            ; $F750  68
  plp                            ; $F751  28
  jsr _func_fb10                 ; $F752  20 10 FB
  lda a:_var_0647                ; $F755  AD 47 06
  cmp #$9B                       ; $F758  C9 9B
  beq _label_f75e                ; $F75A  F0 02
  sty z:_var_0000_indexed        ; $F75C  84 00

_label_f75e:
  ldy #$13                       ; $F75E  A0 13
  ldx #$FF                       ; $F760  A2 FF
  lda #$A5                       ; $F762  A9 A5
  sta a:_var_0647                ; $F764  8D 47 06
  jsr _func_fae9                 ; $F767  20 E9 FA
  rra a:_var_0548_indexed,X      ; $F76A  7F 48 05
  nop                            ; $F76D  EA
  nop                            ; $F76E  EA
  nop                            ; $F76F  EA
  nop                            ; $F770  EA
  jsr _func_faef                 ; $F771  20 EF FA
  lda a:_var_0647                ; $F774  AD 47 06
  cmp #$52                       ; $F777  C9 52
  beq _label_f77d                ; $F779  F0 02
  sty z:_var_0000_indexed        ; $F77B  84 00

_label_f77d:
  iny                            ; $F77D  C8
  lda #$29                       ; $F77E  A9 29
  sta a:_var_0647                ; $F780  8D 47 06
  jsr _func_fafa                 ; $F783  20 FA FA
  rra a:_var_0548_indexed,X      ; $F786  7F 48 05
  nop                            ; $F789  EA
  nop                            ; $F78A  EA
  nop                            ; $F78B  EA
  nop                            ; $F78C  EA
  jsr _func_faff                 ; $F78D  20 FF FA
  lda a:_var_0647                ; $F790  AD 47 06
  cmp #$14                       ; $F793  C9 14
  beq _label_f799                ; $F795  F0 02
  sty z:_var_0000_indexed        ; $F797  84 00

_label_f799:
  iny                            ; $F799  C8
  lda #$37                       ; $F79A  A9 37
  sta a:_var_0647                ; $F79C  8D 47 06
  jsr _func_fb0a                 ; $F79F  20 0A FB
  rra a:_var_0548_indexed,X      ; $F7A2  7F 48 05
  nop                            ; $F7A5  EA
  nop                            ; $F7A6  EA
  nop                            ; $F7A7  EA
  nop                            ; $F7A8  EA
  jsr _func_fb10                 ; $F7A9  20 10 FB
  lda a:_var_0647                ; $F7AC  AD 47 06
  cmp #$9B                       ; $F7AF  C9 9B
  beq _label_f7b5                ; $F7B1  F0 02
  sty z:_var_0000_indexed        ; $F7B3  84 00

_label_f7b5:
  rts                            ; $F7B5  60

_func_f7b6:
  clc                            ; $F7B6  18
  lda #$FF                       ; $F7B7  A9 FF
  sta z:_var_0001                ; $F7B9  85 01
  bit z:_var_0001                ; $F7BB  24 01
  lda #$55                       ; $F7BD  A9 55
  rts                            ; $F7BF  60

_func_f7c0:
  bcs _label_f7cb                ; $F7C0  B0 09
  bpl _label_f7cb                ; $F7C2  10 07
  cmp #$FF                       ; $F7C4  C9 FF
  bne _label_f7cb                ; $F7C6  D0 03
  bvc _label_f7cb                ; $F7C8  50 01
  rts                            ; $F7CA  60

_label_f7cb:
  sty z:_var_0000_indexed        ; $F7CB  84 00
  rts                            ; $F7CD  60

_func_f7ce:
  sec                            ; $F7CE  38
  clv                            ; $F7CF  B8
  lda #$00                       ; $F7D0  A9 00
  rts                            ; $F7D2  60

_func_f7d3:
  bne _label_f7dc                ; $F7D3  D0 07
  bvs _label_f7dc                ; $F7D5  70 05
  bcc _label_f7dc                ; $F7D7  90 03
  bmi _label_f7dc                ; $F7D9  30 01
  rts                            ; $F7DB  60

_label_f7dc:
  sty z:_var_0000_indexed        ; $F7DC  84 00
  rts                            ; $F7DE  60

_func_f7df:
  clc                            ; $F7DF  18
  bit z:_var_0001                ; $F7E0  24 01
  lda #$55                       ; $F7E2  A9 55
  rts                            ; $F7E4  60

_func_f7e5:
  bne _label_f7ee                ; $F7E5  D0 07
  bvc _label_f7ee                ; $F7E7  50 05
  bcs _label_f7ee                ; $F7E9  B0 03
  bmi _label_f7ee                ; $F7EB  30 01
  rts                            ; $F7ED  60

_label_f7ee:
  sty z:_var_0000_indexed        ; $F7EE  84 00
  rts                            ; $F7F0  60

_func_f7f1:
  sec                            ; $F7F1  38
  clv                            ; $F7F2  B8
  lda #$F8                       ; $F7F3  A9 F8
  rts                            ; $F7F5  60

_func_f7f6:
  bcc _label_f801                ; $F7F6  90 09
  bpl _label_f801                ; $F7F8  10 07
  cmp #$E8                       ; $F7FA  C9 E8
  bne _label_f801                ; $F7FC  D0 03
  bvs _label_f801                ; $F7FE  70 01
  rts                            ; $F800  60

_label_f801:
  sty z:_var_0000_indexed        ; $F801  84 00
  rts                            ; $F803  60

_func_f804:
  clc                            ; $F804  18
  bit z:_var_0001                ; $F805  24 01
  lda #$5F                       ; $F807  A9 5F
  rts                            ; $F809  60

_func_f80a:
  bcs _label_f815                ; $F80A  B0 09
  bpl _label_f815                ; $F80C  10 07
  cmp #$F5                       ; $F80E  C9 F5
  bne _label_f815                ; $F810  D0 03
  bvc _label_f815                ; $F812  50 01
  rts                            ; $F814  60

_label_f815:
  sty z:_var_0000_indexed        ; $F815  84 00
  rts                            ; $F817  60

_func_f818:
  sec                            ; $F818  38
  clv                            ; $F819  B8
  lda #$70                       ; $F81A  A9 70
  rts                            ; $F81C  60

_func_f81d:
  bne _label_f826                ; $F81D  D0 07
  bvs _label_f826                ; $F81F  70 05
  bcc _label_f826                ; $F821  90 03
  bmi _label_f826                ; $F823  30 01
  rts                            ; $F825  60

_label_f826:
  sty z:_var_0000_indexed        ; $F826  84 00
  rts                            ; $F828  60

_func_f829:
  clc                            ; $F829  18
  bit z:_var_0001                ; $F82A  24 01
  lda #$00                       ; $F82C  A9 00
  rts                            ; $F82E  60

_func_f82f:
  bmi _label_f83a                ; $F82F  30 09
  bcs _label_f83a                ; $F831  B0 07
  cmp #$69                       ; $F833  C9 69
  bne _label_f83a                ; $F835  D0 03
  bvs _label_f83a                ; $F837  70 01
  rts                            ; $F839  60

_label_f83a:
  sty z:_var_0000_indexed        ; $F83A  84 00
  rts                            ; $F83C  60

_func_f83d:
  sec                            ; $F83D  38
  bit z:_var_0001                ; $F83E  24 01
  lda #$00                       ; $F840  A9 00
  rts                            ; $F842  60

_func_f843:
  bmi _label_f84e                ; $F843  30 09
  bcs _label_f84e                ; $F845  B0 07
  cmp #$6A                       ; $F847  C9 6A
  bne _label_f84e                ; $F849  D0 03
  bvs _label_f84e                ; $F84B  70 01
  rts                            ; $F84D  60

_label_f84e:
  sty z:_var_0000_indexed        ; $F84E  84 00
  rts                            ; $F850  60

_func_f851:
  sec                            ; $F851  38
  clv                            ; $F852  B8
  lda #$7F                       ; $F853  A9 7F
  rts                            ; $F855  60

_func_f856:
  bpl _label_f861                ; $F856  10 09
  bcs _label_f861                ; $F858  B0 07
  cmp #$FF                       ; $F85A  C9 FF
  bne _label_f861                ; $F85C  D0 03
  bvc _label_f861                ; $F85E  50 01
  rts                            ; $F860  60

_label_f861:
  sty z:_var_0000_indexed        ; $F861  84 00
  rts                            ; $F863  60

_func_f864:
  clc                            ; $F864  18
  bit z:_var_0001                ; $F865  24 01
  lda #$7F                       ; $F867  A9 7F
  rts                            ; $F869  60

_func_f86a:
  bpl _label_f875                ; $F86A  10 09
  bcs _label_f875                ; $F86C  B0 07
  cmp #$FF                       ; $F86E  C9 FF
  bne _label_f875                ; $F870  D0 03
  bvs _label_f875                ; $F872  70 01
  rts                            ; $F874  60

_label_f875:
  sty z:_var_0000_indexed        ; $F875  84 00
  rts                            ; $F877  60

_func_f878:
  sec                            ; $F878  38
  clv                            ; $F879  B8
  lda #$7F                       ; $F87A  A9 7F
  rts                            ; $F87C  60

_func_f87d:
  bne _label_f886                ; $F87D  D0 07
  bmi _label_f886                ; $F87F  30 05
  bvs _label_f886                ; $F881  70 03
  bcc _label_f886                ; $F883  90 01
  rts                            ; $F885  60

_label_f886:
  sty z:_var_0000_indexed        ; $F886  84 00
  rts                            ; $F888  60

_func_f889:
  bit z:_var_0001                ; $F889  24 01
  lda #$40                       ; $F88B  A9 40
  rts                            ; $F88D  60

_func_f88e:
  bmi _label_f897                ; $F88E  30 07
  bcc _label_f897                ; $F890  90 05
  bne _label_f897                ; $F892  D0 03
  bvc _label_f897                ; $F894  50 01
  rts                            ; $F896  60

_label_f897:
  sty z:_var_0000_indexed        ; $F897  84 00
  rts                            ; $F899  60

_func_f89a:
  clv                            ; $F89A  B8
  rts                            ; $F89B  60

_func_f89c:
  beq _label_f8a5                ; $F89C  F0 07
  bmi _label_f8a5                ; $F89E  30 05
  bcc _label_f8a5                ; $F8A0  90 03
  bvs _label_f8a5                ; $F8A2  70 01
  rts                            ; $F8A4  60

_label_f8a5:
  sty z:_var_0000_indexed        ; $F8A5  84 00
  rts                            ; $F8A7  60

_func_f8a8:
  beq _label_f8af                ; $F8A8  F0 05
  bpl _label_f8af                ; $F8AA  10 03
  bpl _label_f8af                ; $F8AC  10 01
  rts                            ; $F8AE  60

_label_f8af:
  sty z:_var_0000_indexed        ; $F8AF  84 00
  rts                            ; $F8B1  60

_func_f8b2:
  lda #$80                       ; $F8B2  A9 80
  rts                            ; $F8B4  60

_func_f8b5:
  beq _label_f8bc                ; $F8B5  F0 05
  bpl _label_f8bc                ; $F8B7  10 03
  bcc _label_f8bc                ; $F8B9  90 01
  rts                            ; $F8BB  60

_label_f8bc:
  sty z:_var_0000_indexed        ; $F8BC  84 00
  rts                            ; $F8BE  60

_func_f8bf:
  bne _label_f8c6                ; $F8BF  D0 05
  bmi _label_f8c6                ; $F8C1  30 03
  bcc _label_f8c6                ; $F8C3  90 01
  rts                            ; $F8C5  60

_label_f8c6:
  sty z:_var_0000_indexed        ; $F8C6  84 00
  rts                            ; $F8C8  60

_func_f8c9:
  bcs _label_f8d0                ; $F8C9  B0 05
  beq _label_f8d0                ; $F8CB  F0 03
  bpl _label_f8d0                ; $F8CD  10 01
  rts                            ; $F8CF  60

_label_f8d0:
  sty z:_var_0000_indexed        ; $F8D0  84 00
  rts                            ; $F8D2  60

_func_f8d3:
  bcc _label_f8da                ; $F8D3  90 05
  beq _label_f8da                ; $F8D5  F0 03
  bmi _label_f8da                ; $F8D7  30 01
  rts                            ; $F8D9  60

_label_f8da:
  sty z:_var_0000_indexed        ; $F8DA  84 00
  rts                            ; $F8DC  60

_func_f8dd:
  bit z:_var_0001                ; $F8DD  24 01
  ldy #$40                       ; $F8DF  A0 40
  rts                            ; $F8E1  60

_func_f8e2:
  bmi _label_f8eb                ; $F8E2  30 07
  bcc _label_f8eb                ; $F8E4  90 05
  bne _label_f8eb                ; $F8E6  D0 03
  bvc _label_f8eb                ; $F8E8  50 01
  rts                            ; $F8EA  60

_label_f8eb:
  stx z:_var_0000_indexed        ; $F8EB  86 00
  rts                            ; $F8ED  60

_func_f8ee:
  clv                            ; $F8EE  B8
  rts                            ; $F8EF  60

_func_f8f0:
  beq _label_f8f9                ; $F8F0  F0 07
  bmi _label_f8f9                ; $F8F2  30 05
  bcc _label_f8f9                ; $F8F4  90 03
  bvs _label_f8f9                ; $F8F6  70 01
  rts                            ; $F8F8  60

_label_f8f9:
  stx z:_var_0000_indexed        ; $F8F9  86 00
  rts                            ; $F8FB  60

_func_f8fc:
  beq _label_f903                ; $F8FC  F0 05
  bpl _label_f903                ; $F8FE  10 03
  bpl _label_f903                ; $F900  10 01
  rts                            ; $F902  60

_label_f903:
  stx z:_var_0000_indexed        ; $F903  86 00
  rts                            ; $F905  60

_func_f906:
  ldy #$80                       ; $F906  A0 80
  rts                            ; $F908  60

_func_f909:
  beq _label_f910                ; $F909  F0 05
  bpl _label_f910                ; $F90B  10 03
  bcc _label_f910                ; $F90D  90 01
  rts                            ; $F90F  60

_label_f910:
  stx z:_var_0000_indexed        ; $F910  86 00
  rts                            ; $F912  60

_func_f913:
  bne _label_f91a                ; $F913  D0 05
  bmi _label_f91a                ; $F915  30 03
  bcc _label_f91a                ; $F917  90 01
  rts                            ; $F919  60

_label_f91a:
  stx z:_var_0000_indexed        ; $F91A  86 00
  rts                            ; $F91C  60

_func_f91d:
  bcs _label_f924                ; $F91D  B0 05
  beq _label_f924                ; $F91F  F0 03
  bpl _label_f924                ; $F921  10 01
  rts                            ; $F923  60

_label_f924:
  stx z:_var_0000_indexed        ; $F924  86 00
  rts                            ; $F926  60

_func_f927:
  bcc _label_f92e                ; $F927  90 05
  beq _label_f92e                ; $F929  F0 03
  bmi _label_f92e                ; $F92B  30 01
  rts                            ; $F92D  60

_label_f92e:
  stx z:_var_0000_indexed        ; $F92E  86 00
  rts                            ; $F930  60

_func_f931:
  bit z:_var_0001                ; $F931  24 01
  lda #$40                       ; $F933  A9 40
  sec                            ; $F935  38
  rts                            ; $F936  60

_func_f937:
  bmi _label_f944                ; $F937  30 0B
  bcc _label_f944                ; $F939  90 09
  bne _label_f944                ; $F93B  D0 07
  bvs _label_f944                ; $F93D  70 05
  cmp #$00                       ; $F93F  C9 00
  bne _label_f944                ; $F941  D0 01
  rts                            ; $F943  60

_label_f944:
  sty z:_var_0000_indexed        ; $F944  84 00
  rts                            ; $F946  60

_func_f947:
  clv                            ; $F947  B8
  sec                            ; $F948  38
  lda #$40                       ; $F949  A9 40
  rts                            ; $F94B  60

_func_f94c:
  beq _label_f959                ; $F94C  F0 0B
  bmi _label_f959                ; $F94E  30 09
  bcc _label_f959                ; $F950  90 07
  bvs _label_f959                ; $F952  70 05
  cmp #$01                       ; $F954  C9 01
  bne _label_f959                ; $F956  D0 01
  rts                            ; $F958  60

_label_f959:
  sty z:_var_0000_indexed        ; $F959  84 00
  rts                            ; $F95B  60

_func_f95c:
  lda #$40                       ; $F95C  A9 40
  sec                            ; $F95E  38
  bit z:_var_0001                ; $F95F  24 01
  rts                            ; $F961  60

_func_f962:
  bcs _label_f96f                ; $F962  B0 0B
  beq _label_f96f                ; $F964  F0 09
  bpl _label_f96f                ; $F966  10 07
  bvs _label_f96f                ; $F968  70 05
  cmp #$FF                       ; $F96A  C9 FF
  bne _label_f96f                ; $F96C  D0 01
  rts                            ; $F96E  60

_label_f96f:
  sty z:_var_0000_indexed        ; $F96F  84 00
  rts                            ; $F971  60

_func_f972:
  clc                            ; $F972  18
  lda #$80                       ; $F973  A9 80
  rts                            ; $F975  60

_func_f976:
  bcc _label_f97d                ; $F976  90 05
  cmp #$7F                       ; $F978  C9 7F
  bne _label_f97d                ; $F97A  D0 01
  rts                            ; $F97C  60

_label_f97d:
  sty z:_var_0000_indexed        ; $F97D  84 00
  rts                            ; $F97F  60

_func_f980:
  sec                            ; $F980  38
  lda #$81                       ; $F981  A9 81
  rts                            ; $F983  60

_func_f984:
  bvc _label_f98d                ; $F984  50 07
  bcc _label_f98d                ; $F986  90 05
  cmp #$02                       ; $F988  C9 02
  bne _label_f98d                ; $F98A  D0 01
  rts                            ; $F98C  60

_label_f98d:
  sty z:_var_0000_indexed        ; $F98D  84 00
  rts                            ; $F98F  60

_func_f990:
  ldx #$55                       ; $F990  A2 55
  lda #$FF                       ; $F992  A9 FF
  sta z:_var_0001                ; $F994  85 01
  nop                            ; $F996  EA
  bit z:_var_0001                ; $F997  24 01
  sec                            ; $F999  38
  lda #$01                       ; $F99A  A9 01
  rts                            ; $F99C  60

_func_f99d:
  bcc _label_f9ba                ; $F99D  90 1B
  bne _label_f9ba                ; $F99F  D0 19
  bmi _label_f9ba                ; $F9A1  30 17
  bvc _label_f9ba                ; $F9A3  50 15
  cmp #$00                       ; $F9A5  C9 00
  bne _label_f9ba                ; $F9A7  D0 11
  clv                            ; $F9A9  B8
  lda #$AA                       ; $F9AA  A9 AA
  rts                            ; $F9AC  60

_func_f9ad:
  bcs _label_f9ba                ; $F9AD  B0 0B
  beq _label_f9ba                ; $F9AF  F0 09
  bmi _label_f9ba                ; $F9B1  30 07
  bvs _label_f9ba                ; $F9B3  70 05
  cmp #$55                       ; $F9B5  C9 55
  bne _label_f9ba                ; $F9B7  D0 01
  rts                            ; $F9B9  60

_label_f9ba:
  sty z:_var_0000_indexed        ; $F9BA  84 00
  rts                            ; $F9BC  60

_func_f9bd:
  bit z:_var_0001                ; $F9BD  24 01
  sec                            ; $F9BF  38
  lda #$80                       ; $F9C0  A9 80
  rts                            ; $F9C2  60

_func_f9c3:
  bcc _label_f9e1                ; $F9C3  90 1C
  bne _label_f9e1                ; $F9C5  D0 1A
  bmi _label_f9e1                ; $F9C7  30 18
  bvc _label_f9e1                ; $F9C9  50 16
  cmp #$00                       ; $F9CB  C9 00
  bne _label_f9e1                ; $F9CD  D0 12
  clv                            ; $F9CF  B8
  lda #$55                       ; $F9D0  A9 55
  sec                            ; $F9D2  38
  rts                            ; $F9D3  60

_func_f9d4:
  bcs _label_f9e1                ; $F9D4  B0 0B
  beq _label_f9e1                ; $F9D6  F0 09
  bpl _label_f9e1                ; $F9D8  10 07
  bvs _label_f9e1                ; $F9DA  70 05
  cmp #$AA                       ; $F9DC  C9 AA
  bne _label_f9e1                ; $F9DE  D0 01
  rts                            ; $F9E0  60

_label_f9e1:
  sty z:_var_0000_indexed        ; $F9E1  84 00
  rts                            ; $F9E3  60

_func_f9e4:
  bit z:_var_0001                ; $F9E4  24 01
  sec                            ; $F9E6  38
  lda #$01                       ; $F9E7  A9 01
  rts                            ; $F9E9  60

_func_f9ea:
  bcc _label_fa08                ; $F9EA  90 1C
  beq _label_fa08                ; $F9EC  F0 1A
  bpl _label_fa08                ; $F9EE  10 18
  bvc _label_fa08                ; $F9F0  50 16
  cmp #$80                       ; $F9F2  C9 80
  bne _label_fa08                ; $F9F4  D0 12
  clv                            ; $F9F6  B8
  clc                            ; $F9F7  18
  lda #$55                       ; $F9F8  A9 55
  rts                            ; $F9FA  60

_func_f9fb:
  bcc _label_fa08                ; $F9FB  90 0B
  beq _label_fa08                ; $F9FD  F0 09
  bmi _label_fa08                ; $F9FF  30 07
  bvs _label_fa08                ; $FA01  70 05
  cmp #$2A                       ; $FA03  C9 2A
  bne _label_fa08                ; $FA05  D0 01
  rts                            ; $FA07  60

_label_fa08:
  sty z:_var_0000_indexed        ; $FA08  84 00

_func_fa0a:
  bit z:_var_0001                ; $FA0A  24 01
  sec                            ; $FA0C  38
  lda #$80                       ; $FA0D  A9 80
  rts                            ; $FA0F  60

_func_fa10:
  bcc _label_fa2e                ; $FA10  90 1C
  beq _label_fa2e                ; $FA12  F0 1A
  bmi _label_fa2e                ; $FA14  30 18
  bvc _label_fa2e                ; $FA16  50 16
  cmp #$01                       ; $FA18  C9 01
  bne _label_fa2e                ; $FA1A  D0 12
  clv                            ; $FA1C  B8
  clc                            ; $FA1D  18
  lda #$55                       ; $FA1E  A9 55
  rts                            ; $FA20  60

_func_fa21:
  bcs _label_fa2e                ; $FA21  B0 0B
  beq _label_fa2e                ; $FA23  F0 09
  bpl _label_fa2e                ; $FA25  10 07
  bvs _label_fa2e                ; $FA27  70 05
  cmp #$AA                       ; $FA29  C9 AA
  bne _label_fa2e                ; $FA2B  D0 01
  rts                            ; $FA2D  60

_label_fa2e:
  sty z:_var_0000_indexed        ; $FA2E  84 00
  rts                            ; $FA30  60

_func_fa31:
  bit z:_var_0001                ; $FA31  24 01
  clc                            ; $FA33  18
  lda #$40                       ; $FA34  A9 40
  rts                            ; $FA36  60

_func_fa37:
  bvc _label_fa65                ; $FA37  50 2C
  bcs _label_fa65                ; $FA39  B0 2A
  bmi _label_fa65                ; $FA3B  30 28
  cmp #$40                       ; $FA3D  C9 40
  bne _label_fa65                ; $FA3F  D0 24
  rts                            ; $FA41  60

_func_fa42:
  clv                            ; $FA42  B8
  sec                            ; $FA43  38
  lda #$FF                       ; $FA44  A9 FF
  rts                            ; $FA46  60

_func_fa47:
  bvs _label_fa65                ; $FA47  70 1C
  bne _label_fa65                ; $FA49  D0 1A
  bmi _label_fa65                ; $FA4B  30 18
  bcc _label_fa65                ; $FA4D  90 16
  cmp #$FF                       ; $FA4F  C9 FF
  bne _label_fa65                ; $FA51  D0 12
  rts                            ; $FA53  60

_func_fa54:
  bit z:_var_0001                ; $FA54  24 01
  lda #$F0                       ; $FA56  A9 F0
  rts                            ; $FA58  60

_func_fa59:
  bvc _label_fa65                ; $FA59  50 0A
  beq _label_fa65                ; $FA5B  F0 08
  bpl _label_fa65                ; $FA5D  10 06
  bcc _label_fa65                ; $FA5F  90 04
  cmp #$F0                       ; $FA61  C9 F0
  beq _label_fa67                ; $FA63  F0 02

_label_fa65:
  sty z:_var_0000_indexed        ; $FA65  84 00

_label_fa67:
  rts                            ; $FA67  60

_func_fa68:
  bit z:_var_0001                ; $FA68  24 01
  sec                            ; $FA6A  38
  lda #$75                       ; $FA6B  A9 75
  rts                            ; $FA6D  60

_func_fa6e:
  bvc _label_fae6                ; $FA6E  50 76
  beq _label_fae6                ; $FA70  F0 74
  bmi _label_fae6                ; $FA72  30 72
  bcs _label_fae6                ; $FA74  B0 70
  cmp #$65                       ; $FA76  C9 65
  bne _label_fae6                ; $FA78  D0 6C
  rts                            ; $FA7A  60

_func_fa7b:
  bit z:_var_0001                ; $FA7B  24 01
  clc                            ; $FA7D  18
  lda #$B3                       ; $FA7E  A9 B3
  rts                            ; $FA80  60

_func_fa81:
  bvc _label_fae6                ; $FA81  50 63
  bcc _label_fae6                ; $FA83  90 61
  bpl _label_fae6                ; $FA85  10 5F
  cmp #$FB                       ; $FA87  C9 FB
  bne _label_fae6                ; $FA89  D0 5B
  rts                            ; $FA8B  60

_func_fa8c:
  clv                            ; $FA8C  B8
  clc                            ; $FA8D  18
  lda #$C3                       ; $FA8E  A9 C3
  rts                            ; $FA90  60

_func_fa91:
  bvs _label_fae6                ; $FA91  70 53
  beq _label_fae6                ; $FA93  F0 51
  bpl _label_fae6                ; $FA95  10 4F
  bcs _label_fae6                ; $FA97  B0 4D
  cmp #$D3                       ; $FA99  C9 D3
  bne _label_fae6                ; $FA9B  D0 49
  rts                            ; $FA9D  60

_func_fa9e:
  bit z:_var_0001                ; $FA9E  24 01
  sec                            ; $FAA0  38
  lda #$10                       ; $FAA1  A9 10
  rts                            ; $FAA3  60

_func_faa4:
  bvc _label_fae6                ; $FAA4  50 40
  beq _label_fae6                ; $FAA6  F0 3E
  bmi _label_fae6                ; $FAA8  30 3C
  bcs _label_fae6                ; $FAAA  B0 3A
  cmp #$7E                       ; $FAAC  C9 7E
  bne _label_fae6                ; $FAAE  D0 36
  rts                            ; $FAB0  60

_func_fab1:
  bit z:_var_0001                ; $FAB1  24 01
  clc                            ; $FAB3  18
  lda #$40                       ; $FAB4  A9 40
  rts                            ; $FAB6  60

_func_fab7:
  bvs _label_fae6                ; $FAB7  70 2D
  bcs _label_fae6                ; $FAB9  B0 2B
  bmi _label_fae6                ; $FABB  30 29
  cmp #$53                       ; $FABD  C9 53
  bne _label_fae6                ; $FABF  D0 25
  rts                            ; $FAC1  60

_func_fac2:
  clv                            ; $FAC2  B8
  sec                            ; $FAC3  38
  lda #$FF                       ; $FAC4  A9 FF
  rts                            ; $FAC6  60

_func_fac7:
  bvs _label_fae6                ; $FAC7  70 1D
  beq _label_fae6                ; $FAC9  F0 1B
  bpl _label_fae6                ; $FACB  10 19
  bcc _label_fae6                ; $FACD  90 17
  cmp #$FF                       ; $FACF  C9 FF
  bne _label_fae6                ; $FAD1  D0 13
  rts                            ; $FAD3  60

_func_fad4:
  bit z:_var_0001                ; $FAD4  24 01
  sec                            ; $FAD6  38
  lda #$F0                       ; $FAD7  A9 F0
  rts                            ; $FAD9  60

_func_fada:
  bvs _label_fae6                ; $FADA  70 0A
  beq _label_fae6                ; $FADC  F0 08
  bpl _label_fae6                ; $FADE  10 06
  bcc _label_fae6                ; $FAE0  90 04
  cmp #$B8                       ; $FAE2  C9 B8
  beq _label_fae8                ; $FAE4  F0 02

_label_fae6:
  sty z:_var_0000_indexed        ; $FAE6  84 00

_label_fae8:
  rts                            ; $FAE8  60

_func_fae9:
  bit z:_var_0001                ; $FAE9  24 01
  clc                            ; $FAEB  18
  lda #$B2                       ; $FAEC  A9 B2
  rts                            ; $FAEE  60

_func_faef:
  bvs _label_fb1b                ; $FAEF  70 2A
  bcc _label_fb1b                ; $FAF1  90 28
  bmi _label_fb1b                ; $FAF3  30 26
  cmp #$05                       ; $FAF5  C9 05
  bne _label_fb1b                ; $FAF7  D0 22
  rts                            ; $FAF9  60

_func_fafa:
  clv                            ; $FAFA  B8
  clc                            ; $FAFB  18
  lda #$42                       ; $FAFC  A9 42
  rts                            ; $FAFE  60

_func_faff:
  bvs _label_fb1b                ; $FAFF  70 1A
  bmi _label_fb1b                ; $FB01  30 18
  bcs _label_fb1b                ; $FB03  B0 16
  cmp #$57                       ; $FB05  C9 57
  bne _label_fb1b                ; $FB07  D0 12
  rts                            ; $FB09  60

_func_fb0a:
  bit z:_var_0001                ; $FB0A  24 01
  sec                            ; $FB0C  38
  lda #$75                       ; $FB0D  A9 75
  rts                            ; $FB0F  60

_func_fb10:
  bvs _label_fb1b                ; $FB10  70 09
  bmi _label_fb1b                ; $FB12  30 07
  bcc _label_fb1b                ; $FB14  90 05
  cmp #$11                       ; $FB16  C9 11
  bne _label_fb1b                ; $FB18  D0 01
  rts                            ; $FB1A  60

_label_fb1b:
  sta z:_var_0000_indexed        ; $FB1B  85 00

_func_fb1d:
  bit z:_var_0001                ; $FB1D  24 01
  clc                            ; $FB1F  18
  lda #$B3                       ; $FB20  A9 B3
  rts                            ; $FB22  60

_func_fb23:
  bvc _label_fb75                ; $FB23  50 50
  bcc _label_fb75                ; $FB25  90 4E
  bpl _label_fb75                ; $FB27  10 4C
  cmp #$E1                       ; $FB29  C9 E1
  bne _label_fb75                ; $FB2B  D0 48
  rts                            ; $FB2D  60

_func_fb2e:
  clv                            ; $FB2E  B8
  clc                            ; $FB2F  18
  lda #$42                       ; $FB30  A9 42
  rts                            ; $FB32  60

_func_fb33:
  bvs _label_fb75                ; $FB33  70 40
  beq _label_fb75                ; $FB35  F0 3E
  bmi _label_fb75                ; $FB37  30 3C
  bcc _label_fb75                ; $FB39  90 3A
  cmp #$56                       ; $FB3B  C9 56
  bne _label_fb75                ; $FB3D  D0 36
  rts                            ; $FB3F  60

_func_fb40:
  bit z:_var_0001                ; $FB40  24 01
  sec                            ; $FB42  38
  lda #$75                       ; $FB43  A9 75
  rts                            ; $FB45  60

_func_fb46:
  bvc _label_fb75                ; $FB46  50 2D
  beq _label_fb75                ; $FB48  F0 2B
  bmi _label_fb75                ; $FB4A  30 29
  bcc _label_fb75                ; $FB4C  90 27
  cmp #$6E                       ; $FB4E  C9 6E
  bne _label_fb75                ; $FB50  D0 23
  rts                            ; $FB52  60

_func_fb53:
  bit z:_var_0001                ; $FB53  24 01
  clc                            ; $FB55  18
  lda #$B3                       ; $FB56  A9 B3
  rts                            ; $FB58  60

_func_fb59:
  bvc _label_fb75                ; $FB59  50 1A
  bcc _label_fb75                ; $FB5B  90 18
  bmi _label_fb75                ; $FB5D  30 16
  cmp #$02                       ; $FB5F  C9 02
  bne _label_fb75                ; $FB61  D0 12
  rts                            ; $FB63  60

_func_fb64:
  clv                            ; $FB64  B8
  clc                            ; $FB65  18
  lda #$42                       ; $FB66  A9 42
  rts                            ; $FB68  60

_func_fb69:
  bvs _label_fb75                ; $FB69  70 0A
  beq _label_fb75                ; $FB6B  F0 08
  bmi _label_fb75                ; $FB6D  30 06
  bcs _label_fb75                ; $FB6F  B0 04
  cmp #$42                       ; $FB71  C9 42
  beq _label_fb77                ; $FB73  F0 02

_label_fb75:
  sty z:_var_0000_indexed        ; $FB75  84 00

_label_fb77:
  rts                            ; $FB77  60

.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00 ; $FB78
.byte $80, $80, $ff, $80, $80, $00, $00, $00, $00, $00, $ff, $00, $00, $00, $00, $00 ; $FB88
.byte $01, $01, $ff, $01, $01, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00 ; $FB98
.byte $7c, $fe, $00, $c0, $c0, $fe, $7c, $00, $fe, $fe, $00, $f0, $c0, $fe, $fe, $00 ; $FBA8
.byte $c6, $c6, $02, $fe, $c6, $c6, $c6, $00, $cc, $d8, $00, $f0, $d8, $cc, $c6, $00 ; $FBB8
.byte $c6, $ee, $02, $d6, $c6, $c6, $c6, $00, $c6, $c6, $02, $d6, $ce, $c6, $c6, $00 ; $FBC8
.byte $7c, $fe, $02, $c6, $c6, $fe, $7c, $00, $fc, $fe, $02, $fc, $c0, $c0, $c0, $00 ; $FBD8
.byte $cc, $cc, $00, $78, $30, $30, $30, $00, $18, $18, $18, $18, $18, $18, $18, $00 ; $FBE8
.byte $fc, $fe, $02, $06, $1c, $70, $fe, $00, $fc, $fe, $02, $3c, $3c, $02, $fe, $00 ; $FBF8
.byte $18, $18, $d8, $d8, $fe, $18, $18, $00, $fe, $fe, $00, $80, $fc, $06, $fe, $00 ; $FC08
.byte $7c, $fe, $00, $c0, $fc, $c6, $fe, $00, $fe, $fe, $06, $0c, $18, $10, $30, $00 ; $FC18
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00 ; $FC28
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00 ; $FC38
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00 ; $FC48
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00 ; $FC58
.byte $18, $18, $18, $ff, $ff, $18, $18, $18, $18, $18, $18, $ff, $ff, $00, $00, $00 ; $FC68
.byte $00, $00, $00, $00, $00, $00, $00, $00, $18, $18, $18, $18, $00, $18, $18, $00 ; $FC78
.byte $33, $33, $66, $00, $00, $00, $00, $00, $66, $66, $ff, $66, $ff, $66, $66, $00 ; $FC88
.byte $18, $3e, $60, $3c, $06, $7c, $18, $00, $62, $66, $0c, $18, $30, $66, $46, $00 ; $FC98
.byte $3c, $66, $3c, $38, $67, $66, $3f, $00, $0c, $0c, $18, $00, $00, $00, $00, $00 ; $FCA8
.byte $0c, $18, $30, $30, $30, $18, $0c, $00, $30, $18, $0c, $0c, $0c, $18, $30, $00 ; $FCB8
.byte $00, $66, $3c, $ff, $3c, $66, $00, $00, $00, $18, $18, $7e, $18, $18, $00, $00 ; $FCC8
.byte $00, $00, $00, $00, $00, $18, $18, $30, $00, $00, $00, $6e, $3b, $00, $00, $00 ; $FCD8
.byte $00, $00, $00, $00, $00, $18, $18, $00, $00, $03, $06, $0c, $18, $30, $60, $00 ; $FCE8
.byte $3e, $63, $67, $6b, $73, $63, $3e, $00, $0c, $1c, $0c, $0c, $0c, $0c, $3f, $00 ; $FCF8
.byte $3e, $63, $63, $0e, $38, $63, $7f, $00, $3e, $63, $63, $0e, $63, $63, $3e, $00 ; $FD08
.byte $06, $0e, $1e, $26, $7f, $06, $06, $00, $7f, $63, $60, $7e, $03, $63, $3e, $00 ; $FD18
.byte $3e, $63, $60, $7e, $63, $63, $3e, $00, $7f, $63, $06, $0c, $18, $18, $3c, $00 ; $FD28
.byte $3e, $63, $63, $3e, $63, $63, $3e, $00, $3e, $63, $63, $3f, $03, $63, $3e, $00 ; $FD38
.byte $00, $00, $18, $18, $00, $18, $18, $00, $00, $00, $18, $18, $00, $18, $18, $30 ; $FD48
.byte $0e, $18, $30, $60, $30, $18, $0e, $00, $00, $00, $7e, $00, $7e, $00, $00, $00 ; $FD58
.byte $70, $18, $0c, $06, $0c, $18, $70, $00, $7e, $63, $03, $06, $1c, $00, $18, $18 ; $FD68
.byte $7c, $c6, $ce, $ee, $e0, $e6, $7c, $00, $1c, $36, $63, $7f, $63, $63, $63, $00 ; $FD78
.byte $6e, $73, $63, $7e, $63, $63, $7e, $00, $1e, $33, $60, $60, $60, $33, $1e, $00 ; $FD88
.byte $6c, $76, $63, $63, $63, $66, $7c, $00, $7f, $31, $30, $3c, $30, $31, $7f, $00 ; $FD98
.byte $7f, $31, $30, $3c, $30, $30, $78, $00, $1e, $33, $60, $67, $63, $37, $1d, $00 ; $FDA8
.byte $63, $63, $63, $7f, $63, $63, $63, $00, $3c, $18, $18, $18, $18, $18, $3c, $00 ; $FDB8
.byte $1f, $06, $06, $06, $06, $66, $3c, $00, $66, $66, $6c, $78, $6c, $67, $63, $00 ; $FDC8
.byte $78, $30, $60, $60, $63, $63, $7e, $00, $63, $77, $7f, $6b, $63, $63, $63, $00 ; $FDD8
.byte $63, $73, $7b, $6f, $67, $63, $63, $00, $1c, $36, $63, $63, $63, $36, $1c, $00 ; $FDE8
.byte $6e, $73, $63, $7e, $60, $60, $60, $00, $1c, $36, $63, $6b, $67, $36, $1d, $00 ; $FDF8
.byte $6e, $73, $63, $7e, $6c, $67, $63, $00, $3e, $63, $60, $3e, $03, $63, $3e, $00 ; $FE08
.byte $7e, $5a, $18, $18, $18, $18, $3c, $00, $73, $33, $63, $63, $63, $76, $3c, $00 ; $FE18
.byte $73, $33, $63, $63, $66, $3c, $18, $00, $73, $33, $63, $6b, $7f, $77, $63, $00 ; $FE28
.byte $63, $63, $36, $1c, $36, $63, $63, $00, $33, $63, $63, $36, $1c, $78, $70, $00 ; $FE38
.byte $7f, $63, $06, $1c, $33, $63, $7e, $00, $3c, $30, $30, $30, $30, $30, $3c, $00 ; $FE48
.byte $40, $60, $30, $18, $0c, $06, $02, $00, $3c, $0c, $0c, $0c, $0c, $0c, $3c, $00 ; $FE58
.byte $00, $18, $3c, $7e, $18, $18, $18, $18, $00, $00, $00, $00, $00, $00, $ff, $ff ; $FE68
.byte $30, $30, $18, $00, $00, $00, $00, $00, $00, $00, $3f, $63, $63, $67, $3b, $00 ; $FE78
.byte $60, $60, $6e, $73, $63, $63, $3e, $00, $00, $00, $3e, $63, $60, $63, $3e, $00 ; $FE88
.byte $03, $03, $3b, $67, $63, $63, $3e, $00, $00, $00, $3e, $61, $7f, $60, $3e, $00 ; $FE98
.byte $0e, $18, $18, $3c, $18, $18, $3c, $00, $00, $00, $3e, $60, $63, $63, $3d, $00 ; $FEA8
.byte $60, $60, $6e, $73, $63, $66, $67, $00, $00, $00, $1e, $0c, $0c, $0c, $1e, $00 ; $FEB8
.byte $00, $00, $3f, $06, $06, $06, $66, $3c, $60, $60, $66, $6e, $7c, $67, $63, $00 ; $FEC8
.byte $1c, $0c, $0c, $0c, $0c, $0c, $1e, $00, $00, $00, $6e, $7f, $6b, $62, $67, $00 ; $FED8
.byte $00, $00, $6e, $73, $63, $66, $67, $00, $00, $00, $3e, $63, $63, $63, $3e, $00 ; $FEE8
.byte $00, $00, $3e, $63, $73, $6e, $60, $60, $00, $00, $3e, $63, $67, $3b, $03, $03 ; $FEF8
.byte $00, $00, $6e, $73, $63, $7e, $63, $00, $00, $00, $3e, $71, $1c, $47, $3e, $00 ; $FF08
.byte $06, $0c, $3f, $18, $18, $1b, $0e, $00, $00, $00, $73, $33, $63, $67, $3b, $00 ; $FF18
.byte $00, $00, $73, $33, $63, $66, $3c, $00, $00, $00, $63, $6b, $7f, $77, $63, $00 ; $FF28
.byte $00, $00, $63, $36, $1c, $36, $63, $00, $00, $00, $33, $63, $63, $3f, $03, $3e ; $FF38
.byte $00, $00, $7f, $0e, $1c, $38, $7f, $00, $3c, $42, $99, $a1, $a1, $99, $42, $3c ; $FF48
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00 ; $FF58
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00 ; $FF68

_data_ff78_indexed:
.byte $0f, $06, $12, $33, $33, $06, $12, $33, $38, $06, $12, $33, $3a, $06, $12, $33 ; $FF78
.byte $0f, $06, $12, $33, $33, $06, $12, $33, $38, $06, $12, $33, $3a, $06, $12, $33 ; $FF88

.segment "TILES"

.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $80, $80, $ff, $80, $80, $00, $00, $00, $80, $80, $ff, $80, $80, $00, $00, $00
.byte $00, $00, $ff, $00, $00, $00, $00, $00, $00, $00, $ff, $00, $00, $00, $00, $00
.byte $01, $01, $ff, $01, $01, $00, $00, $00, $01, $01, $ff, $01, $01, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $7c, $fe, $00, $c0, $c0, $fe, $7c, $00, $7c, $fe, $00, $c0, $c0, $fe, $7c, $00
.byte $fe, $fe, $00, $f0, $c0, $fe, $fe, $00, $fe, $fe, $00, $f0, $c0, $fe, $fe, $00
.byte $c6, $c6, $02, $fe, $c6, $c6, $c6, $00, $c6, $c6, $02, $fe, $c6, $c6, $c6, $00
.byte $cc, $d8, $00, $f0, $d8, $cc, $c6, $00, $cc, $d8, $00, $f0, $d8, $cc, $c6, $00
.byte $c6, $ee, $02, $d6, $c6, $c6, $c6, $00, $c6, $ee, $02, $d6, $c6, $c6, $c6, $00
.byte $c6, $c6, $02, $d6, $ce, $c6, $c6, $00, $c6, $c6, $02, $d6, $ce, $c6, $c6, $00
.byte $7c, $fe, $02, $c6, $c6, $fe, $7c, $00, $7c, $fe, $02, $c6, $c6, $fe, $7c, $00
.byte $fc, $fe, $02, $fc, $c0, $c0, $c0, $00, $fc, $fe, $02, $fc, $c0, $c0, $c0, $00
.byte $cc, $cc, $00, $78, $30, $30, $30, $00, $cc, $cc, $00, $78, $30, $30, $30, $00
.byte $18, $18, $18, $18, $18, $18, $18, $00, $18, $18, $18, $18, $18, $18, $18, $00
.byte $fc, $fe, $02, $06, $1c, $70, $fe, $00, $fc, $fe, $02, $06, $1c, $70, $fe, $00
.byte $fc, $fe, $02, $3c, $3c, $02, $fe, $00, $fc, $fe, $02, $3c, $3c, $02, $fe, $00
.byte $18, $18, $d8, $d8, $fe, $18, $18, $00, $18, $18, $d8, $d8, $fe, $18, $18, $00
.byte $fe, $fe, $00, $80, $fc, $06, $fe, $00, $fe, $fe, $00, $80, $fc, $06, $fe, $00
.byte $7c, $fe, $00, $c0, $fc, $c6, $fe, $00, $7c, $fe, $00, $c0, $fc, $c6, $fe, $00
.byte $fe, $fe, $06, $0c, $18, $10, $30, $00, $fe, $fe, $06, $0c, $18, $10, $30, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $18, $18, $18, $ff, $ff, $18, $18, $18, $18, $18, $18, $ff, $ff, $18, $18, $18
.byte $18, $18, $18, $ff, $ff, $00, $00, $00, $18, $18, $18, $ff, $ff, $00, $00, $00
.byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
.byte $18, $18, $18, $18, $00, $18, $18, $00, $18, $18, $18, $18, $00, $18, $18, $00
.byte $33, $33, $66, $00, $00, $00, $00, $00, $33, $33, $66, $00, $00, $00, $00, $00
.byte $66, $66, $ff, $66, $ff, $66, $66, $00, $66, $66, $ff, $66, $ff, $66, $66, $00
.byte $18, $3e, $60, $3c, $06, $7c, $18, $00, $18, $3e, $60, $3c, $06, $7c, $18, $00
.byte $62, $66, $0c, $18, $30, $66, $46, $00, $62, $66, $0c, $18, $30, $66, $46, $00
.byte $3c, $66, $3c, $38, $67, $66, $3f, $00, $3c, $66, $3c, $38, $67, $66, $3f, $00
.byte $0c, $0c, $18, $00, $00, $00, $00, $00, $0c, $0c, $18, $00, $00, $00, $00, $00
.byte $0c, $18, $30, $30, $30, $18, $0c, $00, $0c, $18, $30, $30, $30, $18, $0c, $00
.byte $30, $18, $0c, $0c, $0c, $18, $30, $00, $30, $18, $0c, $0c, $0c, $18, $30, $00
.byte $00, $66, $3c, $ff, $3c, $66, $00, $00, $00, $66, $3c, $ff, $3c, $66, $00, $00
.byte $00, $18, $18, $7e, $18, $18, $00, $00, $00, $18, $18, $7e, $18, $18, $00, $00
.byte $00, $00, $00, $00, $00, $18, $18, $30, $00, $00, $00, $00, $00, $18, $18, $30
.byte $00, $00, $00, $6e, $3b, $00, $00, $00, $00, $00, $00, $6e, $3b, $00, $00, $00
.byte $00, $00, $00, $00, $00, $18, $18, $00, $00, $00, $00, $00, $00, $18, $18, $00
.byte $00, $03, $06, $0c, $18, $30, $60, $00, $00, $03, $06, $0c, $18, $30, $60, $00
.byte $3e, $63, $67, $6b, $73, $63, $3e, $00, $3e, $63, $67, $6b, $73, $63, $3e, $00
.byte $0c, $1c, $0c, $0c, $0c, $0c, $3f, $00, $0c, $1c, $0c, $0c, $0c, $0c, $3f, $00
.byte $3e, $63, $63, $0e, $38, $63, $7f, $00, $3e, $63, $63, $0e, $38, $63, $7f, $00
.byte $3e, $63, $63, $0e, $63, $63, $3e, $00, $3e, $63, $63, $0e, $63, $63, $3e, $00
.byte $06, $0e, $1e, $26, $7f, $06, $06, $00, $06, $0e, $1e, $26, $7f, $06, $06, $00
.byte $7f, $63, $60, $7e, $03, $63, $3e, $00, $7f, $63, $60, $7e, $03, $63, $3e, $00
.byte $3e, $63, $60, $7e, $63, $63, $3e, $00, $3e, $63, $60, $7e, $63, $63, $3e, $00
.byte $7f, $63, $06, $0c, $18, $18, $3c, $00, $7f, $63, $06, $0c, $18, $18, $3c, $00
.byte $3e, $63, $63, $3e, $63, $63, $3e, $00, $3e, $63, $63, $3e, $63, $63, $3e, $00
.byte $3e, $63, $63, $3f, $03, $63, $3e, $00, $3e, $63, $63, $3f, $03, $63, $3e, $00
.byte $00, $00, $18, $18, $00, $18, $18, $00, $00, $00, $18, $18, $00, $18, $18, $00
.byte $00, $00, $18, $18, $00, $18, $18, $30, $00, $00, $18, $18, $00, $18, $18, $30
.byte $0e, $18, $30, $60, $30, $18, $0e, $00, $0e, $18, $30, $60, $30, $18, $0e, $00
.byte $00, $00, $7e, $00, $7e, $00, $00, $00, $00, $00, $7e, $00, $7e, $00, $00, $00
.byte $70, $18, $0c, $06, $0c, $18, $70, $00, $70, $18, $0c, $06, $0c, $18, $70, $00
.byte $7e, $63, $03, $06, $1c, $00, $18, $18, $7e, $63, $03, $06, $1c, $00, $18, $18
.byte $7c, $c6, $ce, $ee, $e0, $e6, $7c, $00, $7c, $c6, $ce, $ee, $e0, $e6, $7c, $00
.byte $1c, $36, $63, $7f, $63, $63, $63, $00, $1c, $36, $63, $7f, $63, $63, $63, $00
.byte $6e, $73, $63, $7e, $63, $63, $7e, $00, $6e, $73, $63, $7e, $63, $63, $7e, $00
.byte $1e, $33, $60, $60, $60, $33, $1e, $00, $1e, $33, $60, $60, $60, $33, $1e, $00
.byte $6c, $76, $63, $63, $63, $66, $7c, $00, $6c, $76, $63, $63, $63, $66, $7c, $00
.byte $7f, $31, $30, $3c, $30, $31, $7f, $00, $7f, $31, $30, $3c, $30, $31, $7f, $00
.byte $7f, $31, $30, $3c, $30, $30, $78, $00, $7f, $31, $30, $3c, $30, $30, $78, $00
.byte $1e, $33, $60, $67, $63, $37, $1d, $00, $1e, $33, $60, $67, $63, $37, $1d, $00
.byte $63, $63, $63, $7f, $63, $63, $63, $00, $63, $63, $63, $7f, $63, $63, $63, $00
.byte $3c, $18, $18, $18, $18, $18, $3c, $00, $3c, $18, $18, $18, $18, $18, $3c, $00
.byte $1f, $06, $06, $06, $06, $66, $3c, $00, $1f, $06, $06, $06, $06, $66, $3c, $00
.byte $66, $66, $6c, $78, $6c, $67, $63, $00, $66, $66, $6c, $78, $6c, $67, $63, $00
.byte $78, $30, $60, $60, $63, $63, $7e, $00, $78, $30, $60, $60, $63, $63, $7e, $00
.byte $63, $77, $7f, $6b, $63, $63, $63, $00, $63, $77, $7f, $6b, $63, $63, $63, $00
.byte $63, $73, $7b, $6f, $67, $63, $63, $00, $63, $73, $7b, $6f, $67, $63, $63, $00
.byte $1c, $36, $63, $63, $63, $36, $1c, $00, $1c, $36, $63, $63, $63, $36, $1c, $00
.byte $6e, $73, $63, $7e, $60, $60, $60, $00, $6e, $73, $63, $7e, $60, $60, $60, $00
.byte $1c, $36, $63, $6b, $67, $36, $1d, $00, $1c, $36, $63, $6b, $67, $36, $1d, $00
.byte $6e, $73, $63, $7e, $6c, $67, $63, $00, $6e, $73, $63, $7e, $6c, $67, $63, $00
.byte $3e, $63, $60, $3e, $03, $63, $3e, $00, $3e, $63, $60, $3e, $03, $63, $3e, $00
.byte $7e, $5a, $18, $18, $18, $18, $3c, $00, $7e, $5a, $18, $18, $18, $18, $3c, $00
.byte $73, $33, $63, $63, $63, $76, $3c, $00, $73, $33, $63, $63, $63, $76, $3c, $00
.byte $73, $33, $63, $63, $66, $3c, $18, $00, $73, $33, $63, $63, $66, $3c, $18, $00
.byte $73, $33, $63, $6b, $7f, $77, $63, $00, $73, $33, $63, $6b, $7f, $77, $63, $00
.byte $63, $63, $36, $1c, $36, $63, $63, $00, $63, $63, $36, $1c, $36, $63, $63, $00
.byte $33, $63, $63, $36, $1c, $78, $70, $00, $33, $63, $63, $36, $1c, $78, $70, $00
.byte $7f, $63, $06, $1c, $33, $63, $7e, $00, $7f, $63, $06, $1c, $33, $63, $7e, $00
.byte $3c, $30, $30, $30, $30, $30, $3c, $00, $3c, $30, $30, $30, $30, $30, $3c, $00
.byte $40, $60, $30, $18, $0c, $06, $02, $00, $40, $60, $30, $18, $0c, $06, $02, $00
.byte $3c, $0c, $0c, $0c, $0c, $0c, $3c, $00, $3c, $0c, $0c, $0c, $0c, $0c, $3c, $00
.byte $00, $18, $3c, $7e, $18, $18, $18, $18, $00, $18, $3c, $7e, $18, $18, $18, $18
.byte $00, $00, $00, $00, $00, $00, $ff, $ff, $00, $00, $00, $00, $00, $00, $ff, $ff
.byte $30, $30, $18, $00, $00, $00, $00, $00, $30, $30, $18, $00, $00, $00, $00, $00
.byte $00, $00, $3f, $63, $63, $67, $3b, $00, $00, $00, $3f, $63, $63, $67, $3b, $00
.byte $60, $60, $6e, $73, $63, $63, $3e, $00, $60, $60, $6e, $73, $63, $63, $3e, $00
.byte $00, $00, $3e, $63, $60, $63, $3e, $00, $00, $00, $3e, $63, $60, $63, $3e, $00
.byte $03, $03, $3b, $67, $63, $63, $3e, $00, $03, $03, $3b, $67, $63, $63, $3e, $00
.byte $00, $00, $3e, $61, $7f, $60, $3e, $00, $00, $00, $3e, $61, $7f, $60, $3e, $00
.byte $0e, $18, $18, $3c, $18, $18, $3c, $00, $0e, $18, $18, $3c, $18, $18, $3c, $00
.byte $00, $00, $3e, $60, $63, $63, $3d, $00, $00, $00, $3e, $60, $63, $63, $3d, $00
.byte $60, $60, $6e, $73, $63, $66, $67, $00, $60, $60, $6e, $73, $63, $66, $67, $00
.byte $00, $00, $1e, $0c, $0c, $0c, $1e, $00, $00, $00, $1e, $0c, $0c, $0c, $1e, $00
.byte $00, $00, $3f, $06, $06, $06, $66, $3c, $00, $00, $3f, $06, $06, $06, $66, $3c
.byte $60, $60, $66, $6e, $7c, $67, $63, $00, $60, $60, $66, $6e, $7c, $67, $63, $00
.byte $1c, $0c, $0c, $0c, $0c, $0c, $1e, $00, $1c, $0c, $0c, $0c, $0c, $0c, $1e, $00
.byte $00, $00, $6e, $7f, $6b, $62, $67, $00, $00, $00, $6e, $7f, $6b, $62, $67, $00
.byte $00, $00, $6e, $73, $63, $66, $67, $00, $00, $00, $6e, $73, $63, $66, $67, $00
.byte $00, $00, $3e, $63, $63, $63, $3e, $00, $00, $00, $3e, $63, $63, $63, $3e, $00
.byte $00, $00, $3e, $63, $73, $6e, $60, $60, $00, $00, $3e, $63, $73, $6e, $60, $60
.byte $00, $00, $3e, $63, $67, $3b, $03, $03, $00, $00, $3e, $63, $67, $3b, $03, $03
.byte $00, $00, $6e, $73, $63, $7e, $63, $00, $00, $00, $6e, $73, $63, $7e, $63, $00
.byte $00, $00, $3e, $71, $1c, $47, $3e, $00, $00, $00, $3e, $71, $1c, $47, $3e, $00
.byte $06, $0c, $3f, $18, $18, $1b, $0e, $00, $06, $0c, $3f, $18, $18, $1b, $0e, $00
.byte $00, $00, $73, $33, $63, $67, $3b, $00, $00, $00, $73, $33, $63, $67, $3b, $00
.byte $00, $00, $73, $33, $63, $66, $3c, $00, $00, $00, $73, $33, $63, $66, $3c, $00
.byte $00, $00, $63, $6b, $7f, $77, $63, $00, $00, $00, $63, $6b, $7f, $77, $63, $00
.byte $00, $00, $63, $36, $1c, $36, $63, $00, $00, $00, $63, $36, $1c, $36, $63, $00
.byte $00, $00, $33, $63, $63, $3f, $03, $3e, $00, $00, $33, $63, $63, $3f, $03, $3e
.byte $00, $00, $7f, $0e, $1c, $38, $7f, $00, $00, $00, $7f, $0e, $1c, $38, $7f, $00
.byte $3c, $42, $99, $a1, $a1, $99, $42, $3c, $3c, $42, $99, $a1, $a1, $99, $42, $3c

.segment "VECTORS"

.addr NMI, Reset, IRQ
