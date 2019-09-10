#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![cfg(any(feature="x86"))]

//#[macro_use]
extern crate rusm;

use rusm::
{
  Assembly,
  Endianness,
  x86::
  {
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
        x87version::i387,
        2,
        2,
      )
      .label      ( "zero operand instructions".to_string ( ),  )
      .x87f2xm1   (                                             )
      .x87fabs    (                                             )
      .x87fchs    (                                             )
      .x87fclex   (                                             )
      .x87fcompp  (                                             )
      .x87fdecstp (                                             )
      .x87fdisi   (                                             )
      .x87feni    (                                             )
      .x87fincstp (                                             )
      .x87finit   (                                             )
      .x87fld1    (                                             )
      .x87fldl2e  (                                             )
      .x87fldl2t  (                                             )
      .x87fldlg2  (                                             )
      .x87fldln2  (                                             )
      .x87fldpi   (                                             )
      .x87fldz    (                                             )
      .x87fnclex  (                                             )
      .x87fndisi  (                                             )
      .x87fneni   (                                             )
      .x87fninit  (                                             )
      .x87fnop    (                                             )
      .x87fpatan  (                                             )
      .x87fprem   (                                             )
      .x87fptan   (                                             )
      .x87frndint (                                             )
      .x87fscale  (                                             )
      .x87fsqrt   (                                             )
      .x87ftst    (                                             )
      .x87fwait   (                                             )
      .x87fxam    (                                             )
      .x87fxtract (                                             )
      .x87fyl2x   (                                             )
      .x87fyl2xp1 (                                             )
      .process    (                                             );

  myAssembly.toFile   ( "build/8087.bin".to_string ( ), )?;
  myAssembly.hexDump  ( 32, 0,  0,                      )?;

  Command::new  ( "objdump"                         )
    .arg        (   "--disassemble-all"             )
    .arg        (   "--disassembler-options=intel"  )
    .arg        (   "--target=binary"               )
    .arg        (   "--architecture=i8086"          )
    .arg        (   "build/8087.bin"                )
    .status     (                                   )
    .unwrap     (                                   );
  Ok  ( ()  )
}
