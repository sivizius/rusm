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
    SegmentRegisterNumber,
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

macro_rules!  declareSimpleMathInstruction
{
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
        ->  Instruction
        {
          let     ( dstThis, dstSize, ) =   dst.this  ( );
          let     ( srcThis, srcSize, ) =   src.this  ( );
          let size                      =   ( dstSize | srcSize ) as  usize;
          Instruction
          (
            InstructionType::x86
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

impl          Assembly
{
  assemblyTwoOperand!           ( x86add, x86::add, );
  assemblyTwoOperand!           ( x86or,  x86::or,  );
  assemblyTwoOperand!           ( x86adc, x86::adc, );
  assemblyTwoOperand!           ( x86sbb, x86::sbb, );
  assemblyTwoOperand!           ( x86and, x86::and, );
  assemblyTwoOperand!           ( x86sub, x86::sub, );
  assemblyTwoOperand!           ( x86xor, x86::xor, );
  assemblyTwoOperand!           ( x86cmp, x86::cmp, );
}

impl          x86
{
  declareSimpleMathInstruction! ( add,    x86::ADD, );
  declareSimpleMathInstruction! ( or,     x86::OR,  );
  declareSimpleMathInstruction! ( adc,    x86::ADC, );
  declareSimpleMathInstruction! ( sbb,    x86::SBB, );
  declareSimpleMathInstruction! ( and,    x86::AND, );
  declareSimpleMathInstruction! ( sub,    x86::SUB, );
  declareSimpleMathInstruction! ( xor,    x86::XOR, );
  declareSimpleMathInstruction! ( cmp,    x86::CMP, );
}

impl          x86instruction
{
  pub fn compileSimpleMath
  (
    mut self,
    //  Instruction
    opcode:                             u8,
    signExtension:                      bool,
    size:                               usize,
    operands:                           &Vec < OperandType >,
    //  Assembly
    version:                            x86version,
    operandSize:                        usize,
    addressSize:                        usize,
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
          OperandType::x86          ( x86operand::GeneralPurposeRegister  { size: _,  rex:      _dstREX,      number:     dstRegister,                                  } ),
          OperandType::Constant     ( immediate                                                                                                                           ),
        )
        |
        (
          OperandType::x86          ( x86operand::GeneralPurposeRegister  { size: _,  rex:      _dstREX,      number:     dstRegister,                                  } ),
          OperandType::Displacement ( immediate                                                                                                                           ),
        )
        =>  if  ( *dstRegister  ==  GeneralPurposeRegisterNumber::AX                              )
//            && !( self.features.hazFeature ( AssemblyFeatures::RandomOpcodeSize ) && rand::random() )
            {
              let     immediate         =   *immediate;
              self.setImmediate
              (
                size,
                immediate,
              );
              match size
              {
                asm::Byte
                =>  if  immediate >= -0x80
                    &&  immediate <=  0xff
                    {
                      self.setOpcode                 ( opcode  | 4 );
                      x86result::Done ( self  )
                    }
                    else
                    {
                      x86result::OutOfBounds
                      {
                        number:         1,
                        value:          immediate,
                        minimum:        -0x80,
                        maximum:        0xff,
                      }
                    },
                asm::Word
                =>  if  immediate >= -0x8000
                    &&  immediate <=  0xffff
                    {
                      self.setOpcode                 ( opcode  | 5 );
                      if  operandSize !=  asm::Word
                      {
                        self.setOperandSizeOverride  ( true        );
                      }
                      x86result::Done ( self  )
                    }
                    else
                    {
                      x86result::OutOfBounds
                      {
                        number:         1,
                        value:          immediate,
                        minimum:        -0x8000,
                        maximum:        0xffff,
                      }
                    },
                asm::DWord
                =>  if  version >= x86version::i386
                    {
                      if  immediate >= -0x80000000
                      &&  immediate <=  0xffffffff
                      {
                        self.setOpcode                 ( opcode  | 5 );
                        if  operandSize ==  asm::Word
                        {
                          self.setOperandSizeOverride  ( true        );
                        }
                        x86result::Done ( self  )
                      }
                      else
                      {
                        x86result::OutOfBounds
                        {
                          number:       1,
                          value:        immediate,
                          minimum:      -0x80000000,
                          maximum:      0xffffffff,
                        }
                      }
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
              self.encodeModRegRMdata
              (
                if  size    ==  asm::Byte
                &&  version <   x86version::amd64
                &&  false
                //&&  self.features.hazFeature ( AssemblyFeatures::RandomOpcode )
                //&&  rand::random()
                {
                  //  0x80 and 0x82 are aliases, but 0x82 is invalid for 64 bit.
                  //  because 0x80 is the default encoding, some disassemblers fail with 0x82.
                  0x82
                }
                else
                {
                  0x80
                },
                signExtension,
                size,
                SegmentRegisterNumber::None,
                0,
                *dstRegister  as  u8  | opcode,
                None,
                Some  ( *immediate ),
                version,
                operandSize,
                addressSize,
              )
            },
        (
          OperandType::x86          ( x86operand::Memory16                { size: _,  segment:  dstSegment,   registers:  dstRegisters, displacement: dstDisplacement,  } ),
          OperandType::Constant     ( immediate                                                                                                                           ),
        )
        |
        (
          OperandType::x86          ( x86operand::Memory16                { size: _,  segment:  dstSegment,   registers:  dstRegisters, displacement: dstDisplacement,  } ),
          OperandType::Displacement ( immediate                                                                                                                           ),
        )
        =>  self.encodeModRegRMdata
            (
              if  size    ==  asm::Byte
              &&  version <   x86version::amd64
              &&  false
              //&&  self.features.hazFeature ( AssemblyFeatures::RandomOpcode )
              //&&  rand::random()
              {
                //  0x80 and 0x82 are aliases, but 0x82 is invalid for 64 bit.
                //  because 0x80 is the default encoding, some disassemblers fail with 0x82.
                0x82
              }
              else
              {
                0x80
              },
              signExtension,
              size,
              *dstSegment,
              0,
              *dstRegisters as  u8  | opcode,
              Some  ( *dstDisplacement  ),
              Some  ( *immediate ),
              version,
              operandSize,
              addressSize,
            ),
        (
          OperandType::x86          ( x86operand::GeneralPurposeRegister  { size: _,  rex:      _dstREX,      number:     dstRegister,                                  } ),
          OperandType::x86          ( x86operand::GeneralPurposeRegister  { size: _,  rex:      _srcREX,      number:     srcRegister,                                  } ),
        )
        =>  if  true
            //&&  self.features.hazFeature ( AssemblyFeatures::RandomOpcode )
            //&&  rand::random()
            {
              self.encodeModRegRMdata
              (
                opcode | 2,
                signExtension,
                size,
                SegmentRegisterNumber::None,
                *dstRegister  as  u8,
                *srcRegister  as  u8,
                None,
                None,
                version,
                operandSize,
                addressSize,
              )
            }
            else
            {
              self.encodeModRegRMdata
              (
                opcode | 0,
                signExtension,
                size,
                SegmentRegisterNumber::None,
                ( *srcRegister  as  u8  ) <<  3,
                *dstRegister  as  u8,
                None,
                None,
                version,
                operandSize,
                addressSize,
              )
            },
        (
          OperandType::x86          ( x86operand::GeneralPurposeRegister  { size: _,  rex:      _dstREX,      number:     dstRegister,                                  } ),
          OperandType::x86          ( x86operand::Memory16                { size: _,  segment:  srcSegment,   registers:  srcRegisters, displacement: srcDisplacement,  } ),
        )
        =>  self.encodeModRegRMdata
            (
              opcode | 2,
              signExtension,
              size,
              *srcSegment,
              *dstRegister  as  u8,
              *srcRegisters as  u8,
              Some  ( *srcDisplacement  ),
              None,
              version,
              operandSize,
              addressSize,
            ),
        (
          OperandType::x86          ( x86operand::Memory16                { size: _,  segment:  dstSegment,   registers:  dstRegisters, displacement: dstDisplacement,  } ),
          OperandType::x86          ( x86operand::GeneralPurposeRegister  { size: _,  rex:      _srcREX,      number:     srcRegister,                                  } ),
        )
        =>  self.encodeModRegRMdata
            (
              opcode | 0,
              signExtension,
              size,
              *dstSegment,
              *srcRegister  as  u8,
              *dstRegisters as  u8,
              Some  ( *dstDisplacement  ),
              None,
              version,
              operandSize,
              addressSize,
            ),
        (
          _,
          OperandType::Reference    ( _                                                                                                                                   ),
        )
        =>  x86result::Rerun,
        (
          _,
          _,
        )
        =>  x86result::InvalidCombinationOfArguments,
      }
    }
    else
    {
      x86result::InvalidNumberOfArguments ( 2 )
    }
  }
}
