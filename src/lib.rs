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

//#[macro_use]
//extern  crate bitflags;
#[macro_use]
extern  crate const_type;

pub mod asm;
pub mod expressions;
pub mod instructions;
mod     operands;
pub mod symbols;
#[cfg(any(feature="x86"))]
pub mod x86;

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
  /// Consumes a list of instructions and appends them to the list of `instructions`.
  ///
  /// # Arguments
  /// * `list`  – List of Instructions.
  pub fn append
  (
    mut self,
    list:                               Vec < Instruction >,
  )
  ->  Self
  {
    for mut instruction                 in  list
    {
      instruction.line                  =   self.lines;
      self.lines                        +=  1;
      self.instructions.push  ( instruction );
    }
    self
  }

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
      AssemblyState::Uncompiled ( _ ) |
      AssemblyState::Compiled   ( _ )
      =>  Err ( "Assembly Has to Be Compiled and Processed Before It Can Be USed.".to_string  ( ) ),
      AssemblyState::Processed  ( bytes )
      =>  Ok  ( bytes.clone ( ).into_boxed_slice  ( ) ),
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
    = if  let AssemblyState::Uncompiled ( mut numRounds ) = self.state
      {
        let mut symbols                 =   SymbolList      ( );
        let mut errors                  =   0;
        let mut done                    =   false;
        'outer:
        for ctrRounds                   in  numRounds ..  self.maxRounds
        {
          let mut address               =   AssemblyAddress ( );
          print!  ( "\n=#=#= round: {} =#=#=",  ctrRounds );
          done                          =   true;
          let mut instructionList       =   vec!  ( ) as  Vec < Instruction >;
          mem::swap
          (
            &mut  instructionList,
            &mut  self.instructions,
          );
          for instruction               in  &mut instructionList
          {
            match instruction.compile
                  (
                    &mut address,
                    &mut symbols,
                    self.endianness,
                    &mut self.architecture,
                    ctrRounds,
                  )
            {
              InstructionResult::Again
              =>  { },
              InstructionResult::Equal  ( width,  space )
              =>  {
                    address.raise       ( width,                  space,                  );
                    done                =   false;
                  }
              InstructionResult::Error  ( messages      )
              =>  {
                    errors              +=  1;
                    for error           in  messages
                    {
                      if  self.raiseError
                          (
                            errors,
                            ctrRounds,
                            error,
                            instruction.line,
                            instruction.this      ( ),
                            instruction.operands  ( ),
                          )
                      {
                        break           'outer;
                      }
                    }
                  },
              InstructionResult::Ready
              =>  {
                    address.raise       ( instruction.width ( ),  instruction.space ( ),  );
                    instruction.ready   (                                                 );
                  },
              InstructionResult::Rerun
              =>  {
                    address.wreck       (                                         );
                    done                =   false;
                  },
            };
          }
          mem::swap
          (
            &mut  instructionList,
            &mut  self.instructions,
          );
          if  done
          {
            numRounds                   =   ctrRounds;
            break                       'outer;
          }
        }
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
          AssemblyState::Compiled ( numRounds )
        }
      }
      else
      {
        self.state
      };
    self
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
      AssemblyState::Uncompiled ( _ ) |
      AssemblyState::Compiled   ( _ )
      =>  Err ( "Assembly Has to Be Compiled and Processed Before It Can Be USed.".to_string  ( ) ),
      AssemblyState::Processed  ( bytes )
      =>  {
            let     size                =   bytes.len ( );
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
                      bytes [ offset  + width * line  + pos ],
                    );
                  }
                  print!  ( "| " );
                  for pos               in  0 ..  width
                  {
                    let     char        =   bytes [ offset  + width * line  + pos ];
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
                      bytes [ offset + width * lines + pos ],
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
                    let char            =   bytes [ offset  + width * lines + pos ];
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

  /// Gathers the raw binary code of each compiled instruction.
  /// The actual output can be obtained calling `bytes()`.
  pub fn process
  (
    mut self,
  )
  ->  Self
  {
    self                                =   self.compile  ( );
    self.state
    = match self.state
      {
        AssemblyState::Uncompiled       ( _ )
        =>  unreachable!  ( ),
        AssemblyState::Compiled         ( _ )
        =>  {
              println!  ( "\ngenerate…" );
              let mut output            =   vec!  ( );
              let mut offset            =   None;
              for instruction           in  &self.instructions
              {
                print!  ( "{}", instruction.format ( 0 ) );
                if  let   Some  ( offset  ) = offset
                {
                  if  let AssemblyAddress::Some
                          {
                            base:       _,
                            offs,
                            size,
                          } = instruction.address ( )
                  {
                    if  size > 0
                    {
                      output.resize ( ( offs  - offset  ) as  usize,  0x00  );
                    }
                  }
                }
                else
                {
                  if  let AssemblyAddress::Some
                          {
                            base,
                            offs:       _,
                            size:       _,
                          } = instruction.address ( )
                  {
                    offset                =   Some  ( base  );
                  }
                }
                output.append ( &mut instruction.bytes  ( ) );
              }
              AssemblyState::Processed  ( output  )
            },
        AssemblyState::Processed        ( _ )
        =>  self.state,
        AssemblyState::Failed
        {
          round,
          ref mut source,
          ref mut message,
        }
        =>  {
              let mut oldSource         =   "process";
              mem::swap
              (
                source,
                &mut  oldSource,
              );
              let mut oldMessage        =   "Cannot Process an Already Failed Assembly.".to_string  ( );
              mem::swap
              (
                message,
                &mut  oldMessage,
              );
              let     line              =   self.lines;
              let     _
              = self.raiseError
                (
                  0,
                  round,
                  oldMessage,
                  line,
                  InstructionType::Internal ( oldSource ),
                  vec!  ( ),
                );
              self.state
            },
      };
    self
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
      AssemblyState::Uncompiled ( _ ) |
      AssemblyState::Compiled   ( _ )
      =>  Err ( "Assembly Has to Be Compiled and Processed Before It Can Be USed.".to_string  ( ) ),
      AssemblyState::Processed  ( bytes )
      =>  {
            let mut file                =   File::create  ( fileName  ).map_err ( | e | e.to_string ( ) )?;
            file.write_all  ( &bytes  ).map_err ( | e | e.to_string ( ) )?;
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

#[derive(Clone,Copy,PartialEq)]
pub enum    AssemblyAddress
{
  None,
  Some
  {
    base:                               u64,
    offs:                               u64,
    size:                               u64,
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
  }
}

impl        AssemblyAddress
{
  pub fn delta
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
        },
        AssemblyAddress::Some
        {
          base:                         thisBase,
          offs:                         thisOffs,
          size:                         _,
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
            } = self
    {
      Some
      (
        format!
        (
          "base=0x{:04x}, offs=0x{:04x}, size=0x{:04x}",
          base,
          offs,
          size,
        )
      )
    }
    else
    {
      None
    }
  }

  pub fn raise
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
            } = self
    {
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

  pub fn wreck
  (
    &mut self,
  )
  {
    *self                               =   AssemblyAddress::None;
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
  Compiled                              ( usize       ),
  Processed                             ( Vec < u8  > ),
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
