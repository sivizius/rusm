use super::
{
  x86,
  operands::
  {
    x86operand,
  },
  super::
  {
    asm::
    {
      asm,
    },
  },
};

#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      GeneralPurposeRegisterNumber
{
  AX                                    =   0,
  CX                                    =   1,
  DX                                    =   2,
  BX                                    =   3,
  SP                                    =   4,
  BP                                    =   5,
  SI                                    =   6,
  DI                                    =   7,
  R8                                    =   8,
  R9                                    =   9,
  R10                                   =   10,
  R11                                   =   11,
  R12                                   =   12,
  R13                                   =   13,
  R14                                   =   14,
  R15                                   =   15,
}

#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      SegmentRegisterNumber
{
  ES                                    =   0x00,
  CS                                    =   0x01,
  SS                                    =   0x02,
  DS                                    =   0x03,
  FS                                    =   0x04,
  GS                                    =   0x05,
  DefaultSS                             =   0xf2,
  DefaultDS                             =   0xf3,
  None                                  =   0xff,
}

impl          SegmentRegisterNumber
{
  pub fn format
  (
    &self,
  ) ->  &'static str
  {
    match self
    {
      SegmentRegisterNumber::CS         =>  "cs",
      SegmentRegisterNumber::SS         =>  "ss",
      SegmentRegisterNumber::DS         =>  "ds",
      SegmentRegisterNumber::ES         =>  "es",
      SegmentRegisterNumber::FS         =>  "fs",
      SegmentRegisterNumber::GS         =>  "gs",
      SegmentRegisterNumber::DefaultSS  =>  "(ss)",
      SegmentRegisterNumber::DefaultDS  =>  "(ds)",
      SegmentRegisterNumber::None       =>  "(--)",
    }
  }
}

macro_rules!  GeneralPurposeRegister
{
  (
    $theName:ident,
    $theSize:expr,
    $hazREX:expr,
    $theNumber:expr,
  )
  =>  {
        pub const $theName:             x86operand
        = x86operand::GeneralPurposeRegister
          {
            size:                       $theSize,
            rex:                        $hazREX,
            number:                     $theNumber,
          };
      }
}

macro_rules!  SegmentRegister
{
  (
    $theName:ident,
    $theNumber:expr,
  )
  => {
        pub const $theName:             x86operand
        = x86operand::SegmentRegister
          {
            number:                     $theNumber,
          };
      }
}

macro_rules!  ControlRegister
{
  (
    $theName:ident,
    $theNumber:expr,
  )
  =>  {
        pub const $theName:             x86operand
        = x86operand::ControlRegister
          {
            number:                     $theNumber,
          };
      }
}

macro_rules!  DebugRegister
{
  (
    $theName:ident,
    $theNumber:expr,
  )
  =>  {
        pub const $theName:             x86operand
        = x86operand::DebugRegister
          {
            number:                     $theNumber,
          };
      }
}

macro_rules!  TestRegister
{
  (
    $theName:ident,
    $theNumber:expr,
  )
  =>  {
        pub const $theName:             x86operand
        = x86operand::TestRegister
          {
            number:                     $theNumber,
          };
      }
}

macro_rules!  MulitMediaRegister
{
  (
    $theName:ident,
    $theSize:expr,
    $theNumber:expr,
  )
  =>  {
        pub const $theName:             x86operand
        = x86operand::MulitMediaRegister
          {
            size:                       $theSize,
            number:                     $theNumber,
          };
      }
}

impl          x86
{
  GeneralPurposeRegister! ( al,   asm::Byte,  false,  GeneralPurposeRegisterNumber::AX,   );
  GeneralPurposeRegister! ( cl,   asm::Byte,  false,  GeneralPurposeRegisterNumber::CX,   );
  GeneralPurposeRegister! ( dl,   asm::Byte,  false,  GeneralPurposeRegisterNumber::DX,   );
  GeneralPurposeRegister! ( bl,   asm::Byte,  false,  GeneralPurposeRegisterNumber::BX,   );
  GeneralPurposeRegister! ( ah,   asm::Byte,  false,  GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( ch,   asm::Byte,  false,  GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( dh,   asm::Byte,  false,  GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( bh,   asm::Byte,  false,  GeneralPurposeRegisterNumber::DI,   );
  GeneralPurposeRegister! ( spl,  asm::Byte,  true,   GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( bpl,  asm::Byte,  true,   GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( sil,  asm::Byte,  true,   GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( dil,  asm::Byte,  true,   GeneralPurposeRegisterNumber::DI,   );

  GeneralPurposeRegister! ( ax,   asm::Word,  false,  GeneralPurposeRegisterNumber::AX,   );
  GeneralPurposeRegister! ( cx,   asm::Word,  false,  GeneralPurposeRegisterNumber::CX,   );
  GeneralPurposeRegister! ( dx,   asm::Word,  false,  GeneralPurposeRegisterNumber::DX,   );
  GeneralPurposeRegister! ( bx,   asm::Word,  false,  GeneralPurposeRegisterNumber::BX,   );
  GeneralPurposeRegister! ( sp,   asm::Word,  false,  GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( bp,   asm::Word,  false,  GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( si,   asm::Word,  false,  GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( di,   asm::Word,  false,  GeneralPurposeRegisterNumber::DI,   );

  GeneralPurposeRegister! ( eax,  asm::DWord, false,  GeneralPurposeRegisterNumber::AX,   );
  GeneralPurposeRegister! ( ecx,  asm::DWord, false,  GeneralPurposeRegisterNumber::CX,   );
  GeneralPurposeRegister! ( edx,  asm::DWord, false,  GeneralPurposeRegisterNumber::DX,   );
  GeneralPurposeRegister! ( ebx,  asm::DWord, false,  GeneralPurposeRegisterNumber::BX,   );
  GeneralPurposeRegister! ( esp,  asm::DWord, false,  GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( ebp,  asm::DWord, false,  GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( esi,  asm::DWord, false,  GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( edi,  asm::DWord, false,  GeneralPurposeRegisterNumber::DI,   );

  GeneralPurposeRegister! ( rax,  asm::QWord, false,  GeneralPurposeRegisterNumber::AX,   );
  GeneralPurposeRegister! ( rcx,  asm::QWord, false,  GeneralPurposeRegisterNumber::CX,   );
  GeneralPurposeRegister! ( rdx,  asm::QWord, false,  GeneralPurposeRegisterNumber::DX,   );
  GeneralPurposeRegister! ( rbx,  asm::QWord, false,  GeneralPurposeRegisterNumber::BX,   );
  GeneralPurposeRegister! ( rsp,  asm::QWord, false,  GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( rbp,  asm::QWord, false,  GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( rsi,  asm::QWord, false,  GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( rdi,  asm::QWord, false,  GeneralPurposeRegisterNumber::DI,   );
  GeneralPurposeRegister! ( r8,   asm::QWord, false,  GeneralPurposeRegisterNumber::R8,   );
  GeneralPurposeRegister! ( r9,   asm::QWord, false,  GeneralPurposeRegisterNumber::R9,   );
  GeneralPurposeRegister! ( r10,  asm::QWord, false,  GeneralPurposeRegisterNumber::R10,  );
  GeneralPurposeRegister! ( r11,  asm::QWord, false,  GeneralPurposeRegisterNumber::R11,  );
  GeneralPurposeRegister! ( r12,  asm::QWord, false,  GeneralPurposeRegisterNumber::R12,  );
  GeneralPurposeRegister! ( r13,  asm::QWord, false,  GeneralPurposeRegisterNumber::R13,  );
  GeneralPurposeRegister! ( r14,  asm::QWord, false,  GeneralPurposeRegisterNumber::R14,  );
  GeneralPurposeRegister! ( r15,  asm::QWord, false,  GeneralPurposeRegisterNumber::R15,  );

  SegmentRegister!        ( cs,                       SegmentRegisterNumber::CS,          );
  SegmentRegister!        ( ss,                       SegmentRegisterNumber::SS,          );
  SegmentRegister!        ( ds,                       SegmentRegisterNumber::DS,          );
  SegmentRegister!        ( es,                       SegmentRegisterNumber::ES,          );
  SegmentRegister!        ( fs,                       SegmentRegisterNumber::FS,          );
  SegmentRegister!        ( gs,                       SegmentRegisterNumber::GS,          );

  ControlRegister!        ( cr0,                      0,                                  );
  ControlRegister!        ( cr1,                      1,                                  );
  ControlRegister!        ( cr2,                      2,                                  );
  ControlRegister!        ( cr3,                      3,                                  );
  ControlRegister!        ( cr4,                      4,                                  );
  ControlRegister!        ( cr5,                      5,                                  );
  ControlRegister!        ( cr6,                      6,                                  );
  ControlRegister!        ( cr7,                      7,                                  );

  DebugRegister!          ( dr0,                      0,                                  );
  DebugRegister!          ( dr1,                      1,                                  );
  DebugRegister!          ( dr2,                      2,                                  );
  DebugRegister!          ( dr3,                      3,                                  );
  DebugRegister!          ( dr4,                      4,                                  );
  DebugRegister!          ( dr5,                      5,                                  );
  DebugRegister!          ( dr6,                      6,                                  );
  DebugRegister!          ( dr7,                      7,                                  );

  TestRegister!           ( tr6,                      6,                                  );
  TestRegister!           ( tr7,                      7,                                  );

  MulitMediaRegister!     ( mm0,  asm::QWord,         0,                                  );
  MulitMediaRegister!     ( mm1,  asm::QWord,         1,                                  );
  MulitMediaRegister!     ( mm2,  asm::QWord,         2,                                  );
  MulitMediaRegister!     ( mm3,  asm::QWord,         3,                                  );
  MulitMediaRegister!     ( mm4,  asm::QWord,         4,                                  );
  MulitMediaRegister!     ( mm5,  asm::QWord,         5,                                  );
  MulitMediaRegister!     ( mm6,  asm::QWord,         6,                                  );
  MulitMediaRegister!     ( mm7,  asm::QWord,         7,                                  );

  MulitMediaRegister!     ( xmm0, asm::XWord,         0,                                  );
  MulitMediaRegister!     ( xmm1, asm::XWord,         1,                                  );
  MulitMediaRegister!     ( xmm2, asm::XWord,         2,                                  );
  MulitMediaRegister!     ( xmm3, asm::XWord,         3,                                  );
  MulitMediaRegister!     ( xmm4, asm::XWord,         4,                                  );
  MulitMediaRegister!     ( xmm5, asm::XWord,         5,                                  );
  MulitMediaRegister!     ( xmm6, asm::XWord,         6,                                  );
  MulitMediaRegister!     ( xmm7, asm::XWord,         7,                                  );

  MulitMediaRegister!     ( ymm0, asm::YWord,         0,                                  );
  MulitMediaRegister!     ( ymm1, asm::YWord,         1,                                  );
  MulitMediaRegister!     ( ymm2, asm::YWord,         2,                                  );
  MulitMediaRegister!     ( ymm3, asm::YWord,         3,                                  );
  MulitMediaRegister!     ( ymm4, asm::YWord,         4,                                  );
  MulitMediaRegister!     ( ymm5, asm::YWord,         5,                                  );
  MulitMediaRegister!     ( ymm6, asm::YWord,         6,                                  );
  MulitMediaRegister!     ( ymm7, asm::YWord,         7,                                  );

  MulitMediaRegister!     ( zmm0, asm::ZWord,         0,                                  );
  MulitMediaRegister!     ( zmm1, asm::ZWord,         1,                                  );
  MulitMediaRegister!     ( zmm2, asm::ZWord,         2,                                  );
  MulitMediaRegister!     ( zmm3, asm::ZWord,         3,                                  );
  MulitMediaRegister!     ( zmm4, asm::ZWord,         4,                                  );
  MulitMediaRegister!     ( zmm5, asm::ZWord,         5,                                  );
  MulitMediaRegister!     ( zmm6, asm::ZWord,         6,                                  );
  MulitMediaRegister!     ( zmm7, asm::ZWord,         7,                                  );
}
