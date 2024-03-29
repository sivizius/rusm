use super::
{
  x87,
  x87expected,
  x87version,
  super::
  {
    Assembly,
    instructions::
    {
      Instruction,
      InstructionType,
    },
    operands::
    {
      OperandType,
    },
    x86::
    {
      x86instruction,
      x86result,
      state::
      {
        x86state,
      },
    },
  },
};

impl          Assembly
{
  assemblyZeroOperand!  ( x87f2xm1,   x87::f2xm1,   );
  assemblyZeroOperand!  ( x87fabs,    x87::fabs,    );
  assemblyZeroOperand!  ( x87fchs,    x87::fchs,    );
  assemblyZeroOperand!  ( x87fclex,   x87::fclex,   );
  assemblyZeroOperand!  ( x87fcompp,  x87::fcompp,  );
  assemblyZeroOperand!  ( x87fcos,    x87::fcos,    );
  assemblyZeroOperand!  ( x87fdecstp, x87::fdecstp, );
  assemblyZeroOperand!  ( x87fdisi,   x87::fdisi,   );
  assemblyZeroOperand!  ( x87feni,    x87::feni,    );
  assemblyZeroOperand!  ( x87fincstp, x87::fincstp, );
  assemblyZeroOperand!  ( x87finit,   x87::finit,   );
  assemblyZeroOperand!  ( x87fld1,    x87::fld1,    );
  assemblyZeroOperand!  ( x87fldl2e,  x87::fldl2e,  );
  assemblyZeroOperand!  ( x87fldl2t,  x87::fldl2t,  );
  assemblyZeroOperand!  ( x87fldlg2,  x87::fldlg2,  );
  assemblyZeroOperand!  ( x87fldln2,  x87::fldln2,  );
  assemblyZeroOperand!  ( x87fldpi,   x87::fldpi,   );
  assemblyZeroOperand!  ( x87fldz,    x87::fldz,    );
  assemblyZeroOperand!  ( x87fnclex,  x87::fnclex,  );
  assemblyZeroOperand!  ( x87fndisi,  x87::fndisi,  );
  assemblyZeroOperand!  ( x87fneni,   x87::fneni,   );
  assemblyZeroOperand!  ( x87fninit,  x87::fninit,  );
  assemblyZeroOperand!  ( x87fnop,    x87::fnop,    );
  assemblyZeroOperand!  ( x87fnsetpm, x87::fnsetpm, );
  assemblyZeroOperand!  ( x87fpatan,  x87::fpatan,  );
  assemblyZeroOperand!  ( x87fprem,   x87::fprem,   );
  assemblyZeroOperand!  ( x87fprem1,  x87::fprem1,  );
  assemblyZeroOperand!  ( x87fptan,   x87::fptan,   );
  assemblyZeroOperand!  ( x87frndint, x87::frndint, );
  assemblyZeroOperand!  ( x87fscale,  x87::fscale,  );
  assemblyZeroOperand!  ( x87fsin,    x87::fsin,    );
  assemblyZeroOperand!  ( x87fsincos, x87::fsincos, );
  assemblyZeroOperand!  ( x87fsqrt,   x87::fsqrt,   );
  assemblyZeroOperand!  ( x87ftst,    x87::ftst,    );
  assemblyZeroOperand!  ( x87fucompp, x87::fucompp, );
  assemblyZeroOperand!  ( x87fwait,   x87::fwait,   );  //  but encoded without escape
  assemblyZeroOperand!  ( x87fxam,    x87::fxam,    );
  assemblyZeroOperand!  ( x87fxtract, x87::fxtract, );
  assemblyZeroOperand!  ( x87fyl2x,   x87::fyl2x,   );
  assemblyZeroOperand!  ( x87fyl2xp1, x87::fyl2xp1, );
}

impl          x86instruction
{
  pub fn compileFloatZeroOperand
  (
    mut self,
    opcode:                             u8,
    fwait:                              bool,
    modRegRM:                           u8,
    operands:                           &Vec < OperandType >,
    expected:                           x87expected,
    fpu:                                x87version,
  )
  ->  x86result
  {
    if  operands.len  ( ) ==  0
    {
      self.setOpcode    ( 0xd8  | opcode  );
      self.setFWait     ( fwait           );
      self.setModRegRM  ( modRegRM        );
      expected.result
      (
        fpu,
        self,
      )
    }
    else
    {
      x86result::InvalidNumberOfArguments ( 0 )
    }
  }
}

impl          x87
{
  x87zeroOperand!       ( f2xm1,      x87::F2XM1,   );
  x87zeroOperand!       ( fabs,       x87::FABS,    );
  x87zeroOperand!       ( fchs,       x87::FCHS,    );
  x87zeroOperand!       ( fclex,      x87::FCLEX,   );
  x87zeroOperand!       ( fcompp,     x87::FCOMPP,  );
  x87zeroOperand!       ( fcos,       x87::FCOS,    );
  x87zeroOperand!       ( fdecstp,    x87::FDECSTP, );
  x87zeroOperand!       ( fdisi,      x87::FDISI,   );
  x87zeroOperand!       ( feni,       x87::FENI,    );
  x87zeroOperand!       ( fincstp,    x87::FINCSTP, );
  x87zeroOperand!       ( finit,      x87::FINIT,   );
  x87zeroOperand!       ( fld1,       x87::FLD1,    );
  x87zeroOperand!       ( fldl2e,     x87::FLDL2E,  );
  x87zeroOperand!       ( fldl2t,     x87::FLDL2T,  );
  x87zeroOperand!       ( fldlg2,     x87::FLDLG2,  );
  x87zeroOperand!       ( fldln2,     x87::FLDLN2,  );
  x87zeroOperand!       ( fldpi,      x87::FLDPI,   );
  x87zeroOperand!       ( fldz,       x87::FLDZ,    );
  x87zeroOperand!       ( fnclex,     x87::FNCLEX,  );
  x87zeroOperand!       ( fndisi,     x87::FNDISI,  );
  x87zeroOperand!       ( fneni,      x87::FNENI,   );
  x87zeroOperand!       ( fninit,     x87::FNINIT,  );
  x87zeroOperand!       ( fnop,       x87::FNOP,    );
  x87zeroOperand!       ( fnsetpm,    x87::FNSETPM, );
  x87zeroOperand!       ( fpatan,     x87::FPATAN,  );
  x87zeroOperand!       ( fprem,      x87::FPREM,   );
  x87zeroOperand!       ( fprem1,     x87::FPREM1,  );
  x87zeroOperand!       ( fptan,      x87::FPTAN,   );
  x87zeroOperand!       ( frndint,    x87::FRNDINT, );
  x87zeroOperand!       ( fscale,     x87::FSCALE,  );
  x87zeroOperand!       ( fsin,       x87::FSIN,    );
  x87zeroOperand!       ( fsincos,    x87::FSINCOS, );
  x87zeroOperand!       ( fsqrt,      x87::FSQRT,   );
  x87zeroOperand!       ( ftst,       x87::FTST,    );
  x87zeroOperand!       ( fucompp,    x87::FUCOMPP, );
  x87zeroOperand!       ( fwait,      x87::FWAIT,   );  //  but encoded without escape
  x87zeroOperand!       ( fxam,       x87::FXAM,    );
  x87zeroOperand!       ( fxtract,    x87::FXTRACT, );
  x87zeroOperand!       ( fyl2x,      x87::FYL2X,   );
  x87zeroOperand!       ( fyl2xp1,    x87::FYL2XP1, );
}
