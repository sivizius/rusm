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
    instructions::
    {
      Instruction,
      InstructionType,
    },
    operands::
    {
      OperandType,
    },
  },
};

macro_rules!  x86zeroOperand {
  (
    $theName:ident,
    $theInstruction:expr,
  )
  =>  {
        pub fn $theName
        (
        )
        -> Instruction
        {
          Instruction
          (
            InstructionType::x86
            {
              architecture:             x86state  ( ),
              instruction:              $theInstruction,
            },
            0,
            vec!  ( ),
          )
        }
      }
}

impl          Assembly
{
  assemblyZeroOperand!  ( x86aaa,     x86::aaa,   );
  assemblyZeroOperand!  ( x86aas,     x86::aas,   );
  assemblyZeroOperand!  ( x86cbw,     x86::cbw,   );
  assemblyZeroOperand!  ( x86clc,     x86::clc,   );
  assemblyZeroOperand!  ( x86cld,     x86::cld,   );
  assemblyZeroOperand!  ( x86cli,     x86::cli,   );
  assemblyZeroOperand!  ( x86cmc,     x86::cmc,   );
  assemblyZeroOperand!  ( x86cmpsb,   x86::cmpsb, );
  assemblyZeroOperand!  ( x86cmpsw,   x86::cmpsw, );
  assemblyZeroOperand!  ( x86cwd,     x86::cwd,   );
  assemblyZeroOperand!  ( x86daa,     x86::daa,   );
  assemblyZeroOperand!  ( x86das,     x86::das,   );
  assemblyZeroOperand!  ( x86hlt,     x86::hlt,   );
  assemblyZeroOperand!  ( x86int3,    x86::int3,  );
  assemblyZeroOperand!  ( x86into,    x86::into,  );
  assemblyZeroOperand!  ( x86iret,    x86::iret,  );
  assemblyZeroOperand!  ( x86lahf,    x86::lahf,  );
  assemblyZeroOperand!  ( x86lodsb,   x86::lodsb, );
  assemblyZeroOperand!  ( x86lodsw,   x86::lodsw, );
  assemblyZeroOperand!  ( x86movsb,   x86::movsb, );
  assemblyZeroOperand!  ( x86movsw,   x86::movsw, );
  assemblyZeroOperand!  ( x86popf,    x86::popf,  );
  assemblyZeroOperand!  ( x86pushf,   x86::pushf, );
  assemblyZeroOperand!  ( x86sahf,    x86::sahf,  );
  assemblyZeroOperand!  ( x86salc,    x86::salc,  );
  assemblyZeroOperand!  ( x86scasb,   x86::scasb, );
  assemblyZeroOperand!  ( x86scasw,   x86::scasw, );
  assemblyZeroOperand!  ( x86stc,     x86::stc,   );
  assemblyZeroOperand!  ( x86std,     x86::std,   );
  assemblyZeroOperand!  ( x86sti,     x86::sti,   );
  assemblyZeroOperand!  ( x86stosb,   x86::stosb, );
  assemblyZeroOperand!  ( x86stosw,   x86::stosw, );
  assemblyZeroOperand!  ( x86wait,    x86::wait,  );
  assemblyZeroOperand!  ( x86xlat,    x86::xlat,  );
}

impl          x86
{
  x86zeroOperand!       ( aaa,        x86::AAA,   );
  x86zeroOperand!       ( aas,        x86::AAS,   );
  x86zeroOperand!       ( cbw,        x86::CBW,   );
  x86zeroOperand!       ( clc,        x86::CLC,   );
  x86zeroOperand!       ( cld,        x86::CLD,   );
  x86zeroOperand!       ( cli,        x86::CLI,   );
  x86zeroOperand!       ( cmc,        x86::CMC,   );
  x86zeroOperand!       ( cmpsb,      x86::CMPSB, );
  x86zeroOperand!       ( cmpsw,      x86::CMPSW, );
  x86zeroOperand!       ( cwd,        x86::CWD,   );
  x86zeroOperand!       ( daa,        x86::DAA,   );
  x86zeroOperand!       ( das,        x86::DAS,   );
  x86zeroOperand!       ( hlt,        x86::HLT,   );
  x86zeroOperand!       ( int3,       x86::INT3,  );
  x86zeroOperand!       ( into,       x86::INTO,  );
  x86zeroOperand!       ( iret,       x86::IRET,  );
  x86zeroOperand!       ( lahf,       x86::LAHF,  );
  x86zeroOperand!       ( lodsb,      x86::LODSB, );
  x86zeroOperand!       ( lodsw,      x86::LODSW, );
  x86zeroOperand!       ( movsb,      x86::MOVSB, );
  x86zeroOperand!       ( movsw,      x86::MOVSW, );
  x86zeroOperand!       ( popf,       x86::POPF,  );
  x86zeroOperand!       ( pushf,      x86::PUSHF, );
  x86zeroOperand!       ( sahf,       x86::SAHF,  );
  x86zeroOperand!       ( salc,       x86::SALC,  );
  x86zeroOperand!       ( scasb,      x86::SCASB, );
  x86zeroOperand!       ( scasw,      x86::SCASW, );
  x86zeroOperand!       ( stc,        x86::STC,   );
  x86zeroOperand!       ( std,        x86::STD,   );
  x86zeroOperand!       ( sti,        x86::STI,   );
  x86zeroOperand!       ( stosb,      x86::STOSB, );
  x86zeroOperand!       ( stosw,      x86::STOSW, );
  x86zeroOperand!       ( wait,       x86::WAIT,  );
  x86zeroOperand!       ( xlat,       x86::XLAT,  );
}

impl          x86instruction
{
  pub fn compileZeroOperand
  (
    mut self,
    opcode:                             u8,
    operands:                           &Vec < OperandType >,
  )
  ->  x86result
  {
    if  operands.len  ( ) ==  0
    {
      self.setOpcode    ( opcode  );
      x86result::Done   ( self    )
    }
    else
    {
      x86result::InvalidNumberOfArguments ( 0 )
    }
  }
}
