pub mod memory;
pub mod operands;
pub mod prefixes;
pub mod registers;
pub mod state;

mod io;
mod jumps;
mod simpleMath;
mod zeroOperands;

use self::
{
  state::
  {
    x86prefixByte,
    x86state,
    x86version,
  },
  registers::
  {
    SegmentRegisterNumber,
  },
};

use super::
{
  Architecture,
  Assembly,
  instructions::
  {
    Instruction,
    InstructionResult,
    InstructionType,
  },
};

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum    x86
{
  AAA,
  AAD,
  AAS,
  AAM,
  ADC,
  ADD,
  AND,
  CALL,
  CBW,
  CLC,
  CLD,
  CLI,
  CMC,
  CMP,
  CMPSB,
  CMPSW,
  CWD,
  DAA,
  DAS,
  DEC,
  DIV,
  ESC,
  HLT,
  IDIV,
  IMUL,
  IN,
  INC,
  INT,
  INT3,
  INTO,
  IRET,
  JB,
  JBE,
  JCXZ,
  JE,
  JL,
  JLE,
  JMP,
  JNB,
  JNBE,
  JNE,
  JNL,
  JNLE,
  JNO,
  JNP,
  JNS,
  JO,
  JP,
  JS,
  LAHF,
  LDS,
  LEA,
  LES,
  LODSB,
  LODSW,
  LOOP,
  LOOPZ,
  LOOPNZ,
  MOV,
  MOVSB,
  MOVSW,
  MUL,
  NEG,
  NOT,
  OR,
  OUT,
  POP,
  POPF,
  PUSH,
  PUSHF,
  RCL,
  RCR,
  RETF,
  RETN,
  ROL,
  ROR,
  SAHF,
  SAL,
  SALC,
  SAR,
  SHL,
  SHR,
  SBB,
  SCASB,
  SCASW,
  STC,
  STD,
  STI,
  STOSB,
  STOSW,
  SUB,
  TEST,
  WAIT,
  XCHG,
  XLAT,
  XOR,
}

#[allow(non_camel_case_types)]
pub struct    x86instruction
{
  theSegmentOverride:                   x86prefixByte,
  hazOperandSizeOverride:               bool,
  hazAddressSizeOverride:               bool,
  hazThreeByteXOP:                      bool,
  hazTwoByteVEX:                        bool,
  hazThreeByteVEX:                      bool,
  theREX:                               x86prefixByte,
  hazTwoByteOpcode:                     bool,
  theOpcode:                            u8,
  theModRegRM:                          Option  < u8  >,
  theSIBByte:                           Option  < u8  >,
  displacementLength:                   usize,
  displacementValue:                    i128,
  immediateLength:                      usize,
  immediateValue:                       i128,
}

impl          x86instruction
{
  pub fn setAddressSizeOverride         ( &mut  self, value:    bool                ) { self.hazAddressSizeOverride =   value;                }
  pub fn setDisplacement
  (
    &mut  self,
    length:                             usize,
    value:                              i128
  )
  {
    self.displacementLength             =   length;
    self.displacementValue              =   value;
  }
  pub fn setImmediate
  (
    &mut  self,
    length:                             usize,
    value:                              i128
  )
  {
    self.immediateLength                =   length;
    self.immediateValue                 =   value;
  }
  pub fn setImmediateLength             ( &mut  self, value:    usize               ) { self.immediateLength        =   value;            }
  pub fn setModRegRM                    ( &mut  self, value:    u8                  ) { self.theModRegRM            =   Some ( value  );  }
  pub fn setOpcode                      ( &mut  self, opcode:   u8                  ) { self.theOpcode              =   opcode;           }
  pub fn setOperandSizeOverride         ( &mut  self, value:    bool                ) { self.hazOperandSizeOverride =   value;            }
  pub fn setREX                         ( &mut  self, value:    x86prefixByte       ) { self.theREX                 =   value;            }
  pub fn setSegmentOverride             ( &mut  self, value:    x86prefixByte       ) { self.theSegmentOverride     =   value;            }
  pub fn setSIBByte                     ( &mut  self, value:    u8                  ) { self.theSIBByte             =   Some ( value  );  }
  pub fn setThreeByteVEX                ( &mut  self, value:    bool                ) { self.hazThreeByteVEX        =   value;            }
  pub fn setThreeByteXOP                ( &mut  self, value:    bool                ) { self.hazThreeByteXOP        =   value;            }
  pub fn setTwoByteOpcode               ( &mut  self, value:    bool                ) { self.hazTwoByteOpcode       =   value;            }
  pub fn setTwoByteVEX                  ( &mut  self, value:    bool                ) { self.hazTwoByteVEX          =   value;            }

  pub fn encodeModRegRMdata
  (
    mut self,
    //  Instruction
    opcode:                             u8,
    signExtension:                      bool,
    size:                               usize,
    //  Operands
    _segment:                            SegmentRegisterNumber,
    regRegisters:                       u8,
    memRegisters:                       u8,
    displacement:                       Option<i128>,
    immediate:                          Option<i128>,
    //  Assembly
    state:                              &x86state,
  )
  -> x86result
  {
    let
    (
      modField,
      dispSize,
    )
    = match displacement
      {
        None                          =>  ( 0xc0, 0,  ),
        Some  ( 0                   ) =>  ( 0x00, 0,  ),
        Some  ( -0x80   ... 0x7f    ) =>  ( 0x40, 1,  ),
        Some  ( -0x8000 ... 0x7fff  ) =>  ( 0x80, 2,  ),
        Some  ( displacement        ) =>  return x86result::InvalidDisplacement  ( displacement  ),
      };
    self.theModRegRM                    =   Some  ( modField  | regRegisters  <<  3 | memRegisters  );
    self.displacementLength             =   dispSize;
    if let  Some  ( dispValue ) = displacement
    {
      self.displacementLength           =   dispSize;
      self.displacementValue            =   dispValue;
    }
    match size
    {
      1
      =>  {
            self.theOpcode              =   opcode  | 0;
            if let Some ( value ) = immediate
            {
              self.immediateValue       =   value;
              if  value >= -0x80
              &&  value <=  0xff
              {
                self.immediateLength    =   1;
                x86result::Ready  ( self  )
              }
              else
              {
                x86result::OutOfBounds
                {
                  number:               1,
                  value:                value,
                  minimum:              -0x80,
                  maximum:              0xff,
                }
              }
            }
            else
            {
              x86result::Ready  ( self  )
            }
          },
      2
      =>  {
            if let Some ( value ) = immediate
            {
              self.immediateValue       =   value;
              if  state.operandSize ( ) ==  4
              {
                self.hazOperandSizeOverride
                                        =   true;
              }
              if        value >= -0x80
              &&        value <=  0x7f
              &&        (
                          signExtension
                        ||
                          state.version ( ) >= x86version::i386
                        )
              //&&       !(
              //            self.features.hazFeature ( AssemblyFeatures::RandomOpcodeSize )
              //          &&
              //            rand::random()
              //          )
              {
                self.theOpcode          =   opcode  | 3;
                self.immediateLength    =   1;
                x86result::Ready  ( self  )
              }
              else  if  value >= -0x8000
                    &&  value <=  0xffff
              {
                self.theOpcode          =   opcode  | 1;
                self.immediateLength    =   2;
                x86result::Ready  ( self  )
              }
              else
              {
                x86result::OutOfBounds
                {
                  number:               1,
                  value:                value,
                  minimum:              -0x8000,
                  maximum:              0xffff,
                }
              }
            }
            else
            {
              self.theOpcode            =   opcode  | 1;
              x86result::Ready  ( self  )
            }
          },
      4 if  state.version ( ) >=  x86version::i386
      =>  {
            if let Some ( value ) = immediate
            {
              if  state.operandSize ( ) ==  2
              {
                self.hazOperandSizeOverride
                                        =   true;
              }
              self.immediateValue       =   value;
              if        value >= -0x80
              &&        value <=  0x7f
              //&&       !(
              //            self.features.hazFeature ( AssemblyFeatures::RandomOpcodeSize )
              //          &&
              //            rand::random()
              //          )
              {
                self.theOpcode          =   opcode  | 3;
                self.immediateLength    =   1;
                x86result::Ready  ( self  )
              }
              else  if  value >= -0x80000000
                    &&  value <=  0xffffffff
              {
                self.theOpcode          =   opcode  | 1;
                self.immediateLength    =   4;
                x86result::Ready  ( self  )
              }
              else
              {
                x86result::OutOfBounds
                {
                  number:               1,
                  value:                value,
                  minimum:              -0x80000000,
                  maximum:              0xffffffff,
                }
              }
            }
            else
            {
              self.theOpcode            =   opcode  | 1;
              x86result::Ready  ( self  )
            }
          },
      _
      =>  x86result::InvalidOperandSize,
    }
  }
}

#[allow(non_camel_case_types)]
pub enum      x86result
{
  Equal                                 ( u64             ),
  InvalidArgumentType,
  InvalidCombinationOfArguments,
  InvalidDisplacement                   ( i128            ),
  InvalidNumberOfArguments              ( usize           ),
  InvalidOperandSize,
  JumpToFar                             ( i128            ),
  NotImplemented                        ( &'static str    ),
  OutOfBounds
  {
    number:                             usize,
    value:                              i128,
    minimum:                            i128,
    maximum:                            i128,
  },
  Ready                                 ( x86instruction  ),
  Rerun,
}

impl          Assembly
{
}

impl          Instruction
{
  pub fn x86compile
  (
    &mut self,
    architecture:                       &mut Architecture,
    round:                              usize,
  )
  ->  InstructionResult
  {
    let     operands                    =   &self.operands ( );
    let     size                        =   self.size      ( );
    if  let InstructionType::x86
            {
              architecture:             ref mut state,
              instruction:              this,
            } = self.thisRef  ( )
    {
      if  let Architecture::x86 ( ref mut architecture  ) = architecture
      {
        //  All
        //    optional prefixes (lock, branch hints, string repeats),
        //    instruction-specific features (randomised operand encodings),
        //    …
        //  which modify the state,
        //    should be processed in the first round,
        //      while the initial x86-instruction has an invalid state.
        //  Therefore this state shall be set in,
        //    and only in,
        //  round zero.
        if  round ==  0
        {
          *state                        =   architecture.clone  ( );
          architecture.reset ( );
        }
        let     instruction
        = x86instruction
          {
            theSegmentOverride:         x86prefixByte::Default,
            hazOperandSizeOverride:     false,
            hazAddressSizeOverride:     false,
            hazThreeByteXOP:            false,
            hazTwoByteVEX:              false,
            hazThreeByteVEX:            false,
            theREX:                     x86prefixByte::Default,
            hazTwoByteOpcode:           false,
            theOpcode:                  0,
            theModRegRM:                None,
            theSIBByte:                 None,
            displacementLength:         0,
            displacementValue:          0,
            immediateLength:            0,
            immediateValue:             0,
          };
        match match this
              {
                x86::AAA      =>  instruction.compileZeroOperand  ( 0x37,                                 ),
                x86::AAD      =>  x86result::NotImplemented       ( "aad"                                 ),
                x86::AAS      =>  instruction.compileZeroOperand  ( 0x3f,                                 ),
                x86::AAM      =>  x86result::NotImplemented       ( "aam",                                ),
                x86::ADC      =>  instruction.compileSimpleMath   ( 0x10, true,   size, operands, state,  ),
                x86::ADD      =>  instruction.compileSimpleMath   ( 0x00, true,   size, operands, state,  ),
                x86::AND      =>  instruction.compileSimpleMath   ( 0x20, false,  size, operands, state,  ),
                x86::CALL     =>  x86result::NotImplemented       ( "call",                               ),
                x86::CBW      =>  instruction.compileZeroOperand  ( 0x98,                                 ),
                x86::CLC      =>  instruction.compileZeroOperand  ( 0xf8,                                 ),
                x86::CLD      =>  instruction.compileZeroOperand  ( 0xfc,                                 ),
                x86::CLI      =>  instruction.compileZeroOperand  ( 0xfa,                                 ),
                x86::CMC      =>  instruction.compileZeroOperand  ( 0xf5,                                 ),
                x86::CMP      =>  instruction.compileSimpleMath   ( 0x38, false,  size, operands, state,  ),
                x86::CMPSB    =>  instruction.compileZeroOperand  ( 0xa6,                                 ),
                x86::CMPSW    =>  instruction.compileZeroOperand  ( 0xa7,                                 ),
                x86::CWD      =>  instruction.compileZeroOperand  ( 0x99,                                 ),
                x86::DAA      =>  instruction.compileZeroOperand  ( 0x27,                                 ),
                x86::DAS      =>  instruction.compileZeroOperand  ( 0x2f,                                 ),
                x86::DEC      =>  x86result::NotImplemented       ( "dec",                                ),
                x86::DIV      =>  x86result::NotImplemented       ( "div",                                ),
                x86::ESC      =>  x86result::NotImplemented       ( "esc",                                ),
                x86::HLT      =>  instruction.compileZeroOperand  ( 0xf4,                                 ),
                x86::IDIV     =>  x86result::NotImplemented       ( "idiv",                               ),
                x86::IMUL     =>  x86result::NotImplemented       ( "imul",                               ),
                x86::IN       =>  instruction.compileIO           ( 0xe4, false,        operands, state,  ),
                x86::INC      =>  x86result::NotImplemented       ( "inc",                                ),
                x86::INT      =>  x86result::NotImplemented       ( "int",                                ),
                x86::INT3     =>  instruction.compileZeroOperand  ( 0xcc,                                 ),
                x86::INTO     =>  instruction.compileZeroOperand  ( 0xce,                                 ),
                x86::IRET     =>  instruction.compileZeroOperand  ( 0xcf,                                 ),
                x86::JB       =>  instruction.compileJump         ( 0x72,               operands,         ),
                x86::JBE      =>  instruction.compileJump         ( 0x76,               operands,         ),
                x86::JCXZ     =>  instruction.compileJump         ( 0xe3,               operands,         ),
                x86::JE       =>  instruction.compileJump         ( 0x74,               operands,         ),
                x86::JL       =>  instruction.compileJump         ( 0x7c,               operands,         ),
                x86::JLE      =>  instruction.compileJump         ( 0x7e,               operands,         ),
                x86::JMP      =>  x86result::NotImplemented       ( "jmp",                                ),
                x86::JNB      =>  instruction.compileJump         ( 0x73,               operands,         ),
                x86::JNBE     =>  instruction.compileJump         ( 0x77,               operands,         ),
                x86::JNE      =>  instruction.compileJump         ( 0x75,               operands,         ),
                x86::JNL      =>  instruction.compileJump         ( 0x7d,               operands,         ),
                x86::JNLE     =>  instruction.compileJump         ( 0x7f,               operands,         ),
                x86::JNO      =>  instruction.compileJump         ( 0x71,               operands,         ),
                x86::JNP      =>  instruction.compileJump         ( 0x7b,               operands,         ),
                x86::JNS      =>  instruction.compileJump         ( 0x79,               operands,         ),
                x86::JO       =>  instruction.compileJump         ( 0x70,               operands,         ),
                x86::JP       =>  instruction.compileJump         ( 0x7a,               operands,         ),
                x86::JS       =>  instruction.compileJump         ( 0x78,               operands,         ),
                x86::LAHF     =>  instruction.compileZeroOperand  ( 0x9f,                                 ),
                x86::LDS      =>  x86result::NotImplemented       ( "lds",                                ),
                x86::LEA      =>  x86result::NotImplemented       ( "lea",                                ),
                x86::LES      =>  x86result::NotImplemented       ( "les",                                ),
                x86::LODSB    =>  instruction.compileZeroOperand  ( 0xac,                                 ),
                x86::LODSW    =>  instruction.compileZeroOperand  ( 0xad,                                 ),
                x86::LOOP     =>  instruction.compileJump         ( 0xe2,               operands,         ),
                x86::LOOPZ    =>  instruction.compileJump         ( 0xe1,               operands,         ),
                x86::LOOPNZ   =>  instruction.compileJump         ( 0xe0,               operands,         ),
                x86::MOV      =>  x86result::NotImplemented       ( "mov",                                ),
                x86::MOVSB    =>  instruction.compileZeroOperand  ( 0xa4,                                 ),
                x86::MOVSW    =>  instruction.compileZeroOperand  ( 0xa5,                                 ),
                x86::MUL      =>  x86result::NotImplemented       ( "mul",                                ),
                x86::NEG      =>  x86result::NotImplemented       ( "neg",                                ),
                x86::NOT      =>  x86result::NotImplemented       ( "not",                                ),
                x86::OR       =>  instruction.compileSimpleMath   ( 0x08, false,  size, operands, state,  ),
                x86::OUT      =>  instruction.compileIO           ( 0xe6, true,         operands, state,  ),
                x86::POP      =>  x86result::NotImplemented       ( "pop",                                ),
                x86::POPF     =>  instruction.compileZeroOperand  ( 0x9d,                                 ),
                x86::PUSH     =>  x86result::NotImplemented       ( "push",                               ),
                x86::PUSHF    =>  instruction.compileZeroOperand  ( 0x9c,                                 ),
                x86::RCL      =>  x86result::NotImplemented       ( "rcl",                                ),
                x86::RCR      =>  x86result::NotImplemented       ( "rcr",                                ),
                x86::RETF     =>  x86result::NotImplemented       ( "retf",                               ),
                x86::RETN     =>  x86result::NotImplemented       ( "retn",                               ),
                x86::ROL      =>  x86result::NotImplemented       ( "rol",                                ),
                x86::ROR      =>  x86result::NotImplemented       ( "ror",                                ),
                x86::SAHF     =>  instruction.compileZeroOperand  ( 0x9e,                                 ),
                x86::SAL      =>  x86result::NotImplemented       ( "sal",                                ),
                x86::SALC     =>  instruction.compileZeroOperand  ( 0xd6,                                 ),
                x86::SAR      =>  x86result::NotImplemented       ( "sar",                                ),
                x86::SHL      =>  x86result::NotImplemented       ( "shl",                                ),
                x86::SHR      =>  x86result::NotImplemented       ( "shr",                                ),
                x86::SBB      =>  instruction.compileSimpleMath   ( 0x18, true,   size, operands, state,  ),
                x86::SCASB    =>  instruction.compileZeroOperand  ( 0xae,                                 ),
                x86::SCASW    =>  instruction.compileZeroOperand  ( 0xaf,                                 ),
                x86::STC      =>  instruction.compileZeroOperand  ( 0xf9,                                 ),
                x86::STD      =>  instruction.compileZeroOperand  ( 0xfd,                                 ),
                x86::STI      =>  instruction.compileZeroOperand  ( 0xfb,                                 ),
                x86::STOSB    =>  instruction.compileZeroOperand  ( 0xaa,                                 ),
                x86::STOSW    =>  instruction.compileZeroOperand  ( 0xab,                                 ),
                x86::SUB      =>  instruction.compileSimpleMath   ( 0x28, true,   size, operands, state,  ),
                x86::TEST     =>  x86result::NotImplemented       ( "test",                               ),
                x86::WAIT     =>  instruction.compileZeroOperand  ( 0xdb,                                 ),
                x86::XCHG     =>  x86result::NotImplemented       ( "xchg",                               ),
                x86::XLAT     =>  instruction.compileZeroOperand  ( 0xd7,                                 ),
                x86::XOR      =>  instruction.compileSimpleMath   ( 0x30, false,  size, operands, state,  ),
                //_             =>  x86result::NotImplemented       ( "???",                                ),
              }
        {
          x86result::Equal                    ( size  )
          =>  InstructionResult::Equal
              (
                size,
                size,
              ),
          x86result::InvalidArgumentType
          =>  InstructionResult::Again.error
              (
                "Invalid Type Of Argument.".to_string ( )
              ),
          x86result::InvalidCombinationOfArguments
          =>  InstructionResult::Again.error
              (
                "Invalid Combination of Arguments.".to_string ( )
              ),
          x86result::InvalidDisplacement      ( disp  )
          =>  InstructionResult::Again.error
              (
                format!
                (
                  "Invalid Displacement {}.",
                  disp,
                )
              ),
          x86result::InvalidNumberOfArguments ( want  )
          =>  InstructionResult::Again.invalidNumberOfArguments
              (
                self.operandsNumber ( ),
                want,
              ),
          x86result::InvalidOperandSize
          =>  InstructionResult::Again.invalidOperandSize
              (
                self.size ( ),
              ),
          x86result::JumpToFar                ( disp  )
          =>  InstructionResult::Again.error
              (
                format!
                (
                  "Destination of Jump To Far Away: {}",
                  disp,
                )
              ),
          x86result::NotImplemented           ( name  )
          =>  InstructionResult::Again.notImplemented
              (
                format!
                (
                  "Instruction ›{}‹",
                  name,
                )
              ),
          x86result::OutOfBounds
          {
            number,
            value,
            minimum,
            maximum,
          }
          =>  InstructionResult::Again.outOfBounds
              (
                number,
                value,
                minimum,
                maximum,
              ),
          x86result::Ready                    ( this  )
          =>  {
                let     theBranchHint   =   state.theBranchHint ( );
                let     theRepeat       =   state.theRepeat     ( );

                //  Group 1
                if  state.hazLock ( )                                       { self.push ( x86prefixByte::Lock.toByte                ( ) ); }
                if  theRepeat                   !=  x86prefixByte::Default  { self.push ( theRepeat.toByte                          ( ) ); }

                //  Group 2
                if  this.theSegmentOverride     !=  x86prefixByte::Default  { self.push ( this.theSegmentOverride.toByte            ( ) ); }
                if  theBranchHint               !=  x86prefixByte::Default  { self.push ( theBranchHint.toByte                      ( ) ); }

                //  Group 3
                if  this.hazOperandSizeOverride                             { self.push ( x86prefixByte::OperandSizeOverride.toByte ( ) ); }

                //  Group 4
                if  this.hazAddressSizeOverride                             { self.push ( x86prefixByte::AddressSizeOverride.toByte ( ) ); }

                if  this.hazThreeByteXOP                                    { self.push ( x86prefixByte::ThreeByteXOP.toByte        ( ) ); }
                if  this.hazTwoByteVEX                                      { self.push ( x86prefixByte::TwoByteVEX.toByte          ( ) ); }
                if  this.hazThreeByteVEX                                    { self.push ( x86prefixByte::ThreeByteVEX.toByte        ( ) ); }
                if  this.theREX                 !=  x86prefixByte::Default  { self.push ( this.theREX.toByte                        ( ) ); }

                //  Opcode
                if  this.hazTwoByteOpcode                                   { self.push ( x86prefixByte::TwoByteOpcode.toByte       ( ) ); }
                self.push   ( this.theOpcode                                                  );

                //  Mod Reg R/M
                if  let Some  ( value ) = this.theModRegRM
                {
                  self.push ( value                                                           );
                }

                //  Scale Index Base
                if  let Some  ( value ) = this.theSIBByte
                {
                  self.push ( value                                                           );
                }

                //  Displacement Value
                for ctr                 in  0 ..  this.displacementLength
                {
                  self.push ( ( ( this.displacementValue  >>  ( 8 * ctr ) ) & 0xff  ) as  u8  );
                }

                //  Immediate Value
                for ctr                 in  0 ..  this.immediateLength
                {
                  self.push ( ( ( this.immediateValue     >>  ( 8 * ctr ) ) & 0xff  ) as  u8  );
                }

                //  And Return
                let     length          =   self.length ( );
                self.setWidthAndSpace
                (
                  length,
                  length,
                );
                InstructionResult::Ready
              },
          x86result::Rerun
          =>  InstructionResult::Rerun,
        }
      }
      else
      {
        InstructionResult::Again.wrongArchitecture  ( *architecture,  "x86",  )
      }
    }
    else
    {
      InstructionResult::Again.wrongInstructionSet  ( self.this ( ),  "x86",  )
    }
  }
}
