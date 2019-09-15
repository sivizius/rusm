#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum Memory16Registers
{
  BXSI                                  =   0x00,
  BXDI                                  =   0x01,
  BPSI                                  =   0x02,
  BPDI                                  =   0x03,
  SI                                    =   0x04,
  DI                                    =   0x05,
  BP                                    =   0x06,
  BX                                    =   0x07,
  DISP                                  =   0x86,
  INVALID                               =   0xff,
}

/*
impl          x86
{
  pub fn Memory16
  (
    size:                               usize,
    segment:                            SegmentRegisterNumber,
    registers:                          Memory16Registers,
    displacement:                       i128,
  ) ->  x86operand
  {
    x86operand::Memory16
    {
      size,
      segment,
      registers,
      displacement,
    }
  }
}
*/

#[macro_export]
macro_rules! x86Mem16finally
{
  (
    $kind:expr,
    $size:expr,
    $segment:expr,
    $(  $token:tt )*
  )
  =>  {
        Expression
        (
          vec!
          [
            $(
              nextToken!
              (
                $token
              ),
            )*
            ExpressionToken::x86
            (
              x86operand::Memory16
              {
                kind:                     $kind,
                size:                     $size,
                segment:                  $segment,
                registers:                Memory16Registers::INVALID,
                displacement:             0,
              }
            )
          ]
        )
      };
}

#[macro_export]
macro_rules! x86Mem16segment
{
  ( $kind:expr, $size:expr, cs, $(  $token:tt )*  ) =>  { x86Mem16finally!  ( $kind,  $size,  SegmentRegisterNumber::CS,        $(  $token  )*  ) };
  ( $kind:expr, $size:expr, ss, $(  $token:tt )*  ) =>  { x86Mem16finally!  ( $kind,  $size,  SegmentRegisterNumber::SS,        $(  $token  )*  ) };
  ( $kind:expr, $size:expr, ds, $(  $token:tt )*  ) =>  { x86Mem16finally!  ( $kind,  $size,  SegmentRegisterNumber::DS,        $(  $token  )*  ) };
  ( $kind:expr, $size:expr, es, $(  $token:tt )*  ) =>  { x86Mem16finally!  ( $kind,  $size,  SegmentRegisterNumber::ES,        $(  $token  )*  ) };
  ( $kind:expr, $size:expr, fs, $(  $token:tt )*  ) =>  { x86Mem16finally!  ( $kind,  $size,  SegmentRegisterNumber::FS,        $(  $token  )*  ) };
  ( $kind:expr, $size:expr, gs, $(  $token:tt )*  ) =>  { x86Mem16finally!  ( $kind,  $size,  SegmentRegisterNumber::GS,        $(  $token  )*  ) };
  ( $kind:expr, $size:expr, @,  $(  $token:tt )*  ) =>  { x86Mem16finally!  ( $kind,  $size,  SegmentRegisterNumber::DefaultDS, $(  $token  )*  ) };
}

//  TODO: x86Mem16pointer, x86Mem16signed, x86Mem16unsigned, x86Mem16integer,

#[macro_export]
macro_rules! x86Mem16
{
  ( ptr                             [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Pointer,   asm::Null,    @,      $(  $token  )*  ) };
  ( ptr                 $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Pointer,   asm::Null,    $sreg,  $(  $token  )*  ) };
  ( byte                            [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::Byte,    @,      $(  $token  )*  ) };
  ( byte                $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::Byte,    $sreg,  $(  $token  )*  ) };
  ( word                            [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::Word,    @,      $(  $token  )*  ) };
  ( word                $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::Word,    $sreg,  $(  $token  )*  ) };
  ( dword                           [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::DWord,   @,      $(  $token  )*  ) };
  ( dword               $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::DWord,   $sreg,  $(  $token  )*  ) };
  ( fword                           [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::FWord,   @,      $(  $token  )*  ) };
  ( fword               $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::FWord,   $sreg,  $(  $token  )*  ) };
  ( pword                           [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::PWord,   @,      $(  $token  )*  ) };
  ( pword               $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::PWord,   $sreg,  $(  $token  )*  ) };
  ( qword                           [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::QWord,   @,      $(  $token  )*  ) };
  ( qword               $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::QWord,   $sreg,  $(  $token  )*  ) };
  ( tbyte                           [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TByte,   @,      $(  $token  )*  ) };
  ( tbyte               $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TByte,   $sreg,  $(  $token  )*  ) };
  ( tword                           [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TWord,   @,      $(  $token  )*  ) };
  ( tword               $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TWord,   $sreg,  $(  $token  )*  ) };
  ( dqword                          [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::DQWord,  @,      $(  $token  )*  ) };
  ( dqword              $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::DQWord,  $sreg,  $(  $token  )*  ) };
  ( xword                           [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TWord,   @,      $(  $token  )*  ) };
  ( xword               $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TWord,   $sreg,  $(  $token  )*  ) };
  ( qqword                          [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::QQWord,  @,      $(  $token  )*  ) };
  ( qqword              $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::QQWord,  $sreg,  $(  $token  )*  ) };
  ( yword                           [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TWord,   @,      $(  $token  )*  ) };
  ( yword               $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TWord,   $sreg,  $(  $token  )*  ) };
  ( dqqword                         [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::DQQWord, @,      $(  $token  )*  ) };
  ( dqqword             $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::DQQWord, $sreg,  $(  $token  )*  ) };
  ( zword                           [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TWord,   @,      $(  $token  )*  ) };
  ( zword               $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Integer,   asm::TWord,   $sreg,  $(  $token  )*  ) };
  ( single                          [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::DWord,   @,      $(  $token  )*  ) };
  ( single              $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::DWord,   $sreg,  $(  $token  )*  ) };
  ( double                          [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::QWord,   @,      $(  $token  )*  ) };
  ( double              $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::QWord,   $sreg,  $(  $token  )*  ) };
  ( float32                         [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::DWord,   @,      $(  $token  )*  ) };
  ( float32             $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::DWord,   $sreg,  $(  $token  )*  ) };
  ( float64                         [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::QWord,   @,      $(  $token  )*  ) };
  ( float64             $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::QWord,   $sreg,  $(  $token  )*  ) };
  ( float80                         [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::TWord,   @,      $(  $token  )*  ) };
  ( float80             $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::IEEE754,   asm::TWord,   $sreg,  $(  $token  )*  ) };
  ( bcd                             [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::BCD,       asm::TWord,   @,      $(  $token  )*  ) };
  ( bcd                 $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::BCD,       asm::TWord,   $sreg,  $(  $token  )*  ) };
  ( int   $size:literal             [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Signed,    $size,        @,      $(  $token  )*  ) };
  ( int   $size:literal $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Signed,    $size,        $sreg,  $(  $token  )*  ) };
  ( uint  $size:literal             [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Unsigned,  $size,        @,      $(  $token  )*  ) };
  ( uint  $size:literal $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( asmKind::Unsigned,  $size,        $sreg,  $(  $token  )*  ) };
}
