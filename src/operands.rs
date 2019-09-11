use super::
{
  AssemblyAddress,
  instructions::
  {
    Instruction,
  },
  expressions::
  {
    Expression,
  },
  symbols::
  {
    SymbolIdentifier,
    SymbolReference,
  },
};

#[cfg(any(feature="x86"))]
use super::
{
  x86::
  {
    operands::
    {
      x86operand,
    },
  },
  x87::
  {
    operands::
    {
      x87operand,
    },
  },
};

use std::
{
  char,
};

pub trait     Operand
{
  fn this   ( self ) -> ( OperandType, usize );
}

impl          Operand                   for i128
{
  fn this   ( self ) -> ( OperandType,  usize,  ) { ( OperandType::Constant     ( self            ), 0 ) }
}

impl          Operand                   for char
{
  fn this   ( self ) -> ( OperandType,  usize,  ) { ( OperandType::Constant     ( self  as  i128  ), 0 ) }
}

#[allow(non_camel_case_types)]
#[derive(Clone,Debug)]
pub enum      OperandType
{
  Symbol                                ( SymbolIdentifier                  ),
  Reference                             ( SymbolReference                   ),
  Address                               ( AssemblyAddress                   ),
  Expression                            ( Expression                        ),
  Tuple                                 ( Vec               < OperandType > ),
  Instructions                          ( Vec               < Instruction > ),
  // non-abstract
  Constant                              ( i128                              ),
  Displacement                          ( i128                              ),
  Intersegment
  {
    offset:                             i128,
    segment:                            i128,
  },
  #[cfg(any(feature="x86"))]
  x86                                   ( x86operand                        ),
  #[cfg(any(feature="x86"))]
  x87                                   ( x87operand                        ),
}

impl OperandType
{
  pub fn isAbstract
  (
    &self,
  ) -> bool
  {
    match self
    {
      OperandType::Symbol     ( _ ) |
      OperandType::Reference  ( _ ) |
      OperandType::Address    ( _ ) |
      OperandType::Expression ( _ ) |
      OperandType::Tuple      ( _ ) =>  true,
      _                             =>  false,
    }
  }

  pub fn print
  (
    &self,
    _size:                               usize,
  )
  {
    print!  ( " {},", self.format (  ) );
  }

  pub fn format
  (
    &self,
    //size:                               usize,
  ) ->  String
  {
    match self
    {
      OperandType::Symbol                 ( name        )
      =>  format! ( "${{{}}}", name ),
      OperandType::Reference              ( reference   )
      =>  format! ( "$({})", reference),
      OperandType::Expression             ( expression  )
      =>  expression.format ( ),
      OperandType::Tuple                  ( tuple       )
      =>  {
            let mut output              =   vec!  ( );
            for item                    in  tuple
            {
              output.push ( item.format (   ) );
            }
            format! ( "({:?})", output  )
          },
      OperandType::Constant               ( constant    )
      =>  format!
          (
            "(0x{:x}, ›{}‹, {})",
            constant,
            if  let Some  ( uchar ) = char::from_u32  ( *constant as  u32 )
            {
              uchar
            }
            else
            {
              '.'
            },
            constant,
          ),
      OperandType::Displacement           ( constant    )
      =>  if *constant < 0
          {
            format! ( "@-{:04x}", -constant )
          }
          else
          {
            format! ( "@+{:04x}", constant )
          },
      OperandType::Intersegment           { offset, segment }
      =>  format! ( "@{}:{}", segment, offset ),
      #[cfg(any(feature="x86"))]
      OperandType::x86                    ( this            )
      =>  this.format ( ),
      #[cfg(any(feature="x86"))]
      OperandType::x87                    ( this            )
      =>  this.format ( ),
      _
      =>  unimplemented!(),
    }
  }
}
