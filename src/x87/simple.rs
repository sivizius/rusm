use super::
{
  x87,
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
      registers::
      {
        GeneralPurposeRegisterNumber,
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
  assemblyOneOperand! ( x87fldcw,   x87::fldcw,   );
  assemblyOneOperand! ( x87fldenv,  x87::fldenv,  );
  assemblyOneOperand! ( x87fnsave,  x87::fnsave,  );
  assemblyOneOperand! ( x87fnstcw,  x87::fnstcw,  );
  assemblyOneOperand! ( x87fnstenv, x87::fnstenv, );
  assemblyOneOperand! ( x87fnstsw,  x87::fnstsw,  );
  assemblyOneOperand! ( x87frstor,  x87::frstor,  );
  assemblyOneOperand! ( x87fsave,   x87::fsave,   );
  assemblyOneOperand! ( x87fstcw,   x87::fstcw,   );
  assemblyOneOperand! ( x87fstenv,  x87::fstenv,  );
  assemblyOneOperand! ( x87fstsw,   x87::fstsw,   );
}

impl          x86instruction
{
  pub fn compileFloatSimple
  (
    mut self,
    opcode:                             u8,
    fwait:                              bool,
    subcode:                            u8,
    operands:                           &Vec < OperandType >,
    simple:                             x87simple,
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
        OperandType::x86  ( x86operand::GeneralPurposeRegister  {                         size,     rex:  false,  number: GeneralPurposeRegisterNumber::AX,               } )
        if  simple  ==  x87simple::SW
        =>  if  *size ==  asm::Word
            {
              self.setOpcode    ( 0xdf  );
              self.setModRegRM  ( 0xe0  );
              x86result::Done ( self  )
            }
            else
            {
              x86result::InvalidOperandSize
            },
        OperandType::x86  ( x86operand::Memory16                { kind: asmKind::Pointer, size: _,  segment,      registers,                                displacement, } )
        if  simple  ==  x87simple::ENV
        ||  simple  ==  x87simple::SAVE
        =>  self.encodeModRegRMdata
            (
              0xd8  | opcode,
              false,
              asm::Null,
              *segment,
              subcode,
              *registers as  u8,
              Some  ( *displacement  ),
              None,
              cpu,
              operandSize,
              addressSize,
            ),
        OperandType::x86  ( x86operand::Memory16                { kind: asmKind::Integer, size,     segment,      registers,                                displacement, } )
        if  simple  ==  x87simple::CW
        ||  simple  ==  x87simple::SW
        =>  if  *size ==  asm::Word
            {
              self.encodeModRegRMdata
              (
                0xd8  | opcode,
                false,
                asm::Null,
                *segment,
                subcode,
                *registers as  u8,
                Some  ( *displacement  ),
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
    else
    {
      x86result::InvalidNumberOfArguments ( 1 )
    }
  }
}

impl          x87
{
  x87oneOperand!      ( fldcw,      x87::FLDCW,   );
  x87oneOperand!      ( fldenv,     x87::FLDENV,  );
  x87oneOperand!      ( fnsave,     x87::FNSAVE,  );
  x87oneOperand!      ( fnstcw,     x87::FNSTCW,  );
  x87oneOperand!      ( fnstenv,    x87::FNSTENV, );
  x87oneOperand!      ( fnstsw,     x87::FNSTSW,  );
  x87oneOperand!      ( frstor,     x87::FRSTOR,  );
  x87oneOperand!      ( fsave,      x87::FSAVE,   );
  x87oneOperand!      ( fstcw,      x87::FSTCW,   );
  x87oneOperand!      ( fstenv,     x87::FSTENV,  );
  x87oneOperand!      ( fstsw,      x87::FSTSW,   );
}

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      x87simple
{
  CW,
  ENV,
  SAVE,
  SW,
}