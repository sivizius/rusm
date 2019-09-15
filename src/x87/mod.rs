macro_rules!  x87zeroOperand {
  (
    $theName:ident,
    $theInstruction:expr,
  )
  =>  {
        pub fn $theName
        (
        )
        -> Instruction
        {
          Instruction
          (
            InstructionType::x87
            {
              architecture:             x86state  ( ),
              instruction:              $theInstruction,
            },
            0,
            vec!  ( ),
          )
        }
      }
}

macro_rules!  x87oneOperand {
  (
    $theName:ident,
    $theInstruction:expr,
  )
  =>  {
        pub fn $theName
        (
          op:                           impl  Operand,
        )
        -> Instruction
        {
          let     ( this, size, )       =   op.this ( );
          Instruction
          (
            InstructionType::x87
            {
              architecture:             x86state  ( ),
              instruction:              $theInstruction,
            },
            size,
            vec!
            (
              this,
            ),
          )
        }
      }
}

macro_rules!  x87twoOperand {
  (
    $theName:ident,
    $theInstruction:expr,
  )
  =>  {
        pub fn $theName
        (
          dst:                          impl  Operand,
          src:                          impl  Operand,
        )
        -> Instruction
        {
          let     ( dstThis, dstSize, ) =   dst.this  ( );
          let     ( srcThis, srcSize, ) =   src.this  ( );
          let size                      =   ( dstSize | srcSize ) as  usize;
          Instruction
          (
            InstructionType::x87
            {
              architecture:             x86state  ( ),
              instruction:              $theInstruction,
            },
            size,
            vec!
            (
              dstThis,
              srcThis,
            ),
          )
        }
      }
}

pub mod operands;
mod     cmov;
mod     load;
mod     math;
mod     misc;
mod     registerOperand;
mod     simple;
mod     zeroOperands;

use super::
{
  Architecture,
  instructions::
  {
    Instruction,
    InstructionResult,
    InstructionType,
  },
  x86::
  {
    x86instruction,
    x86result,
  },
  x87::
  {
    simple::x87simple,
  },
};

impl          Instruction
{
  pub fn x87compile
  (
    &mut self,
    architecture:                       &mut Architecture,
    round:                              usize,
  )
  ->  InstructionResult
  {
    if  let Architecture::x86 ( ref mut architecture  ) = architecture
    {
      let
      (
        operands,
        _size,
        instruction,
      )                                 =   self.x86init  ( );
      let     operands                  =   &operands;
      if  let InstructionType::x87
              {
                architecture:           ref mut state,
                instruction:            this,
              } = self.thisRefMut ( )
      {
        let
        (
          cpu,
          fpu,
          operandSize,
          addressSize,
          hazLock,
          theBranchHint,
          theRepeat,
        )
        = state.init
          (
            architecture,
            round,
          );
        if  fpu !=  x87version::None
        {
          let     result
          = match this
            {               //  compiler                            ( opcode, fwait?, subcode,  operands, configuration,              fpu,  cpu,  operandSize,  addressSize,  ),
              x87::F2XM1    =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf0,     operands, x87expected::Default,       fpu,                                    ),
              x87::FABS     =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xe1,     operands, x87expected::Default,       fpu,                                    ),
              x87::FADD     =>  instruction.compileFloatMath        ( 0xd8,   false,  0,        operands, x87flags::Default,                cpu,  operandSize,  addressSize,  ),
              x87::FADDP    =>  instruction.compileFloatMath        ( 0xda,   false,  0,        operands, x87flags::DefaultPop,             cpu,  operandSize,  addressSize,  ),
              x87::FBLD     =>  instruction.compileFloatLoad        (         false,            operands, x87flags::DecimalLoad,            cpu,  operandSize,  addressSize,  ),
              x87::FBSTP    =>  instruction.compileFloatMath        ( 0xd9,   false,  2,        operands, x87flags::DecimalStorePop,        cpu,  operandSize,  addressSize,  ),
              x87::FCHS     =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xe0,     operands, x87expected::Default,       fpu,                                    ),
              x87::FCLEX    =>  instruction.compileFloatZeroOperand ( 0xdb,   true,   0xe2,     operands, x87expected::Default,       fpu,                                    ),
              x87::FCMOVB   =>  instruction.compileFloatCMove       ( 0xda,   false,  0xc0,     operands,                             fpu,                                    ),
              x87::FCMOVBE  =>  instruction.compileFloatCMove       ( 0xda,   false,  0xd0,     operands,                             fpu,                                    ),
              x87::FCMOVE   =>  instruction.compileFloatCMove       ( 0xda,   false,  0xc8,     operands,                             fpu,                                    ),
              x87::FCMOVNB  =>  instruction.compileFloatCMove       ( 0xdb,   false,  0xc0,     operands,                             fpu,                                    ),
              x87::FCMOVNBE =>  instruction.compileFloatCMove       ( 0xdb,   false,  0xd0,     operands,                             fpu,                                    ),
              x87::FCMOVNE  =>  instruction.compileFloatCMove       ( 0xdb,   false,  0xc8,     operands,                             fpu,                                    ),
              x87::FCMOVNU  =>  instruction.compileFloatCMove       ( 0xdb,   false,  0xd8,     operands,                             fpu,                                    ),
              x87::FCMOVU   =>  instruction.compileFloatCMove       ( 0xda,   false,  0xd8,     operands,                             fpu,                                    ),
              x87::FCOM     =>  instruction.compileFloatMath        ( 0xd8,   false,  2,        operands, x87flags::LazyCompare,            cpu,  operandSize,  addressSize,  ),
              x87::FCOMI    =>  instruction.compileFloatRegOperand  ( 0xdb,   false,  0xf0,     operands, x87expected::OverPentium,   fpu,                                    ),
              x87::FCOMIP   =>  instruction.compileFloatRegOperand  ( 0xdf,   false,  0xf0,     operands, x87expected::OverPentium,   fpu,                                    ),
              x87::FCOMP    =>  instruction.compileFloatMath        ( 0xd8,   false,  3,        operands, x87flags::LazyCompare,            cpu,  operandSize,  addressSize,  ),
              x87::FCOMPP   =>  instruction.compileFloatZeroOperand ( 0xde,   false,  0xd9,     operands, x87expected::Default,       fpu,                                    ),
              x87::FCOS     =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xff,     operands, x87expected::Over387,       fpu,                                    ),
              x87::FDECSTP  =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf6,     operands, x87expected::Default,       fpu,                                    ),
              x87::FDISI    =>  instruction.compileFloatZeroOperand ( 0xdb,   true,   0xe1,     operands, x87expected::Only8087,      fpu,                                    ),
              x87::FDIV     =>  instruction.compileFloatMath        ( 0xd8,   false,  6,        operands, x87flags::Default,                cpu,  operandSize,  addressSize,  ),
              x87::FDIVP    =>  instruction.compileFloatMath        ( 0xda,   false,  6,        operands, x87flags::DefaultPop,             cpu,  operandSize,  addressSize,  ),
              x87::FDIVR    =>  instruction.compileFloatMath        ( 0xd8,   false,  7,        operands, x87flags::Default,                cpu,  operandSize,  addressSize,  ),
              x87::FDIVRP   =>  instruction.compileFloatMath        ( 0xda,   false,  7,        operands, x87flags::DefaultPop,             cpu,  operandSize,  addressSize,  ),
              x87::FENI     =>  instruction.compileFloatZeroOperand ( 0xdb,   true,   0xe0,     operands, x87expected::Only8087,      fpu,                                    ),
              x87::FFREE    =>  instruction.compileFloatRegOperand  ( 0xdd,   false,  0xc0,     operands, x87expected::Default,       fpu,                                    ),
              x87::FFREEP   =>  instruction.compileFloatRegOperand  ( 0xdf,   false,  0xc0,     operands, x87expected::Default,       fpu,                                    ),
              x87::FIADD    =>  instruction.compileFloatMath        ( 0xd8,   false,  0,        operands, x87flags::Integer,                cpu,  operandSize,  addressSize,  ),
              x87::FICOM    =>  instruction.compileFloatMath        ( 0xd8,   false,  2,        operands, x87flags::Integer,                cpu,  operandSize,  addressSize,  ),
              x87::FICOMP   =>  instruction.compileFloatMath        ( 0xd8,   false,  3,        operands, x87flags::Integer,                cpu,  operandSize,  addressSize,  ),
              x87::FIDIV    =>  instruction.compileFloatMath        ( 0xd8,   false,  6,        operands, x87flags::Integer,                cpu,  operandSize,  addressSize,  ),
              x87::FIDIVR   =>  instruction.compileFloatMath        ( 0xd8,   false,  7,        operands, x87flags::Integer,                cpu,  operandSize,  addressSize,  ),
              x87::FILD     =>  instruction.compileFloatLoad        (         false,            operands, x87flags::Integer,                cpu,  operandSize,  addressSize,  ),
              x87::FIMUL    =>  instruction.compileFloatMath        ( 0xd8,   false,  0x01,     operands, x87flags::Integer,                cpu,  operandSize,  addressSize,  ),
              x87::FINCSTP  =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf7,     operands, x87expected::Default,       fpu,                                    ),
              x87::FINIT    =>  instruction.compileFloatZeroOperand ( 0xdb,   true,   0xe3,     operands, x87expected::Default,       fpu,                                    ),
              x87::FIST     =>  instruction.compileFloatMath        ( 0xd9,   false,  2,        operands, x87flags::IntegerStore,           cpu,  operandSize,  addressSize,  ),
              x87::FISTP    =>  instruction.compileFloatMath        ( 0xd9,   false,  2,        operands, x87flags::IntegerStorePop,        cpu,  operandSize,  addressSize,  ),
              x87::FISTTP   =>  instruction.compileFloatIntSetTPop  (         false,            operands,                             fpu,  cpu,  operandSize,  addressSize,  ),
              x87::FISUB    =>  instruction.compileFloatMath        ( 0xd8,   false,  4,        operands, x87flags::Integer,                cpu,  operandSize,  addressSize,  ),
              x87::FISUBR   =>  instruction.compileFloatMath        ( 0xda,   false,  5,        operands, x87flags::Integer,                cpu,  operandSize,  addressSize,  ),
              x87::FLD      =>  instruction.compileFloatLoad        (         false,            operands, x87flags::Lazy,                   cpu,  operandSize,  addressSize,  ),
              x87::FLD1     =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xe8,     operands, x87expected::Default,       fpu,                                    ),
              x87::FLDCW    =>  instruction.compileFloatSimple      ( 0xd9,   true,   5,        operands, x87simple::CW,                    cpu,  operandSize,  addressSize,  ),
              x87::FLDENV   =>  instruction.compileFloatSimple      ( 0xd9,   false,  4,        operands, x87simple::ENV,                   cpu,  operandSize,  addressSize,  ),
              x87::FLDL2E   =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xea,     operands, x87expected::Default,       fpu,                                    ),
              x87::FLDL2T   =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xe9,     operands, x87expected::Default,       fpu,                                    ),
              x87::FLDLG2   =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xec,     operands, x87expected::Default,       fpu,                                    ),
              x87::FLDLN2   =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xed,     operands, x87expected::Default,       fpu,                                    ),
              x87::FLDPI    =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xeb,     operands, x87expected::Default,       fpu,                                    ),
              x87::FLDZ     =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xee,     operands, x87expected::Default,       fpu,                                    ),
              x87::FMUL     =>  instruction.compileFloatMath        ( 0xd8,   false,  1,        operands, x87flags::Default,                cpu,  operandSize,  addressSize,  ),
              x87::FMULP    =>  instruction.compileFloatMath        ( 0xda,   false,  1,        operands, x87flags::DefaultPop,             cpu,  operandSize,  addressSize,  ),
              x87::FNCLEX   =>  instruction.compileFloatZeroOperand ( 0xdb,   false,  0xe2,     operands, x87expected::Default,       fpu,                                    ),
              x87::FNDISI   =>  instruction.compileFloatZeroOperand ( 0xdb,   false,  0xe1,     operands, x87expected::Only8087,      fpu,                                    ),
              x87::FNENI    =>  instruction.compileFloatZeroOperand ( 0xdb,   false,  0xe0,     operands, x87expected::Only8087,      fpu,                                    ),
              x87::FNINIT   =>  instruction.compileFloatZeroOperand ( 0xdb,   false,  0xe3,     operands, x87expected::Default,       fpu,                                    ),
              x87::FNOP     =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xd0,     operands, x87expected::Default,       fpu,                                    ),
              x87::FNSAVE   =>  instruction.compileFloatSimple      ( 0xdd,   false,  6,        operands, x87simple::SAVE,                  cpu,  operandSize,  addressSize,  ),
              x87::FNSETPM  =>  instruction.compileFloatZeroOperand ( 0xdb,   true,   0xe4,     operands, x87expected::Only287,       fpu,                                    ),
              x87::FNSTCW   =>  instruction.compileFloatSimple      ( 0xd9,   false,  7,        operands, x87simple::CW,                    cpu,  operandSize,  addressSize,  ),
              x87::FNSTENV  =>  instruction.compileFloatSimple      ( 0xd9,   false,  6,        operands, x87simple::ENV,                   cpu,  operandSize,  addressSize,  ),
              x87::FNSTSW   =>  instruction.compileFloatSimple      ( 0xdd,   false,  7,        operands, x87simple::SW,                    cpu,  operandSize,  addressSize,  ),
              x87::FPATAN   =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf3,     operands, x87expected::Default,       fpu,                                    ),
              x87::FPREM    =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf8,     operands, x87expected::Default,       fpu,                                    ),
              x87::FPREM1   =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf5,     operands, x87expected::Over387,       fpu,                                    ),
              x87::FPTAN    =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf2,     operands, x87expected::Default,       fpu,                                    ),
              x87::FRNDINT  =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xfc,     operands, x87expected::Default,       fpu,                                    ),
              x87::FRSTOR   =>  instruction.compileFloatSimple      ( 0xdd,   false,  4,        operands, x87simple::SAVE,                  cpu,  operandSize,  addressSize,  ),
              x87::FSAVE    =>  instruction.compileFloatSimple      ( 0xdd,   true,   6,        operands, x87simple::SAVE,                  cpu,  operandSize,  addressSize,  ),
              x87::FSCALE   =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xfd,     operands, x87expected::Default,       fpu,                                    ),
              x87::FSIN     =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xfe,     operands, x87expected::Over387,       fpu,                                    ),
              x87::FSINCOS  =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xfb,     operands, x87expected::Over387,       fpu,                                    ),
              x87::FSQRT    =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xfa,     operands, x87expected::Default,       fpu,                                    ),
              x87::FST      =>  instruction.compileFloatMath        ( 0xd9,   false,  2,        operands, x87flags::DefaultStore,           cpu,  operandSize,  addressSize,  ),
              x87::FSTCW    =>  instruction.compileFloatSimple      ( 0xd9,   true,   7,        operands, x87simple::CW,                    cpu,  operandSize,  addressSize,  ),
              x87::FSTENV   =>  instruction.compileFloatSimple      ( 0xd9,   true,   6,        operands, x87simple::ENV,                   cpu,  operandSize,  addressSize,  ),
              x87::FSTP     =>  instruction.compileFloatMath        ( 0xd9,   false,  3,        operands, x87flags::DefaultStorePop,        cpu,  operandSize,  addressSize,  ),
              x87::FSTSW    =>  instruction.compileFloatSimple      ( 0xdd,   true,   7,        operands, x87simple::SW,                    cpu,  operandSize,  addressSize,  ),
              x87::FSUB     =>  instruction.compileFloatMath        ( 0xd8,   false,  4,        operands, x87flags::Default,                cpu,  operandSize,  addressSize,  ),
              x87::FSUBP    =>  instruction.compileFloatMath        ( 0xda,   false,  4,        operands, x87flags::DefaultPop,             cpu,  operandSize,  addressSize,  ),
              x87::FSUBR    =>  instruction.compileFloatMath        ( 0xd8,   false,  5,        operands, x87flags::Default,                cpu,  operandSize,  addressSize,  ),
              x87::FSUBRP   =>  instruction.compileFloatMath        ( 0xda,   false,  5,        operands, x87flags::DefaultPop,             cpu,  operandSize,  addressSize,  ),
              x87::FTST     =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xe4,     operands, x87expected::Default,       fpu,                                    ),
              x87::FUCOM    =>  instruction.compileFloatRegOperand  ( 0xdd,   false,  0xe0,     operands, x87expected::Over387,       fpu,                                    ),
              x87::FUCOMI   =>  instruction.compileFloatRegOperand  ( 0xdb,   false,  0xe8,     operands, x87expected::OverPentium,   fpu,                                    ),
              x87::FUCOMIP  =>  instruction.compileFloatRegOperand  ( 0xdf,   false,  0xe8,     operands, x87expected::OverPentium,   fpu,                                    ),
              x87::FUCOMP   =>  instruction.compileFloatRegOperand  ( 0xdd,   false,  0xe8,     operands, x87expected::Over387,       fpu,                                    ),
              x87::FUCOMPP  =>  instruction.compileFloatZeroOperand ( 0xda,   false,  0xe9,     operands, x87expected::Over387,       fpu,                                    ),
              x87::FWAIT    =>  instruction.compileZeroOperand      ( 0x9b,                     operands,                                                                     ),
              x87::FXAM     =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xe5,     operands, x87expected::Default,       fpu,                                    ),
              x87::FXCH     =>  instruction.compileFloatRegOperand  ( 0xd9,   false,  0xc8,     operands, x87expected::Default,       fpu,                                    ),
              x87::FXTRACT  =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf4,     operands, x87expected::Default,       fpu,                                    ),
              x87::FYL2X    =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf1,     operands, x87expected::Default,       fpu,                                    ),
              x87::FYL2XP1  =>  instruction.compileFloatZeroOperand ( 0xd9,   false,  0xf9,     operands, x87expected::Default,       fpu,                                    ),
            };
          self.x86processResult
          (
            cpu,
            hazLock,
            theBranchHint,
            theRepeat,
            result,
          )
        }
        else
        {
          InstructionResult::Again.noVersion          (                 "x87",  )
        }
      }
      else
      {
        InstructionResult::Again.wrongInstructionSet  ( self.this ( ),  "x87",  )
      }
    }
    else
    {
      InstructionResult::Again.wrongArchitecture      ( *architecture,  "x86",  )
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      x87
{
  F2XM1,
  FABS,
  FADD,
  FADDP,
  FBLD,
  FBSTP,
  FCHS,
  FCLEX,
  FCMOVB,
  FCMOVBE,
  FCMOVE,
  FCMOVNB,
  FCMOVNBE,
  FCMOVNE,
  FCMOVNU,
  FCMOVU,
  FCOM,
  FCOMI,
  FCOMIP,
  FCOMP,
  FCOMPP,
  FCOS,
  FDECSTP,
  FDISI,
  FDIV,
  FDIVP,
  FDIVR,
  FDIVRP,
  FENI,
  FFREE,
  FFREEP,
  FIADD,
  FICOM,
  FICOMP,
  FIDIV,
  FIDIVR,
  FILD,
  FIMUL,
  FINCSTP,
  FINIT,
  FIST,
  FISTP,
  FISTTP,
  FISUB,
  FISUBR,
  FLD,
  FLD1,
  FLDCW,
  FLDENV,
  FLDL2E,
  FLDL2T,
  FLDLG2,
  FLDLN2,
  FLDPI,
  FLDZ,
  FMUL,
  FMULP,
  FNCLEX,
  FNDISI,
  FNENI,
  FNINIT,
  FNOP,
  FNSAVE,
  FNSETPM,
  FNSTCW,
  FNSTENV,
  FNSTSW,
  FPATAN,
  FPREM,
  FPREM1,
  FPTAN,
  FRNDINT,
  FRSTOR,
  FSAVE,
  FSCALE,
  FSIN,
  FSINCOS,
  FSQRT,
  FST,
  FSTCW,
  FSTENV,
  FSTP,
  FSTSW,
  FSUB,
  FSUBP,
  FSUBR,
  FSUBRP,
  FTST,
  FUCOM,
  FUCOMI,
  FUCOMIP,
  FUCOMP,
  FUCOMPP,
  FWAIT,
  FXAM,
  FXCH,
  FXTRACT,
  FYL2X,
  FYL2XP1,
}

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      x87expected
{
  Default,
  Only                                  ( x87version  ),
  Minimal                               ( x87version  ),
}

impl          x87expected
{
  pub const Only8087:             Self  =   x87expected::Only    ( x87version::i8087    );
  pub const Only287:              Self  =   x87expected::Only    ( x87version::i287     );
  pub const Over387:              Self  =   x87expected::Minimal ( x87version::i387     );
  pub const OverPentium:          Self  =   x87expected::Minimal ( x87version::Pentium  );
}

impl          x87expected
{
  pub fn result
  (
    self,
    version:                            x87version,
    instruction:                        x86instruction,
    
  )
  ->  x86result
  {
    match self
    {
      x87expected::Default
      =>  x86result::Done   ( instruction ),
      x87expected::Only     ( expected    )
      =>  if  false //  TODO: Flag To Disable This Warning
          {
            x86result::Done ( instruction )
          }
          else
          {
            x86result::Warn
            (
              instruction,
              vec!
              (
                format!
                (
                  "This Instruction Is Equivalent To `fnop`, Unless x87-Version Is {}.",
                  expected.name ( ),
                )
              ),
            )
          },
      x87expected::Minimal  ( expected  )
      =>  if  version >=  expected
          {
            x86result::Done ( instruction )
          }
          else
          {
            x86result::WrongVersion
            {
              have:                     version,
              want:                     expected,
            }
          },
    }
  }
}

bitflags!
{
  #[allow(non_camel_case_types)]
  pub struct  x87flags:                 usize
  {
    const Float                         =   0b0000_0000_0001;
    const Stack                         =   0b0000_0000_0010;
    const Integer                       =   0b0000_0000_0100;
    const Decimal                       =   0b0000_0000_1000;

    const Store                         =   0b0000_0001_0000;
    const Compare                       =   0b0000_0010_0000;
    const Pop                           =   0b0000_0100_0000;

    const FloatStack                    =   Self::Float.bits  | Self::Stack.bits                                                                                    ;
    const Default                       =   Self::Float.bits  | Self::Stack.bits  | Self::Integer.bits                                                              ;
    const DefaultPop                    =                       Self::Stack.bits  | Self::Integer.bits  | Self::Decimal.bits  |                       Self::Pop.bits;
    const DefaultStore                  =   Self::Float.bits  | Self::Stack.bits  | Self::Integer.bits  | Self::Decimal.bits  | Self::Store.bits                    ;
    const DefaultStorePop               =   Self::Float.bits  | Self::Stack.bits  | Self::Integer.bits  | Self::Decimal.bits  | Self::Store.bits    | Self::Pop.bits;
    const IntegerStore                  =                                           Self::Integer.bits  |                       Self::Store.bits                    ;
    const IntegerStorePop               =                                           Self::Integer.bits  |                       Self::Store.bits    | Self::Pop.bits;
    const DecimalLoad                   =                                                                 Self::Decimal.bits                                        ;
    const DecimalStorePop               =                                                                 Self::Decimal.bits  | Self::Store.bits    | Self::Pop.bits;
    const Lazy                          =   Self::Float.bits  | Self::Stack.bits  | Self::Integer.bits  | Self::Decimal.bits                                        ;
    const LazyCompare                   =   Self::Float.bits  | Self::Stack.bits  | Self::Integer.bits  | Self::Decimal.bits  | Self::Compare.bits                  ;
  }
}

impl          x87flags
{
  pub fn  isCompare
  (
    &self,
  )
  ->  bool
  {
     ( self.bits  & Self::Compare.bits  ) !=  0
  }

  pub fn  isDecimal
  (
    &self,
  )
  ->  bool
  {
     ( self.bits  & Self::Decimal.bits  ) !=  0
  }

  pub fn  isFloat
  (
    &self,
  )
  ->  bool
  {
     ( self.bits  & Self::Float.bits    ) !=  0
  }

  pub fn  isInteger
  (
    &self,
  )
  ->  bool
  {
     ( self.bits  & Self::Integer.bits  ) !=  0
  }

  pub fn  isPop
  (
    &self,
  )
  ->  bool
  {
     ( self.bits  & Self::Pop.bits      ) !=  0
  }

  pub fn  isStack
  (
    &self,
  )
  ->  bool
  {
     ( self.bits  & Self::Stack.bits    ) !=  0
  }

  pub fn  isStore
  (
    &self,
  )
  ->  bool
  {
     ( self.bits  & Self::Store.bits    ) !=  0
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      x87version
{
  None,
  i8087,
  i287,
  i387,
  Pentium,
}

impl          x87version
{
  pub fn name
  (
    &self,
  )
  ->  &'static str
  {
    match self
    {
      x87version::None    =>  "None",
      x87version::i8087   =>  "8087",
      x87version::i287    =>  "80287",
      x87version::i387    =>  "80387",
      x87version::Pentium =>  "Pentium",
    }
  }
}
