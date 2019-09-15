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
      memory::
      {
        Memory16Registers,
      },
      operands::
      {
        x86operand,
      },
      registers::
      {
        SegmentRegisterNumber,
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
  assemblyTwoOperand! ( x87fadd,    x87::fadd,    );
  assemblyTwoOperand! ( x87faddp,   x87::faddp,   );
  assemblyTwoOperand! ( x87fbstp,   x87::fbstp,   );
  assemblyTwoOperand! ( x87fcom,    x87::fcom,    );
  assemblyTwoOperand! ( x87fcomp,   x87::fcomp,   );
  assemblyTwoOperand! ( x87fdiv,    x87::fdiv,    );
  assemblyTwoOperand! ( x87fdivp,   x87::fdivp,   );
  assemblyTwoOperand! ( x87fdivr,   x87::fdivr,   );
  assemblyTwoOperand! ( x87fdivrp,  x87::fdivrp,  );
  assemblyTwoOperand! ( x87fiadd,   x87::fiadd,   );
  assemblyTwoOperand! ( x87ficom,   x87::ficom,   );
  assemblyTwoOperand! ( x87ficomp,  x87::ficomp,  );
  assemblyTwoOperand! ( x87fidiv,   x87::fidiv,   );
  assemblyTwoOperand! ( x87fidivr,  x87::fidivr,  );
  assemblyTwoOperand! ( x87fimul,   x87::fimul,   );
  assemblyTwoOperand! ( x87fist,    x87::fist,    );
  assemblyTwoOperand! ( x87fistp,   x87::fistp,   );
  assemblyTwoOperand! ( x87fisub,   x87::fisub,   );
  assemblyTwoOperand! ( x87fisubr,  x87::fisubr,  );
  assemblyTwoOperand! ( x87fmul,    x87::fmul,    );
  assemblyTwoOperand! ( x87fmulp,   x87::fmulp,   );
  assemblyTwoOperand! ( x87fst,     x87::fst,     );
  assemblyTwoOperand! ( x87fstp,    x87::fstp,    );
  assemblyTwoOperand! ( x87fsub,    x87::fsub,    );
  assemblyTwoOperand! ( x87fsubp,   x87::fsubp,   );
  assemblyTwoOperand! ( x87fsubr,   x87::fsubr,   );
  assemblyTwoOperand! ( x87fsubrp,  x87::fsubrp,  );
}

impl          x86instruction
{
  pub fn compileFloatMemory
  (
    self,
    opcode:                             u8,
    subcode:                            u8,
    flags:                              x87flags,
    kind:                               asmKind,
    size:                               usize,
    segment:                            SegmentRegisterNumber,
    registers:                          Memory16Registers,
    displacement:                       i128,
    cpu:                                x86version,
    operandSize:                        usize,
    addressSize:                        usize,
  )
  ->  x86result
  {
    match kind
    {
      asmKind::Integer
      if  flags.isInteger ( ) ==  true
      =>  match size
          {
            asm::Word
            =>  self.encodeModRegRMdata
                (
                  0xde  | opcode,
                  false,
                  asm::Null,
                  segment,
                  subcode,
                  registers as  u8,
                  Some  ( displacement  ),
                  None,
                  cpu,
                  operandSize,
                  addressSize,
                ),
            asm::DWord
            =>  self.encodeModRegRMdata
                (
                  0xda  | opcode,
                  false,
                  asm::Null,
                  segment,
                  subcode,
                  registers as  u8,
                  Some  ( displacement  ),
                  None,
                  cpu,
                  operandSize,
                  addressSize,
                ),
            asm::QWord
            if  flags.isPop ( ) ==  true
            =>  self.encodeModRegRMdata
                (
                  0xdf  | opcode,
                  false,
                  asm::Null,
                  segment,
                  subcode,
                  registers as  u8,
                  Some  ( displacement  ),
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
      =>  match size
          {
            asm::DWord
            =>  self.encodeModRegRMdata
                (
                  0xd8  | opcode,
                  false,
                  asm::Null,
                  segment,
                  subcode,
                  registers as  u8,
                  Some  ( displacement  ),
                  None,
                  cpu,
                  operandSize,
                  addressSize,
                ),
            asm::QWord
            =>  self.encodeModRegRMdata
                (
                  0xdc  | opcode,
                  false,
                  asm::Null,
                  segment,
                  subcode,
                  registers as  u8,
                  Some  ( displacement  ),
                  None,
                  cpu,
                  operandSize,
                  addressSize,
                ),
              asm::TWord
              if  flags.isPop ( ) ==  true
              =>  self.encodeModRegRMdata
                  (
                    0xdb,
                    false,
                    asm::Null,
                    segment,
                    7,
                    registers as  u8,
                    Some  ( displacement  ),
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
      =>  if  size  ==  asm::TWord
          {
            self.encodeModRegRMdata
            (
              0xdf,
              false,
              asm::Null,
              segment,
              6,
              registers as  u8,
              Some  ( displacement  ),
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
    }
  }

  pub fn compileFloatMath
  (
    mut self,
    opcode:                             u8,
    fwait:                              bool,
    subcode:                            u8,
    operands:                           &Vec < OperandType >,
    flags:                              x87flags,
    cpu:                                x86version,
    operandSize:                        usize,
    addressSize:                        usize,
  )
  ->  x86result
  {
    self.setFWait                   ( fwait                           );
    if  operands.len  ( ) ==  2
    {
      match
      (
        &operands  [ 0 ],
        &operands  [ 1 ],
      )
      {
        (
          OperandType::x87  ( x87operand::Stack     ( 0                                               ) ),
          OperandType::x87  ( x87operand::Stack     ( number                                          ) ),
        )
        if  *number < 8
        &&  flags.isStack   ( ) ==  true
        &&  flags.isStore   ( ) ==  false
        &&  flags.isPop     ( ) ==  false
        =>  {
              self.setOpcode    ( 0xd8  | opcode        | 0       );
              self.setModRegRM  ( 0xc0  | subcode <<  3 | number  );
              x86result::Done   ( self                            )
            },
        (
          OperandType::x87  ( x87operand::Stack     ( number                                          ) ),
          OperandType::x87  ( x87operand::Stack     ( 0                                               ) ),
        )
        if  *number < 8
        &&  flags.isStack   ( ) ==  true
        &&  flags.isCompare ( ) ==  false
        =>  {
              self.setOpcode    ( 0xd8  | opcode        | 4       );
              self.setModRegRM  ( 0xc0  | subcode <<  3 | number  );
              x86result::Done   ( self                            )
            },
        (
          OperandType::x87  ( x87operand::Stack     ( 0                                               ) ),
          OperandType::x86  ( x86operand::Memory16  { kind, size, segment,  registers,  displacement, } ),
        )
        if  flags.isStore   ( ) ==  false
        &&  flags.isPop     ( ) ==  false
        =>  self.compileFloatMemory
            (
              opcode  & 1,
              subcode,
              flags,
              *kind,
              *size,
              *segment,
              *registers,
              *displacement,
              cpu,
              operandSize,
              addressSize,
            ),
        (
          OperandType::x86  ( x86operand::Memory16  { kind, size, segment,  registers,  displacement, } ),
          OperandType::x87  ( x87operand::Stack     ( 0                                               ) ),
        )
        if  flags.isStore   ( ) ==  true
        =>  self.compileFloatMemory
            (
              opcode  & 1,
              subcode,
              flags,
              *kind,
              *size,
              *segment,
              *registers,
              *displacement,
              cpu,
              operandSize,
              addressSize,
            ),
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

impl          x87
{
  x87twoOperand!      ( fadd,       x87::FADD,    );
  x87twoOperand!      ( faddp,      x87::FADDP,   );
  x87twoOperand!      ( fbstp,      x87::FBSTP,   );
  x87twoOperand!      ( fcom,       x87::FCOM,    );
  x87twoOperand!      ( fcomp,      x87::FCOMP,   );
  x87twoOperand!      ( fdiv,       x87::FDIV,    );
  x87twoOperand!      ( fdivp,      x87::FDIVP,   );
  x87twoOperand!      ( fdivr,      x87::FDIVR,   );
  x87twoOperand!      ( fdivrp,     x87::FDIVRP,  );
  x87twoOperand!      ( fiadd,      x87::FIADD,   );
  x87twoOperand!      ( ficom,      x87::FICOM,   );
  x87twoOperand!      ( ficomp,     x87::FICOMP,  );
  x87twoOperand!      ( fidiv,      x87::FIDIV,   );
  x87twoOperand!      ( fidivr,     x87::FIDIVR,  );
  x87twoOperand!      ( fimul,      x87::FIMUL,   );
  x87twoOperand!      ( fist,       x87::FIST,    );
  x87twoOperand!      ( fistp,      x87::FISTP,   );
  x87twoOperand!      ( fisub,      x87::FISUB,   );
  x87twoOperand!      ( fisubr,     x87::FISUBR,  );
  x87twoOperand!      ( fmul,       x87::FMUL,    );
  x87twoOperand!      ( fmulp,      x87::FMULP,   );
  x87twoOperand!      ( fst,        x87::FST,     );
  x87twoOperand!      ( fstp,       x87::FSTP,    );
  x87twoOperand!      ( fsub,       x87::FSUB,    );
  x87twoOperand!      ( fsubp,      x87::FSUBP,   );
  x87twoOperand!      ( fsubr,      x87::FSUBR,   );
  x87twoOperand!      ( fsubrp,     x87::FSUBRP,  );
}
