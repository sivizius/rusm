
/*
  pub fn setBranchHint                  ( &mut  self, value:    u8                  ) { self.theBranchHint          =   value;                }
  pub fn setLock                        ( &mut  self, value:    bool                ) { self.hazLock                =   value;            }
  pub fn setRepeat                      ( &mut  self, value:    u8                  ) { self.theRepeat              =   value;            }

  pub fn fail
  (
    &self,
    message:                            String,
  ) -> Result<Option<usize>, String>
  {
    print!  ( "Line {}: ", self.line );
    //Self::printType ( &self.instruction );
    print! ( "{:?}", &self.instruction );
    for   operand                       in  self.getOperands()
    {
      operand.print ( self.size );
    }
    println!  ( "" );
    Err ( message )
  }

  pub fn print
  (
    &self,
  )
  {
    if self.instruction > InstructionType::ActualInstruction
    {
      if let Some ( address ) = self.address
      {
        print!  ( "{:04x}:{:016x} ",  address.base, address.offs );
      }
      else
      {
        print!  ( "None:None             " );
      }
    }
    else
    {
      print!    ( "                      " );
    }
    //Self::printType ( &self.instruction );
    print!      ( "{:?}", &self.instruction );
    for   operand                       in  self.getOperands()
    {
      operand.print ( self.size );
    }
    println!    ( "" );
  }

  pub fn printType
  (
    theType:                            &InstructionType,
  )
  {
    match theType
    {
      InstructionType::Label            ( identifier  )
      =>  print!  ( "label {}", identifier  ),
      InstructionType::ADD
      =>  print!  ( "add"                   ),
      InstructionType::OR
      =>  print!  ( "or "                   ),
      InstructionType::ADC
      =>  print!  ( "adc"                   ),
      InstructionType::SBB
      =>  print!  ( "sbb"                   ),
      InstructionType::AND
      =>  print!  ( "and"                   ),
      InstructionType::SUB
      =>  print!  ( "sub"                   ),
      InstructionType::XOR
      =>  print!  ( "xor"                   ),
      InstructionType::CMP
      =>  print!  ( "cmp"                   ),
      _
      =>  print!  ( "???"                   ),
    }
  }

  pub fn failOperandSize
  (
    &self,
  ) -> Result<Option<usize>, String>
  {
    self.fail
    (
      if  self.size ==  0
      {
        format!
        (
          "Operand Size not Specified",
        )
      }
      else
      {
        format!
        (
          "Invalid Operand Size {}",
          self.size,
        )
      }
    )
  }

  pub fn failOutOfBounds
  (
    &self,
    lowerBound:                         i128,
    upperBound:                         i128,
    immediate:                          i128,
  ) -> Result<Option<usize>, String>
  {
    self.fail
    (
      format!
      (
        "Value Out of Bonds [{},{}] {}",
        lowerBound,
        upperBound,
        immediate,
      )
    )
  }

  pub fn getBranchHint                  ( &self )     ->  u8                          { self.theBranchHint                                    }
  pub fn getDisplacement                ( &self )     ->  ( usize, i128 )             { ( self.displacementLength,  self.displacementValue  ) }
  pub fn getImmediate                   ( &self )     ->  ( usize, i128 )             { ( self.immediateLength,     self.immediateValue     ) }
  pub fn getModRegRM                    ( &self )     ->  Option  < u8  >             { self.theModRegRM                                      }
  pub fn getOpcode                      ( &self )     ->  u8                          { self.theOpcode                                        }
  pub fn getRepeat                      ( &self )     ->  u8                          { self.theRepeat                                        }
  pub fn getREX                         ( &self )     ->  u8                          { self.theREX                                           }
  pub fn getSegmentOverride             ( &self )     ->  u8                          { self.theSegmentOverride                               }
  pub fn getSIBByte                     ( &self )     ->  Option  < u8  >             { self.theSIBByte                                       }

  pub fn hazAddressSizeOverride         ( &self )     ->  bool                        { self.hazAddressSizeOverride                           }
  pub fn hazBranchHint                  ( &self )     ->  bool                        { self.theBranchHint          !=  0                     }
  pub fn hazLock                        ( &self )     ->  bool                        { self.hazLock                                          }
  pub fn hazOperandSizeOverride         ( &self )     ->  bool                        { self.hazOperandSizeOverride                           }
  pub fn hazRepeat                      ( &self )     ->  bool                        { self.theRepeat              !=  0                     }
  pub fn hazREX                         ( &self )     ->  bool                        { self.theREX                 !=  0                     }
  pub fn hazSegmentOverride             ( &self )     ->  bool                        { self.theSegmentOverride     !=  0                     }
  pub fn hazThreeByteVEX                ( &self )     ->  bool                        { self.hazThreeByteVEX                                  }
  pub fn hazThreeByteXOP                ( &self )     ->  bool                        { self.hazThreeByteXOP                                  }
  pub fn hazTwoByteOpcode               ( &self )     ->  bool                        { self.hazTwoByteOpcode                                 }
  pub fn hazTwoByteVEX                  ( &self )     ->  bool                        { self.hazTwoByteVEX                                    }

*/
