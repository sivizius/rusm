use super::
{
  Architecture,
  AssemblyAddress,
  Endianness,
  operands::
  {
    OperandType,
  },
  symbols::
  {
    SymbolList,
    Variable,
  },
};

#[cfg(any(feature="x86"))]
use super::
{
  asm::
  {
    asm,
  },
  x86::
  {
    x86,
    state::
    {
      x86prefix,
      x86state,
    },
  },
  x87::
  {
    x87,
  },
};

/// The actual Instruction
///
/// # Members
/// *  abstract instruction
///   * `this`      – instruction type
///   * `size`      – instruction size (e.q. of registers. not length!),
///   * `operands`  – list of operands,
///   * `seed`
/// * for processing
///   * `seed`      – seed for randomised instruction encoding,
///   * `address`   – calculated address,
///   * `ready`     – already compiled?
///   * `width`     – actual length of fully compiled instruction,
///   * `space`     – reserved length of fully compiled instruction,
///   * `output`    – raw bytes of fully compiled instruction,
/// * for debugging
///   * `line`      – line number of instruction.
#[derive(Clone,Debug)]
pub struct  Instruction
{
  //  abstract instruction
  this:                                 InstructionType,
  size:                                 usize,
  operands:                             Vec     < OperandType         >,

  //  for processing
  _seed:                                usize,  //should be its own type
  address:                              AssemblyAddress,
  ready:                                bool,
  width:                                u64,
  space:                                u64,
  output:                               Vec     < u8                  >,

  //  for debugging
  pub line:                             usize,
}

pub fn Instruction
(
  this:                                 InstructionType,
  size:                                 usize,
  operands:                             Vec     < OperandType         >,
)
->  Instruction
{
  Instruction
  {
    this,
    size,
    operands,
    _seed:                              0,
    address:                            AssemblyAddress::None,
    ready:                              false,
    width:                              0,
    space:                              0,
    output:                             vec!  ( ),
    line:                               0,
  }
}

impl        Instruction
{
  pub fn address
  (
    &self,
  )
  ->  AssemblyAddress
  {
    self.address
  }

  pub fn append
  (
    &mut self,
    list:                               &mut  Vec < u8  >,
  )
  {
    self.output.append  ( list  );
  }

  pub fn bytes
  (
    &self,
  )
  ->  Vec < u8  >
  {
    self.output.clone ( )
  }

  /// Compiles instruction
  ///
  /// # Arguments
  /// * `address`       – Address of This Instruction,
  /// * `symbols`       – List of Symbols,
  /// * `endianness`    – Default Endianness,
  /// * `architecture`  – Instruction Set Architecture,
  /// * `round`         – Compilation Round.
  pub fn compile
  (
    &mut self,
    address:                            &mut  AssemblyAddress,
    symbols:                            &mut  SymbolList,
    _output:                            &mut  Vec < u8  >,
    endianness:                         Endianness,
    architecture:                       &mut  Architecture,
    round:                              usize,
  )
  ->  InstructionResult
  {
    self.address                        =   *address;

    if !( self.ready )
    {
      //print!  ( "{}", self.format ( 0 ) );
      //  try to resolve expressions and labels
      for operand                       in  &mut self.operands
      {
        if    let OperandType::Expression ( expression  ) = operand
        {
          match expression.solve  ( )
          {
            Ok
            (
              (
                newSize,
                newOperand,
              )
            )
            =>  {
                  *operand              =   newOperand;
                  if let  Some  ( newSize  ) = newSize
                  {
                    self.size           |=  newSize;
                  }
                },
            Err ( message )
            =>  return  InstructionResult::Again.error  ( message.to_string ( ) ),
          }
        }
        else
        {
          if  let OperandType::Symbol     ( identifier  ) = operand
          {
            let reference               =   symbols.expect  ( identifier.to_string  ( ) );
            *operand                    =   OperandType::Reference  ( reference );
          }
          if  let OperandType::Reference  ( reference   ) = operand
          {
            match symbols.obtain
                  (
                    *reference,
                    round,
                  )
            {
              Err ( message         )
              =>  return  InstructionResult::Again.error  ( message.to_string ( ) ),
              Ok  ( None            )
              =>  {},
              Ok  ( Some  ( value ) )
              =>  match value
                  {
                    OperandType::Address  ( destination )
                    =>  if  let Some  ( displacement  ) = address.distanceMemory  ( destination )
                        {
                          *operand      =   OperandType::Displacement ( displacement  );
                        }
                    _
                    =>  *operand        =   value,
                  }
            }
          }
        }
      }

      match &mut self.this
      {
        InstructionType::Internal       ( _             )
        =>  InstructionResult::Again,
        InstructionType::asm            ( _             )
        =>  self.asmCompile
            (
              address,
              symbols,
              endianness,
              round,
            ),
        #[cfg(any(feature="x86"))]
        InstructionType::x86            { ..            }
        =>  self.x86compile
            (
              architecture,
              round,
            ),
        #[cfg(any(feature="x86"))]
        InstructionType::x86prefix      ( _             )
        =>  self.x86prefixCompile
            (
              architecture,
            ),
        #[cfg(any(feature="x86"))]
        InstructionType::x87            { ..            }
        =>  self.x87compile
            (
              architecture,
              round,
            ),
        //_
        //=>  InstructionResult::Again.error                                              ( "Unexpected Instruction. This should not happen here!".to_string  ( ) ),
      }
    }
    else
    {
      InstructionResult::Ready
      {
        warnings:                       None,
        width:                          self.width,
        space:                          self.space,
      }
    }
  }

  pub fn format
  (
    &self,
    level:                              usize,
  )
  ->  String
  {
    let mut output                      =   "".to_string  ( );
    if  let Some  ( text  ) = self.address.format  ( )
    {
      let     length                    =   self.output.len ( );
      output                            +=  &format!
                                            (
                                              "\n{:ident$}{}, length=0x{:01x}",
                                              "",
                                              text,
                                              length,
                                              ident=level,
                                            );
      if  length  > 0
      {
        output                          +=  &format! ( ": <"       );
        for   byte                      in  &self.output
        {
          output                        +=  &format!
                                            (
                                              " {:02x}",
                                              byte,
                                            );
        }
        output                          +=  &format! ( " >\n"      );
      }
      else
      {
        output                          +=  &format! ( "\n"       );
      }
    }
    else
    {
      output                            +=  &format!
                                            (
                                              "\n{:ident$}None:None\n",
                                              "",
                                              ident=level,
                                            );
    }
    output                              +=  &self.this.format ( level );
    for   operand                       in  &self.operands
    {
      output                            +=  &format!
                                            (
                                              "{:ident$}└ {}\n",
                                              "",
                                              operand.format  ( ),
                                              ident=level + 2,
                                            );
    }
    output
  }

  pub fn length
  (
    &self,
  )
  ->  u64
  {
    self.output.len ( ) as  u64
  }

  pub fn operands
  (
    &self,
  )
  ->  Vec < OperandType >
  {
    self.operands.clone ( )
  }

  pub fn operandsNumber
  (
    &self,
  )
  ->  usize
  {
    self.operands.len ( )
  }

  pub fn operandsRef
  (
    &self,
  )
  ->  &Vec < OperandType >
  {
    &self.operands
  }

  pub fn ready
  (
    &mut self,
  )
  {
    self.ready                          =   true;
  }

  pub fn push
  (
    &mut self,
    value:                              u8,
  )
  {
    self.output.push  ( value );
  }

  pub fn setWidthAndSpace
  (
    &mut self,
    width:                              u64,
    space:                              u64,
  )
  {
    self.width                          =   width;
    self.space                          =   space;
  }

  pub fn size
  (
    &self,
  )
  ->  usize
  {
    self.size
  }

  pub fn space
  (
    &self,
  )
  ->  u64
  {
    self.space
  }

  pub fn this
  (
    &self,
  )
  ->  InstructionType
  {
    self.this.clone ( )
  }

  pub fn thisRefMut
  (
    &mut self,
  )
  ->  &mut InstructionType
  {
    &mut self.this
  }

  pub fn thisRef
  (
    &self,
  )
  ->  &InstructionType
  {
    &self.this
  }

  pub fn thisSet
  (
    &mut self,
    instruction:                        InstructionType,
  )
  {
    self.this                           =   instruction;
  }

  pub fn width
  (
    &self,
  )
  ->  u64
  {
    self.width
  }
}

pub enum    InstructionResult
{
  Again,                                                                  //  abstract and ready, but recompile every round.
  Bytes                                 ( Vec     < u8              > ),  //  append this bytes here and ready.
  Equal                                 ( u64,    u64,                ),  //  not ready, but known length.
  Error                                 ( Vec     < String          > ),  //  failure.
  Global                                ( Variable                    ),  //  abstract and ready.
  Place                                 ( Vec     < Instruction     > ),  //  append these instruction here.
  Ready
  {
    warnings:                           Option
                                        <
                                          Vec
                                          <
                                            String
                                          >
                                        >,
    width:                              u64,
    space:                              u64,
  },                                                                      //  everything fine, do not have be touched ever again, but there might be warnings.
  Replace                               ( InstructionType             ),  //  replace instruction.
  Rerun,                                                                  //  not ready, run again.
}

impl        InstructionResult
{
  pub fn fail
  (
    message:                            String,
  )
  ->  Self
  {
    InstructionResult::Error  ( vec!  ( message ) )
  }

  pub fn error
  (
    mut self,
    message:                            String,
  )
  ->  Self
  {
    if  let InstructionResult::Error  ( ref mut fails ) = self
    {
      fails.push  ( message );
      self
    }
    else
    {
      InstructionResult::Error  ( vec!  ( message ) )
    }
  }

  pub fn invalidArgument
  (
    self,
    number:                             usize,
  )
  ->  Self
  {
    self.error
    (
      format!
      (
        "Invalid Argument {}.",
        number,
      )
    )
  }

  pub fn invalidNumberOfArguments
  (
    self,
    have:                               usize,
    want:                               usize,
  )
  ->  Self
  {
    self.error
    (
      format!
      (
        "Invalid Number of Arguments: Have {}, but Want {}.",
        have,
        want,
      )
    )
  }

  pub fn invalidOperandSize
  (
    self,
    size:                               usize,
  )
  ->  Self
  {
    self.error
    (
      if  size ==  0
      {
        format!
        (
          "Size Not Specified.",
        )
      }
      else
      {
        format!
        (
          "Invalid Size {}.",
          size,
        )
      }
    )
  }

  pub fn minimalVersion
  (
    self,
    arch:                               &'static str,
    have:                               &'static str,
    want:                               &'static str,
  )
  ->  Self
  {
    self.error
    (
      format!
      (
        "The {}-Version Is {}, But The Minimal Version For This Instruction is {}.",
        arch,
        have,
        want,
      )
    )
  }

  pub fn notImplemented
  (
    self,
    this:                               String,
  )
  ->  Self
  {
    self.error
    (
      format!
      (
        "{} Is Not Implented Yet, Sorry.",
        this,
      )
    )
  }

  pub fn noVersion
  (
    self,
    architecture:                       &'static str,
  )
  ->  Self
  {
    self.error
    (
      format!
      (
        "The {}-Version Is None, Therefore No Instruction Of This Instruction Set Can Be Compiled.",
        architecture,
      )
    )
  }

  pub fn outOfBounds
  (
    self,
    number:                             usize,
    immediate:                          i128,
    lowerBound:                         i128,
    upperBound:                         i128,
  )
  ->  InstructionResult
  {
    self.error
    (
      format!
      (
        "Value {} of Operand {} Out of Bounds [{},{}]",
        immediate,
        number,
        lowerBound,
        upperBound,
      )
    )
  }

  pub fn wrongArchitecture
  (
    self,
    have:                               Architecture,
    want:                               &'static str,
  )
  ->  Self
  {
    self.error
    (
      format!
      (
        "Wrong Instruction Set Architecture: Have {:?}, but Want {}.",
        have,
        want,
      )
    )
  }

  pub fn wrongInstructionSet
  (
    self,
    have:                               InstructionType,
    want:                               &'static str,
  )
  ->  Self
  {
    self.error
    (
      format!
      (
        "Wrong Instruction Set: Have {:?}, but Want {}.",
        have,
        want,
      )
    )
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone,Debug)]
pub enum    InstructionType
{
  Internal                              ( &'static str        ),
  asm                                   ( asm                 ),
  #[cfg(any(feature="x86"))]
  x86
  {
    architecture:                       x86state,
    instruction:                        x86,
  },
  #[cfg(any(feature="x86"))]
  x86prefix                             ( x86prefix           ),
  #[cfg(any(feature="x86"))]
  x87
  {
    architecture:                       x86state,
    instruction:                        x87,
  }
}

impl          InstructionType
{
  pub fn format
  (
    &self,
    level:                              usize,
  )
  ->  String
  {
    format!
    (
      "{:ident$}{:?}\n",
      "",
      &self,
      ident=level,
    )
  }
}
