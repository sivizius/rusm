use super::
{
  Assembly,
  AssemblyAddress,
  Endianness,
  instructions::
  {
    Instruction,
    InstructionResult,
    InstructionType,
  },
  operands::
  {
    Operand,
    OperandType,
  },
  symbols::
  {
    SymbolIdentifier,
    SymbolList,
    SymbolReference,
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
    Read,
    Seek,
    SeekFrom,
  },
};

macro_rules!  asmDataInstruction {
  (
    $theData:ident,
    $theResv:ident,
    $theSize:expr,
  )
  =>  {
        pub fn $theData
        (
          data:                         Vec < impl Operand  >,
        )
        -> Instruction
        {
          Instruction
          (
            InstructionType::asm
            (
              asm::EmitData
              {
                minimum:                -1  <<  ( $theSize  * 8 - 1 ),
                maximum:                ( 1 <<  ( $theSize  * 8 ) ) - 1,
                endianness:             Endianness::Default,
                skip:                   0,
              }
            ),
            $theSize,
            data.into_iter  ( ).map ( | x | ( x.this  ( ) ).0 ).collect ( ),
          )
        }
        pub fn $theResv
        (
          length:                       impl Operand,
        )
        ->  Instruction
        {
          Instruction
          (
            InstructionType::asm  ( asm::WantData ),
            $theSize,
            vec!  ( length.this ( ).0 ),
          )
        }
      }
}

/// Create common instructions as standalone instructions of a pseudo instruction set.
/// There instructions are not connected to any `Assembly`,
///   but could be `push`ed or `append`ed to an existing `Assembly`.
#[allow(non_camel_case_types)]
#[derive(Clone,Debug)]
pub enum      asm
{
  Label                                 ( SymbolIdentifier                ),
  Reference                             ( SymbolReference                 ),
  EmitData
  {
    minimum:                            i128,
    maximum:                            i128,
    endianness:                         Endianness,
    skip:                               usize,
  },
  ReadFile
  {
    offs:                               i128,
    size:                               Option  < u64 >,
    name:                               String,
  },
  WantData,
  Append                                ( Option  < Vec < Instruction > > ),
}

impl          asm
{
  pub const Byte:                 usize =   1;
  pub const Word:                 usize =   2;
  pub const DWord:                usize =   4;
  pub const FWord:                usize =   6;
  pub const PWord:                usize =   6;
  pub const QWord:                usize =   8;
  pub const TByte:                usize =   10;
  pub const TWord:                usize =   10;
  pub const DQWord:               usize =   16;
  pub const XWord:                usize =   16;
  pub const QQWord:               usize =   32;
  pub const YWord:                usize =   32;
  pub const DQQWord:              usize =   64;
  pub const ZWord:                usize =   64;

  asmDataInstruction! ( db,   rb,   asm::Byte,    );
  asmDataInstruction! ( dw,   rw,   asm::Word,    );
  asmDataInstruction! ( dd,   rd,   asm::DWord,   );
  asmDataInstruction! ( df,   rf,   asm::FWord,   );
  asmDataInstruction! ( dp,   rp,   asm::PWord,   );
  asmDataInstruction! ( dq,   rq,   asm::QWord,   );
  asmDataInstruction! ( dt,   rt,   asm::TWord,   );
  asmDataInstruction! ( ddq,  rdq,  asm::DQWord,  );
  asmDataInstruction! ( dx,   rx,   asm::XWord,   );
  asmDataInstruction! ( dqq,  rqq,  asm::QQWord,  );
  asmDataInstruction! ( dy,   ry,   asm::YWord,   );
  asmDataInstruction! ( ddqq, rdqq, asm::DQQWord, );
  asmDataInstruction! ( dz,   rz,   asm::ZWord,   );

  /// Consumes a list of instructions and appends them to the list of `instructions`.
  ///
  /// # Arguments
  /// * `list`  – List of Instructions.
  pub fn append
  (
    list:                               Vec < Instruction >,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::asm  ( asm::Append ( Some  ( list  ) ) ),
      0,
      vec!  ( ),
    )
  }

  /// Add some raw data.
  ///
  /// # Arguments
  /// * `size`  – length of each value in bytes,
  /// * `data`  – list of values.
  pub fn emit
  (
    size:                               usize,
    data:                               Vec < impl Operand  >,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::asm
      (
        asm::EmitData
        {
          minimum:                      -1 <<  ( size * 8 - 1 ),
          maximum:                      ( 1 <<  ( size * 8 ) ) - 1,
          endianness:                   Endianness::Default,
          skip:                         0,
        }
      ),
      size,
      data.into_iter  ( ).map ( | x | ( x.this  ( ) ).0 ).collect ( ),
    )
  }

  /// Add content of file.
  ///
  /// # Arguments
  /// * `offs`  – Offset, From Where To Read,
  /// * `size`  – Length Of Data,
  /// * `name`  – Name Of File.
  pub fn file
  (
    offs:                               i128,
    size:                               Option  < u64 >,
    name:                               String,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::asm
      (
        asm::ReadFile
        {
          offs,
          size,
          name,
        }
      ),
      0,
      vec!  ( ),
    )
  }

  /// Add a label which can be used to reference this point in other instructions,
  ///   but will be ignored in the generation of the raw code.
  ///
  /// # Arguments
  /// * `name`  – symbol, that can be used in other instructions to refer to this point.
  pub fn label
  (
    name:                               String,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::asm  ( asm::Label ( name ) ),
      0,
      vec!  ( ),
    )
  }

  /// Add an utf8-`String`.
  ///
  /// # Arguments
  /// *  `text`  – String
  pub fn utf8
  (
    text:                               String,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::asm
      (
        asm::EmitData
        {
          minimum:                      0,
          maximum:                      0xff,
          endianness:                   Endianness::LittleEndian,
          skip:                         0,
        }
      ),
      1,
      text.into_bytes ( ).into_iter  ( ).map ( | x | ( (  x as  i128  ).this  ( ) ).0 ).collect ( ),
    )
  }
}

impl          Assembly
{
  assemblyListOperand!    ( db,     asm::db,      );
  assemblyListOperand!    ( dw,     asm::dw,      );
  assemblyListOperand!    ( dd,     asm::dd,      );
  assemblyListOperand!    ( df,     asm::df,      );
  assemblyListOperand!    ( dp,     asm::dp,      );
  assemblyListOperand!    ( dq,     asm::dq,      );
  assemblyListOperand!    ( dt,     asm::dt,      );
  assemblyListOperand!    ( ddq,    asm::ddq,     );
  assemblyListOperand!    ( dx,     asm::dx,      );
  assemblyListOperand!    ( dqq,    asm::dqq,     );
  assemblyListOperand!    ( dy,     asm::dy,      );
  assemblyListOperand!    ( ddqq,   asm::ddqq,    );
  assemblyListOperand!    ( dz,     asm::dz,      );
  assemblyStringOperand!  ( label,  asm::label,   );
  assemblyOneOperand!     ( rb,     asm::rb,      );
  assemblyOneOperand!     ( rw,     asm::rw,      );
  assemblyOneOperand!     ( rd,     asm::rd,      );
  assemblyOneOperand!     ( rf,     asm::rf,      );
  assemblyOneOperand!     ( rp,     asm::rp,      );
  assemblyOneOperand!     ( rq,     asm::rq,      );
  assemblyOneOperand!     ( rt,     asm::rt,      );
  assemblyOneOperand!     ( rdq,    asm::rdq,     );
  assemblyOneOperand!     ( rx,     asm::rx,      );
  assemblyOneOperand!     ( rqq,    asm::rqq,     );
  assemblyOneOperand!     ( ry,     asm::ry,      );
  assemblyOneOperand!     ( rdqq,   asm::rdqq,    );
  assemblyOneOperand!     ( rz,     asm::rz,      );
  assemblyStringOperand!  ( utf8,   asm::utf8,    );

  pub fn append
  (
    self,
    list:                               Vec < Instruction >,
  )
  ->  Self  { self.push ( asm::append ( list  ) ) }

  pub fn emit
  (
    self,
    size:                               usize,
    data:                               Vec < impl Operand  >,
  )
  ->  Self  { self.push ( asm::emit   ( size, data, ) ) }

  pub fn file
  (
    self,
    offs:                               i128,
    size:                               Option  < u64 >,
    name:                               String,
  )
  ->  Self  { self.push ( asm::file   ( offs, size, name, ) ) }
}

impl          Instruction
{
  pub fn asmCompile
  (
    &mut self,
    address:                            &mut  AssemblyAddress,
    symbols:                            &mut  SymbolList,
    endianness:                         Endianness,
    round:                              usize,
  )
  ->  InstructionResult
  {
    let     size                        =   self.size     ( );
    let     operands                    =   self.operands ( );
    let     number                      =   operands.len  ( );
    if  let InstructionType::asm
            (
              this
            ) = self.thisRefMut ( )
    {
      let mut bytes                     =   vec!  ( );
      let mut width                     =   0;
      let mut space                     =   0;
      let mut replace                   =   None;
      let     result
      = match this
        {
          asm::Append                   ( ref mut instructions  )
          =>  if  let Some  ( instructions  ) = instructions.take ( )
              {
                InstructionResult::Place                                                ( instructions                                            )
              }
              else
              {
                InstructionResult::Again.error                                          ( "Cannot Append: Instruction List Is None".to_string ( ) )
              },
          asm::EmitData
          {
            minimum,
            maximum,
            endianness:                 dataEndianness,
            skip,
          }
          =>  {
                let mut result          =   InstructionResult::Ready  ( None  );
                let
                (
                  lowerByte,
                  upperByte,
                )
                = match if  *dataEndianness ==  Endianness::Default
                        {
                          endianness
                        }
                        else
                        {
                          *dataEndianness
                        }
                  {
                    Endianness::LittleEndian  |
                    Endianness::Default
                    =>  (
                          0,
                          size,
                        ),
                    Endianness::BigEndian
                    =>  (
                          size,
                          0,
                        ),
                  };
                for (
                      count,
                      operand,
                    )                   in  operands.iter  ( ).skip ( *skip ).enumerate  ( )
                {
                  match operand
                  {
                    OperandType::Constant     ( value ) |
                    OperandType::Displacement ( value )
                    =>  if  *value  <=  *maximum
                        &&  *value  >=  *minimum
                        {
                          for ctr       in  lowerByte .. upperByte
                          {
                            bytes.push  ( ( ( *value  >>  ( 8 * ctr ) ) & 0xff  ) as  u8  );
                          }
                          //*skip         =   count;  //  not really necessary, but might be useful for debugging?
                          width         +=  1;
                          space         +=  1;
                        }
                        else
                        {
                          //*skip         =   count;  //  not really necessary, but might be useful for debugging?
                          result        =   result.outOfBounds                          ( count,                    *value,           *minimum, *maximum,         );
                        },
                    _
                    =>  if  operand.isAbstract ( )
                        {
                          *skip         =   count;
                          result        =   InstructionResult::Rerun;
                          break;
                        }
                        else
                        {
                          //*skip         =   count;  //  not really necessary, but might be useful for debugging?
                          result        =   result.invalidArgument                      ( count                                                                   );
                        },
                  }
                }
                result
              },
          asm::Label                    ( identifier    )
          =>  if  let Ok ( reference )
                      = symbols.define
                        (
                          identifier.to_string  ( ),
                          Some  ( OperandType::Address  ( address.clone ( ) ) ),
                          round,
                        )
              {
                replace
                = Some
                  (
                    InstructionType::asm
                    (
                      asm::Reference
                      (
                        reference,
                      )
                    )
                  );
                InstructionResult::Again
              }
              else
              {
                InstructionResult::Again.error                                            ( "Label already defined".to_string ( )                                   )
              },
          asm::ReadFile
          {
            offs,
            size,
            name,
          }
          =>  match File::open  ( &name )
              {
                Ok  ( mut file  )
                =>  match if  *offs >=  0
                          {
                            file.seek ( SeekFrom::Start ( *offs as  u64 ) )
                          }
                          else
                          {
                            file.seek ( SeekFrom::End   ( *offs as  i64 ) )
                          }
                    {
                      Ok  ( offs  )
                      =>  match file.metadata ( )
                          {
                            Ok  ( info  )
                            =>  {
                                  let mut buffer
                                        =   Vec::new  ( );
                                  let     length
                                        =   info.len  ( ) - offs;
                                  let
                                  (
                                    theWidth,
                                    theSpace,
                                  )
                                  = if  let Some  ( size  ) = size
                                    {
                                      (
                                        *size,
                                        if  *size > length
                                        {
                                          length
                                        }
                                        else
                                        {
                                          *size
                                        },
                                      )
                                    }
                                    else
                                    {
                                      (
                                        length,
                                        length,
                                      )
                                    };
                                  width =   theWidth;
                                  space =   theWidth;
                                  buffer.resize
                                  (
                                    theSpace  as  usize,
                                    0x00,
                                  );
                                  match file.read_exact ( &mut  buffer  )
                                  {
                                    Ok  ( _     )
                                    =>  {
                                          buffer.resize
                                          (
                                            theWidth  as  usize,
                                            0x00,
                                          );
                                          bytes.append ( &mut  buffer  );
                                          InstructionResult::Ready  ( None  )
                                        },
                                    Err ( error )
                                    =>  InstructionResult::Again.error
                                        (
                                          format!
                                          (
                                            "Cannot Read File ›{}‹: ›{}‹",
                                            name,
                                            error,
                                          )
                                        ),
                                  }
                                },
                            Err ( error )
                            =>  InstructionResult::Again.error
                                (
                                  format!
                                  (
                                    "Cannot Read Metadata Of File ›{}‹: ›{}‹",
                                    name,
                                    error,
                                  )
                                ),
                          },
                      Err ( error )
                      =>  InstructionResult::Again.error
                          (
                            format!
                            (
                              "Cannot Seek To Offset {} Of File ›{}‹: ›{}‹",
                              offs,
                              name,
                              error,
                            )
                          ),
                    },
                Err ( error )
                =>  InstructionResult::Again.error
                    (
                      format!
                      (
                        "Cannot Read Open File ›{}‹: ›{}‹",
                        name,
                        error,
                      )
                    ),
              },
          asm::Reference                ( reference     )
          =>  if  let Some ( error )
                      = symbols.modify
                        (
                          *reference,
                          Some  ( OperandType::Address  ( address.clone ( ) ) ),
                          round,
                        )
              {
                InstructionResult::Again.error                                            ( error.to_string ( )                                                   )
              }
              else
              {
                InstructionResult::Again
              },
          asm::WantData
          =>  {
                if  number  == 1
                {
                  let     operand       =   &operands [ 0 ];
                  match operand
                  {
                    OperandType::Constant     ( value ) |
                    OperandType::Displacement ( value )
                    =>  if  *value  >=  0
                        &&  *value  <=  0xffffffffffffffff
                        {
                          width         =   0;
                          space         =   *value  as  u64 * size  as  u64;
                          InstructionResult::Ready  ( None  )
                        }
                        else
                        {
                          InstructionResult::Again.outOfBounds                            ( 0,                        *value,           0,  0xffffffffffffffff,     )
                        },
                    _
                    =>  if  operand.isAbstract ( )
                        {
                          InstructionResult::Rerun
                        }
                        else
                        {
                          InstructionResult::Again.invalidArgument                        ( 1,                                                                      )
                        },
                  }
                }
                else
                {
                  InstructionResult::Again.invalidNumberOfArguments                       ( number,                   1,                                            )
                }
            },
        };
      if  let Some  ( this  ) = replace
      {
        self.thisSet  ( this  );
      }
      self.setWidthAndSpace
      (
        width,
        space,
      );
      self.append ( &mut bytes );
      result
    }
    else
    {
      InstructionResult::Again.wrongInstructionSet  ( self.this ( ),  "asm",  )
    }
  }
}
