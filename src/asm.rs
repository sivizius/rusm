use super::
{
  Assembly,
  Endianness,
  instructions::
  {
    Instruction,
    InstructionType,
  },
  operands::
  {
    Operand,
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
            InstructionType::EmitData
            {
              minimum:                  -1  <<  ( $theSize  * 8 - 1 ),
              maximum:                  ( 1 <<  ( $theSize  * 8 ) ) - 1,
              endianness:               Endianness::Default,
              skip:                     0,
            },
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
            InstructionType::WantData,
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
pub struct    asm;
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
      InstructionType::Append ( list  ),
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
      InstructionType::EmitData
      {
        minimum:                        -1 <<  ( size * 8 - 1 ),
        maximum:                        ( 1 <<  ( size * 8 ) ) - 1,
        endianness:                     Endianness::Default,
        skip:                           0,
      },
      size,
      data.into_iter  ( ).map ( | x | ( x.this  ( ) ).0 ).collect ( ),
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
      InstructionType::Label ( name ),
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
      InstructionType::EmitData
      {
        minimum:                        0,
        maximum:                        0xff,
        endianness:                     Endianness::LittleEndian,
        skip:                           0,
      },
      1,
      text.into_bytes ( ).into_iter  ( ).map ( | x | ( (  x as  i128  ).this  ( ) ).0 ).collect ( ),
    )
  }
}

impl          Assembly
{
  //assemblyOneOperand!     ( append, asm::append,  );
  assemblyListOperand!    ( db,     asm::db,      );
  assemblyListOperand!    ( dw,     asm::dw,      );
  assemblyListOperand!    ( dd,     asm::dd,      );
  assemblyListOperand!    ( dq,     asm::dq,      );
  assemblyStringOperand!  ( label,  asm::label,   );
  assemblyOneOperand!     ( rb,     asm::rb,      );
  assemblyOneOperand!     ( rw,     asm::rw,      );
  assemblyOneOperand!     ( rd,     asm::rd,      );
  assemblyOneOperand!     ( rq,     asm::rq,      );
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
}