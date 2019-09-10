use super::
{
  x87,
  super::
  {
    operands::
    {
      Operand,
      OperandType,
    },
  },
};

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      x87operand
{
  Stack                                 ( u8  ),
}

impl          x87
{
  pub fn st
  (
    number:                             u8,
  )
  ->  x87operand
  {
    x87operand::Stack ( number  )
  }
}

impl          x87operand
{
  pub fn format
  (
    &self,
  )
  ->  String
  {
    match self
    {
      x87operand::Stack ( number  )
      =>  {
            format!
            (
              "st({})",
              number,
            )
          },
    }
  }
}

impl          Operand                   for x87operand
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
    match self
    {
      x87operand::Stack ( _ )
      =>  {
            (
              OperandType::x87  ( self  ),
              10,
            )
          },
    }
  }
}