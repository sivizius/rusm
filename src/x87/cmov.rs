use super::
{
  x87,
  x87expected,
  x87version,
  operands::
  {
    x87operand,
  },
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
      Operand,
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
  assemblyTwoOperand! ( x87fcmova,    x87::fcmova,    );
  assemblyTwoOperand! ( x87fcmovae,   x87::fcmovae,   );
  assemblyTwoOperand! ( x87fcmovb,    x87::fcmovb,    );
  assemblyTwoOperand! ( x87fcmovbe,   x87::fcmovbe,   );
  assemblyTwoOperand! ( x87fcmove,    x87::fcmove,    );
  assemblyTwoOperand! ( x87fcmovna,   x87::fcmovna,   );
  assemblyTwoOperand! ( x87fcmovnae,  x87::fcmovnae,  );
  assemblyTwoOperand! ( x87fcmovnb,   x87::fcmovnb,   );
  assemblyTwoOperand! ( x87fcmovnbe,  x87::fcmovnbe,  );
  assemblyTwoOperand! ( x87fcmovne,   x87::fcmovne,   );
  assemblyTwoOperand! ( x87fcmovno,   x87::fcmovno,   );
  assemblyTwoOperand! ( x87fcmovnu,   x87::fcmovnu,   );
  assemblyTwoOperand! ( x87fcmovo,    x87::fcmovo,    );
  assemblyTwoOperand! ( x87fcmovu,    x87::fcmovu,    );
}

impl          x86instruction
{
  pub fn compileFloatCMove
  (
    mut self,
    opcode:                             u8,
    fwait:                              bool,
    subcode:                            u8,
    operands:                           &Vec < OperandType >,
    fpu:                                x87version,
  )
  ->  x86result
  {
    if  operands.len  ( ) ==  2
    {
      match
      (
        &operands [ 0 ],
        &operands [ 1 ],
      )
      {
        (
          OperandType::x87  ( x87operand::Stack ( 0       ) ),
          OperandType::x87  ( x87operand::Stack ( number  ) ),
        )
        if  *number < 8
        =>  {
              self.setOpcode    ( 0xd8    | opcode  );
              self.setFWait     ( fwait             );
              self.setModRegRM  ( subcode | *number );
              x87expected::Minimal
              (
                x87version::Pentium,
              ).result
              (
                fpu,
                self,
              )
            },
        _
        =>  x86result::InvalidCombinationOfArguments,
      }
    }
    else
    {
      x86result::InvalidNumberOfArguments ( 2 )
    }
  }
}

impl          x87
{
  x87twoOperand!      ( fcmova,       x87::FCMOVNBE,  );
  x87twoOperand!      ( fcmovae,      x87::FCMOVNB,   );
  x87twoOperand!      ( fcmovb,       x87::FCMOVB,    );
  x87twoOperand!      ( fcmovbe,      x87::FCMOVBE,   );
  x87twoOperand!      ( fcmove,       x87::FCMOVE,    );
  x87twoOperand!      ( fcmovna,      x87::FCMOVBE,   );
  x87twoOperand!      ( fcmovnae,     x87::FCMOVB,    );
  x87twoOperand!      ( fcmovnb,      x87::FCMOVNB,   );
  x87twoOperand!      ( fcmovnbe,     x87::FCMOVNBE,  );
  x87twoOperand!      ( fcmovne,      x87::FCMOVNE,   );
  x87twoOperand!      ( fcmovno,      x87::FCMOVU,    );
  x87twoOperand!      ( fcmovnu,      x87::FCMOVNU,   );
  x87twoOperand!      ( fcmovo,       x87::FCMOVNU,   );
  x87twoOperand!      ( fcmovu,       x87::FCMOVU,    );
}
