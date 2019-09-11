use super::
{
  x86,
  x86instruction,
  x86result,
  operands::
  {
    x86operand,
  },
  registers::
  {
    GeneralPurposeRegisterNumber,
  },
  state::
  {
    x86state,
    x86version,
  },
  super::
  {
    Assembly,
    asm::
    {
      asm,
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
  },
};

impl          Assembly
{
  assemblyTwoOperand! ( x86in,  x86::iin, );
  assemblyTwoOperand! ( x86out, x86::out, );
}

impl          x86
{
  pub fn iin
  (
    dst:                                impl  Operand,
    src:                                impl  Operand,
  )
  ->  Instruction
  {
    let     ( dstThis,  dstSize,  )     =   dst.this  ( );
    let     ( srcThis,  _,        )     =   src.this  ( );
    Instruction
    (
      InstructionType::x86
      {
        architecture:                   x86state  ( ),
        instruction:                    x86::IN,
      },
      dstSize,
      vec!
      (
        dstThis,
        srcThis,
      ),
    )
  }

  pub fn out
  (
    dst:                                impl  Operand,
    src:                                impl  Operand,
  )
  ->  Instruction
  {
    let     ( dstThis,  _,        )     =   dst.this  ( );
    let     ( srcThis,  srcSize,  )     =   src.this  ( );
    Instruction
    (
      InstructionType::x86
      {
        architecture:                   x86state  ( ),
        instruction:                    x86::OUT,
      },
      srcSize,
      vec!
      (
        dstThis,
        srcThis,
      ),
    )
  }
}

impl          x86instruction
{
  /// `direction` – Direction: input=false, output=true
  pub fn compileIO
  (
    mut self,
    //  Instruction
    opcode:                             u8,
    direction:                          bool,
    operands:                           &Vec < OperandType >,
    version:                            x86version,
    operandSize:                        usize,
    _addressSize:                       usize,
  )
  ->  x86result
  {
    if  operands.len  ( ) ==  2
    {
      match
      (
        direction,
        &operands [ 0 ],
        &operands [ 1 ],
      )
      {
        (
          false,  //  input
          OperandType::x86      ( x86operand::GeneralPurposeRegister  { size,             rex:  false,  number: GeneralPurposeRegisterNumber::AX, } ),
          OperandType::Constant ( immediate                                                                                                         ),
        )
        |
        (
          true,   //  output
          OperandType::Constant ( immediate                                                                                                         ),
          OperandType::x86      ( x86operand::GeneralPurposeRegister  { size,             rex:  false,  number: GeneralPurposeRegisterNumber::AX, } ),
        )
        =>  if  *immediate  >=  0x00
            &&  *immediate  <=  0xff
            {
              self.setImmediate ( asm::Byte,  *immediate, );
              match *size
              {
                asm::Byte
                =>  {
                      self.setOpcode  ( opcode      );
                      x86result::Done ( self        )
                    }
                asm::Word
                =>  {
                      if  operandSize !=  asm::Word
                      {
                        self.setOperandSizeOverride ( true  );
                      }
                      self.setOpcode  ( opcode  | 1 );
                      x86result::Done ( self        )
                    },
                asm::DWord
                =>  if  version >= x86version::i386
                    {
                      if  operandSize ==  asm::Word
                      {
                        self.setOperandSizeOverride ( true  );
                      }
                      self.setOpcode  ( opcode  | 1 );
                      x86result::Done ( self        )
                    }
                    else
                    {
                      x86result::MinimalVersion ( x86version::i386  )
                    },
                _
                =>  x86result::InvalidOperandSize,
              }
            }
            else
            {
              x86result::OutOfBounds
              {
                number:               0,
                value:                *immediate,
                minimum:              0x00,
                maximum:              0xff,
              }
            },
        (
          false,  //  input
          OperandType::x86      ( x86operand::GeneralPurposeRegister  { size,             rex:  false,  number: GeneralPurposeRegisterNumber::AX, } ),
          OperandType::x86      ( x86operand::GeneralPurposeRegister  { size: asm::Word,  rex:  false,  number: GeneralPurposeRegisterNumber::DX, } ),
        )
        |
        (
          true,   //  output
          OperandType::x86      ( x86operand::GeneralPurposeRegister  { size: asm::Word,  rex:  false,  number: GeneralPurposeRegisterNumber::DX, } ),
          OperandType::x86      ( x86operand::GeneralPurposeRegister  { size,             rex:  false,  number: GeneralPurposeRegisterNumber::AX, } ),
        )
        =>  match *size
            {
              asm::Byte
              =>  {
                    self.setOpcode  ( opcode  | 8 );
                    x86result::Done ( self        )
                  }
              asm::Word
              =>  {
                    if  operandSize !=  asm::Word
                    {
                      self.setOperandSizeOverride ( true  );
                    }
                    self.setOpcode  ( opcode  | 9 );
                    x86result::Done ( self        )
                  },
              asm::DWord
              =>  if  version >= x86version::i386
                  {
                    if  operandSize ==  asm::Word
                    {
                      self.setOperandSizeOverride ( true  );
                    }
                    self.setOpcode  ( opcode  | 9 );
                    x86result::Done ( self        )
                  }
                  else
                  {
                    x86result::MinimalVersion ( x86version::i386  )
                  },
              _
              =>  x86result::InvalidOperandSize,
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