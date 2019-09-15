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
  asm::
  {
    asm,
    asmKind,
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
    x87,
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
        x87version::Pentium,
        2,
        2,
      )
      .label        ( "zero operand instructions".to_string ( ),                                                  )
      .x87f2xm1     (                                                                                             )
      .x87fabs      (                                                                                             )
      .x87fchs      (                                                                                             )
      .x87fclex     (                                                                                             )
      .x87fcompp    (                                                                                             )
      .x87fdecstp   (                                                                                             )
      .x87fdisi     (                                                                                             )
      .x87feni      (                                                                                             )
      .x87fincstp   (                                                                                             )
      .x87finit     (                                                                                             )
      .x87fld1      (                                                                                             )
      .x87fldl2e    (                                                                                             )
      .x87fldl2t    (                                                                                             )
      .x87fldlg2    (                                                                                             )
      .x87fldln2    (                                                                                             )
      .x87fldpi     (                                                                                             )
      .x87fldz      (                                                                                             )
      .x87fnclex    (                                                                                             )
      .x87fndisi    (                                                                                             )
      .x87fneni     (                                                                                             )
      .x87fninit    (                                                                                             )
      .x87fnop      (                                                                                             )
      .x87fpatan    (                                                                                             )
      .x87fprem     (                                                                                             )
      .x87fptan     (                                                                                             )
      .x87frndint   (                                                                                             )
      .x87fscale    (                                                                                             )
      .x87fsqrt     (                                                                                             )
      .x87ftst      (                                                                                             )
      .x87fwait     (                                                                                             )
      .x87fxam      (                                                                                             )
      .x87fxtract   (                                                                                             )
      .x87fyl2x     (                                                                                             )
      .x87fyl2xp1   (                                                                                             )
      .x87fnsetpm   (                                                                                             )
      .label        ( "load instructions".to_string ( ),                                                          )
      .x87fbld      ( x86Mem16! ( bcd     [ x86_bp  x86_si  + ] ),                                                )
      .x87fild      ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),                                                )
      .x87fild      ( x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),                                                )
      .x87fild      ( x86Mem16! ( qword   [ x86_bp  x86_si  + ] ),                                                )
      .x87fld       ( x87::st ( 1 )                                                                               )
      .x87fld       ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),                                                )
      .x87fld       ( x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),                                                )
      .x87fld       ( x86Mem16! ( qword   [ x86_bp  x86_si  + ] ),                                                )
      .x87fld       ( x86Mem16! ( float32 [ x86_bp  x86_si  + ] ),                                                )
      .x87fld       ( x86Mem16! ( float64 [ x86_bp  x86_si  + ] ),                                                )
      .x87fld       ( x86Mem16! ( float80 [ x86_bp  x86_si  + ] ),                                                )
      .x87fld       ( x86Mem16! ( bcd     [ x86_bp  x86_si  + ] ),                                                )
      .label        ( "simple instructions".to_string ( ),                                                        )
      .x87fldcw     ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),                                                )
      .x87fldenv    ( x86Mem16! ( ptr     [ x86_bp  x86_si  + ] ),                                                )
      .x87fnsave    ( x86Mem16! ( ptr     [ x86_bp  x86_si  + ] ),                                                )
      .x87fnstcw    ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),                                                )
      .x87fnstenv   ( x86Mem16! ( ptr     [ x86_bp  x86_si  + ] ),                                                )
      .x87fnstsw    ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),                                                )
      .x87fnstsw    ( x86::ax,                                                                                    )
      .x87frstor    ( x86Mem16! ( ptr     [ x86_bp  x86_si  + ] ),                                                )
      .x87fsave     ( x86Mem16! ( ptr     [ x86_bp  x86_si  + ] ),                                                )
      .x87fstcw     ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),                                                )
      .x87fstenv    ( x86Mem16! ( ptr     [ x86_bp  x86_si  + ] ),                                                )
      .x87fstsw     ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),                                                )
      .x87fstsw     ( x86::ax,                                                                                    )
      .label        ( "›math‹ instructions".to_string ( ),                                                        )
      .x87fadd      ( x87::st ( 0 ),                                x87::st ( 4 ),                                )
      .x87fadd      ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fadd      ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fadd      ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fadd      ( x87::st ( 0 ),                                x86Mem16! ( single  [ x86_bp  x86_si  + ] ),  )
      .x87fadd      ( x87::st ( 0 ),                                x86Mem16! ( double  [ x86_bp  x86_si  + ] ),  )
      .x87faddp     ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fbstp     ( x86Mem16! ( bcd     [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fcom      ( x87::st ( 0 ),                                x87::st ( 4 ),                                )
      .x87fcom      ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fcom      ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fcom      ( x87::st ( 0 ),                                x86Mem16! ( single  [ x86_bp  x86_si  + ] ),  )
      .x87fcom      ( x87::st ( 0 ),                                x86Mem16! ( double  [ x86_bp  x86_si  + ] ),  )
      .x87fcomp     ( x87::st ( 0 ),                                x87::st ( 4 ),                                )
      .x87fcomp     ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fcomp     ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fcomp     ( x87::st ( 0 ),                                x86Mem16! ( single  [ x86_bp  x86_si  + ] ),  )
      .x87fcomp     ( x87::st ( 0 ),                                x86Mem16! ( double  [ x86_bp  x86_si  + ] ),  )
      .x87fdiv      ( x87::st ( 0 ),                                x87::st ( 4 ),                                )
      .x87fdiv      ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fdiv      ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fdiv      ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fdiv      ( x87::st ( 0 ),                                x86Mem16! ( single  [ x86_bp  x86_si  + ] ),  )
      .x87fdiv      ( x87::st ( 0 ),                                x86Mem16! ( double  [ x86_bp  x86_si  + ] ),  )
      .x87fdivp     ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fdivr     ( x87::st ( 0 ),                                x87::st ( 4 ),                                )
      .x87fdivr     ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fdivr     ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fdivr     ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fdivr     ( x87::st ( 0 ),                                x86Mem16! ( single  [ x86_bp  x86_si  + ] ),  )
      .x87fdivr     ( x87::st ( 0 ),                                x86Mem16! ( double  [ x86_bp  x86_si  + ] ),  )
      .x87fdivrp    ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fiadd     ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fiadd     ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87ficom     ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87ficom     ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87ficomp    ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87ficomp    ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fidiv     ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fidiv     ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fidivr    ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fidivr    ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fimul     ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fimul     ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fist      ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fist      ( x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fistp     ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fistp     ( x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fistp     ( x86Mem16! ( qword   [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fisub     ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fisub     ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fisubr    ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fisubr    ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fmul      ( x87::st ( 0 ),                                x87::st ( 5 ),                                )
      .x87fmul      ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fmul      ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fmul      ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fmul      ( x87::st ( 0 ),                                x86Mem16! ( single  [ x86_bp  x86_si  + ] ),  )
      .x87fmul      ( x87::st ( 0 ),                                x86Mem16! ( double  [ x86_bp  x86_si  + ] ),  )
      .x87fmulp     ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fst       ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fst       ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fst       ( x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fst       ( x86Mem16! ( float32 [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fst       ( x86Mem16! ( float64 [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fstp      ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fstp      ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fstp      ( x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fstp      ( x86Mem16! ( qword   [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fstp      ( x86Mem16! ( float32 [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fstp      ( x86Mem16! ( float64 [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fstp      ( x86Mem16! ( float80 [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fstp      ( x86Mem16! ( bcd     [ x86_bp  x86_si  + ] ),  x87::st ( 0 ),                                )
      .x87fsub      ( x87::st ( 0 ),                                x87::st ( 4 ),                                )
      .x87fsub      ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fsub      ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fsub      ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fsub      ( x87::st ( 0 ),                                x86Mem16! ( single  [ x86_bp  x86_si  + ] ),  )
      .x87fsub      ( x87::st ( 0 ),                                x86Mem16! ( double  [ x86_bp  x86_si  + ] ),  )
      .x87fsubp     ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fsubr     ( x87::st ( 0 ),                                x87::st ( 4 ),                                )
      .x87fsubr     ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .x87fsubr     ( x87::st ( 0 ),                                x86Mem16! ( word    [ x86_bp  x86_si  + ] ),  )
      .x87fsubr     ( x87::st ( 0 ),                                x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),  )
      .x87fsubr     ( x87::st ( 0 ),                                x86Mem16! ( single  [ x86_bp  x86_si  + ] ),  )
      .x87fsubr     ( x87::st ( 0 ),                                x86Mem16! ( double  [ x86_bp  x86_si  + ] ),  )
      .x87fsubrp    ( x87::st ( 4 ),                                x87::st ( 0 ),                                )
      .label        ( "cmov instructions".to_string ( ),                                                          )
      .x87fcmova    ( x87::st ( 0 ),                                x87::st ( 7 ),                                )
      .x87fcmovae   ( x87::st ( 0 ),                                x87::st ( 6 ),                                )
      .x87fcmovb    ( x87::st ( 0 ),                                x87::st ( 5 ),                                )
      .x87fcmovbe   ( x87::st ( 0 ),                                x87::st ( 4 ),                                )
      .x87fcmove    ( x87::st ( 0 ),                                x87::st ( 3 ),                                )
      .x87fcmovna   ( x87::st ( 0 ),                                x87::st ( 2 ),                                )
      .x87fcmovnae  ( x87::st ( 0 ),                                x87::st ( 1 ),                                )
      .x87fcmovnb   ( x87::st ( 0 ),                                x87::st ( 7 ),                                )
      .x87fcmovnbe  ( x87::st ( 0 ),                                x87::st ( 6 ),                                )
      .x87fcmovne   ( x87::st ( 0 ),                                x87::st ( 5 ),                                )
      .x87fcmovno   ( x87::st ( 0 ),                                x87::st ( 4 ),                                )
      .x87fcmovnu   ( x87::st ( 0 ),                                x87::st ( 3 ),                                )
      .x87fcmovo    ( x87::st ( 0 ),                                x87::st ( 2 ),                                )
      .x87fcmovu    ( x87::st ( 0 ),                                x87::st ( 1 ),                                )
      .label        ( "trigonometric instructions".to_string  ( ),                                                )
      .x87fcos      (                                                                                             )
      .x87fsin      (                                                                                             )
      .x87fsincos   (                                                                                             )
      .label        ( "register operand instructions".to_string ( ),                                              )
      .x87ffree     ( x87::st ( 1 ),                                                                              )
      .x87ffreep    ( x87::st ( 2 ),                                                                              )
      .x87fucom     ( x87::st ( 3 ),                                                                              )
      .x87fucomp    ( x87::st ( 4 ),                                                                              )
      .x87fxch      ( x87::st ( 5 ),                                                                              )
      .label        ( "misc instructions".to_string ( ),                                                          )
      .x87fisttp    ( x86Mem16! ( word    [ x86_bp  x86_si  + ] ),                                                )
      .x87fisttp    ( x86Mem16! ( dword   [ x86_bp  x86_si  + ] ),                                                )
      .x87fisttp    ( x86Mem16! ( qword   [ x86_bp  x86_si  + ] ),                                                )
      .compile      (                                                                                             );

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
