//#![allow(warnings)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#![feature(try_trait)]


macro_rules!  assemblyZeroOperand
{
  (
    $theName:ident,
    $theArchitecture:ident::$theInstruction:ident,
  )
  =>  {
        pub fn $theName
        (
          self,
        )
        ->  Self
        {
          self.push ( $theArchitecture::$theInstruction (                   ) )
        }
      }
}

macro_rules!  assemblyOneOperand
{
  (
    $theName:ident,
    $theArchitecture:ident::$theInstruction:ident,
  )
  =>  {
        pub fn $theName
        (
          self,
          op0:                          impl Operand,
        )
        ->  Self
        {
          self.push ( $theArchitecture::$theInstruction ( op0,              ) )
        }
      }
}

macro_rules!  assemblyTwoOperand
{
  (
    $theName:ident,
    $theArchitecture:ident::$theInstruction:ident,
  )
  =>  {
        pub fn $theName
        (
          self,
          op0:                          impl Operand,
          op1:                          impl Operand,
        )
        ->  Self
        {
          self.push ( $theArchitecture::$theInstruction ( op0,  op1,        ) )
        }
      }
}
/*
macro_rules!  assemblyThreeOperand
{
  (
    $theName:ident,
    $theArchitecture:ident::$theInstruction:ident,
  )
  =>  {
        pub fn $theName
        (
          self,
          op0:                          impl Operand,
          op1:                          impl Operand,
          op2:                          impl Operand,
        )
        ->  Self
        {
          self.push ( $theArchitecture::$theInstruction ( op0,  op1,  op2,  ) )
        }
      }
}
*/

macro_rules!  assemblyListOperand
{
  (
    $theName:ident,
    $theArchitecture:ident::$theInstruction:ident,
  )
  =>  {
        pub fn $theName
        (
          self,
          op0:                          Vec < impl  Operand >
        )
        ->  Self
        {
          self.push ( $theArchitecture::$theInstruction ( op0               ) )
        }
      }
}

macro_rules!  assemblyStringOperand
{
  (
    $theName:ident,
    $theArchitecture:ident::$theInstruction:ident,
  )
  =>  {
        pub fn $theName
        (
          self,
          op0:                          String,
        )
        ->  Self
        {
          self.push ( $theArchitecture::$theInstruction ( op0,              ) )
        }
      }
}

//#[macro_use]
//extern  crate bitflags;
#[macro_use]
extern  crate const_type;

pub mod asm;
pub mod expressions;
pub mod instructions;
pub mod operands;
pub mod symbols;
#[cfg(any(feature="x86"))]
pub mod x86;
#[cfg(any(feature="x86"))]
pub mod x87;

use crate::
{
  instructions::
  {
    Instruction,
    InstructionResult,
    InstructionType,
  },
  operands::
  {
    OperandType,
  },
  symbols::
  {
    SymbolList,
  },
};

#[cfg(any(feature="x86"))]
use crate::
{
  x86::
  {
    state::
    {
      x86state,
    },
  },
};

use std::
{
  fs::
  {
    File,
  },
  io::
  {
    Write,
  },
  mem,
};

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Debug,PartialEq,PartialOrd)]
pub enum      Architecture
{
  None,
  #[cfg(any(feature="x86"))]
  x86                                   ( x86state  ),
}

/// Central assembly object.
///
/// # Members
/// * `endianness`    – Default Endianness of Data, Not Operands,
/// * `maxRounds`     – Maximum Number of Rounds per Compilation,
/// * `maxErrors`     – Maximum Number of Errors, Before Compilation Will Crash,
/// * `state`         – State of This Assembly,
/// * `instructions`  – list of instructions,
/// * `m̀essages`      – Errors, Warnings and Notes,
/// * `lines`         – Line Counter for Debugging.
pub struct    Assembly
{
  //  Initial Parameters, Should Be Constant
  endianness:                           Endianness,
  maxRounds:                            usize,
  maxErrors:                            usize,
  //  State of the Assembly, Will Be Changed by Frontend
  state:                                AssemblyState,
  instructions:                         Vec < Instruction     >,
  architecture:                         Architecture,
  //  Values for Debugging, Will Be Changed by Backend
  messages:                             Vec < AssemblyMessage >,
  lines:                                usize,
}

/// Constructor for `Assembly`
///
/// # Arguments
/// * `endianness`    – Default Endianness of Data, Not Operands,
/// * `maxRounds`     – Maximum Number of Rounds per Compilation,
/// * `maxErrors`     – Maximum Number of Errors, Before Compilation Will Crash.
pub fn        Assembly
(
  endianness:                           Endianness,
  maxRounds:                            usize,
  maxErrors:                            usize,
)
->  Assembly
{
  Assembly
  {
    endianness,
    maxRounds,
    maxErrors,
    state:                              AssemblyState::Uncompiled ( 0 ),
    instructions:                       vec!  ( ),
    architecture:                       Architecture::None,
    messages:                           vec!  ( ),
    lines:                              0,
  }
}

impl        Assembly
{

  /// Returns a boxed slice of the raw bytes, if possible.
  pub fn bytes
  (
    &mut  self,
  )
  ->  Result
      <
        Box < [ u8  ] >,
        String,
      >
  {
    match &self.state
    {
      AssemblyState::Uncompiled ( _   )
      =>  Err ( "Assembly Has to Be Compiled Before It Can Be USed.".to_string  ( ) ),
      AssemblyState::Compiled
      {
        rounds:                         _,
        output,
      }
      =>  Ok
          (
            output.clone ( ).into_boxed_slice  ( )
          ),
      AssemblyState::Failed
      {
        round:                          _,
        source,
        message,
      }
      =>  {
            println!  ( "{}", &self.logs ( ) );
            Err
            (
              format!
              (
                "@{}: {}",
                source,
                message,
              )
            )
          },
    }
  }

  /// Compiles every instruction, if not already compiled.
  pub fn compile
  (
    mut   self,
  )
  ->  Self
  {
    self.state
    = if  let AssemblyState::Uncompiled ( numRounds ) = self.state
      {
        let mut symbols                 =   SymbolList        ( );
        let mut errors                  =   0;
        let mut done                    =   false;
        let mut buffer                  =   vec!              ( );
        let mut instructionList         =   vec!  ( ) as  Vec < Instruction >;
        mem::swap
        (
          &mut  instructionList,
          &mut  self.instructions,
        );
        'outer:
        for ctrRounds                   in  numRounds ..  self.maxRounds
        {
          buffer.clear  ( );
          let mut address               =   AssemblyAddress ( );
          //print!  ( "\n=#=#= round: {} =#=#=",  ctrRounds );
          Self::list ( &instructionList  );
          done
          = match self.compileList
                  (
                    &mut address,
                    &mut symbols,
                    &mut buffer,
                    ctrRounds,
                    &mut errors,
                    &mut instructionList,
                  )
            {
              None            =>  break 'outer,
              Some  ( done  ) =>  done,
            };
          Self::list ( &instructionList  );
          if  done
          {
            break                       'outer;
          }
        }
        mem::swap
        (
          &mut  instructionList,
          &mut  self.instructions,
        );
        if        errors  > 0
        {
          AssemblyState::Failed
          {
            round:                      numRounds,
            source:                     "compile",
            message:                    "Cannot Compile. See Error Messages.".to_string ( ),
          }
        }
        else  if  !done
        {
          AssemblyState::Failed
          {
            round:                      numRounds,
            source:                     "compile",
            message:
            format!
            (
              "Cannot be Compiled in {} Rounds.",
              numRounds,
            ),
          }
        }
        else
        {
          println!
          (
            "{}",
            self.logs ( ),
          );
          AssemblyState::Compiled
          {
            rounds:                     numRounds,
            output:                     buffer,
          }
        }
      }
      else
      {
        self.state
      };
    self
  }

  fn compileList
  (
    &mut self,
    address:                            &mut  AssemblyAddress,
    symbols:                            &mut  SymbolList,
    output:                             &mut  Vec < u8          >,
    round:                              usize,
    errors:                             &mut  usize,
    instructionList:                    &mut  Vec < Instruction >,
  )
  ->  Option  < bool  >
  {
    let mut done                        =   true;
    for instruction                     in  instructionList
    {
      match instruction.compile
            (
              address,
              symbols,
              output,
              self.endianness,
              &mut self.architecture,
              round,
            )
      {
        InstructionResult::Again
        =>  { },
        InstructionResult::Equal  ( width,  space     )
        =>  {
              address.append      ( width,                  space,                  );
              done                      =   false;
            }
        InstructionResult::Error  ( messages          )
        =>  {
              *errors                   +=  1;
              for error                 in  messages
              {
                if  self.raiseError
                    (
                      *errors,
                      round,
                      error,
                      instruction.line,
                      instruction.this      ( ),
                      instruction.operands  ( ),
                    )
                {
                  return                None;
                }
              }
            },
        InstructionResult::Place  ( mut instructions  )
        =>  {
              match self.compileList
                    (
                      address,
                      symbols,
                      output,
                      round,
                      errors,
                      &mut instructions,
                    )
              {
                None
                =>  return              None,
                Some  ( result  )
                =>  {
                      done              &=  result;
                      *instruction      =   asm::asm::append  ( instructions );
                    },
              }
            },
        InstructionResult::Ready  ( warnings          )
        =>  {
              if  let Some  ( warnings  ) = warnings
              {
                for message             in  warnings
                {
                  if  self.raiseWarning
                      (
                        *errors,
                        round,
                        message,
                        instruction.line,
                        instruction.this      ( ),
                        instruction.operands  ( ),
                      )
                  {
                    return              None;
                  }
                }
              }
              if  let Some  ( pointer ) = address.ptrFile ( )
              {
                let mut buffer          =   instruction.bytes ( );
                let     length          =   buffer.len        ( );
                if  length  > 0
                {
                  output.resize
                  (
                    pointer as  usize,
                    0x00,
                  );
                  output.append ( &mut  buffer  );
                }
              }
              address.append      ( instruction.width ( ),  instruction.space ( ),  );
              instruction.ready   (                                                 );
            },
        InstructionResult::Rerun
        =>  {
              address.invalidate  (                                         );
              done                =   false;
            },
      }
    }
    Some  ( done  )
  }

  pub fn hexDump
  (
    &self,
    width:                              usize,
    offset:                             usize,
    mut length:                         usize,
  )
  ->  Result
      <
        usize,
        String,
      >
  {
    match &self.state
    {
      AssemblyState::Uncompiled ( _   )
      =>  Err ( "Assembly Has to Be Compiled Before It Can Be USed.".to_string  ( ) ),
      AssemblyState::Compiled
      {
        rounds:                         _,
        output,
      }
      =>  {
            let     size                =   output.len ( );
            if  offset  <=  size
            {
              if  length  ==  0
              {
                length                  =   size - offset;
              }
              if  length  <=  ( size  - offset  )
              {
                let   lines             =   length  / width;
                for line                in  0 ..  lines
                {
                  for pos               in  0 ..  width
                  {
                    print!
                    (
                      "{:02x} ",
                      output  [ offset  + width * line  + pos ],
                    );
                  }
                  print!  ( "| " );
                  for pos               in  0 ..  width
                  {
                    let     char        =   output  [ offset  + width * line  + pos ];
                    if  (
                          char  >=  0x20
                        &&
                          char  <=  0x7e
                        )
                    ||  ( char  >=  0xa0  )
                    {
                      print!
                      (
                        "{}",
                        char  as  char,
                      );
                    }
                    else
                    {
                      print!  ( "." );
                    }
                  }
                  println!    ( ""  );
                }
                let     remainder       =   length % width;
                if remainder > 0
                {
                  for pos               in  0 ..  remainder
                  {
                    print!
                    (
                      "{:02x} ",
                      output  [ offset + width * lines + pos ],
                    );
                  }
                  print!
                  (
                    "{:ident$}| ",
                    "",
                    ident=3 * ( width - remainder )
                  );
                  for pos               in  0 ..  remainder
                  {
                    let char            =   output  [ offset  + width * lines + pos ];
                    if  (
                          char  >=  0x20
                        &&
                          char  <=  0x7e
                        )
                    ||  ( char  >=  0xa0  )
                    {
                      print!
                      (
                        "{}",
                        char  as  char,
                      );
                    }
                    else
                    {
                      print!  ( "." );
                    }
                  }
                  println!    ( ""  );
                }
                Ok ( length )
              }
              else
              {
                Err ( "Length Out Of Bonds".to_string ( ) )
              }
            }
            else
            {
              Err ( "Offset Out Of Bonds".to_string ( ) )
            }
          },
      AssemblyState::Failed
      {
        round:                          _,
        source,
        message,
      }
      =>  {
            println!  ( "{}", &self.logs ( ) );
            Err
            (
              format!
              (
                "@{}: {}",
                source,
                message,
              )
            )
          },
    }
  }

  pub fn list
  (
    instructions:                       &Vec  < Instruction >,
  )
  {
    for instruction                     in  instructions
    {
      match instruction.thisRef ( )
      {
        InstructionType::asm  ( asm::asm::Append  ( ref instructions  ) )
        =>  if  let Some  ( instructions  ) = instructions
            {
              Self::list  ( &instructions );
            }
            else
            {
              print!  ( "{:#}", instruction.format ( 0 ) );
            },
        _
        =>  print!  ( "{:#}", instruction.format ( 0 ) ),
      }
    }
  }

  pub fn logs
  (
    &self,
  )
  ->  String
  {
    let mut output                      =   "".to_string  ( );
    for message                         in  &self.messages
    {
      output
      +=  &match message
          {
            AssemblyMessage::Warning
            {
              round,
              message,
              line,
              instruction,
              operands,
            }
            =>  format!
                (
                  "\x1b[93mWARNING:\x1b[0m \x1b[4mAt Round {}, On Line {}\x1b[0m:\n         \x1b[1m{}\x1b[0m\n         {:?}\n{}\x1b[0m\n",
                  round,
                  line,
                  message,
                  instruction,
                  {
                    let mut arguments   =   "".to_string  ( );
                    for (
                          index,
                          operand,
                        )                   in  operands.iter ( ).enumerate ( )
                    {
                      arguments         +=  &format!
                                            (
                                              "{:ident$}└ #0x{:02x}: {}\n",
                                              "",
                                              index,
                                              operand.format  ( ),
                                              ident=11,
                                            );
                    }
                    arguments
                  },
                ),
            AssemblyMessage::Error
            {
              round,
              message,
              line,
              instruction,
              operands,
            }
            =>  format!
                (
                  "\x1b[91mERROR:\x1b[0m \x1b[4mAt Round {}, On Line {}\x1b[0m:\n       \x1b[1m{}\x1b[0m\n       {:?}\n{}\n\x1b[0m",
                  round,
                  line,
                  message,
                  instruction,
                  {
                    let mut arguments   =   "".to_string  ( );
                    for (
                          index,
                          operand,
                        )                   in  operands.iter ( ).enumerate ( )
                    {
                      arguments         +=  &format!
                                            (
                                              "{:ident$}└ #0x{:02x}: {}\n",
                                              "",
                                              index,
                                              operand.format  ( ),
                                              ident=9,
                                            );
                    }
                    arguments
                  },
                ),
          };
    }
    output
  }

  /// Appends a single instruction to the list of `instructions`.
  ///
  /// # Arguments
  /// * `this`  – instruction.
  pub fn push
  (
    mut   self,
    mut this:                           Instruction,
  )
  ->  Self
  {
    this.line                           =   self.lines;
    self.lines                          +=  1;
    self.instructions.push  ( this  );
    self
  }

  /// Adds an error message to the list of messages.
  ///
  /// # Arguments
  /// * `errors`      – Number of Errors Already Occured,
  /// * `round`       – Round, When the Error Occured,
  /// * `message`     – Actual Message,
  /// * `instruction` – Instructions and
  /// * `operands`    – Operands, Where the Error Occured.
  pub fn raiseError
  (
    &mut self,
    errors:                             usize,
    round:                              usize,
    message:                            String,
    line:                               usize,
    instruction:                        InstructionType,
    operands:                           Vec < OperandType >,
  )
  ->  bool
  {
    self.messages.push
    (
      AssemblyMessage::Error
      {
        round,
        message,
        line,
        instruction,
        operands,
      }
    );
    ( self.maxErrors  > 0 ) &&  ( errors  >=  self.maxErrors  )
  }

  /// Adds an warning message to the list of messages.
  ///
  /// # Arguments
  /// * `round`       – Round, When the Error Occured,
  /// * `message`     – Actual Message,
  /// * `instruction` – Instructions and
  /// * `operands`    – Operands, Where the Error Occured.
  pub fn raiseWarning
  (
    &mut self,
    errors:                             usize,
    round:                              usize,
    message:                            String,
    line:                               usize,
    instruction:                        InstructionType,
    operands:                           Vec < OperandType >,
  )
  ->  bool
  {
    //  TODO: Warnings = Errors?
    if false
    {
      self.raiseError
      (
        errors,
        round,
        message,
        line,
        instruction,
        operands,
      )
    }
    else
    {
      self.messages.push
      (
        AssemblyMessage::Warning
        {
          round,
          message,
          line,
          instruction,
          operands,
        }
      );
      false
    }
  }

  /// Resets any processing of an assembly, but does not alter the list of instruction.
  pub fn reset
  (
    mut self,
  )
  ->  Self
  {
    self.state                          =   AssemblyState::Uncompiled ( 0 );
    self.messages                       =   vec!  ( );
    self
  }

  pub fn toFile
  (
    &self,
    fileName:                           String,
  )
  ->  Result
      <
        (),
        String,
      >
  {
    match &self.state
    {
      AssemblyState::Uncompiled ( _   )
      =>  Err ( "Assembly Has to Be Compiled Before It Can Be USed.".to_string  ( ) ),
      AssemblyState::Compiled
      {
        rounds:                         _,
        output,
      }
      =>  {
            let mut file                =   File::create  ( fileName  ).map_err ( | e | e.to_string ( ) )?;
            file.write_all  ( &output ).map_err ( | e | e.to_string ( ) )?;
            file.sync_data  (         ).map_err ( | e | e.to_string ( ) )
          },
      AssemblyState::Failed
      {
        round:                          _,
        source,
        message,
      }
      =>  {
            println!  ( "{}", &self.logs ( ) );
            Err
            (
              format!
              (
                "@{}: {}",
                source,
                message,
              )
            )
          },
    }
  }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum    AssemblyAddress
{
  None,
  Some
  {
    base:                               u64,
    offs:                               u64,
    size:                               u64,
    file:                               u64,
  },
}

pub fn      AssemblyAddress
(
)
->  AssemblyAddress
{
  AssemblyAddress::Some
  {
    base:                               0,
    offs:                               0,
    size:                               0,
    file:                               0,
  }
}

impl        AssemblyAddress
{
  pub fn append
  (
    &mut self,
    width:                              u64,
    space:                              u64,
  )
  {
    if  let AssemblyAddress::Some
            {
              base:                     _,
              ref mut offs,
              ref mut size,
              ref mut file,
            } = self
    {
      *file                             +=  space;
      if  width > 0
      {
        *size                           =   *offs + width;
        *offs                           =   *offs + space;
      }
      else
      {
        *offs                           =   *offs + space;
      }
    }
  }

  pub fn distanceMemory
  (
    &self,
    this:                               AssemblyAddress,
  )
  ->  Option
      <
        i128
      >
  {
    match
    (
      self,
      this,
    )
    {
      (
        AssemblyAddress::Some
        {
          base:                         selfBase,
          offs:                         selfOffs,
          size:                         _,
          file:                         _,
        },
        AssemblyAddress::Some
        {
          base:                         thisBase,
          offs:                         thisOffs,
          size:                         _,
          file:                         _,
        },
      )
      =>  Some
          (
            ( thisBase  as  i128  + thisOffs  as  i128  ) - ( *selfBase as  i128  + *selfOffs as  i128  )
          ),
      _
      =>  None,
    }
  }

  pub fn format
  (
    &self,
  )
  ->  Option  < String  >
  {
    if  let AssemblyAddress::Some
            {
              base,
              offs,
              size,
              file,
            } = self
    {
      Some
      (
        format!
        (
          "base=0x{:04x}, offs=0x{:04x}, size=0x{:04x}, file=0x{:04x}",
          base,
          offs,
          size,
          file,
        )
      )
    }
    else
    {
      None
    }
  }

  pub fn invalidate
  (
    &mut self,
  )
  {
    *self                               =   AssemblyAddress::None;
  }

  pub fn organise
  (
    &mut self,
    newBase:                            u64,
  )
  {
    if  let AssemblyAddress::Some
            {
              ref mut base,
              ref mut offs,
              ref mut size,
              ref mut file,
            } = self
    {
      *file                             =   *file + *size - *offs;
      *base                             =   newBase;
      *offs                             =   0;
      *size                             =   0;
    }
  }

  pub fn ptrFile
  (
    &self,
  )
  ->  Option
      <
        u64,
      >
  {
    if  let AssemblyAddress::Some
            {
              base:                     _,
              offs:                     _,
              size:                     _,
              file,
            } = self
    {
      Some  ( *file )
    }
    else
    {
      None
    }
  }

  pub fn ptrMemory
  (
    &self,
  )
  ->  Option
      <
        u64,
      >
  {
    if  let AssemblyAddress::Some
            {
              base,
              offs,
              size:                     _,
              file:                     _,
            } = self
    {
      Some  ( base  + offs  )
    }
    else
    {
      None
    }
  }
}

pub enum      AssemblyMessage
{
  Warning
  {
    round:                              usize,
    message:                            String,
    line:                               usize,
    instruction:                        InstructionType,
    operands:                           Vec < OperandType >,
  },
  Error
  {
    round:                              usize,
    message:                            String,
    line:                               usize,
    instruction:                        InstructionType,
    operands:                           Vec < OperandType >,
  },
}

#[derive(Debug,PartialEq,PartialOrd)]
pub enum      AssemblyState
{
  Uncompiled                            ( usize       ),
  Compiled
  {
    rounds:                             usize,
    output:                             Vec < u8  >,
  },
  Failed
  {
    round:                              usize,
    source:                             &'static  str,
    message:                            String,
  },
}

#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      Endianness
{
  Default,
  LittleEndian,
  //MiddleEndian,
  BigEndian,
}
