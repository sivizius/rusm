use super::
{
  AssemblyAddress,
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
};

pub trait Operand
{
  fn this   ( self ) -> ( OperandType, usize );
}

impl Operand                            for i128
{
  fn this   ( self ) -> ( OperandType,  usize,  ) { ( OperandType::Constant ( self            ), 0 ) }
}

impl Operand                            for char
{
  fn this   ( self ) -> ( OperandType,  usize,  ) { ( OperandType::Constant ( self  as  i128  ), 0 ) }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum OperandType
{
  Symbol                                ( SymbolIdentifier                  ),
  Reference                             ( SymbolReference                   ),
  Address                               ( AssemblyAddress                   ),
  Expression                            ( Expression                        ),
  Tuple                                 ( Vec               < OperandType > ),
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
      =>  format! ( "({})", constant ),
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
      _
      =>  unimplemented!(),
    }
  }
}