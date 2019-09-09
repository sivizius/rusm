#![allow(non_camel_case_types)]

use super::
{
  super::
  {
    Architecture,
    Assembly,
    instructions::
    {
      Instruction,
      InstructionResult,
      InstructionType,
    },
  },
};

impl          Assembly
{
  pub fn x86config
  (
    self,
    version:                            x86version,
    operandSize:                        usize,
    addressSize:                        usize,
  )
  ->  Self
  {
    self.push
    (
      x86prefix::config
      (
        version,
        operandSize,
        addressSize,
      )
    )
  }

  assemblyZeroOperand!  ( x86lock,            x86prefix::lock,            );
  assemblyZeroOperand!  ( x86rep,             x86prefix::rep,             );
  assemblyZeroOperand!  ( x86repe,            x86prefix::repe,            );
  assemblyZeroOperand!  ( x86repne,           x86prefix::repne,           );
  assemblyZeroOperand!  ( x86repnz,           x86prefix::repnz,           );
  assemblyZeroOperand!  ( x86repz,            x86prefix::repz,            );
  assemblyZeroOperand!  ( x86usualleNotTaken, x86prefix::usuallyNotTaken, );
  assemblyZeroOperand!  ( x86usualleTaken,    x86prefix::usuallyTaken,    );
}

impl          Instruction
{
  pub fn x86prefixCompile
  (
    &mut self,
    architecture:                       &mut Architecture,
  )
  ->  InstructionResult
  {
    if  let InstructionType::x86prefix  ( prefix  ) = self.thisRef  ( )
    {
      if        let x86prefix::Config
                    {
                      version,
                      operandSize,
                      addressSize,
                    } = prefix
      {
        *architecture
        = Architecture::x86
          (
            x86state
            {
              version:                  *version,
              operandSize:              *operandSize,
              addressSize:              *addressSize,
              hazLock:                  false,
              theBranchHint:            x86prefixByte::Default,
              theRepeat:                x86prefixByte::Default,
            },
          );
        InstructionResult::Ready
      }
      else  if  let Architecture::x86 ( ref mut state ) = architecture
      {
        match prefix
        {
          x86prefix::Lock             =>  state.hazLock       =   true,
          x86prefix::Repeat           =>  state.theRepeat     =   x86prefixByte::Repeat,
          x86prefix::RepeatEqual      =>  state.theRepeat     =   x86prefixByte::RepeatEqual,
          x86prefix::RepeatNotEqual   =>  state.theRepeat     =   x86prefixByte::RepeatNotEqual,
          x86prefix::RepeatNotZero    =>  state.theRepeat     =   x86prefixByte::RepeatNotZero,
          x86prefix::RepeatZero       =>  state.theRepeat     =   x86prefixByte::RepeatZero,
          x86prefix::UsuallyNotTaken  =>  state.theBranchHint =   x86prefixByte::BranchNotTaken,
          x86prefix::UsuallyTaken     =>  state.theBranchHint =   x86prefixByte::BranchTaken,
          _                           =>  unreachable!  ( ),
        }
        InstructionResult::Ready
      }
      else
      {
        InstructionResult::Again.wrongArchitecture  ( *architecture,  "x86",        )
      }
    }
    else
    {
      InstructionResult::Again.wrongInstructionSet  ( self.this ( ),  "x86prefix",  )
    }
  }
}

#[derive(Copy,Clone,Debug,PartialEq,PartialOrd)]
pub enum      x86prefix
{
  Config
  {
    version:                            x86version,
    operandSize:                        usize,
    addressSize:                        usize,
  },
  Lock,
  Repeat,
  RepeatEqual,
  RepeatNotEqual,
  RepeatNotZero,
  RepeatZero,
  UsuallyNotTaken,
  UsuallyTaken,
}

macro_rules!  x86prefix
{
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
            InstructionType::x86prefix  ( $theInstruction ),
            0,
            vec!  ( ),
          )
        }
      }
}

impl          x86prefix
{
  pub fn config
  (
    version:                            x86version,
    operandSize:                        usize,
    addressSize:                        usize,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::x86prefix
      (
        x86prefix::Config
        {
          version,
          operandSize,
          addressSize,
        }
      ),
      0,
      vec!  ( ),
    )
  }
  x86prefix!            ( lock,               x86prefix::Lock,            );
  x86prefix!            ( rep,                x86prefix::Repeat,          );
  x86prefix!            ( repe,               x86prefix::RepeatEqual,     );
  x86prefix!            ( repne,              x86prefix::RepeatNotEqual,  );
  x86prefix!            ( repnz,              x86prefix::RepeatNotZero,   );
  x86prefix!            ( repz,               x86prefix::RepeatZero,      );
  x86prefix!            ( usuallyNotTaken,    x86prefix::UsuallyNotTaken, );
  x86prefix!            ( usuallyTaken,       x86prefix::UsuallyTaken,    );
}

Const!
{
  pub x86prefixByte:                    u8
  {
    Default                             =   0x00,
    AddressSizeOverride                 =   0x67,
    BranchNotTaken                      =   0x2e,
    BranchTaken                         =   0x3e,
    Lock                                =   0xf0,
    OperandSizeOverride                 =   0x66,
    Repeat                              =   0xf3,
    RepeatEqual                         =   x86prefixByte::Repeat.0,
    RepeatNotEqual                      =   0xf2,
    RepeatNotZero                       =   x86prefixByte::RepeatNotEqual.0,
    RepeatZero                          =   x86prefixByte::Repeat.0,
    SegmentOverrideCS                   =   0x26,
    SegmentOverrideSS                   =   0x2e,
    SegmentOverrideDS                   =   0x36,
    SegmentOverrideES                   =   0x3e,
    SegmentOverrideFS                   =   0x64,
    SegmentOverrideGS                   =   0x65,
    ThreeByteXOP                        =   0x8f,
    TwoByteVEX                          =   0xc5,
    ThreeByteVEX                        =   0xc4,
    TwoByteOpcode                       =   0x0f,
  }
}

impl          x86prefixByte
{
  pub fn toByte
  (
    &self
  )
  ->  u8
  {
    self.0
  }
}

#[derive(Copy,Clone,Debug,PartialEq,PartialOrd)]
pub struct    x86state
{
  version:                              x86version,
  operandSize:                          usize,
  addressSize:                          usize,
  hazLock:                              bool,
  theBranchHint:                        x86prefixByte,
  theRepeat:                            x86prefixByte,
}

pub fn x86state
(
)
->  x86state
{
  x86state
  {
    version:                            x86version::i8086,
    operandSize:                        2,
    addressSize:                        2,
    hazLock:                            false,
    theBranchHint:                      x86prefixByte::Default,
    theRepeat:                          x86prefixByte::Default,
  }
}

impl          x86state
{
  pub fn addressSize
  (
    &self,
  )
  ->  usize
  {
    self.addressSize
  }

  pub fn hazLock
  (
    &self
  )
  ->  bool
  {
    self.hazLock
  }

  pub fn operandSize
  (
    &self,
  )
  ->  usize
  {
    self.operandSize
  }

  pub fn reset
  (
    &mut self
  )
  {
    self.hazLock                        =   false;
    self.theBranchHint                  =   x86prefixByte::Default;
    self.theRepeat                      =   x86prefixByte::Default;
  }

  pub fn theBranchHint
  (
    &self
  )
  ->  x86prefixByte
  {
    self.theBranchHint
  }

  pub fn theRepeat
  (
    &self
  )
  ->  x86prefixByte
  {
    self.theRepeat
  }

  pub fn version
  (
    &self
  )
  ->  x86version
  {
    self.version
  }
}

#[derive(Copy,Clone,Debug,PartialEq,PartialOrd)]
pub enum      x86version
{
  i8086,
  i186,
  i286,
  i386,
  i486,
  Pentium,
  Pentium2,
  amd64,
}
