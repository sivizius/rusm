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
  assemblyOneOperand! ( x87ffree,   x87::ffree,   );
  assemblyOneOperand! ( x87ffreep,  x87::ffreep,  );
  assemblyOneOperand! ( x87fucom,   x87::fucom,   );
  assemblyOneOperand! ( x87fucomp,  x87::fucomp,  );
  assemblyOneOperand! ( x87fxch,    x87::fxch,    );
}

impl          x86instruction
{
  pub fn compileFloatRegOperand
  (
    mut self,
    opcode:                             u8,
    fwait:                              bool,
    subcode:                            u8,
    operands:                           &Vec < OperandType >,
    expected:                           x87expected,
    fpu:                                x87version,
  )
  ->  x86result
  {
    if  operands.len  ( ) ==  1
    {
      match &operands [ 0 ]
      {
        OperandType::x87  ( x87operand::Stack ( number  ) )
        if  *number < 8
        =>  {
              self.setOpcode    ( 0xd8    | opcode  );
              self.setFWait     ( fwait             );
              self.setModRegRM  ( subcode | *number );
              expected.result
              (
                fpu,
                self,
              )
            },
        _
        =>  x86result::InvalidArgumentType,
      }
    }
    else
    {
      x86result::InvalidNumberOfArguments ( 1 )
    }
  }
}

impl          x87
{
  x87oneOperand!      ( ffree,      x87::FFREE,   );
  x87oneOperand!      ( ffreep,     x87::FFREEP,  );
  x87oneOperand!      ( fucom,      x87::FUCOM,   );
  x87oneOperand!      ( fucomp,     x87::FUCOMP,  );
  x87oneOperand!      ( fxch,       x87::FXCH,    );
}
