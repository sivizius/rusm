use super::
{
  operands::
  {
    Operand,
    OperandType,
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

#[derive(Clone,Debug)]
pub struct  Expression ( pub Vec<ExpressionToken> );

impl        Operand                     for Expression
{
  fn this   ( self ) -> ( OperandType, usize )  { ( OperandType::Expression ( self ), 0 ) }
}

impl        Expression
{
  pub fn calculate
  (
    mut stack:                          &mut  Vec < ExpressionToken >,
  )
  ->  Result
      <
        Vec           < ExpressionToken >,
        &'static str,
      >
  {
    if let  Some  ( token ) = stack.pop()
    {
      match token
      {
        ExpressionToken::Constant       ( _     )
        =>  Ok              ( vec!  ( token ) ),
        #[cfg(any(feature="x86"))]
        ExpressionToken::x86            ( value )
        =>  value.calculate ( stack ),
        _
        if  token >= ExpressionToken::Add
        =>  {
              let mut tmp2              =   Expression::calculate ( &mut stack )?;
              let mut tmp1              =   Expression::calculate ( &mut stack )?;
              let     val1              =   if let  [ ExpressionToken::Constant ( val1  ) ] = tmp1.as_slice() { Some ( val1  ) } else  { None  };
              let     val2              =   if let  [ ExpressionToken::Constant ( val2  ) ] = tmp2.as_slice() { Some ( val2  ) } else  { None  };
              if  val1  !=  None
              &&  val2  !=  None
              {
                let     val1            =   val1.unwrap();
                let     val2            =   val2.unwrap();
                Ok
                (
                  vec!
                  (
                    ExpressionToken::Constant
                    (
                      match token
                      {
                        ExpressionToken::Add                =>  val1 + val2,
                        ExpressionToken::Substract          =>  val1 - val2,
                        ExpressionToken::Multiply           =>  val1 * val2,
                        ExpressionToken::Divide             =>  val1 / val2,
                        ExpressionToken::Modulo             =>  val1 % val2,
                        ExpressionToken::BitwiseAnd         =>  val1 & val2,
                        ExpressionToken::BitwiseOr          =>  val1 | val2,
                        ExpressionToken::BitwiseXor         =>  val1 ^ val2,
                        ExpressionToken::LogicalAnd         =>  ( ( *val1 !=  0 ) &&  ( *val2 !=  0 ) ) as  i128,
                        ExpressionToken::LogicalOr          =>  ( ( *val1 !=  0 ) ||  ( *val2 !=  0 ) ) as  i128,
                        ExpressionToken::LogicalXor         =>  ( ( *val1 !=  0 ) ^   ( *val2 !=  0 ) ) as  i128,
                        _                                   =>  unreachable!(),
                      }
                    )
                  )
                )
              }
              else  if  token ==  ExpressionToken::Add
                    &&  val1  ==  Some  ( &0  )
              {
                //  0 + b = b
                Ok          ( tmp2  )
              }
              else  if  token ==  ExpressionToken::Add
                    &&  val2  ==  Some  ( &0  )
              {
                //  a + 0 = a
                Ok          ( tmp1  )
              }
              else  if  token ==  ExpressionToken::Substract
                    &&  val2  ==  Some  ( &0  )
              {
                //  a - 0 = a
                Ok          ( tmp1  )
              }
              else  if  token ==  ExpressionToken::Multiply
                    &&  val1  ==  Some  ( &1  )
              {
                //  1 * b = b
                Ok          ( tmp2  )
              }
              else  if  token ==  ExpressionToken::Multiply
                    &&  val2  ==  Some  ( &1  )
              {
                //  a * 1 = a
                Ok          ( tmp1  )
              }
              else  if  token ==  ExpressionToken::Multiply
                    &&  ( val1  ==  Some  ( &0  ) ||  val2  ==  Some  ( &0  ) )
              {
                //  a * 0 = 0 * b = 0
                Ok          ( vec!  ( ExpressionToken::Constant ( 0 ) ) )
              }
              else  if  token ==  ExpressionToken::Divide
                    &&  val1  ==  Some  ( &0  )
              {
                //  0 / b = 0
                Ok          ( vec!  ( ExpressionToken::Constant ( 0 ) ) )
              }
              else  if  token ==  ExpressionToken::Divide
                    &&  val2  ==  Some  ( &1  )
              {
                //  a / 1 = a
                Ok          ( tmp1  )
              }
              else
              {
                tmp1.append ( &mut tmp2 );
                tmp1.push   ( token     );
                Ok          ( tmp1      )
              }
            },
        _
        if  token >= ExpressionToken::Neg
        =>  {
              let mut tmp1                =   Expression::calculate ( &mut stack )?;
              if let  [ ExpressionToken::Constant ( val1  ) ] = tmp1.as_slice()
              {
                Ok
                (
                  vec!
                  (
                    ExpressionToken::Constant
                    (
                      match token
                      {
                        ExpressionToken::Neg                =>  -val1,
                        ExpressionToken::BitwiseNot         =>  !val1,
                        ExpressionToken::LogicalNot         =>  ( *val1 == 0 ) as i128,
                        _                                   =>  unreachable!(),
                      }
                    )
                  )
                )
              }
              else
              {
                tmp1.push   ( token       );
                Ok          ( tmp1        )
              }
            },
        _
        =>  {
              println!        ( "_{:?}_", token );
              unimplemented!  (                 );
            },
      }
    }
    else
    {
      Err
      (
        "Operands Expected in calculation(), but Stack is Emtpy."
      )
    }
  }

  pub fn dimension
  (
    mut stack:                          &mut Vec<ExpressionToken>,
    item:                               &ExpressionToken,
  )
  ->  Result
      <
        (
          i128,
          Vec         < ExpressionToken >,
        ),
        &'static str,
      >
  {
    if let  Some  ( token ) = stack.pop()
    {
      if token == *item
      {
        Ok                              ( ( 1,            vec!  ( ExpressionToken::Constant ( 0 ) ) ) )   //←
      }
      else
      {
        match token
        {
          ExpressionToken::Constant     ( _ )
          =>  Ok                        ( ( 0,            vec!  ( token ) ) ),  //←
          #[cfg(any(feature="x86"))]
          ExpressionToken::x86          ( _ )
          =>  Ok                        ( ( 0,            vec!  ( token ) ) ),  //←
          ExpressionToken::Add        |
          ExpressionToken::Substract
          =>  {
                let ( mul2, mut rest2 ) =   Expression::dimension ( &mut stack, item  )?;
                let ( mul1, mut rest1 ) =   Expression::dimension ( &mut stack, item  )?;
                rest1.append    ( &mut rest2                  );
                if token == ExpressionToken::Add
                {
                  rest1.push    ( ExpressionToken::Add        );
                  Ok                    ( ( mul1 + mul2,  rest1           ) )   //←
                }
                else
                {
                  rest1.push    ( ExpressionToken::Substract  );
                  Ok                    ( ( mul1 - mul2,  rest1           ) )   //←
                }
              },
          ExpressionToken::Multiply
          =>  {
                let ( mul2, mut rest2 ) =   Expression::dimension ( &mut stack, item  )?;
                let ( mul1, mut rest1 ) =   Expression::dimension ( &mut stack, item  )?;
                let     val1            =   if let  [ ExpressionToken::Constant ( val1  ) ] = rest1.as_slice() { Some ( val1  ) } else  { None  };
                let     val2            =   if let  [ ExpressionToken::Constant ( val2  ) ] = rest2.as_slice() { Some ( val2  ) } else  { None  };
                if        mul2  !=  0
                &&        val1  !=  None
                {
                  //  <a> * <c·x> = <ac·x>
                  let     val1          =   val1.unwrap();
                  Ok                    ( ( val1 * mul2,  rest2           ) )   //←
                }
                else  if  mul1  !=  0
                      &&  val2  !=  None
                {
                  //  <c·x> * <b> = <bc·x>
                  let     val2          =   val2.unwrap();
                  Ok                    ( ( val2 * mul1,  rest1           ) )   //←
                }
                else
                {
                  rest1.append  ( &mut rest2                  );
                  rest1.push    ( token                       );
                  Ok                    ( ( 0,            rest1           ) )   //←
                }
              },
          ExpressionToken::Neg
          =>  {
                let ( mul1, mut rest1 ) =   Expression::dimension ( &mut stack, item  )?;
                rest1.push      ( ExpressionToken::Neg        );
                Ok                      ( ( -mul1,        rest1           ) )   //←
              },
          _
          if  token > ExpressionToken::Multiply
          =>  {
                let mut tmp2            =   Expression::calculate ( &mut stack )?;
                let mut tmp1            =   Expression::calculate ( &mut stack )?;
                tmp1.append     ( &mut tmp2                   );
                tmp1.push       ( token                       );
                Ok                      ( ( 0,            tmp1            ) )   //←
              },
          _
          if  token > ExpressionToken::Neg
          =>  {
                let mut tmp1            =   Expression::calculate ( &mut stack )?;
                tmp1.push       ( token                       );
                Ok                      ( ( 0,            tmp1            ) )   //←
              }
          _
          =>  {
                println!        ( "_{:?}_", token );
                unimplemented!  (                 );
              },
        }
      }
    }
    else
    {
      Err
      (
        "Operands Expected in dimension(), but Stack is Emtpy."
      )
    }
  }

  pub fn solve
  (
    &self
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
    let     stack                       =   Expression::calculate ( &mut self.0.clone() )?;
    match stack.as_slice()
    {
      [ ExpressionToken::Constant ( value ) ]
      =>  Ok  ( ( Some  ( 0 ),      OperandType::Constant               ( *value                                                                  ) ) ),
      #[cfg(any(feature="x86"))]
      [ ExpressionToken::x86      ( value ) ]
      =>  value.solve ( ),
      _
      =>  Ok  ( ( None,             OperandType::Expression             ( Expression  ( stack )                                                   ) ) ),
    }
  }

  pub fn format
  (
    &self
  ) ->  String
  {
    let mut output                      =   "".to_string();
    for token                           in  &self.0
    {
      if output != ""
      {
        output                          +=  " "
      }
      match token
      {
        ExpressionToken::Constant       ( value )
                                        =>  output  +=  &format!      ( "{}", value ),
        ExpressionToken::Neg            =>  output  +=  "~",
        ExpressionToken::Add            =>  output  +=  "+",
        ExpressionToken::Substract      =>  output  +=  "-",
        ExpressionToken::Multiply       =>  output  +=  "*",
        ExpressionToken::Divide         =>  output  +=  "/",
        ExpressionToken::Modulo         =>  output  +=  "%",
        ExpressionToken::BitwiseNot     =>  output  +=  "!",
        ExpressionToken::BitwiseAnd     =>  output  +=  "&",
        ExpressionToken::BitwiseOr      =>  output  +=  "|",
        ExpressionToken::BitwiseXor     =>  output  +=  "^",
        ExpressionToken::LogicalNot     =>  output  +=  "!!",
        ExpressionToken::LogicalAnd     =>  output  +=  "&&",
        ExpressionToken::LogicalOr      =>  output  +=  "||",
        ExpressionToken::LogicalXor     =>  output  +=  "^^",
        #[cfg(any(feature="x86"))]
        ExpressionToken::x86            ( value )        
                                        =>  output  +=  &value.format (             ),
        //_                               =>  output  +=  "…",
      };
    }
    output
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone,Debug,PartialEq,PartialOrd)]
pub enum ExpressionToken
{
  //  Operands
  Constant                              ( i128        ),
  #[cfg(any(feature="x86"))]
  x86                                   ( x86operand  ),
  //  One Operand Operators
  Neg,
  BitwiseNot,
  LogicalNot,
  //  Two Operand Operators
  Add,
  Substract,
  Multiply,
  Divide,
  Modulo,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  LogicalAnd,
  LogicalOr,
  LogicalXor,
}

#[macro_export]
macro_rules! nextToken
{
  ( ~                   )               =>  { ExpressionToken::Neg                            };
  ( +                   )               =>  { ExpressionToken::Add                            };
  ( -                   )               =>  { ExpressionToken::Substract                      };
  ( *                   )               =>  { ExpressionToken::Multiply                       };
  ( /                   )               =>  { ExpressionToken::Divide                         };
  ( !                   )               =>  { ExpressionToken::BitwiseNot                     };
  ( &                   )               =>  { ExpressionToken::BitwiseAnd                     };
  ( |                   )               =>  { ExpressionToken::BitwiseOr                      };
  ( ^                   )               =>  { ExpressionToken::BitwiseXor                     };
  ( !!                  )               =>  { ExpressionToken::LogicalNot                     };
  ( &&                  )               =>  { ExpressionToken::LogicalAnd                     };
  ( ||                  )               =>  { ExpressionToken::LogicalOr                      };
  ( ^^                  )               =>  { ExpressionToken::LogicalXor                     };
  ( not                 )               =>  { ExpressionToken::LogicalNot                     };
  ( and                 )               =>  { ExpressionToken::LogicalAnd                     };
  ( or                  )               =>  { ExpressionToken::LogicalOr                      };
  ( xor                 )               =>  { ExpressionToken::LogicalXor                     };
  ( x86_al              )               =>  { ExpressionToken::x86              ( x86::al   ) };
  ( x86_bl              )               =>  { ExpressionToken::x86              ( x86::bl   ) };
  ( x86_cl              )               =>  { ExpressionToken::x86              ( x86::cl   ) };
  ( x86_dl              )               =>  { ExpressionToken::x86              ( x86::dl   ) };
  ( x86_ah              )               =>  { ExpressionToken::x86              ( x86::ah   ) };
  ( x86_bh              )               =>  { ExpressionToken::x86              ( x86::bh   ) };
  ( x86_ch              )               =>  { ExpressionToken::x86              ( x86::ch   ) };
  ( x86_dh              )               =>  { ExpressionToken::x86              ( x86::dh   ) };
  ( x86_bpl             )               =>  { ExpressionToken::x86              ( x86::bpl  ) };
  ( x86_dil             )               =>  { ExpressionToken::x86              ( x86::dil  ) };
  ( x86_sil             )               =>  { ExpressionToken::x86              ( x86::sil  ) };
  ( x86_spl             )               =>  { ExpressionToken::x86              ( x86::spl  ) };
  ( x86_ax              )               =>  { ExpressionToken::x86              ( x86::ax   ) };
  ( x86_bx              )               =>  { ExpressionToken::x86              ( x86::bx   ) };
  ( x86_cx              )               =>  { ExpressionToken::x86              ( x86::cx   ) };
  ( x86_dx              )               =>  { ExpressionToken::x86              ( x86::dx   ) };
  ( x86_bp              )               =>  { ExpressionToken::x86              ( x86::bp   ) };
  ( x86_di              )               =>  { ExpressionToken::x86              ( x86::di   ) };
  ( x86_si              )               =>  { ExpressionToken::x86              ( x86::si   ) };
  ( x86_sp              )               =>  { ExpressionToken::x86              ( x86::sp   ) };
  ( x86_eax             )               =>  { ExpressionToken::x86              ( x86::eax  ) };
  ( x86_ebx             )               =>  { ExpressionToken::x86              ( x86::ebx  ) };
  ( x86_ecx             )               =>  { ExpressionToken::x86              ( x86::ecx  ) };
  ( x86_edx             )               =>  { ExpressionToken::x86              ( x86::edx  ) };
  ( x86_ebp             )               =>  { ExpressionToken::x86              ( x86::ebp  ) };
  ( x86_edi             )               =>  { ExpressionToken::x86              ( x86::edi  ) };
  ( x86_esi             )               =>  { ExpressionToken::x86              ( x86::esi  ) };
  ( x86_esp             )               =>  { ExpressionToken::x86              ( x86::esp  ) };
  ( x86_rax             )               =>  { ExpressionToken::x86              ( x86::rax  ) };
  ( x86_rbx             )               =>  { ExpressionToken::x86              ( x86::rbx  ) };
  ( x86_rcx             )               =>  { ExpressionToken::x86              ( x86::rcx  ) };
  ( x86_rdx             )               =>  { ExpressionToken::x86              ( x86::rdx  ) };
  ( x86_rbp             )               =>  { ExpressionToken::x86              ( x86::rbp  ) };
  ( x86_rdi             )               =>  { ExpressionToken::x86              ( x86::rdi  ) };
  ( x86_rsi             )               =>  { ExpressionToken::x86              ( x86::rsi  ) };
  ( x86_rsp             )               =>  { ExpressionToken::x86              ( x86::rsp  ) };
  ( x86_r8              )               =>  { ExpressionToken::x86              ( x86::r8   ) };
  ( x86_r9              )               =>  { ExpressionToken::x86              ( x86::r9   ) };
  ( x86_r10             )               =>  { ExpressionToken::x86              ( x86::r10  ) };
  ( x86_r11             )               =>  { ExpressionToken::x86              ( x86::r11  ) };
  ( x86_r12             )               =>  { ExpressionToken::x86              ( x86::r12  ) };
  ( x86_r13             )               =>  { ExpressionToken::x86              ( x86::r13  ) };
  ( x86_r14             )               =>  { ExpressionToken::x86              ( x86::r14  ) };
  ( x86_r15             )               =>  { ExpressionToken::x86              ( x86::r15  ) };
  ( x86_cs              )               =>  { ExpressionToken::x86              ( x86::cs   ) };
  ( x86_ss              )               =>  { ExpressionToken::x86              ( x86::ss   ) };
  ( x86_ds              )               =>  { ExpressionToken::x86              ( x86::ds   ) };
  ( x86_es              )               =>  { ExpressionToken::x86              ( x86::es   ) };
  ( x86_fs              )               =>  { ExpressionToken::x86              ( x86::fs   ) };
  ( x86_gs              )               =>  { ExpressionToken::x86              ( x86::gs   ) };
  ( $value:literal      )               =>  { ExpressionToken::Constant         ( $value      ) };
}

#[macro_export]
macro_rules! expression
{
  ( $( $token:tt )* )                   =>  { Expression ( vec![ $(  nextToken!  ( $token  ),  )*  ] ) };
}
