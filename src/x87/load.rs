use super::
{
  x87,
  x87flags,
  operands::
  {
    x87operand,
  },
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
  assemblyOneOperand! ( x87fbld,  x87::fbld,  );
  assemblyOneOperand! ( x87fild,  x87::fild,  );
  assemblyOneOperand! ( x87fld,   x87::fld,   );
}

impl          x86instruction
{
  pub fn compileFloatLoad
  (
    mut self,
    fwait:                              bool,
    operands:                           &Vec < OperandType >,
    flags:                              x87flags,
    cpu:                                x86version,
    operandSize:                        usize,
    addressSize:                        usize,
  )
  ->  x86result
  {
    if  operands.len  ( ) ==  1
    {
      self.setFWait             ( fwait           );
      match &operands  [ 0 ]
      {
        OperandType::x87  ( x87operand::Stack     ( number                                          ) )
        if  *number < 8
        &&  flags.isStack ( ) ==  true
        =>  {
              self.setOpcode    ( 0xd9            );
              self.setModRegRM  ( 0xc0  | number  );
              x86result::Done   ( self            )
            },
        OperandType::x86  ( x86operand::Memory16  { kind, size, segment,  registers,  displacement, } )
        =>  match *kind
            {
              asmKind::Integer
              if  flags.isInteger ( ) ==  true
              =>  match *size
                  {
                    asm::Word
                    =>  self.encodeModRegRMdata
                        (
                          0xdf,
                          false,
                          asm::Null,
                          *segment,
                          0,
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
                          0,
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
                          0xdf,
                          false,
                          asm::Null,
                          *segment,
                          5,
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
              asmKind::IEEE754
              if  flags.isFloat   ( ) ==  true
              =>  match *size
                  {
                    asm::DWord
                    =>  self.encodeModRegRMdata
                        (
                          0xd9,
                          false,
                          asm::Null,
                          *segment,
                          0,
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
                          0,
                          *registers  as  u8,
                          Some  ( *displacement ),
                          None,
                          cpu,
                          operandSize,
                          addressSize,
                        ),
                    asm::TWord
                    =>  self.encodeModRegRMdata
                        (
                          0xdb,
                          false,
                          asm::Null,
                          *segment,
                          5,
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
              asmKind::BCD
              if  flags.isDecimal ( ) ==  true
              =>  if  *size ==  asm::TWord
                  {
                    self.encodeModRegRMdata
                    (
                      0xdf,
                      false,
                      asm::Null,
                      *segment,
                      4,
                      *registers  as  u8,
                      Some  ( *displacement ),
                      None,
                      cpu,
                      operandSize,
                      addressSize,
                    )
                  }
                  else
                  {
                    x86result::InvalidOperandSize
                  },
              _
              =>  x86result::InvalidArgumentType,
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
  x87oneOperand!      ( fbld,     x87::FBLD,  );
  x87oneOperand!      ( fild,     x87::FILD,  );
  x87oneOperand!      ( fld,      x87::FLD,   );
}
