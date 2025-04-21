# APU Notes

## What is the APU?

- The APU is the **Audio Processing Unit**, which generates sound for games.
- It's built into the CPU, and is mapped in the ranges of $4000-$4013, $4015 and $4017

## APU Channels

- The APU Consists of 5 channels:
    - `$4000 - $4003` controls the first pulse wave.
    - `$4004 - $4007` controls the second pulse wave.
    - `$4008 - $400B` controls the triangle wave.
    - `$400C - $400F` controls the noise channel.
    - There's also the DMC channel for sample playback.

## Pulse Wave channels

- The first two channels are pulse waves.

- The NES Pulse wave channel generates a square wave.

- You can control the Pulse wave chanel with the following registers:

```
$ 4000 and $4004: $DD11VVVV
```
- DD is the duty cycle:
    - DD = 00 represents 12.5%
    - DD = 01 represents 25%
    - DD = 10 represents 50%
    - DD = 11 represents 75%
- The Duty Cycle controls the timber.
- VVVV is the volume

```
$4002 and $4006: $LLLL LLLL
```
- This represents the low 8 bits of the period

```
$4003 and $4007: ---- -HHH
```
- This represents the high bit of the raw period.

- The period controls the pitch of the note


## Triange Wave Channel

- the triangle wave channel is controlled with the following registers:

```
$4008: $1U------
```
- When you set the U bit to 1 or 0, it will enable the "linear counter", which lets the triangle wave play if the counter is not zero.

```
$ 4017: $10000000
```

- this will immediately apply the Linear counter reload, unmuting the channel, instead of waiting for the next frame tick.


## Noise channel

- You can control the volume, period and "tone mode"

```
$ 400C: $--11VVVV
```
- VVVV is the volume

```
$400E: $T---PPPP
```
- T is the tone mode.
- P is the period.
