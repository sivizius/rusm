use super::
{
  x87,
  x87version,
  super::
  {
    Assembly,
    asm::
    {
      asm,
      asmKind,
    },
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
      operands::
      {
        x86operand,
      },
      state::
      {
        x86state,
        x86version,
      },
    },
  },
};

impl          Assembly
{
  assemblyOneOperand! ( x87fisttp,  x87::fisttp,  );
}

impl          x86instruction
{
  pub fn compileFloatIntSetTPop
  (
    mut self,
    fwait:                              bool,
    operands:                           &Vec < OperandType >,
    fpu:                                x87version,
    cpu:                                x86version,
    operandSize:                        usize,
    addressSize:                        usize,
  )
  ->  x86result
  {
    if  operands.len  ( ) ==  1
    {
      if  fpu >=  x87version::Pentium
      {
        self.setFWait ( fwait );
        match &operands [ 0 ]
        {
          OperandType::x86  ( x86operand::Memory16  { kind: asmKind::Integer, size,     segment,  registers,  displacement, } )
          =>  match *size
              {
                asm::Word
                =>  self.encodeModRegRMdata
                    (
                      0xdf,
                      false,
                      asm::Null,
                      *segment,
                      1,
                      *registers  as  u8,
                      Some  ( *displacement ),
                      None,
                      cpu,
                      operandSize,
                      addressSize,
                    ),
                asm::DWord
                =>  self.encodeModRegRMdata
                    (
                      0xdb,
                      false,
                      asm::Null,
                      *segment,
                      1,
                      *registers  as  u8,
                      Some  ( *displacement ),
                      None,
                      cpu,
                      operandSize,
                      addressSize,
                    ),
                asm::QWord
                =>  self.encodeModRegRMdata
                    (
                      0xdd,
                      false,
                      asm::Null,
                      *segment,
                      1,
                      *registers  as  u8,
                      Some  ( *displacement ),
                      None,
                      cpu,
                      operandSize,
                      addressSize,
                    ),
                _
                =>  x86result::InvalidOperandSize,
              },
          _
          =>  x86result::InvalidArgumentType,
        }
      }
      else
      {
        x86result::WrongVersion
        {
          have:                         fpu,
          want:                         x87version::Pentium,
        }
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
  x87oneOperand!      ( fisttp,     x87::FISTTP,  );
}