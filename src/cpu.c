#include "../include/cpu.h"
#include "../include/bus.h"
#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>
#include <stdlib.h>

#ifdef CPU_UNIT_TESTS
#include <criterion/criterion.h>
#endif

typedef struct
{
    uint8_t A;
    uint8_t B;
    uint8_t C;
    uint8_t D;
    uint8_t E;
    uint8_t F;
    uint8_t H;
    uint8_t L;
} registers_t;

static registers_t registers;

struct flags
{
    uint8_t Z : 1;
    uint8_t N : 1;
    uint8_t H : 1;
    uint8_t C : 1;
} flag;

static uint64_t total_clock_cycles;
static uint8_t cycles_left;
static uint16_t PC; // program counter
static uint16_t SP; // stack pointer

// register IDs
static uint8_t *r16id[4][2];

// memory R/W
static void read(uint16_t address, uint8_t *byte)
{
    cpuread(address, byte);
}

static void write(uint16_t address, uint8_t byte)
{
    cpuwrite(address, byte);
}

void initializecpu()
{
    memset(&registers, 0, sizeof(registers_t));
    memset(&flag, 0, sizeof(struct flags));
    total_clock_cycles = 0;
    cycles_left = 0;
    SP = 0;
    PC = 0x0;
    // initialize 16 bit reg array

    r16id[0][0] = &registers.B;
    r16id[0][1] = &registers.C;
    r16id[1][0] = &registers.D;
    r16id[1][1] = &registers.E;
    r16id[2][0] = &registers.H;
    r16id[2][1] = &registers.L;
    r16id[3][0] = (uint8_t *)&SP + 1;
    r16id[3][1] = (uint8_t *)&SP;
}


void printcpustate()
{
    printf("A: %x, B: %x, C: %x, D: %x, E: %x, F: %x, H: %x, L: %x\n", registers.A, registers.B, registers.C, registers.D, registers.E, registers.F, registers.H, registers.L);
}
static void combine(uint16_t *output, uint8_t hi, uint8_t lo)
{
    uint16_t hibyte = (uint16_t)hi;
    uint16_t lowbyte = (uint16_t)lo;
    *output = (uint16_t)(hibyte << 8 | lowbyte);
}
static void apart(uint16_t input, uint8_t *hi, uint8_t *lo)
{
    *hi = (uint8_t)(input >> 8);
    *lo = (uint8_t)(input & 0xff);
}

static void ld_r16_imm16(uint8_t registernum)
{
    uint8_t *mhi = r16id[registernum][0];
    uint8_t *mlo = r16id[registernum][1];
    uint8_t lob, hib;
    read(PC++, &lob);
    read(PC++, &hib);
    uint16_t word;
    combine(&word, hib, lob);
    apart(word, mhi, mlo);
}

static void ld_r16mem_a(uint8_t registernum)
{
    uint8_t *mhi = r16id[registernum][0];
    uint8_t *mlo = r16id[registernum][1];
    uint16_t word;
    combine(&word, *mhi, *mlo);
    write(word, registers.A);
}
static void ld_a_r16mem(uint8_t registernum)
{
    uint8_t *mhi = r16id[registernum][0];
    uint8_t *mlo = r16id[registernum][1];
    uint16_t word;
    combine(&word, *mhi, *mlo);
    read(word, &registers.A);
}

static void sptor16()
{
    uint8_t byte;
    read(SP, &byte);
    uint8_t lo, hi;
    read(PC++, &lo);
    read(PC++, &hi);
    uint16_t address;
    combine(&address, hi, lo);
    write(address, byte);
}

static void incr16(uint8_t registernum)
{
    uint8_t *mhi = r16id[registernum][0];
    uint8_t *mlo = r16id[registernum][1];
    uint16_t word;
    combine(&word, *mhi, *mlo);
    word++;
    apart(word, mhi, mlo);
}

static void dec16(uint8_t registernum)
{
    uint8_t *mhi = r16id[registernum][0];
    uint8_t *mlo = r16id[registernum][1];
    uint16_t word;
    combine(&word, *mhi, *mlo);
    word--;
    apart(word, mhi, mlo);
}
static void addhlr16(uint8_t registernum){

}
void clock()
{
    if (cycles_left == 0)
    {
        uint8_t opcode;
        read(PC++, &opcode);
        if ((opcode & 0b11001111) == 00000001)
        {
            ld_r16_imm16((opcode >> 4) & 3);
            cycles_left = 12;
        }
        else if ((opcode & 0b11001111) == 0b00000010)
        {
            ld_r16mem_a((opcode >> 4) & 3);
            cycles_left = 8;
        }
        else if ((opcode & 0b11001111) == 0b00001010)
        {
            ld_a_r16mem((opcode >> 4) & 3);
            cycles_left = 8;
        }
        else if (opcode == 0b00001000)
        {
            sptor16();
            cycles_left = 20;
        }
        else if ((opcode & 0b11001111) == 0b0000011)
        {
            incr16((opcode >> 4) & 3);
            cycles_left = 8;
        }
        else if((opcode & 0b11001111) == 0b00001011)  //0	0	Operand (r16)	1	0	1	1
        {
            dec16((opcode >> 4) & 3);
            cycles_left = 8;
        }
        else if((opcode & 0b11001111) == 0b00001001){ //add hl, r16	0	0	Operand (r16)	1	0	0	1
            addhlr16((opcode >> 4) & 3);
            cycles_left = 8;
        }
    }

    cycles_left--;
    total_clock_cycles++;
}

#ifdef CPU_UNIT_TESTS


Test(cputests, ldr16immtest)
{
    initializecpu();
    initializebus();
    // moving DEAD to BC
    write(0x0000, 0xAD);
    write(0x0001, 0xDE);
    ld_r16_imm16(3);
    freebus();

    cr_expect(SP == 0xDEAD, "ldr16immtest instruction - FAILED!\n");
}
Test(cputests, ldr16memtest)
{
    initializebus();
    initializecpu();
    write(0x0000, 0xBE);
    write(0x0001, 0xEF);
    registers.A = 69;
    registers.B = 0x7F;
    registers.C = 0xFF;
    ld_r16mem_a(0);
    uint8_t result;
    read(0x7FFF, &result);
    freebus();

    cr_expect(result == 69, "ldr16memtest - FAILED\n");
}
Test(cputests, ldra16memtest)
{
    initializebus();
    initializecpu();

    registers.H = 0x7F;
    registers.L = 0xFF;
    write(0x7FFF, 95);
    ld_a_r16mem(2);
    freebus();

    cr_expect(registers.A == 95, "ldra16memtest - FAILED\n");
}

Test(cputests, sptor16memtest)
{
    initializebus();
    initializecpu();
    SP = 0x7FFF;
    write(0x7FFF, 69);
    write(0x0000, 0x08);
    write(0x0001, 0x00);
    write(0x0002, 0x10);
    clock();
    uint8_t result;
    read(0x1000, &result);
    freebus();

    cr_expect(result == 69, "sptor16memtest - FAILED!\n");
}

Test(cputests, incr16test)
{
    initializebus();
    initializecpu();
    write(0x0000, 0x03);
    uint16_t word;
    registers.B = 0x00;
    registers.C = 0xFF;
    clock();
    combine(&word, registers.B, registers.C);

    freebus();
    cr_expect(word == 0x0100, "incr16test - FAILED\n");
}

Test(cputests, dec16test)
{
    initializebus();
    initializecpu();
    write(0x0000, 0x0B);
    uint16_t word;
    registers.B = 0x00;
    registers.C = 0xFF;
    clock();
    combine(&word, registers.B, registers.C);
    freebus();
    cr_expect(word == 0x00FE, "dec16test - FAILED\n");
}
#endif

//                                                  :~:
//                                                :5###5:
//                               ~??!.            J@&&&@B.
//                             .5&&@@B^        :~!5&&&&&B?^^:.         :::.
//                             :#@@@@@Y7JY5YJ7YPB##B#&&&#BBGPY?77!^..7G#BBG:
//                             :5&&@@&&&###BBBBB#########BBBGP5Y5PGPP#&&&&#~
//                          ^7YB#BB&&&&&###############BBBBBBBBBBBBBB#&#BB7
//                 .!!!~:^7YGGGPPG###################BBBBBBBBBBBBBBBBBBBPP5J~.
//                 5@&@@&B##GPGB#####################BBBBBBBBBBBBBBBBBBBBGYJJJ7.
//                 P@@@@@&#B###################BBBBBBBBBBBBBBBBBBBGGGBBBBBGPJ?5P!^7JJ?!.
//                 J&&&@@@&##################BBBBGBBBBBBBBBBBPGBBBBGGGGGGGGGG55PGB&&#B#?
//               ^P##&#&&&#############BB####BBBBGBBBBBBBBBBBPPBBBBBGGGGGGGPPGPPB#B##BY:
//              7#&####B########B######BB###BBBBGBBBBBBBBBBBBGGPGGGGGGPPPPPPGGGPP##GB7
//             J&&#############B######BB###BBBBBGBBBBBBBBBBBBG#PPPPPP55YY5PGGGGGPPGPBP7!~:
//           :5&&#####B#######BB####BBBB#BBBBBBGGBBBBBBBBBBBGG##GGGGP55P5PPP555PP5P5P5J..7~
//          :G&#######B#######GB#B###BGBBBBBBBBGGGBBBBBGGGGGGG#&#GP5PPGGPPGGPPPPPPJYYYJ^ ~!
//         .5&#######BB######BB#B####BGBBBBBBBBGGG#BBGGGGGGGGG#&&#GPGGGGGPPGGGGPP5YY5J?7.!!!?~
//         J##B######B######BGB#BBBBBGBBBBBBBBGPGB#BGGGGGGGGGPGBBBBGPGGGGPPPPGGPP5JJP5J?!77^:7.
//        !###B######B######BGB#BBBBBGBBBBBBBBGPGGBGGGGGGGGGGG#&####BPGGGP5PPGGPP5JJYYY??7.  7:
//       :G###B#####BB###B#BBB##BBBBBGBBBBBBBBGPPGGGGGGGGGGPPG#&&&&&&#GPGPP5PPPPPP5YYJJ7777^^?.
//       7####B#####BG#&&#BBGBBBBBBBBGBBBBBBBBGPPGBGGGGGGGGGPG#&&&###&#GPPP5PPPPPP5YJYY?7!7?7^
//       J####BB####BB#&&#BBGBBBBBBBBGBBBBBBBBGPGGBBGGGGGGGGPG#&########BPP5PPPPPP5JY5J?7!.!!
//       5####BB###B#GB##BBBGBBBBBBBBGBBBBBBBBBGGGBBBBBBBBBGPG&##&#&#####BPYPPPPPP5YJYJ?77: !!
//      .G####BB#BBBBBGBBBBBGBBBBBBBBGBBBBBBBBBGGGBBBBBBBBBGPB###&#G##&&###PPPPPPPP5JJJY777~!:
//      .P##BBBGBBBBBBGBBBBBBBBBBBBBGGG#BBBBBBBGBGBBBBBBBBBGPBBG5YJ?JJY5G##PPPPPPPPY5555?7!!
//       5#BBB#BGBBBBBGGBBBBBG######BGG########BBGBBBBBBBBBGGBGYYPGBGGP5YYPP5PPPPP5Y5555J7.!.
//       ?#BBBBBBBBBBBGGBBBB#GG######GGB#######BGBGBBBBBBBBGG###&&&######BBP5PPPPP5Y5555Y7.~^
//       ^BBBBBBBGBGBBBGGB####G######BGGB#######GBGBBBBBBBBGG##########BBBGP5PPP5YY55555Y?::~
//        YBBBBBBBGGB##BBBB####G######GBB##############################BBBPPPGGP5555555YY?^ !.
//        ^GBBBBBBBGGB##BBBB###BB###&&&&&&&&&&&&&&&&#####################BPPGGGP5GB555YJJ?^ !:
//         JGGBBBBBBGGB###BGB####&&&&&&&&&&&&&&&&&&######################GPPGGP55GB5YYJY?7: ~^
//         .5YPBBBBBBGGGB##BGB&#&&&&&&&&&&&&&&&&&#######################BGGBBGYY5G5YYJJJ~7^ ^~
//          ^Y!5BBBGGBBBGBB###&&&&&&&&&&&&&&&&&&&##########################BBP5PPJJYYJY^ !^ :~
//           ^Y7?GBBGGGBBGGB&&&&&&&&&&&&&&&&&&&&#######################BB#BBBPGY7?YJJJ^ .7: :~
//            :J7^?PBBGGGGPP#&&&&&&&&&&&&&&&&&&##########BB############BBBBBJ?7!?YJJ?:  .7: :~
//              ^7!~7YPGGPPYYG&&&&&&&&&&&&&&&##########################BB#GJ!!7JYJJ!.   ^7. :~
//                ^~!~!?JJJ5YJYG&&&&&&&&#&##########################BB##B5?!7?YYJ!:     ~!  :~
//    .!7JYYJ7^      :^~~!!7YJ??YG#&&&&##############################BG5?!7?YJ?~:       !~  ^~
//    !BGB#&@@&BJ^          .?7:^!?YPB############################BG5J77~~!!^:         .7:  ~^
//    !GGGGG#&&&@&5^         ..   .YYJYPGBB###################BGPY?7^ .^^::.           .^   ^.
//    !GGGGGP#&&&&&#PP?:          .G#GP5555PPGGGBBBB####BBGPY?7!^^:  ^JPGBGGJ:
//    !GPPPGGG&&&&&&##&#Y^:...^~?YPG##GGBP555555555555P55J?~:     :^JBBBBBBBB5:
//    !GPPB#&B#&#&#&&#B#&BGBGB#&&&&#B#####B555555555555YGGG55J!^^?GBGGBBBBBGGBJ
//    !PPB&##BB######BB###BB&#########B####G55GBG555555PPGBG###BPPGBGPBBBBGGGG5.
//    ~G#####BB#######B###BGB#########B#####GB####P55GB#GGGBBBBBBP5GPPGGBGGGGG5.
//    ~#####BGB#######BGB##GGBBBBBBBBBBBBBBBBBBBBBGYGBBBGPGBBBBBBG55GGPPGGGGGGJ
//    .^^^^^::^^^^^^^^^::^^:::^:::::::::::::::::::::::::::::::::::::::::::::::.