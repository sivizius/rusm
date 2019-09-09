use super::
{
  x86,
  operands::
  {
    x86operand,
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
  GeneralPurposeRegister! ( al,   1,  false,  GeneralPurposeRegisterNumber::AX,   );
  GeneralPurposeRegister! ( cl,   1,  false,  GeneralPurposeRegisterNumber::CX,   );
  GeneralPurposeRegister! ( dl,   1,  false,  GeneralPurposeRegisterNumber::DX,   );
  GeneralPurposeRegister! ( bl,   1,  false,  GeneralPurposeRegisterNumber::BX,   );
  GeneralPurposeRegister! ( ah,   1,  false,  GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( ch,   1,  false,  GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( dh,   1,  false,  GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( bh,   1,  false,  GeneralPurposeRegisterNumber::DI,   );
  GeneralPurposeRegister! ( spl,  1,  true,   GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( bpl,  1,  true,   GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( sil,  1,  true,   GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( dil,  1,  true,   GeneralPurposeRegisterNumber::DI,   );

  GeneralPurposeRegister! ( ax,   2,  false,  GeneralPurposeRegisterNumber::AX,   );
  GeneralPurposeRegister! ( cx,   2,  false,  GeneralPurposeRegisterNumber::CX,   );
  GeneralPurposeRegister! ( dx,   2,  false,  GeneralPurposeRegisterNumber::DX,   );
  GeneralPurposeRegister! ( bx,   2,  false,  GeneralPurposeRegisterNumber::BX,   );
  GeneralPurposeRegister! ( sp,   2,  false,  GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( bp,   2,  false,  GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( si,   2,  false,  GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( di,   2,  false,  GeneralPurposeRegisterNumber::DI,   );

  GeneralPurposeRegister! ( eax,  4,  false,  GeneralPurposeRegisterNumber::AX,   );
  GeneralPurposeRegister! ( ecx,  4,  false,  GeneralPurposeRegisterNumber::CX,   );
  GeneralPurposeRegister! ( edx,  4,  false,  GeneralPurposeRegisterNumber::DX,   );
  GeneralPurposeRegister! ( ebx,  4,  false,  GeneralPurposeRegisterNumber::BX,   );
  GeneralPurposeRegister! ( esp,  4,  false,  GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( ebp,  4,  false,  GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( esi,  4,  false,  GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( edi,  4,  false,  GeneralPurposeRegisterNumber::DI,   );

  GeneralPurposeRegister! ( rax,  8,  false,  GeneralPurposeRegisterNumber::AX,   );
  GeneralPurposeRegister! ( rcx,  8,  false,  GeneralPurposeRegisterNumber::CX,   );
  GeneralPurposeRegister! ( rdx,  8,  false,  GeneralPurposeRegisterNumber::DX,   );
  GeneralPurposeRegister! ( rbx,  8,  false,  GeneralPurposeRegisterNumber::BX,   );
  GeneralPurposeRegister! ( rsp,  8,  false,  GeneralPurposeRegisterNumber::SP,   );
  GeneralPurposeRegister! ( rbp,  8,  false,  GeneralPurposeRegisterNumber::BP,   );
  GeneralPurposeRegister! ( rsi,  8,  false,  GeneralPurposeRegisterNumber::SI,   );
  GeneralPurposeRegister! ( rdi,  8,  false,  GeneralPurposeRegisterNumber::DI,   );
  GeneralPurposeRegister! ( r8,   8,  false,  GeneralPurposeRegisterNumber::R8,   );
  GeneralPurposeRegister! ( r9,   8,  false,  GeneralPurposeRegisterNumber::R9,   );
  GeneralPurposeRegister! ( r10,  8,  false,  GeneralPurposeRegisterNumber::R10,  );
  GeneralPurposeRegister! ( r11,  8,  false,  GeneralPurposeRegisterNumber::R11,  );
  GeneralPurposeRegister! ( r12,  8,  false,  GeneralPurposeRegisterNumber::R12,  );
  GeneralPurposeRegister! ( r13,  8,  false,  GeneralPurposeRegisterNumber::R13,  );
  GeneralPurposeRegister! ( r14,  8,  false,  GeneralPurposeRegisterNumber::R14,  );
  GeneralPurposeRegister! ( r15,  8,  false,  GeneralPurposeRegisterNumber::R15,  );

  SegmentRegister!        ( cs,               SegmentRegisterNumber::CS,          );
  SegmentRegister!        ( ss,               SegmentRegisterNumber::SS,          );
  SegmentRegister!        ( ds,               SegmentRegisterNumber::DS,          );
  SegmentRegister!        ( es,               SegmentRegisterNumber::ES,          );
  SegmentRegister!        ( fs,               SegmentRegisterNumber::FS,          );
  SegmentRegister!        ( gs,               SegmentRegisterNumber::GS,          );

  ControlRegister!        ( cr0,              0,                                  );
  ControlRegister!        ( cr1,              1,                                  );
  ControlRegister!        ( cr2,              2,                                  );
  ControlRegister!        ( cr3,              3,                                  );
  ControlRegister!        ( cr4,              4,                                  );
  ControlRegister!        ( cr5,              5,                                  );
  ControlRegister!        ( cr6,              6,                                  );
  ControlRegister!        ( cr7,              7,                                  );

  DebugRegister!          ( dr0,              0,                                  );
  DebugRegister!          ( dr1,              1,                                  );
  DebugRegister!          ( dr2,              2,                                  );
  DebugRegister!          ( dr3,              3,                                  );
  DebugRegister!          ( dr4,              4,                                  );
  DebugRegister!          ( dr5,              5,                                  );
  DebugRegister!          ( dr6,              6,                                  );
  DebugRegister!          ( dr7,              7,                                  );

  TestRegister!           ( tr6,              6,                                  );
  TestRegister!           ( tr7,              7,                                  );

  MulitMediaRegister!     ( mm0,  8,          0,                                  );
  MulitMediaRegister!     ( mm1,  8,          1,                                  );
  MulitMediaRegister!     ( mm2,  8,          2,                                  );
  MulitMediaRegister!     ( mm3,  8,          3,                                  );
  MulitMediaRegister!     ( mm4,  8,          4,                                  );
  MulitMediaRegister!     ( mm5,  8,          5,                                  );
  MulitMediaRegister!     ( mm6,  8,          6,                                  );
  MulitMediaRegister!     ( mm7,  8,          7,                                  );

  MulitMediaRegister!     ( xmm0, 16,         0,                                  );
  MulitMediaRegister!     ( xmm1, 16,         1,                                  );
  MulitMediaRegister!     ( xmm2, 16,         2,                                  );
  MulitMediaRegister!     ( xmm3, 16,         3,                                  );
  MulitMediaRegister!     ( xmm4, 16,         4,                                  );
  MulitMediaRegister!     ( xmm5, 16,         5,                                  );
  MulitMediaRegister!     ( xmm6, 16,         6,                                  );
  MulitMediaRegister!     ( xmm7, 16,         7,                                  );

  MulitMediaRegister!     ( ymm0, 32,         0,                                  );
  MulitMediaRegister!     ( ymm1, 32,         1,                                  );
  MulitMediaRegister!     ( ymm2, 32,         2,                                  );
  MulitMediaRegister!     ( ymm3, 32,         3,                                  );
  MulitMediaRegister!     ( ymm4, 32,         4,                                  );
  MulitMediaRegister!     ( ymm5, 32,         5,                                  );
  MulitMediaRegister!     ( ymm6, 32,         6,                                  );
  MulitMediaRegister!     ( ymm7, 32,         7,                                  );

  MulitMediaRegister!     ( zmm0, 64,         0,                                  );
  MulitMediaRegister!     ( zmm1, 64,         1,                                  );
  MulitMediaRegister!     ( zmm2, 64,         2,                                  );
  MulitMediaRegister!     ( zmm3, 64,         3,                                  );
  MulitMediaRegister!     ( zmm4, 64,         4,                                  );
  MulitMediaRegister!     ( zmm5, 64,         5,                                  );
  MulitMediaRegister!     ( zmm6, 64,         6,                                  );
  MulitMediaRegister!     ( zmm7, 64,         7,                                  );
}
