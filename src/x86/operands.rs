use super::
{
  x86,
  memory::
  {
    Memory16Registers,
  },
  registers::
  {
    GeneralPurposeRegisterNumber,
    SegmentRegisterNumber,
  },
  super::
  {
    expressions::
    {
      Expression,
      ExpressionToken,
    },
    operands::
    {
      Operand,
      OperandType,
    },
  },
};

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      x86operand
{
  ControlRegister
  {
    number:                             u8,
  },
  DebugRegister
  {
    number:                             u8,
  },
  GeneralPurposeRegister
  {
    size:                               usize,
    rex:                                bool,
    number:                             GeneralPurposeRegisterNumber,
  },
  Memory16
  {
    size:                               usize,
    segment:                            SegmentRegisterNumber,
    registers:                          Memory16Registers,
    displacement:                       i128,
  },
  MulitMediaRegister
  {
    size:                               usize,
    number:                             u8,
  },
  SegmentRegister
  {
    number:                             SegmentRegisterNumber,
  },
  TestRegister
  {
    number:                             u8,
  },
}

impl          x86operand
{
  pub fn calculate
  (
    self,
    mut stack:                          &mut  Vec < ExpressionToken >,
  )
  ->  Result
      <
        Vec           < ExpressionToken >,
        &'static str,
      >
  {
    match self
    {
      x86operand::Memory16
      {
        size,
        segment,
        registers:                      _,
        displacement:                   _,
      }
      =>  {
            let mut rest                =   Expression::calculate ( &mut stack )?;
            let ( mulBX,  mut rest  )   =   Expression::dimension ( &mut rest,  &ExpressionToken::x86 ( x86::bx ) )?;
            let ( mulBP,  mut rest  )   =   Expression::dimension ( &mut rest,  &ExpressionToken::x86 ( x86::bp ) )?;
            let ( mulSI,  mut rest  )   =   Expression::dimension ( &mut rest,  &ExpressionToken::x86 ( x86::si ) )?;
            let ( mulDI,  mut rest  )   =   Expression::dimension ( &mut rest,  &ExpressionToken::x86 ( x86::di ) )?;
            let rest                    =   Expression::calculate ( &mut rest )?;
            if let  [ ExpressionToken::Constant ( value ) ] = rest.as_slice()
            {
              match ( mulBX,  mulBP,  mulSI,  mulDI,  segment )
              {
                ( 1,  0,  1,  0,  _                               )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment,                                    registers: Memory16Registers::BXSI, displacement: *value  } ) ) ),
                ( 1,  0,  0,  1,  _                               )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment,                                    registers: Memory16Registers::BXDI, displacement: *value  } ) ) ),
                ( 0,  1,  1,  0,  SegmentRegisterNumber::DefaultDS  )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment:  SegmentRegisterNumber::DefaultSS, registers: Memory16Registers::BPSI, displacement: *value  } ) ) ),
                ( 0,  1,  1,  0,  _                               )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment,                                    registers: Memory16Registers::BPSI, displacement: *value  } ) ) ),
                ( 0,  1,  0,  1,  SegmentRegisterNumber::DefaultDS  )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment:  SegmentRegisterNumber::DefaultSS, registers: Memory16Registers::BPDI, displacement: *value  } ) ) ),
                ( 0,  1,  0,  1,  _                               )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment,                                    registers: Memory16Registers::BPDI, displacement: *value  } ) ) ),
                ( 0,  0,  1,  0,  _                               )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment,                                    registers: Memory16Registers::SI,   displacement: *value  } ) ) ),
                ( 0,  0,  0,  1,  _                               )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment,                                    registers: Memory16Registers::DI,   displacement: *value  } ) ) ),
                ( 0,  1,  0,  0,  SegmentRegisterNumber::DefaultDS  )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment:  SegmentRegisterNumber::DefaultSS, registers: Memory16Registers::BP,   displacement: *value  } ) ) ),
                ( 0,  1,  0,  0,  _                               )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment,                                    registers: Memory16Registers::BP,   displacement: *value  } ) ) ),
                ( 1,  0,  0,  0,  _                               )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment,                                    registers: Memory16Registers::BX,   displacement: *value  } ) ) ),
                ( 0,  0,  0,  0,  _                               )
                =>  Ok  ( vec!  ( ExpressionToken::x86  ( x86operand::Memory16 { size,  segment,                                    registers: Memory16Registers::DISP, displacement: *value  } ) ) ),
                ( _,  _,  _,  _,  _                               )
                =>  {
                      println!  ( "{}·Bx + {}·BP + {}·SI + {}·DI + {}", mulBX,  mulBP,  mulSI,  mulDI,  value );
                      Err ( "Invalid Combination of Registers for 16 Bit Addressing" )
                    }
              }
            }
            else
            {
              println!  ( "{:?}", rest );
              Err         ( "Non-Constant Rest as Displacement in Memory Address Calculation" )
            }
          },
      _
      =>  Ok  ( vec!  ( ExpressionToken::x86  ( self  ) ) ),
    }
  }

  pub fn format
  (
    &self,
  )
  ->  String
  {
    match self
    {
      x86operand::ControlRegister
      {
        number,
      }
      =>  format! ( "cr{}", number  ),
      x86operand::DebugRegister
      {
        number,
      }
      =>  format! ( "dr{}", number  ),
      x86operand::GeneralPurposeRegister
      {
        number,
        size,
        rex,
      }
      =>  match size
          {
            1 =>  {
                    match number
                    {
                      GeneralPurposeRegisterNumber::AX            =>  format! ( "al",                             ),
                      GeneralPurposeRegisterNumber::CX            =>  format! ( "cl",                             ),
                      GeneralPurposeRegisterNumber::DX            =>  format! ( "dl",                             ),
                      GeneralPurposeRegisterNumber::BX            =>  format! ( "bl",                             ),
                      GeneralPurposeRegisterNumber::SP  if  *rex  =>  format! ( "ah",                             ),
                      GeneralPurposeRegisterNumber::SP            =>  format! ( "spl",                            ),
                      GeneralPurposeRegisterNumber::BP  if  *rex  =>  format! ( "ch",                             ),  
                      GeneralPurposeRegisterNumber::BP            =>  format! ( "bpl",                            ),
                      GeneralPurposeRegisterNumber::SI  if  *rex  =>  format! ( "dh",                             ),
                      GeneralPurposeRegisterNumber::SI            =>  format! ( "sil",                            ),
                      GeneralPurposeRegisterNumber::DI  if  *rex  =>  format! ( "bh",                             ),
                      GeneralPurposeRegisterNumber::DI            =>  format! ( "dil",                            ),
                      _                                           =>  format! ( "r{}b",           *number as  u8, ),
                    }
                  },
            2 =>  {
                    match number
                    {
                      GeneralPurposeRegisterNumber::AX            =>  format! ( "ax",                             ),
                      GeneralPurposeRegisterNumber::CX            =>  format! ( "cx",                             ),
                      GeneralPurposeRegisterNumber::DX            =>  format! ( "dx",                             ),
                      GeneralPurposeRegisterNumber::BX            =>  format! ( "bx",                             ),
                      GeneralPurposeRegisterNumber::SP            =>  format! ( "sp",                             ),
                      GeneralPurposeRegisterNumber::BP            =>  format! ( "bp",                             ),
                      GeneralPurposeRegisterNumber::SI            =>  format! ( "si",                             ),
                      GeneralPurposeRegisterNumber::DI            =>  format! ( "di",                             ),
                      _                                           =>  format! ( "r{}w",           *number as  u8, ),
                    }
                  },
            4 =>  {
                    match number
                    {
                      GeneralPurposeRegisterNumber::AX            =>  format! ( "eax",                            ),
                      GeneralPurposeRegisterNumber::CX            =>  format! ( "ecx",                            ),
                      GeneralPurposeRegisterNumber::DX            =>  format! ( "edx",                            ),
                      GeneralPurposeRegisterNumber::BX            =>  format! ( "ebx",                            ),
                      GeneralPurposeRegisterNumber::SP            =>  format! ( "esp",                            ),
                      GeneralPurposeRegisterNumber::BP            =>  format! ( "ebp",                            ),
                      GeneralPurposeRegisterNumber::SI            =>  format! ( "esi",                            ),
                      GeneralPurposeRegisterNumber::DI            =>  format! ( "edi",                            ),
                      _                                           =>  format! ( "r{}d",           *number as  u8, ),
                    }
                  },
            8 =>  {
                    match number
                    {
                      GeneralPurposeRegisterNumber::AX            =>  format! ( "rax",                            ),
                      GeneralPurposeRegisterNumber::CX            =>  format! ( "rcx",                            ),
                      GeneralPurposeRegisterNumber::DX            =>  format! ( "rdx",                            ),
                      GeneralPurposeRegisterNumber::BX            =>  format! ( "rbx",                            ),
                      GeneralPurposeRegisterNumber::SP            =>  format! ( "rsp",                            ),
                      GeneralPurposeRegisterNumber::BP            =>  format! ( "rbp",                            ),
                      GeneralPurposeRegisterNumber::SI            =>  format! ( "rsi",                            ),
                      GeneralPurposeRegisterNumber::DI            =>  format! ( "rdi",                            ),
                      _                                           =>  format! ( "r{}",            *number as  u8, ),
                    }
                  },
            _                                                     =>  format! ( "({})r{}?", size, *number as  u8, ),
          },
      x86operand::Memory16
      {
        size,
        segment,
        registers,
        displacement,
      }
      =>  format!
          (
            "{} {}:[ {}{} ]",
            match size
            {
              1 =>  "byte".to_string  ( ),
              2 =>  "word".to_string  ( ),
              4 =>  "dword".to_string ( ),
              8 =>  "qword".to_string ( ),
              _ =>  format! ( "{}", size ),
            },
            segment.format  ( ),
            displacement,
            match registers
            {
              Memory16Registers::BXSI   =>  " + bx + si",
              Memory16Registers::BXDI   =>  " + bx + di",
              Memory16Registers::BPSI   =>  " + bp + si",
              Memory16Registers::BPDI   =>  " + bp + di",
              Memory16Registers::SI     =>  " + si",
              Memory16Registers::DI     =>  " + di",
              Memory16Registers::BP     =>  " + bp",
              Memory16Registers::BX     =>  " + bx",
              Memory16Registers::DISP   =>  "",
              _                         =>  " + ???",
            },
          ),
      x86operand::MulitMediaRegister
      {
        number,
        size,
      }
      =>  {
            match size
            {
               8  =>  format! ( "mm{}",             number  ),
              16  =>  format! ( "xmm{}",            number  ),
              32  =>  format! ( "ymm{}",            number  ),
              64  =>  format! ( "zmm{}",            number  ),
              _   =>  format! ( "({})mm{}?",  size, number  ),
            }
          },
      x86operand::SegmentRegister
      {
        number,
      }
      =>  number.format ( ).to_string ( ),
      x86operand::TestRegister
      {
        number,
      }
      =>  format! ( "tr{}", number  ),
    }
  }

  pub fn solve
  (
    self,
  )
  ->  Result
      <
        (
          Option      < usize >,
          OperandType,
        ),
        &'static str,
      >
  {
    let
    (
      this,
      size,
    )                                   =   self.this ( );
    Ok
    (
      (
        Some  ( size  ),
        this,
      )
    )
  }
}

impl          Operand                   for x86operand
{
  fn this
  (
    self
  )
  ->  (
        OperandType,
        usize,
      )
  {
    let     size
    = match self
      {
        x86operand::ControlRegister
        {
          number:                       _,
        }                               =>  4,
        x86operand::DebugRegister
        {
          number:                       _,
        }                               =>  4,
        x86operand::GeneralPurposeRegister
        {
          number:                       _,
          size,
          rex:                          _,
        }                               =>  size,
        x86operand::MulitMediaRegister
        {
          number:                       _,
          size,
        }                               =>  size,
        x86operand::Memory16
        {
          size,
          segment:                      _,
          registers:                    _,
          displacement:                 _,
        }                               =>  size,
        x86operand::SegmentRegister
        {
          number:                       _,
        }                               =>  4,
        x86operand::TestRegister
        {
          number:                       _,
        }                               =>  4,
      };
    (
      OperandType::x86  ( self  ),
      size,
    )
  }
}
