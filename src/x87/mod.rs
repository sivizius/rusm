pub mod operands;
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
    x86result,
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
      let     _operands                 =   &operands;
      if  let InstructionType::x87
              {
                architecture:           ref mut state,
                instruction:            this,
              } = self.thisRefMut ( )
      {
        let
        (
          _cpu,
          fpu,
          _operandSize,
          _addressSize,
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
            {
              x87::F2XM1    =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xf0, fpu,  x87expected::Default,   ),
              x87::FABS     =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xe1, fpu,  x87expected::Default,   ),
              x87::FADD     =>  x86result::NotImplemented           ( "fadd",                                           ),
              x87::FADDP    =>  x86result::NotImplemented           ( "faddp",                                          ),
              x87::FBLD     =>  x86result::NotImplemented           ( "fbld",                                           ),
              x87::FBSTP    =>  x86result::NotImplemented           ( "fbstp",                                          ),
              x87::FCHS     =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xe0, fpu,  x87expected::Default,   ),
              x87::FCLEX    =>  instruction.compileFloatZeroOperand ( 0x03, true,   0xe2, fpu,  x87expected::Default,   ),
              x87::FCMOVB   =>  x86result::NotImplemented           ( "fcmovb",                                         ),
              x87::FCMOVBE  =>  x86result::NotImplemented           ( "fcmovbe",                                        ),
              x87::FCMOVE   =>  x86result::NotImplemented           ( "fcmove",                                         ),
              x87::FCMOVNB  =>  x86result::NotImplemented           ( "fcmovnb",                                        ),
              x87::FCMOVNBE =>  x86result::NotImplemented           ( "fcmovnbe",                                       ),
              x87::FCMOVNE  =>  x86result::NotImplemented           ( "fcmovne",                                        ),
              x87::FCMOVNU  =>  x86result::NotImplemented           ( "fcmovnu",                                        ),
              x87::FCMOVU   =>  x86result::NotImplemented           ( "fcmovu",                                         ),
              x87::FCOM     =>  x86result::NotImplemented           ( "fcom",                                           ),
              x87::FCOMI    =>  x86result::NotImplemented           ( "fcomi",                                          ),
              x87::FCOMIP   =>  x86result::NotImplemented           ( "fcomip",                                         ),
              x87::FCOMP    =>  x86result::NotImplemented           ( "fcomp",                                          ),
              x87::FCOMP3   =>  x86result::NotImplemented           ( "fcomp3",                                         ),
              x87::FCOMP5   =>  x86result::NotImplemented           ( "fcomp5",                                         ),
              x87::FCOMPP   =>  instruction.compileFloatZeroOperand ( 0x06, false,  0xd9, fpu,  x87expected::Default,   ),
              x87::FCOS     =>  x86result::NotImplemented           ( "fcos",                                           ),
              x87::FDECSTP  =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xf6, fpu,  x87expected::Default,   ),
              x87::FDISI    =>  instruction.compileFloatZeroOperand ( 0x03, true,   0xe1, fpu,  x87expected::Only8087,  ),
              x87::FDIV     =>  x86result::NotImplemented           ( "fdiv",                                           ),
              x87::FDIVP    =>  x86result::NotImplemented           ( "fdivp",                                          ),
              x87::FDIVR    =>  x86result::NotImplemented           ( "fdivr",                                          ),
              x87::FDIVRP   =>  x86result::NotImplemented           ( "fdivrp",                                         ),
              x87::FENI     =>  instruction.compileFloatZeroOperand ( 0x03, true,    0xe0, fpu,  x87expected::Only8087, ),
              x87::FFREE    =>  x86result::NotImplemented           ( "ffree",                                          ),
              x87::FFREEP   =>  x86result::NotImplemented           ( "ffreep",                                         ),
              x87::FIADD    =>  x86result::NotImplemented           ( "fiadd",                                          ),
              x87::FICOM    =>  x86result::NotImplemented           ( "ficom",                                          ),
              x87::FICOMP   =>  x86result::NotImplemented           ( "ficomp",                                         ),
              x87::FIDIV    =>  x86result::NotImplemented           ( "fidiv",                                          ),
              x87::FIDIVR   =>  x86result::NotImplemented           ( "fidivr",                                         ),
              x87::FILD     =>  x86result::NotImplemented           ( "fild",                                           ),
              x87::FIMUL    =>  x86result::NotImplemented           ( "fimul",                                          ),
              x87::FINCSTP  =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xf7, fpu,  x87expected::Default,   ),
              x87::FINIT    =>  instruction.compileFloatZeroOperand ( 0x03, true,   0xe3, fpu,  x87expected::Default,   ),
              x87::FIST     =>  x86result::NotImplemented           ( "fist",                                           ),
              x87::FISTP    =>  x86result::NotImplemented           ( "fistp",                                          ),
              x87::FISTTP   =>  x86result::NotImplemented           ( "fisttp",                                         ),
              x87::FISUB    =>  x86result::NotImplemented           ( "fisub",                                          ),
              x87::FISUBR   =>  x86result::NotImplemented           ( "fisubr",                                         ),
              x87::FLD      =>  x86result::NotImplemented           ( "fld",                                            ),
              x87::FLD1     =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xe8, fpu,  x87expected::Default,   ),
              x87::FLDCW    =>  x86result::NotImplemented           ( "fldcw",                                          ),
              x87::FLDENV   =>  x86result::NotImplemented           ( "fldenv",                                         ),
              x87::FLDL2E   =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xea, fpu,  x87expected::Default,   ),
              x87::FLDL2T   =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xe9, fpu,  x87expected::Default,   ),
              x87::FLDLG2   =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xec, fpu,  x87expected::Default,   ),
              x87::FLDLN2   =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xed, fpu,  x87expected::Default,   ),
              x87::FLDPI    =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xeb, fpu,  x87expected::Default,   ),
              x87::FLDZ     =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xee, fpu,  x87expected::Default,   ),
              x87::FMUL     =>  x86result::NotImplemented           ( "fmul",                                           ),
              x87::FMULP    =>  x86result::NotImplemented           ( "fmulp",                                          ),
              x87::FNCLEX   =>  instruction.compileFloatZeroOperand ( 0x03, false,  0xe2, fpu,  x87expected::Default,   ),
              x87::FNDISI   =>  instruction.compileFloatZeroOperand ( 0x03, false,  0xe1, fpu,  x87expected::Only8087,  ),
              x87::FNENI    =>  instruction.compileFloatZeroOperand ( 0x03, false,  0xe0, fpu,  x87expected::Only8087,  ),
              x87::FNINIT   =>  instruction.compileFloatZeroOperand ( 0x03, false,  0xe3, fpu,  x87expected::Default,   ),
              x87::FNOP     =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xd0, fpu,  x87expected::Default,   ),
              x87::FNSAVE   =>  x86result::NotImplemented           ( "fnsave",                                         ),
              x87::FNSETPM  =>  x86result::NotImplemented           ( "fnsetpm",                                        ),
              x87::FNSTCW   =>  x86result::NotImplemented           ( "fnstcw",                                         ),
              x87::FNSTENV  =>  x86result::NotImplemented           ( "fnstenv",                                        ),
              x87::FNSTSW   =>  x86result::NotImplemented           ( "fnstsw",                                         ),
              x87::FPATAN   =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xf3, fpu,  x87expected::Default,   ),
              x87::FPREM    =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xf8, fpu,  x87expected::Default,   ),
              x87::FPREM1   =>  x86result::NotImplemented           ( "fprem1",                                         ),
              x87::FPTAN    =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xf2, fpu,  x87expected::Default,   ),
              x87::FRNDINT  =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xfc, fpu,  x87expected::Default,   ),
              x87::FRSTOR   =>  x86result::NotImplemented           ( "frstor",                                         ),
              x87::FSAVE    =>  x86result::NotImplemented           ( "fsave",                                          ),
              x87::FSCALE   =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xfd, fpu,  x87expected::Default,   ),
              x87::FSIN     =>  x86result::NotImplemented           ( "fsin",                                           ),
              x87::FSINCOS  =>  x86result::NotImplemented           ( "fsincos",                                        ),
              x87::FSQRT    =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xfa, fpu,  x87expected::Default,   ),
              x87::FST      =>  x86result::NotImplemented           ( "fst",                                            ),
              x87::FSTCW    =>  x86result::NotImplemented           ( "fstcw",                                          ),
              x87::FSTENV   =>  x86result::NotImplemented           ( "fstenv",                                         ),
              x87::FSTP     =>  x86result::NotImplemented           ( "fstp",                                           ),
              x87::FSTP1    =>  x86result::NotImplemented           ( "fstp1",                                          ),
              x87::FSTP8    =>  x86result::NotImplemented           ( "fstp8",                                          ),
              x87::FSTP9    =>  x86result::NotImplemented           ( "fstp9",                                          ),
              x87::FSTSW    =>  x86result::NotImplemented           ( "fstsw",                                          ),
              x87::FSUB     =>  x86result::NotImplemented           ( "fsub",                                           ),
              x87::FSUBP    =>  x86result::NotImplemented           ( "fsubp",                                          ),
              x87::FSUBR    =>  x86result::NotImplemented           ( "fsubr",                                          ),
              x87::FSUBRP   =>  x86result::NotImplemented           ( "fsubrp",                                         ),
              x87::FTST     =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xe4, fpu,  x87expected::Default,   ),
              x87::FUCOM    =>  x86result::NotImplemented           ( "fucom",                                          ),
              x87::FUCOMI   =>  x86result::NotImplemented           ( "fucomi",                                         ),
              x87::FUCOMIP  =>  x86result::NotImplemented           ( "fucomip",                                        ),
              x87::FUCOMP   =>  x86result::NotImplemented           ( "fucomp",                                         ),
              x87::FUCOMPP  =>  x86result::NotImplemented           ( "fucompp",                                        ),
              x87::FWAIT    =>  instruction.compileZeroOperand      ( 0x9b,                                             ),
              x87::FXAM     =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xe5, fpu,  x87expected::Default,   ),
              x87::FXCH     =>  x86result::NotImplemented           ( "fxch",                                           ),
              x87::FXCH4    =>  x86result::NotImplemented           ( "fxch4",                                          ),
              x87::FXCH7    =>  x86result::NotImplemented           ( "fxch7",                                          ),
              x87::FXRSTOR  =>  x86result::NotImplemented           ( "fxrstor",                                        ),
              x87::FXSAVE   =>  x86result::NotImplemented           ( "fxsave",                                         ),
              x87::FXTRACT  =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xf4, fpu,  x87expected::Default,   ),
              x87::FYL2X    =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xf1, fpu,  x87expected::Default,   ),
              x87::FYL2XP1  =>  instruction.compileFloatZeroOperand ( 0x01, false,  0xf9, fpu,  x87expected::Default,   ),
            };
          self.x86processResult
          (
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
  FCOMP3,
  FCOMP5,
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
  FSTP1,
  FSTP8,
  FSTP9,
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
  FXCH4,
  FXCH7,
  FXRSTOR,
  FXSAVE,
  FXTRACT,
  FYL2X,
  FYL2XP1,
}

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      x87expected
{
  Default,
  Only8087,
  Over80387,
}

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      x87version
{
  None,
  i8087,
  i287,
  i387,
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
      x87version::None  =>  "None",
      x87version::i8087 =>  "8087",
      x87version::i287  =>  "80287",
      x87version::i387  =>  "80387",
    }
  }
}
