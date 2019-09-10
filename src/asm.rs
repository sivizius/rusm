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

/// Create common instructions as standalone instructions of a pseudo instruction set.
/// There instructions are not connected to any `Assembly`,
///   but could be `push`ed or `append`ed to an existing `Assembly`.
#[allow(non_camel_case_types)]
pub struct    asm;
impl          asm
{
  /// Add some raw bytes (8 bit).
  ///
  /// # Arguments
  /// *  `data`  – list of values.
  pub fn db
  (
    data:                               Vec < impl Operand  >,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::EmitData
      {
        minimum:                        -0x7f,
        maximum:                        0xff,
        endianness:                     Endianness::Default,
        skip:                           0,
      },
      1,
      data.into_iter  ( ).map ( | x | ( x.this  ( ) ).0 ).collect ( ),
    )
  }

  /// Add some raw words (16 bit).
  ///
  /// # Arguments
  /// * `data`  – list of values.
  pub fn dw
  (
    data:                               Vec < impl Operand  >,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::EmitData
      {
        minimum:                        -0x7fff,
        maximum:                        0xffff,
        endianness:                     Endianness::Default,
        skip:                           0,
      },
      2,
      data.into_iter  ( ).map ( | x | ( x.this  ( ) ).0 ).collect ( ),
    )
  }

  /// Add some raw dwords (32 bit).
  ///
  /// # Arguments
  /// * `data`  – list of values.
  pub fn dd
  (
    data:                               Vec < impl Operand  >,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::EmitData
      {
        minimum:                        -0x7fff_ffff,
        maximum:                        0xffff_ffff,
        endianness:                     Endianness::Default,
        skip:                           0,
      },
      4,
      data.into_iter  ( ).map ( | x | ( x.this  ( ) ).0 ).collect ( ),
    )
  }

  /// Add some raw qwords (64 bit).
  ///
  /// # Arguments
  /// * `data`  – list of values.
  pub fn dq
  (
    data:                               Vec < impl Operand  >,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::EmitData
      {
        minimum:                        -0x7fff_ffff_ffff_ffff,
        maximum:                        0xffff_ffff_ffff_ffff,
        endianness:                     Endianness::Default,
        skip:                           0,
      },
      8,
      data.into_iter  ( ).map ( | x | ( x.this  ( ) ).0 ).collect ( ),
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

  /// Reserve some raw bytes (8 bit).
  ///
  /// # Arguments
  /// *  `length` – space to be reserved in bytes.
  pub fn rb
  (
    length:                             impl Operand,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::WantData,
      1,
      vec!  ( length.this ( ).0 ),
    )
  }

  /// Reserve some raw words (16 bit).
  ///
  /// # Arguments
  /// *  `length` – space to be reserved in words.
  pub fn rw
  (
    length:                             impl Operand,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::WantData,
      2,
      vec!  ( length.this ( ).0 ),
    )
  }

  /// Reserve some raw dwords (32 bit).
  ///
  /// # Arguments
  /// *  `length` – space to be reserved in dwords.
  pub fn rd
  (
    length:                             impl Operand,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::WantData,
      4,
      vec!  ( length.this ( ).0 ),
    )
  }

  /// Reserve some raw qwords (64 bit).
  ///
  /// # Arguments
  /// *  `length` – space to be reserved in qwords.
  pub fn rq
  (
    length:                             impl Operand,
  )
  ->  Instruction
  {
    Instruction
    (
      InstructionType::WantData,
      8,
      vec!  ( length.this ( ).0 ),
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
  assemblyListOperand!    ( db,     asm::db,    );
  assemblyListOperand!    ( dw,     asm::dw,    );
  assemblyListOperand!    ( dd,     asm::dd,    );
  assemblyListOperand!    ( dq,     asm::dq,    );
  assemblyStringOperand!  ( label,  asm::label, );
  assemblyOneOperand!     ( rb,     asm::rb,    );
  assemblyOneOperand!     ( rw,     asm::rw,    );
  assemblyOneOperand!     ( rd,     asm::rd,    );
  assemblyOneOperand!     ( rq,     asm::rq,    );
  assemblyStringOperand!  ( utf8,   asm::utf8,  );

  pub fn emit
  (
    self,
    size:                               usize,
    data:                               Vec < impl Operand  >,
  )
  ->  Self  { self.push ( asm::emit   ( size, data, ) ) }
}