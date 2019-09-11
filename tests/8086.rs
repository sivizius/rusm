#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![cfg(any(feature="x86"))]

#[macro_use]
extern crate rusm;

use rusm::
{
  Assembly,
  Endianness,
  expressions::
  {
    Expression,
    ExpressionToken,
  },
  symbols::
  {
    Symbol,
  },
  x86::
  {
    x86,
    memory::
    {
      Memory16Registers,
    },
    operands::
    {
      x86operand,
    },
    registers::
    {
      SegmentRegisterNumber
    },
    state::
    {
      x86version,
    },
  },
  x87::
  {
    x87version,
  },
};

use std::
{
  process::
  {
    Command,
  },
};

#[test]
fn main
(
) ->  Result
      <
        (),
        String
      >
{
  let     myAssembly
  = Assembly
    (
      Endianness::LittleEndian,
      10,
      10,
    )
      .x86config
      (
        x86version::i8086,
        x87version::i8087,
        2,
        2,
      )
      .label    ( "simple math instructions 8 bit".to_string  ( ),                                                                  )
      .x86add   ( x86::cl,                                                      x86::dl,                                            ) //  Register  to  Register
      .x86add   ( x86Mem16! ( byte  [ x86_bp  x86_si  0x80  - + ] ),            x86::dl,                                            ) //  Register  to  Memory
      .x86add   ( x86::dl,                                                      x86Mem16! ( byte  [ x86_bp  x86_si  0x80  - + ] ),  ) //  Memory    to  Register
      .x86add   ( x86::cl,                                                      0x90,                                               ) //  Immediate to  Register
      .x86add   ( x86Mem16! ( byte  [ x86_bp  x86_si  0x80  - + ] ),            0x42,                                               ) //  Immediate to  Memory
      .x86add   ( x86::al,                                                      0x23,                                               ) //  Immediate to  Accumulator
      //.x86add   ( x86::al,                                                      Symbol  ( "simple math instructions 16 bit" ),      ) //  Symbol    to  Accumulator
      .label    ( "simple math instructions 16 bit".to_string ( ),                                                                  )
      .x86lock  (                                                                                                                   ) //  Lock Prefix for Next Instruction
        .x86add ( x86::cx,                                                      x86::dx,                                            ) //  Register  to  Register
      .x86add   ( x86Mem16! ( word  [ x86_bp  x86_si  0x80  - + ] ),            x86::dx,                                            ) //  Register  to  Memory
      .x86add   ( x86::dx,                                                      x86Mem16! ( word  [ x86_bp  x86_si  0x80  - + ] ),  ) //  Memory    to  Register
      .x86add   ( x86::cx,                                                      0x32,                                               ) //  Immediate to  Register  Sign Extended
      .x86add   ( x86::cx,                                                      0x9000,                                             ) //  Immediate to  Register
      .x86add   ( x86Mem16! ( word  [ x86_bp  x86_si  0x80  - + ] ),            0x42,                                               ) //  Immediate to  Memory    Sign Extended
      .x86add   ( x86Mem16! ( word  [ x86_bp  x86_si  0x80  - + ] ),            0x1337,                                             ) //  Immediate to  Memory
      .x86add   ( x86::ax,                                                      0x1337,                                             ) //  Immediate to  Accumulator
      .label    ( "jump instruction".to_string  ( ),                                                                                )
      .x86je    ( Symbol  ( "simple math instructions 16 bit".to_string ( ) ),                                                      ) //  Conditional Jump backward
      .x86jz    ( Symbol  ( "one byte instructions".to_string ( ) ),                                                                ) //  Conditional Jump forward
      .label    ( "one byte instructions".to_string ( ),                                                                            )
      .x86iret  (                                                                                                                   )
      .x86movsw (                                                                                                                   )
      .label    ( "in/out instructions".to_string ( ),                                                                              )
      .x86in    ( x86::ax,                                                      0x13,                                               )
      .x86in    ( x86::al,                                                      0x37,                                               )
      .x86in    ( x86::ax,                                                      x86::dx,                                            )
      .x86in    ( x86::al,                                                      x86::dx,                                            )
      .x86out   ( 0x42,                                                         x86::ax,                                            )
      .x86out   ( 0x23,                                                         x86::al,                                            )
      .x86out   ( x86::dx,                                                      x86::ax,                                            )
      .x86out   ( x86::dx,                                                      x86::al,                                            )
      .compile  (                                                                                                                   );

  myAssembly.toFile   ( "build/8086.bin".to_string ( ), )?;
  myAssembly.hexDump  ( 32, 0,  0,                      )?;

  Command::new  ( "objdump"                         )
    .arg        (   "--disassemble-all"             )
    .arg        (   "--disassembler-options=intel"  )
    .arg        (   "--target=binary"               )
    .arg        (   "--architecture=i8086"          )
    .arg        (   "build/8086.bin"                )
    .status     (                                   )
    .unwrap     (                                   );
  Ok  ( ()  )
}
