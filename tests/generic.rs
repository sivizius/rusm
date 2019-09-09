#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate rusm;

use rusm::
{
  Assembly,
  Endianness,
  asm::
  {
    asm,
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
      .label        ( "generic assembly instruction".to_string  ( ),              )
      .db           ( vec!  ( 0x13, 0x37, 0x23, 0x42, 0x19, 0x96, 0x10, 0x03, ),  )
      .append
      (
        vec!
        (
          asm::rb   ( 16,                                                         ),
          asm::db   ( vec!  ( 'H',  'E',  'L',  'L',  'O',  '!',  ),              ),
          asm::rb   ( 16,                                                         ),
          asm::utf8 ( "Hello World".to_string ( )                                 ),
          asm::rb   ( 16,                                                         ),
        )
      )
      .process      (                                                             );

  myAssembly.toFile   ( "build/generic.bin".to_string ( ),  )?;
  myAssembly.hexDump  ( 32, 0,  0,                          )?;
  Ok  ( ()  )
}
