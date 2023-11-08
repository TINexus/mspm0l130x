# [doc = "Register `INT_EVENT0_IMASK` reader"] pub type R = crate :: R < INT_EVENT0_IMASK_SPEC > ; # [doc = "Register `INT_EVENT0_IMASK` writer"] pub type W = crate :: W < INT_EVENT0_IMASK_SPEC > ; # [doc = "Field `INT_EVENT0_IMASK_RTOUT` reader - Enable UARTOUT Receive Time-Out Interrupt."] pub type INT_EVENT0_IMASK_RTOUT_R = crate :: BitReader < INT_EVENT0_IMASK_RTOUT_A > ; # [doc = "Enable UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_RTOUT_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_RTOUT_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_RTOUT_SET = 1 , } impl From < INT_EVENT0_IMASK_RTOUT_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_RTOUT_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_RTOUT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_RTOUT_A { match self . bits { false => INT_EVENT0_IMASK_RTOUT_A :: INT_EVENT0_IMASK_RTOUT_CLR , true => INT_EVENT0_IMASK_RTOUT_A :: INT_EVENT0_IMASK_RTOUT_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_rtout_clr (& self) -> bool { * self == INT_EVENT0_IMASK_RTOUT_A :: INT_EVENT0_IMASK_RTOUT_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_rtout_set (& self) -> bool { * self == INT_EVENT0_IMASK_RTOUT_A :: INT_EVENT0_IMASK_RTOUT_SET } } # [doc = "Field `INT_EVENT0_IMASK_RTOUT` writer - Enable UARTOUT Receive Time-Out Interrupt."] pub type INT_EVENT0_IMASK_RTOUT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_RTOUT_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_RTOUT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_rtout_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_RTOUT_A :: INT_EVENT0_IMASK_RTOUT_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_rtout_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_RTOUT_A :: INT_EVENT0_IMASK_RTOUT_SET) } } # [doc = "Field `INT_EVENT0_IMASK_FRMERR` reader - Enable UART Framing Error Interrupt."] pub type INT_EVENT0_IMASK_FRMERR_R = crate :: BitReader < INT_EVENT0_IMASK_FRMERR_A > ; # [doc = "Enable UART Framing Error Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_FRMERR_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_FRMERR_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_FRMERR_SET = 1 , } impl From < INT_EVENT0_IMASK_FRMERR_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_FRMERR_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_FRMERR_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_FRMERR_A { match self . bits { false => INT_EVENT0_IMASK_FRMERR_A :: INT_EVENT0_IMASK_FRMERR_CLR , true => INT_EVENT0_IMASK_FRMERR_A :: INT_EVENT0_IMASK_FRMERR_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_frmerr_clr (& self) -> bool { * self == INT_EVENT0_IMASK_FRMERR_A :: INT_EVENT0_IMASK_FRMERR_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_frmerr_set (& self) -> bool { * self == INT_EVENT0_IMASK_FRMERR_A :: INT_EVENT0_IMASK_FRMERR_SET } } # [doc = "Field `INT_EVENT0_IMASK_FRMERR` writer - Enable UART Framing Error Interrupt."] pub type INT_EVENT0_IMASK_FRMERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_FRMERR_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_FRMERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_frmerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_FRMERR_A :: INT_EVENT0_IMASK_FRMERR_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_frmerr_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_FRMERR_A :: INT_EVENT0_IMASK_FRMERR_SET) } } # [doc = "Field `INT_EVENT0_IMASK_PARERR` reader - Enable UART Parity Error Interrupt."] pub type INT_EVENT0_IMASK_PARERR_R = crate :: BitReader < INT_EVENT0_IMASK_PARERR_A > ; # [doc = "Enable UART Parity Error Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_PARERR_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_PARERR_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_PARERR_SET = 1 , } impl From < INT_EVENT0_IMASK_PARERR_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_PARERR_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_PARERR_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_PARERR_A { match self . bits { false => INT_EVENT0_IMASK_PARERR_A :: INT_EVENT0_IMASK_PARERR_CLR , true => INT_EVENT0_IMASK_PARERR_A :: INT_EVENT0_IMASK_PARERR_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_parerr_clr (& self) -> bool { * self == INT_EVENT0_IMASK_PARERR_A :: INT_EVENT0_IMASK_PARERR_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_parerr_set (& self) -> bool { * self == INT_EVENT0_IMASK_PARERR_A :: INT_EVENT0_IMASK_PARERR_SET } } # [doc = "Field `INT_EVENT0_IMASK_PARERR` writer - Enable UART Parity Error Interrupt."] pub type INT_EVENT0_IMASK_PARERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_PARERR_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_PARERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_parerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_PARERR_A :: INT_EVENT0_IMASK_PARERR_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_parerr_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_PARERR_A :: INT_EVENT0_IMASK_PARERR_SET) } } # [doc = "Field `INT_EVENT0_IMASK_BRKERR` reader - Enable UART Break Error Interrupt."] pub type INT_EVENT0_IMASK_BRKERR_R = crate :: BitReader < INT_EVENT0_IMASK_BRKERR_A > ; # [doc = "Enable UART Break Error Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_BRKERR_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_BRKERR_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_BRKERR_SET = 1 , } impl From < INT_EVENT0_IMASK_BRKERR_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_BRKERR_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_BRKERR_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_BRKERR_A { match self . bits { false => INT_EVENT0_IMASK_BRKERR_A :: INT_EVENT0_IMASK_BRKERR_CLR , true => INT_EVENT0_IMASK_BRKERR_A :: INT_EVENT0_IMASK_BRKERR_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_brkerr_clr (& self) -> bool { * self == INT_EVENT0_IMASK_BRKERR_A :: INT_EVENT0_IMASK_BRKERR_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_brkerr_set (& self) -> bool { * self == INT_EVENT0_IMASK_BRKERR_A :: INT_EVENT0_IMASK_BRKERR_SET } } # [doc = "Field `INT_EVENT0_IMASK_BRKERR` writer - Enable UART Break Error Interrupt."] pub type INT_EVENT0_IMASK_BRKERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_BRKERR_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_BRKERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_brkerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_BRKERR_A :: INT_EVENT0_IMASK_BRKERR_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_brkerr_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_BRKERR_A :: INT_EVENT0_IMASK_BRKERR_SET) } } # [doc = "Field `INT_EVENT0_IMASK_OVRERR` reader - Enable UART Receive Overrun Error Interrupt."] pub type INT_EVENT0_IMASK_OVRERR_R = crate :: BitReader < INT_EVENT0_IMASK_OVRERR_A > ; # [doc = "Enable UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_OVRERR_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_OVRERR_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_OVRERR_SET = 1 , } impl From < INT_EVENT0_IMASK_OVRERR_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_OVRERR_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_OVRERR_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_OVRERR_A { match self . bits { false => INT_EVENT0_IMASK_OVRERR_A :: INT_EVENT0_IMASK_OVRERR_CLR , true => INT_EVENT0_IMASK_OVRERR_A :: INT_EVENT0_IMASK_OVRERR_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_ovrerr_clr (& self) -> bool { * self == INT_EVENT0_IMASK_OVRERR_A :: INT_EVENT0_IMASK_OVRERR_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_ovrerr_set (& self) -> bool { * self == INT_EVENT0_IMASK_OVRERR_A :: INT_EVENT0_IMASK_OVRERR_SET } } # [doc = "Field `INT_EVENT0_IMASK_OVRERR` writer - Enable UART Receive Overrun Error Interrupt."] pub type INT_EVENT0_IMASK_OVRERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_OVRERR_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_OVRERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_ovrerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_OVRERR_A :: INT_EVENT0_IMASK_OVRERR_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_ovrerr_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_OVRERR_A :: INT_EVENT0_IMASK_OVRERR_SET) } } # [doc = "Field `INT_EVENT0_IMASK_RXNE` reader - Enable Negative Edge on UARTxRXD Interrupt."] pub type INT_EVENT0_IMASK_RXNE_R = crate :: BitReader < INT_EVENT0_IMASK_RXNE_A > ; # [doc = "Enable Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_RXNE_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_RXNE_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_RXNE_SET = 1 , } impl From < INT_EVENT0_IMASK_RXNE_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_RXNE_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_RXNE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_RXNE_A { match self . bits { false => INT_EVENT0_IMASK_RXNE_A :: INT_EVENT0_IMASK_RXNE_CLR , true => INT_EVENT0_IMASK_RXNE_A :: INT_EVENT0_IMASK_RXNE_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_rxne_clr (& self) -> bool { * self == INT_EVENT0_IMASK_RXNE_A :: INT_EVENT0_IMASK_RXNE_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_rxne_set (& self) -> bool { * self == INT_EVENT0_IMASK_RXNE_A :: INT_EVENT0_IMASK_RXNE_SET } } # [doc = "Field `INT_EVENT0_IMASK_RXNE` writer - Enable Negative Edge on UARTxRXD Interrupt."] pub type INT_EVENT0_IMASK_RXNE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_RXNE_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_RXNE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_rxne_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_RXNE_A :: INT_EVENT0_IMASK_RXNE_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_rxne_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_RXNE_A :: INT_EVENT0_IMASK_RXNE_SET) } } # [doc = "Field `INT_EVENT0_IMASK_RXPE` reader - Enable Positive Edge on UARTxRXD Interrupt."] pub type INT_EVENT0_IMASK_RXPE_R = crate :: BitReader < INT_EVENT0_IMASK_RXPE_A > ; # [doc = "Enable Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_RXPE_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_RXPE_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_RXPE_SET = 1 , } impl From < INT_EVENT0_IMASK_RXPE_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_RXPE_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_RXPE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_RXPE_A { match self . bits { false => INT_EVENT0_IMASK_RXPE_A :: INT_EVENT0_IMASK_RXPE_CLR , true => INT_EVENT0_IMASK_RXPE_A :: INT_EVENT0_IMASK_RXPE_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_rxpe_clr (& self) -> bool { * self == INT_EVENT0_IMASK_RXPE_A :: INT_EVENT0_IMASK_RXPE_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_rxpe_set (& self) -> bool { * self == INT_EVENT0_IMASK_RXPE_A :: INT_EVENT0_IMASK_RXPE_SET } } # [doc = "Field `INT_EVENT0_IMASK_RXPE` writer - Enable Positive Edge on UARTxRXD Interrupt."] pub type INT_EVENT0_IMASK_RXPE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_RXPE_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_RXPE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_rxpe_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_RXPE_A :: INT_EVENT0_IMASK_RXPE_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_rxpe_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_RXPE_A :: INT_EVENT0_IMASK_RXPE_SET) } } # [doc = "Field `INT_EVENT0_IMASK_RXINT` reader - Enable UART Receive Interrupt."] pub type INT_EVENT0_IMASK_RXINT_R = crate :: BitReader < INT_EVENT0_IMASK_RXINT_A > ; # [doc = "Enable UART Receive Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_RXINT_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_RXINT_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_RXINT_SET = 1 , } impl From < INT_EVENT0_IMASK_RXINT_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_RXINT_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_RXINT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_RXINT_A { match self . bits { false => INT_EVENT0_IMASK_RXINT_A :: INT_EVENT0_IMASK_RXINT_CLR , true => INT_EVENT0_IMASK_RXINT_A :: INT_EVENT0_IMASK_RXINT_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_rxint_clr (& self) -> bool { * self == INT_EVENT0_IMASK_RXINT_A :: INT_EVENT0_IMASK_RXINT_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_rxint_set (& self) -> bool { * self == INT_EVENT0_IMASK_RXINT_A :: INT_EVENT0_IMASK_RXINT_SET } } # [doc = "Field `INT_EVENT0_IMASK_RXINT` writer - Enable UART Receive Interrupt."] pub type INT_EVENT0_IMASK_RXINT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_RXINT_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_RXINT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_rxint_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_RXINT_A :: INT_EVENT0_IMASK_RXINT_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_rxint_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_RXINT_A :: INT_EVENT0_IMASK_RXINT_SET) } } # [doc = "Field `INT_EVENT0_IMASK_TXINT` reader - Enable UART Transmit Interrupt."] pub type INT_EVENT0_IMASK_TXINT_R = crate :: BitReader < INT_EVENT0_IMASK_TXINT_A > ; # [doc = "Enable UART Transmit Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_TXINT_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_TXINT_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_TXINT_SET = 1 , } impl From < INT_EVENT0_IMASK_TXINT_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_TXINT_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_TXINT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_TXINT_A { match self . bits { false => INT_EVENT0_IMASK_TXINT_A :: INT_EVENT0_IMASK_TXINT_CLR , true => INT_EVENT0_IMASK_TXINT_A :: INT_EVENT0_IMASK_TXINT_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_txint_clr (& self) -> bool { * self == INT_EVENT0_IMASK_TXINT_A :: INT_EVENT0_IMASK_TXINT_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_txint_set (& self) -> bool { * self == INT_EVENT0_IMASK_TXINT_A :: INT_EVENT0_IMASK_TXINT_SET } } # [doc = "Field `INT_EVENT0_IMASK_TXINT` writer - Enable UART Transmit Interrupt."] pub type INT_EVENT0_IMASK_TXINT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_TXINT_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_TXINT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_txint_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_TXINT_A :: INT_EVENT0_IMASK_TXINT_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_txint_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_TXINT_A :: INT_EVENT0_IMASK_TXINT_SET) } } # [doc = "Field `INT_EVENT0_IMASK_EOT` reader - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."] pub type INT_EVENT0_IMASK_EOT_R = crate :: BitReader < INT_EVENT0_IMASK_EOT_A > ; # [doc = "Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_EOT_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_EOT_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_EOT_SET = 1 , } impl From < INT_EVENT0_IMASK_EOT_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_EOT_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_EOT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_EOT_A { match self . bits { false => INT_EVENT0_IMASK_EOT_A :: INT_EVENT0_IMASK_EOT_CLR , true => INT_EVENT0_IMASK_EOT_A :: INT_EVENT0_IMASK_EOT_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_eot_clr (& self) -> bool { * self == INT_EVENT0_IMASK_EOT_A :: INT_EVENT0_IMASK_EOT_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_eot_set (& self) -> bool { * self == INT_EVENT0_IMASK_EOT_A :: INT_EVENT0_IMASK_EOT_SET } } # [doc = "Field `INT_EVENT0_IMASK_EOT` writer - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."] pub type INT_EVENT0_IMASK_EOT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_EOT_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_EOT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_eot_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_EOT_A :: INT_EVENT0_IMASK_EOT_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_eot_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_EOT_A :: INT_EVENT0_IMASK_EOT_SET) } } # [doc = "Field `INT_EVENT0_IMASK_ADDR_MATCH` reader - Enable Address Match Interrupt."] pub type INT_EVENT0_IMASK_ADDR_MATCH_R = crate :: BitReader < INT_EVENT0_IMASK_ADDR_MATCH_A > ; # [doc = "Enable Address Match Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_ADDR_MATCH_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_ADDR_MATCH_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_ADDR_MATCH_SET = 1 , } impl From < INT_EVENT0_IMASK_ADDR_MATCH_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_ADDR_MATCH_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_ADDR_MATCH_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_ADDR_MATCH_A { match self . bits { false => INT_EVENT0_IMASK_ADDR_MATCH_A :: INT_EVENT0_IMASK_ADDR_MATCH_CLR , true => INT_EVENT0_IMASK_ADDR_MATCH_A :: INT_EVENT0_IMASK_ADDR_MATCH_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_addr_match_clr (& self) -> bool { * self == INT_EVENT0_IMASK_ADDR_MATCH_A :: INT_EVENT0_IMASK_ADDR_MATCH_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_addr_match_set (& self) -> bool { * self == INT_EVENT0_IMASK_ADDR_MATCH_A :: INT_EVENT0_IMASK_ADDR_MATCH_SET } } # [doc = "Field `INT_EVENT0_IMASK_ADDR_MATCH` writer - Enable Address Match Interrupt."] pub type INT_EVENT0_IMASK_ADDR_MATCH_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_ADDR_MATCH_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_ADDR_MATCH_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_addr_match_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_ADDR_MATCH_A :: INT_EVENT0_IMASK_ADDR_MATCH_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_addr_match_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_ADDR_MATCH_A :: INT_EVENT0_IMASK_ADDR_MATCH_SET) } } # [doc = "Field `INT_EVENT0_IMASK_CTS` reader - Enable UART Clear to Send Modem Interrupt. 0 = Interrupt disabled"] pub type INT_EVENT0_IMASK_CTS_R = crate :: BitReader < INT_EVENT0_IMASK_CTS_A > ; # [doc = "Enable UART Clear to Send Modem Interrupt. 0 = Interrupt disabled\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_CTS_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_CTS_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_CTS_SET = 1 , } impl From < INT_EVENT0_IMASK_CTS_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_CTS_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_CTS_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_CTS_A { match self . bits { false => INT_EVENT0_IMASK_CTS_A :: INT_EVENT0_IMASK_CTS_CLR , true => INT_EVENT0_IMASK_CTS_A :: INT_EVENT0_IMASK_CTS_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_cts_clr (& self) -> bool { * self == INT_EVENT0_IMASK_CTS_A :: INT_EVENT0_IMASK_CTS_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_cts_set (& self) -> bool { * self == INT_EVENT0_IMASK_CTS_A :: INT_EVENT0_IMASK_CTS_SET } } # [doc = "Field `INT_EVENT0_IMASK_CTS` writer - Enable UART Clear to Send Modem Interrupt. 0 = Interrupt disabled"] pub type INT_EVENT0_IMASK_CTS_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_CTS_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_CTS_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_cts_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_CTS_A :: INT_EVENT0_IMASK_CTS_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_cts_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_CTS_A :: INT_EVENT0_IMASK_CTS_SET) } } # [doc = "Field `INT_EVENT0_IMASK_DMA_DONE_RX` reader - Enable DMA Done on RX Event Channel"] pub type INT_EVENT0_IMASK_DMA_DONE_RX_R = crate :: BitReader < INT_EVENT0_IMASK_DMA_DONE_RX_A > ; # [doc = "Enable DMA Done on RX Event Channel\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_DMA_DONE_RX_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_DMA_DONE_RX_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_DMA_DONE_RX_SET = 1 , } impl From < INT_EVENT0_IMASK_DMA_DONE_RX_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_DMA_DONE_RX_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_DMA_DONE_RX_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_DMA_DONE_RX_A { match self . bits { false => INT_EVENT0_IMASK_DMA_DONE_RX_A :: INT_EVENT0_IMASK_DMA_DONE_RX_CLR , true => INT_EVENT0_IMASK_DMA_DONE_RX_A :: INT_EVENT0_IMASK_DMA_DONE_RX_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_dma_done_rx_clr (& self) -> bool { * self == INT_EVENT0_IMASK_DMA_DONE_RX_A :: INT_EVENT0_IMASK_DMA_DONE_RX_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_dma_done_rx_set (& self) -> bool { * self == INT_EVENT0_IMASK_DMA_DONE_RX_A :: INT_EVENT0_IMASK_DMA_DONE_RX_SET } } # [doc = "Field `INT_EVENT0_IMASK_DMA_DONE_RX` writer - Enable DMA Done on RX Event Channel"] pub type INT_EVENT0_IMASK_DMA_DONE_RX_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_DMA_DONE_RX_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_DMA_DONE_RX_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_dma_done_rx_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_DMA_DONE_RX_A :: INT_EVENT0_IMASK_DMA_DONE_RX_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_dma_done_rx_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_DMA_DONE_RX_A :: INT_EVENT0_IMASK_DMA_DONE_RX_SET) } } # [doc = "Field `INT_EVENT0_IMASK_DMA_DONE_TX` reader - Enable DMA Done on TX Event Channel"] pub type INT_EVENT0_IMASK_DMA_DONE_TX_R = crate :: BitReader < INT_EVENT0_IMASK_DMA_DONE_TX_A > ; # [doc = "Enable DMA Done on TX Event Channel\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_DMA_DONE_TX_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_DMA_DONE_TX_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_DMA_DONE_TX_SET = 1 , } impl From < INT_EVENT0_IMASK_DMA_DONE_TX_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_DMA_DONE_TX_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_DMA_DONE_TX_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_DMA_DONE_TX_A { match self . bits { false => INT_EVENT0_IMASK_DMA_DONE_TX_A :: INT_EVENT0_IMASK_DMA_DONE_TX_CLR , true => INT_EVENT0_IMASK_DMA_DONE_TX_A :: INT_EVENT0_IMASK_DMA_DONE_TX_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_dma_done_tx_clr (& self) -> bool { * self == INT_EVENT0_IMASK_DMA_DONE_TX_A :: INT_EVENT0_IMASK_DMA_DONE_TX_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_dma_done_tx_set (& self) -> bool { * self == INT_EVENT0_IMASK_DMA_DONE_TX_A :: INT_EVENT0_IMASK_DMA_DONE_TX_SET } } # [doc = "Field `INT_EVENT0_IMASK_DMA_DONE_TX` writer - Enable DMA Done on TX Event Channel"] pub type INT_EVENT0_IMASK_DMA_DONE_TX_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_DMA_DONE_TX_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_DMA_DONE_TX_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_dma_done_tx_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_DMA_DONE_TX_A :: INT_EVENT0_IMASK_DMA_DONE_TX_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_dma_done_tx_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_DMA_DONE_TX_A :: INT_EVENT0_IMASK_DMA_DONE_TX_SET) } } # [doc = "Field `INT_EVENT0_IMASK_NERR` reader - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"] pub type INT_EVENT0_IMASK_NERR_R = crate :: BitReader < INT_EVENT0_IMASK_NERR_A > ; # [doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_NERR_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_NERR_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_NERR_SET = 1 , } impl From < INT_EVENT0_IMASK_NERR_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_NERR_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_NERR_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_NERR_A { match self . bits { false => INT_EVENT0_IMASK_NERR_A :: INT_EVENT0_IMASK_NERR_CLR , true => INT_EVENT0_IMASK_NERR_A :: INT_EVENT0_IMASK_NERR_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_nerr_clr (& self) -> bool { * self == INT_EVENT0_IMASK_NERR_A :: INT_EVENT0_IMASK_NERR_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_nerr_set (& self) -> bool { * self == INT_EVENT0_IMASK_NERR_A :: INT_EVENT0_IMASK_NERR_SET } } # [doc = "Field `INT_EVENT0_IMASK_NERR` writer - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"] pub type INT_EVENT0_IMASK_NERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_NERR_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_NERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_nerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_NERR_A :: INT_EVENT0_IMASK_NERR_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_nerr_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_NERR_A :: INT_EVENT0_IMASK_NERR_SET) } } impl R { # [doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."] # [inline (always)] pub fn int_event0_imask_rtout (& self) -> INT_EVENT0_IMASK_RTOUT_R { INT_EVENT0_IMASK_RTOUT_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Enable UART Framing Error Interrupt."] # [inline (always)] pub fn int_event0_imask_frmerr (& self) -> INT_EVENT0_IMASK_FRMERR_R { INT_EVENT0_IMASK_FRMERR_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Enable UART Parity Error Interrupt."] # [inline (always)] pub fn int_event0_imask_parerr (& self) -> INT_EVENT0_IMASK_PARERR_R { INT_EVENT0_IMASK_PARERR_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Enable UART Break Error Interrupt."] # [inline (always)] pub fn int_event0_imask_brkerr (& self) -> INT_EVENT0_IMASK_BRKERR_R { INT_EVENT0_IMASK_BRKERR_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - Enable UART Receive Overrun Error Interrupt."] # [inline (always)] pub fn int_event0_imask_ovrerr (& self) -> INT_EVENT0_IMASK_OVRERR_R { INT_EVENT0_IMASK_OVRERR_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - Enable Negative Edge on UARTxRXD Interrupt."] # [inline (always)] pub fn int_event0_imask_rxne (& self) -> INT_EVENT0_IMASK_RXNE_R { INT_EVENT0_IMASK_RXNE_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - Enable Positive Edge on UARTxRXD Interrupt."] # [inline (always)] pub fn int_event0_imask_rxpe (& self) -> INT_EVENT0_IMASK_RXPE_R { INT_EVENT0_IMASK_RXPE_R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 10 - Enable UART Receive Interrupt."] # [inline (always)] pub fn int_event0_imask_rxint (& self) -> INT_EVENT0_IMASK_RXINT_R { INT_EVENT0_IMASK_RXINT_R :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - Enable UART Transmit Interrupt."] # [inline (always)] pub fn int_event0_imask_txint (& self) -> INT_EVENT0_IMASK_TXINT_R { INT_EVENT0_IMASK_TXINT_R :: new (((self . bits >> 11) & 1) != 0) } # [doc = "Bit 12 - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."] # [inline (always)] pub fn int_event0_imask_eot (& self) -> INT_EVENT0_IMASK_EOT_R { INT_EVENT0_IMASK_EOT_R :: new (((self . bits >> 12) & 1) != 0) } # [doc = "Bit 13 - Enable Address Match Interrupt."] # [inline (always)] pub fn int_event0_imask_addr_match (& self) -> INT_EVENT0_IMASK_ADDR_MATCH_R { INT_EVENT0_IMASK_ADDR_MATCH_R :: new (((self . bits >> 13) & 1) != 0) } # [doc = "Bit 14 - Enable UART Clear to Send Modem Interrupt. 0 = Interrupt disabled"] # [inline (always)] pub fn int_event0_imask_cts (& self) -> INT_EVENT0_IMASK_CTS_R { INT_EVENT0_IMASK_CTS_R :: new (((self . bits >> 14) & 1) != 0) } # [doc = "Bit 15 - Enable DMA Done on RX Event Channel"] # [inline (always)] pub fn int_event0_imask_dma_done_rx (& self) -> INT_EVENT0_IMASK_DMA_DONE_RX_R { INT_EVENT0_IMASK_DMA_DONE_RX_R :: new (((self . bits >> 15) & 1) != 0) } # [doc = "Bit 16 - Enable DMA Done on TX Event Channel"] # [inline (always)] pub fn int_event0_imask_dma_done_tx (& self) -> INT_EVENT0_IMASK_DMA_DONE_TX_R { INT_EVENT0_IMASK_DMA_DONE_TX_R :: new (((self . bits >> 16) & 1) != 0) } # [doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"] # [inline (always)] pub fn int_event0_imask_nerr (& self) -> INT_EVENT0_IMASK_NERR_R { INT_EVENT0_IMASK_NERR_R :: new (((self . bits >> 17) & 1) != 0) } } impl W { # [doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_rtout (& mut self) -> INT_EVENT0_IMASK_RTOUT_W < INT_EVENT0_IMASK_SPEC , 0 > { INT_EVENT0_IMASK_RTOUT_W :: new (self) } # [doc = "Bit 1 - Enable UART Framing Error Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_frmerr (& mut self) -> INT_EVENT0_IMASK_FRMERR_W < INT_EVENT0_IMASK_SPEC , 1 > { INT_EVENT0_IMASK_FRMERR_W :: new (self) } # [doc = "Bit 2 - Enable UART Parity Error Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_parerr (& mut self) -> INT_EVENT0_IMASK_PARERR_W < INT_EVENT0_IMASK_SPEC , 2 > { INT_EVENT0_IMASK_PARERR_W :: new (self) } # [doc = "Bit 3 - Enable UART Break Error Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_brkerr (& mut self) -> INT_EVENT0_IMASK_BRKERR_W < INT_EVENT0_IMASK_SPEC , 3 > { INT_EVENT0_IMASK_BRKERR_W :: new (self) } # [doc = "Bit 4 - Enable UART Receive Overrun Error Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_ovrerr (& mut self) -> INT_EVENT0_IMASK_OVRERR_W < INT_EVENT0_IMASK_SPEC , 4 > { INT_EVENT0_IMASK_OVRERR_W :: new (self) } # [doc = "Bit 5 - Enable Negative Edge on UARTxRXD Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_rxne (& mut self) -> INT_EVENT0_IMASK_RXNE_W < INT_EVENT0_IMASK_SPEC , 5 > { INT_EVENT0_IMASK_RXNE_W :: new (self) } # [doc = "Bit 6 - Enable Positive Edge on UARTxRXD Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_rxpe (& mut self) -> INT_EVENT0_IMASK_RXPE_W < INT_EVENT0_IMASK_SPEC , 6 > { INT_EVENT0_IMASK_RXPE_W :: new (self) } # [doc = "Bit 10 - Enable UART Receive Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_rxint (& mut self) -> INT_EVENT0_IMASK_RXINT_W < INT_EVENT0_IMASK_SPEC , 10 > { INT_EVENT0_IMASK_RXINT_W :: new (self) } # [doc = "Bit 11 - Enable UART Transmit Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_txint (& mut self) -> INT_EVENT0_IMASK_TXINT_W < INT_EVENT0_IMASK_SPEC , 11 > { INT_EVENT0_IMASK_TXINT_W :: new (self) } # [doc = "Bit 12 - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."] # [inline (always)] # [must_use] pub fn int_event0_imask_eot (& mut self) -> INT_EVENT0_IMASK_EOT_W < INT_EVENT0_IMASK_SPEC , 12 > { INT_EVENT0_IMASK_EOT_W :: new (self) } # [doc = "Bit 13 - Enable Address Match Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_imask_addr_match (& mut self) -> INT_EVENT0_IMASK_ADDR_MATCH_W < INT_EVENT0_IMASK_SPEC , 13 > { INT_EVENT0_IMASK_ADDR_MATCH_W :: new (self) } # [doc = "Bit 14 - Enable UART Clear to Send Modem Interrupt. 0 = Interrupt disabled"] # [inline (always)] # [must_use] pub fn int_event0_imask_cts (& mut self) -> INT_EVENT0_IMASK_CTS_W < INT_EVENT0_IMASK_SPEC , 14 > { INT_EVENT0_IMASK_CTS_W :: new (self) } # [doc = "Bit 15 - Enable DMA Done on RX Event Channel"] # [inline (always)] # [must_use] pub fn int_event0_imask_dma_done_rx (& mut self) -> INT_EVENT0_IMASK_DMA_DONE_RX_W < INT_EVENT0_IMASK_SPEC , 15 > { INT_EVENT0_IMASK_DMA_DONE_RX_W :: new (self) } # [doc = "Bit 16 - Enable DMA Done on TX Event Channel"] # [inline (always)] # [must_use] pub fn int_event0_imask_dma_done_tx (& mut self) -> INT_EVENT0_IMASK_DMA_DONE_TX_W < INT_EVENT0_IMASK_SPEC , 16 > { INT_EVENT0_IMASK_DMA_DONE_TX_W :: new (self) } # [doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"] # [inline (always)] # [must_use] pub fn int_event0_imask_nerr (& mut self) -> INT_EVENT0_IMASK_NERR_W < INT_EVENT0_IMASK_SPEC , 17 > { INT_EVENT0_IMASK_NERR_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event0_imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event0_imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT0_IMASK_SPEC ; impl crate :: RegisterSpec for INT_EVENT0_IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event0_imask::R`](R) reader structure"] impl crate :: Readable for INT_EVENT0_IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`int_event0_imask::W`](W) writer structure"] impl crate :: Writable for INT_EVENT0_IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT0_IMASK to value 0"] impl crate :: Resettable for INT_EVENT0_IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }