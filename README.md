rusm
====

***rusm*** is an assembler written in Rust.
It cannot parse a text (yet), but assembler-instructions can be addded to an `Assembly`-object, which can be compiled.
This is useful to
  dynamical create assembly,
  use Rust to calculate things and
  can be even used as as backend of a compiler for highlevel languages,
    but the available instructions and instruction sets are still limited.

Instruction Sets
----------------

- [ ] asm – generic instructions like `label`, `db`, etc.
- [ ] x86 – Intel x86 and AMD amd64
  - [ ] 8086  – working on
  - [ ] 80186 – …
  - [ ] 80286 – …
  - [ ] 80386 – …
  - [ ] 80486 – …
- [x] x87 – Instructions for the Floating Point Unit


Participation
-------------

I could use some help with

1.  implementing new instruction sets,
2.  correcting spelling, grammar, typography, ….

Please Note: There are some [conventions](conventions.md) to have at least some kind of rules to follow.