# [doc = "Register `INT_EVENT1_RIS` reader"] pub type R = crate :: R < INT_EVENT1_RIS_SPEC > ; # [doc = "Field `INT_EVENT1_RIS_DIO0` reader - DIO0 event"] pub type INT_EVENT1_RIS_DIO0_R = crate :: BitReader < INT_EVENT1_RIS_DIO0_A > ; # [doc = "DIO0 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO0_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO0_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO0_SET = 1 , } impl From < INT_EVENT1_RIS_DIO0_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO0_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO0_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO0_A { match self . bits { false => INT_EVENT1_RIS_DIO0_A :: INT_EVENT1_RIS_DIO0_CLR , true => INT_EVENT1_RIS_DIO0_A :: INT_EVENT1_RIS_DIO0_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio0_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO0_A :: INT_EVENT1_RIS_DIO0_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio0_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO0_A :: INT_EVENT1_RIS_DIO0_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO1` reader - DIO1 event"] pub type INT_EVENT1_RIS_DIO1_R = crate :: BitReader < INT_EVENT1_RIS_DIO1_A > ; # [doc = "DIO1 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO1_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO1_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO1_SET = 1 , } impl From < INT_EVENT1_RIS_DIO1_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO1_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO1_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO1_A { match self . bits { false => INT_EVENT1_RIS_DIO1_A :: INT_EVENT1_RIS_DIO1_CLR , true => INT_EVENT1_RIS_DIO1_A :: INT_EVENT1_RIS_DIO1_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio1_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO1_A :: INT_EVENT1_RIS_DIO1_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio1_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO1_A :: INT_EVENT1_RIS_DIO1_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO2` reader - DIO2 event"] pub type INT_EVENT1_RIS_DIO2_R = crate :: BitReader < INT_EVENT1_RIS_DIO2_A > ; # [doc = "DIO2 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO2_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO2_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO2_SET = 1 , } impl From < INT_EVENT1_RIS_DIO2_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO2_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO2_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO2_A { match self . bits { false => INT_EVENT1_RIS_DIO2_A :: INT_EVENT1_RIS_DIO2_CLR , true => INT_EVENT1_RIS_DIO2_A :: INT_EVENT1_RIS_DIO2_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio2_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO2_A :: INT_EVENT1_RIS_DIO2_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio2_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO2_A :: INT_EVENT1_RIS_DIO2_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO3` reader - DIO3 event"] pub type INT_EVENT1_RIS_DIO3_R = crate :: BitReader < INT_EVENT1_RIS_DIO3_A > ; # [doc = "DIO3 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO3_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO3_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO3_SET = 1 , } impl From < INT_EVENT1_RIS_DIO3_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO3_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO3_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO3_A { match self . bits { false => INT_EVENT1_RIS_DIO3_A :: INT_EVENT1_RIS_DIO3_CLR , true => INT_EVENT1_RIS_DIO3_A :: INT_EVENT1_RIS_DIO3_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio3_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO3_A :: INT_EVENT1_RIS_DIO3_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio3_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO3_A :: INT_EVENT1_RIS_DIO3_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO4` reader - DIO4 event"] pub type INT_EVENT1_RIS_DIO4_R = crate :: BitReader < INT_EVENT1_RIS_DIO4_A > ; # [doc = "DIO4 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO4_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO4_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO4_SET = 1 , } impl From < INT_EVENT1_RIS_DIO4_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO4_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO4_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO4_A { match self . bits { false => INT_EVENT1_RIS_DIO4_A :: INT_EVENT1_RIS_DIO4_CLR , true => INT_EVENT1_RIS_DIO4_A :: INT_EVENT1_RIS_DIO4_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio4_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO4_A :: INT_EVENT1_RIS_DIO4_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio4_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO4_A :: INT_EVENT1_RIS_DIO4_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO5` reader - DIO5 event"] pub type INT_EVENT1_RIS_DIO5_R = crate :: BitReader < INT_EVENT1_RIS_DIO5_A > ; # [doc = "DIO5 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO5_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO5_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO5_SET = 1 , } impl From < INT_EVENT1_RIS_DIO5_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO5_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO5_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO5_A { match self . bits { false => INT_EVENT1_RIS_DIO5_A :: INT_EVENT1_RIS_DIO5_CLR , true => INT_EVENT1_RIS_DIO5_A :: INT_EVENT1_RIS_DIO5_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio5_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO5_A :: INT_EVENT1_RIS_DIO5_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio5_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO5_A :: INT_EVENT1_RIS_DIO5_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO6` reader - DIO6 event"] pub type INT_EVENT1_RIS_DIO6_R = crate :: BitReader < INT_EVENT1_RIS_DIO6_A > ; # [doc = "DIO6 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO6_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO6_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO6_SET = 1 , } impl From < INT_EVENT1_RIS_DIO6_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO6_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO6_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO6_A { match self . bits { false => INT_EVENT1_RIS_DIO6_A :: INT_EVENT1_RIS_DIO6_CLR , true => INT_EVENT1_RIS_DIO6_A :: INT_EVENT1_RIS_DIO6_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio6_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO6_A :: INT_EVENT1_RIS_DIO6_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio6_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO6_A :: INT_EVENT1_RIS_DIO6_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO7` reader - DIO7 event"] pub type INT_EVENT1_RIS_DIO7_R = crate :: BitReader < INT_EVENT1_RIS_DIO7_A > ; # [doc = "DIO7 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO7_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO7_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO7_SET = 1 , } impl From < INT_EVENT1_RIS_DIO7_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO7_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO7_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO7_A { match self . bits { false => INT_EVENT1_RIS_DIO7_A :: INT_EVENT1_RIS_DIO7_CLR , true => INT_EVENT1_RIS_DIO7_A :: INT_EVENT1_RIS_DIO7_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio7_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO7_A :: INT_EVENT1_RIS_DIO7_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio7_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO7_A :: INT_EVENT1_RIS_DIO7_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO8` reader - DIO8 event"] pub type INT_EVENT1_RIS_DIO8_R = crate :: BitReader < INT_EVENT1_RIS_DIO8_A > ; # [doc = "DIO8 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO8_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO8_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO8_SET = 1 , } impl From < INT_EVENT1_RIS_DIO8_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO8_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO8_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO8_A { match self . bits { false => INT_EVENT1_RIS_DIO8_A :: INT_EVENT1_RIS_DIO8_CLR , true => INT_EVENT1_RIS_DIO8_A :: INT_EVENT1_RIS_DIO8_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio8_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO8_A :: INT_EVENT1_RIS_DIO8_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio8_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO8_A :: INT_EVENT1_RIS_DIO8_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO9` reader - DIO9 event"] pub type INT_EVENT1_RIS_DIO9_R = crate :: BitReader < INT_EVENT1_RIS_DIO9_A > ; # [doc = "DIO9 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO9_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO9_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO9_SET = 1 , } impl From < INT_EVENT1_RIS_DIO9_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO9_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO9_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO9_A { match self . bits { false => INT_EVENT1_RIS_DIO9_A :: INT_EVENT1_RIS_DIO9_CLR , true => INT_EVENT1_RIS_DIO9_A :: INT_EVENT1_RIS_DIO9_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio9_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO9_A :: INT_EVENT1_RIS_DIO9_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio9_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO9_A :: INT_EVENT1_RIS_DIO9_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO10` reader - DIO10 event"] pub type INT_EVENT1_RIS_DIO10_R = crate :: BitReader < INT_EVENT1_RIS_DIO10_A > ; # [doc = "DIO10 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO10_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO10_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO10_SET = 1 , } impl From < INT_EVENT1_RIS_DIO10_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO10_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO10_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO10_A { match self . bits { false => INT_EVENT1_RIS_DIO10_A :: INT_EVENT1_RIS_DIO10_CLR , true => INT_EVENT1_RIS_DIO10_A :: INT_EVENT1_RIS_DIO10_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio10_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO10_A :: INT_EVENT1_RIS_DIO10_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio10_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO10_A :: INT_EVENT1_RIS_DIO10_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO11` reader - DIO11 event"] pub type INT_EVENT1_RIS_DIO11_R = crate :: BitReader < INT_EVENT1_RIS_DIO11_A > ; # [doc = "DIO11 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO11_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO11_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO11_SET = 1 , } impl From < INT_EVENT1_RIS_DIO11_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO11_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO11_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO11_A { match self . bits { false => INT_EVENT1_RIS_DIO11_A :: INT_EVENT1_RIS_DIO11_CLR , true => INT_EVENT1_RIS_DIO11_A :: INT_EVENT1_RIS_DIO11_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio11_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO11_A :: INT_EVENT1_RIS_DIO11_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio11_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO11_A :: INT_EVENT1_RIS_DIO11_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO12` reader - DIO12 event"] pub type INT_EVENT1_RIS_DIO12_R = crate :: BitReader < INT_EVENT1_RIS_DIO12_A > ; # [doc = "DIO12 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO12_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO12_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO12_SET = 1 , } impl From < INT_EVENT1_RIS_DIO12_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO12_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO12_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO12_A { match self . bits { false => INT_EVENT1_RIS_DIO12_A :: INT_EVENT1_RIS_DIO12_CLR , true => INT_EVENT1_RIS_DIO12_A :: INT_EVENT1_RIS_DIO12_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio12_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO12_A :: INT_EVENT1_RIS_DIO12_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio12_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO12_A :: INT_EVENT1_RIS_DIO12_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO13` reader - DIO13 event"] pub type INT_EVENT1_RIS_DIO13_R = crate :: BitReader < INT_EVENT1_RIS_DIO13_A > ; # [doc = "DIO13 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO13_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO13_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO13_SET = 1 , } impl From < INT_EVENT1_RIS_DIO13_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO13_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO13_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO13_A { match self . bits { false => INT_EVENT1_RIS_DIO13_A :: INT_EVENT1_RIS_DIO13_CLR , true => INT_EVENT1_RIS_DIO13_A :: INT_EVENT1_RIS_DIO13_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio13_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO13_A :: INT_EVENT1_RIS_DIO13_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio13_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO13_A :: INT_EVENT1_RIS_DIO13_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO14` reader - DIO14 event"] pub type INT_EVENT1_RIS_DIO14_R = crate :: BitReader < INT_EVENT1_RIS_DIO14_A > ; # [doc = "DIO14 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO14_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO14_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO14_SET = 1 , } impl From < INT_EVENT1_RIS_DIO14_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO14_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO14_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO14_A { match self . bits { false => INT_EVENT1_RIS_DIO14_A :: INT_EVENT1_RIS_DIO14_CLR , true => INT_EVENT1_RIS_DIO14_A :: INT_EVENT1_RIS_DIO14_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio14_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO14_A :: INT_EVENT1_RIS_DIO14_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio14_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO14_A :: INT_EVENT1_RIS_DIO14_SET } } # [doc = "Field `INT_EVENT1_RIS_DIO15` reader - DIO15 event"] pub type INT_EVENT1_RIS_DIO15_R = crate :: BitReader < INT_EVENT1_RIS_DIO15_A > ; # [doc = "DIO15 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_RIS_DIO15_A { # [doc = "0: CLR"] INT_EVENT1_RIS_DIO15_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_RIS_DIO15_SET = 1 , } impl From < INT_EVENT1_RIS_DIO15_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_RIS_DIO15_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_RIS_DIO15_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_RIS_DIO15_A { match self . bits { false => INT_EVENT1_RIS_DIO15_A :: INT_EVENT1_RIS_DIO15_CLR , true => INT_EVENT1_RIS_DIO15_A :: INT_EVENT1_RIS_DIO15_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_ris_dio15_clr (& self) -> bool { * self == INT_EVENT1_RIS_DIO15_A :: INT_EVENT1_RIS_DIO15_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_ris_dio15_set (& self) -> bool { * self == INT_EVENT1_RIS_DIO15_A :: INT_EVENT1_RIS_DIO15_SET } } impl R { # [doc = "Bit 0 - DIO0 event"] # [inline (always)] pub fn int_event1_ris_dio0 (& self) -> INT_EVENT1_RIS_DIO0_R { INT_EVENT1_RIS_DIO0_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - DIO1 event"] # [inline (always)] pub fn int_event1_ris_dio1 (& self) -> INT_EVENT1_RIS_DIO1_R { INT_EVENT1_RIS_DIO1_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - DIO2 event"] # [inline (always)] pub fn int_event1_ris_dio2 (& self) -> INT_EVENT1_RIS_DIO2_R { INT_EVENT1_RIS_DIO2_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - DIO3 event"] # [inline (always)] pub fn int_event1_ris_dio3 (& self) -> INT_EVENT1_RIS_DIO3_R { INT_EVENT1_RIS_DIO3_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - DIO4 event"] # [inline (always)] pub fn int_event1_ris_dio4 (& self) -> INT_EVENT1_RIS_DIO4_R { INT_EVENT1_RIS_DIO4_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - DIO5 event"] # [inline (always)] pub fn int_event1_ris_dio5 (& self) -> INT_EVENT1_RIS_DIO5_R { INT_EVENT1_RIS_DIO5_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - DIO6 event"] # [inline (always)] pub fn int_event1_ris_dio6 (& self) -> INT_EVENT1_RIS_DIO6_R { INT_EVENT1_RIS_DIO6_R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 7 - DIO7 event"] # [inline (always)] pub fn int_event1_ris_dio7 (& self) -> INT_EVENT1_RIS_DIO7_R { INT_EVENT1_RIS_DIO7_R :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Bit 8 - DIO8 event"] # [inline (always)] pub fn int_event1_ris_dio8 (& self) -> INT_EVENT1_RIS_DIO8_R { INT_EVENT1_RIS_DIO8_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - DIO9 event"] # [inline (always)] pub fn int_event1_ris_dio9 (& self) -> INT_EVENT1_RIS_DIO9_R { INT_EVENT1_RIS_DIO9_R :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - DIO10 event"] # [inline (always)] pub fn int_event1_ris_dio10 (& self) -> INT_EVENT1_RIS_DIO10_R { INT_EVENT1_RIS_DIO10_R :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - DIO11 event"] # [inline (always)] pub fn int_event1_ris_dio11 (& self) -> INT_EVENT1_RIS_DIO11_R { INT_EVENT1_RIS_DIO11_R :: new (((self . bits >> 11) & 1) != 0) } # [doc = "Bit 12 - DIO12 event"] # [inline (always)] pub fn int_event1_ris_dio12 (& self) -> INT_EVENT1_RIS_DIO12_R { INT_EVENT1_RIS_DIO12_R :: new (((self . bits >> 12) & 1) != 0) } # [doc = "Bit 13 - DIO13 event"] # [inline (always)] pub fn int_event1_ris_dio13 (& self) -> INT_EVENT1_RIS_DIO13_R { INT_EVENT1_RIS_DIO13_R :: new (((self . bits >> 13) & 1) != 0) } # [doc = "Bit 14 - DIO14 event"] # [inline (always)] pub fn int_event1_ris_dio14 (& self) -> INT_EVENT1_RIS_DIO14_R { INT_EVENT1_RIS_DIO14_R :: new (((self . bits >> 14) & 1) != 0) } # [doc = "Bit 15 - DIO15 event"] # [inline (always)] pub fn int_event1_ris_dio15 (& self) -> INT_EVENT1_RIS_DIO15_R { INT_EVENT1_RIS_DIO15_R :: new (((self . bits >> 15) & 1) != 0) } } # [doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event1_ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT1_RIS_SPEC ; impl crate :: RegisterSpec for INT_EVENT1_RIS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event1_ris::R`](R) reader structure"] impl crate :: Readable for INT_EVENT1_RIS_SPEC { } # [doc = "`reset()` method sets INT_EVENT1_RIS to value 0"] impl crate :: Resettable for INT_EVENT1_RIS_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }