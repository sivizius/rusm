use super::
{
  x86,
  x86instruction,
  x86result,
  state::
  {
    x86state,
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

macro_rules! theInstruction
{
  (
    $theName:ident,
    $theInstruction:expr,
  )
  =>  {
        pub fn $theName
        (
          label:                        impl  Operand,
        )
        ->  Instruction
        {
          let
          (
            thisLabel,
            _,
          )                             =   label.this  ( );
          Instruction
          (
            InstructionType::x86
            {
              architecture:             x86state  ( ),
              instruction:              $theInstruction,
            },
            0,
            vec!
            (
              thisLabel,
            ),
          )
        }
      }
}

impl          Assembly
{
  assemblyOneOperand! ( x86jb,      x86::jb,      );
  assemblyOneOperand! ( x86jbe,     x86::jbe,     );
  assemblyOneOperand! ( x86jcxz,    x86::jcxz,    );
  assemblyOneOperand! ( x86je,      x86::je,      );
  assemblyOneOperand! ( x86jl,      x86::jl,      );
  assemblyOneOperand! ( x86jle,     x86::jle,     );
  assemblyOneOperand! ( x86jnb,     x86::jnb,     );
  assemblyOneOperand! ( x86jnbe,    x86::jnbe,    );
  assemblyOneOperand! ( x86jne,     x86::jne,     );
  assemblyOneOperand! ( x86jnl,     x86::jnl,     );
  assemblyOneOperand! ( x86jnle,    x86::jnle,    );
  assemblyOneOperand! ( x86jno,     x86::jno,     );
  assemblyOneOperand! ( x86jnp,     x86::jnp,     );
  assemblyOneOperand! ( x86jns,     x86::jns,     );
  assemblyOneOperand! ( x86jo,      x86::jo,      );
  assemblyOneOperand! ( x86jp,      x86::jp,      );
  assemblyOneOperand! ( x86js,      x86::js,      );
  assemblyOneOperand! ( x86jz,      x86::je,      );
  assemblyOneOperand! ( x86loop,    x86::looop,   );
  assemblyOneOperand! ( x86loopz,   x86::loopz,   );
  assemblyOneOperand! ( x86loopnz,  x86::loopnz,  );
}

impl          x86
{
  theInstruction!     ( jb,         x86::JB,      );
  theInstruction!     ( jbe,        x86::JBE,     );
  theInstruction!     ( jcxz,       x86::JCXZ,    );
  theInstruction!     ( je,         x86::JE,      );
  theInstruction!     ( jl,         x86::JL,      );
  theInstruction!     ( jle,        x86::JLE,     );
  theInstruction!     ( jnb,        x86::JNB,     );
  theInstruction!     ( jnbe,       x86::JNBE,    );
  theInstruction!     ( jne,        x86::JNE,     );
  theInstruction!     ( jnl,        x86::JNL,     );
  theInstruction!     ( jnle,       x86::JNLE,    );
  theInstruction!     ( jno,        x86::JNO,     );
  theInstruction!     ( jnp,        x86::JNP,     );
  theInstruction!     ( jns,        x86::JNS,     );
  theInstruction!     ( jo,         x86::JO,      );
  theInstruction!     ( jp,         x86::JP,      );
  theInstruction!     ( js,         x86::JS,      );
  theInstruction!     ( jz,         x86::JE,      );
  theInstruction!     ( looop,      x86::LOOP,    );
  theInstruction!     ( loopz,      x86::LOOPZ,   );
  theInstruction!     ( loopnz,     x86::LOOPNZ,  );
}

impl          x86instruction
{
  pub fn compileJump
  (
    mut self,
    //  Instruction
    opcode:                             u8,
    operands:                           &Vec < OperandType >,
    //  Assembly
  )
  ->  x86result
  {
    if  operands.len  ( ) ==  1
    {
      match operands  [ 0 ]
      {
        OperandType::Displacement ( mut displacement  )
        =>  {
              displacement              -=  2;
              if  displacement  >= -0x80
              &&  displacement  <=  0x7f
              {
                self.setOpcode    ( opcode,                   );
                self.setImmediate ( asm::Byte,  displacement, );
                x86result::Done   ( self                      )
              }
              else
              {
                x86result::JumpToFar  ( displacement  )
              }
            },
        OperandType::Reference    ( _                 )
        =>  x86result::Equal  ( 2 ),
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