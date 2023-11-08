# [doc = "Register `MFIFOCTL` reader"] pub type R = crate :: R < MFIFOCTL_SPEC > ; # [doc = "Register `MFIFOCTL` writer"] pub type W = crate :: W < MFIFOCTL_SPEC > ; # [doc = "Field `MFIFOCTL_TXTRIG` reader - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."] pub type MFIFOCTL_TXTRIG_R = crate :: FieldReader < MFIFOCTL_TXTRIG_A > ; # [doc = "TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum MFIFOCTL_TXTRIG_A { # [doc = "4: LEVEL_4"] MFIFOCTL_TXTRIG_LEVEL_4 = 4 , # [doc = "5: LEVEL_5"] MFIFOCTL_TXTRIG_LEVEL_5 = 5 , # [doc = "6: LEVEL_6"] MFIFOCTL_TXTRIG_LEVEL_6 = 6 , # [doc = "7: LEVEL_7"] MFIFOCTL_TXTRIG_LEVEL_7 = 7 , } impl From < MFIFOCTL_TXTRIG_A > for u8 { # [inline (always)] fn from (variant : MFIFOCTL_TXTRIG_A) -> Self { variant as _ } } impl crate :: FieldSpec for MFIFOCTL_TXTRIG_A { type Ux = u8 ; } impl MFIFOCTL_TXTRIG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < MFIFOCTL_TXTRIG_A > { match self . bits { 4 => Some (MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_4) , 5 => Some (MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_5) , 6 => Some (MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_6) , 7 => Some (MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_7) , _ => None , } } # [doc = "LEVEL_4"] # [inline (always)] pub fn is_mfifoctl_txtrig_level_4 (& self) -> bool { * self == MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_4 } # [doc = "LEVEL_5"] # [inline (always)] pub fn is_mfifoctl_txtrig_level_5 (& self) -> bool { * self == MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_5 } # [doc = "LEVEL_6"] # [inline (always)] pub fn is_mfifoctl_txtrig_level_6 (& self) -> bool { * self == MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_6 } # [doc = "LEVEL_7"] # [inline (always)] pub fn is_mfifoctl_txtrig_level_7 (& self) -> bool { * self == MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_7 } } # [doc = "Field `MFIFOCTL_TXTRIG` writer - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."] pub type MFIFOCTL_TXTRIG_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 3 , O , MFIFOCTL_TXTRIG_A > ; impl < 'a , REG , const O : u8 > MFIFOCTL_TXTRIG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "LEVEL_4"] # [inline (always)] pub fn mfifoctl_txtrig_level_4 (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_4) } # [doc = "LEVEL_5"] # [inline (always)] pub fn mfifoctl_txtrig_level_5 (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_5) } # [doc = "LEVEL_6"] # [inline (always)] pub fn mfifoctl_txtrig_level_6 (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_6) } # [doc = "LEVEL_7"] # [inline (always)] pub fn mfifoctl_txtrig_level_7 (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_TXTRIG_A :: MFIFOCTL_TXTRIG_LEVEL_7) } } # [doc = "Field `MFIFOCTL_TXFLUSH` reader - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."] pub type MFIFOCTL_TXFLUSH_R = crate :: BitReader < MFIFOCTL_TXFLUSH_A > ; # [doc = "TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum MFIFOCTL_TXFLUSH_A { # [doc = "0: NOFLUSH"] MFIFOCTL_TXFLUSH_NOFLUSH = 0 , # [doc = "1: FLUSH"] MFIFOCTL_TXFLUSH_FLUSH = 1 , } impl From < MFIFOCTL_TXFLUSH_A > for bool { # [inline (always)] fn from (variant : MFIFOCTL_TXFLUSH_A) -> Self { variant as u8 != 0 } } impl MFIFOCTL_TXFLUSH_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> MFIFOCTL_TXFLUSH_A { match self . bits { false => MFIFOCTL_TXFLUSH_A :: MFIFOCTL_TXFLUSH_NOFLUSH , true => MFIFOCTL_TXFLUSH_A :: MFIFOCTL_TXFLUSH_FLUSH , } } # [doc = "NOFLUSH"] # [inline (always)] pub fn is_mfifoctl_txflush_noflush (& self) -> bool { * self == MFIFOCTL_TXFLUSH_A :: MFIFOCTL_TXFLUSH_NOFLUSH } # [doc = "FLUSH"] # [inline (always)] pub fn is_mfifoctl_txflush_flush (& self) -> bool { * self == MFIFOCTL_TXFLUSH_A :: MFIFOCTL_TXFLUSH_FLUSH } } # [doc = "Field `MFIFOCTL_TXFLUSH` writer - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."] pub type MFIFOCTL_TXFLUSH_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , MFIFOCTL_TXFLUSH_A > ; impl < 'a , REG , const O : u8 > MFIFOCTL_TXFLUSH_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NOFLUSH"] # [inline (always)] pub fn mfifoctl_txflush_noflush (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_TXFLUSH_A :: MFIFOCTL_TXFLUSH_NOFLUSH) } # [doc = "FLUSH"] # [inline (always)] pub fn mfifoctl_txflush_flush (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_TXFLUSH_A :: MFIFOCTL_TXFLUSH_FLUSH) } } # [doc = "Field `MFIFOCTL_RXTRIG` reader - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."] pub type MFIFOCTL_RXTRIG_R = crate :: FieldReader < MFIFOCTL_RXTRIG_A > ; # [doc = "RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum MFIFOCTL_RXTRIG_A { # [doc = "4: LEVEL_5"] MFIFOCTL_RXTRIG_LEVEL_5 = 4 , # [doc = "5: LEVEL_6"] MFIFOCTL_RXTRIG_LEVEL_6 = 5 , # [doc = "6: LEVEL_7"] MFIFOCTL_RXTRIG_LEVEL_7 = 6 , # [doc = "7: LEVEL_8"] MFIFOCTL_RXTRIG_LEVEL_8 = 7 , } impl From < MFIFOCTL_RXTRIG_A > for u8 { # [inline (always)] fn from (variant : MFIFOCTL_RXTRIG_A) -> Self { variant as _ } } impl crate :: FieldSpec for MFIFOCTL_RXTRIG_A { type Ux = u8 ; } impl MFIFOCTL_RXTRIG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < MFIFOCTL_RXTRIG_A > { match self . bits { 4 => Some (MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_5) , 5 => Some (MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_6) , 6 => Some (MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_7) , 7 => Some (MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_8) , _ => None , } } # [doc = "LEVEL_5"] # [inline (always)] pub fn is_mfifoctl_rxtrig_level_5 (& self) -> bool { * self == MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_5 } # [doc = "LEVEL_6"] # [inline (always)] pub fn is_mfifoctl_rxtrig_level_6 (& self) -> bool { * self == MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_6 } # [doc = "LEVEL_7"] # [inline (always)] pub fn is_mfifoctl_rxtrig_level_7 (& self) -> bool { * self == MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_7 } # [doc = "LEVEL_8"] # [inline (always)] pub fn is_mfifoctl_rxtrig_level_8 (& self) -> bool { * self == MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_8 } } # [doc = "Field `MFIFOCTL_RXTRIG` writer - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."] pub type MFIFOCTL_RXTRIG_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 3 , O , MFIFOCTL_RXTRIG_A > ; impl < 'a , REG , const O : u8 > MFIFOCTL_RXTRIG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "LEVEL_5"] # [inline (always)] pub fn mfifoctl_rxtrig_level_5 (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_5) } # [doc = "LEVEL_6"] # [inline (always)] pub fn mfifoctl_rxtrig_level_6 (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_6) } # [doc = "LEVEL_7"] # [inline (always)] pub fn mfifoctl_rxtrig_level_7 (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_7) } # [doc = "LEVEL_8"] # [inline (always)] pub fn mfifoctl_rxtrig_level_8 (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_RXTRIG_A :: MFIFOCTL_RXTRIG_LEVEL_8) } } # [doc = "Field `MFIFOCTL_RXFLUSH` reader - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."] pub type MFIFOCTL_RXFLUSH_R = crate :: BitReader < MFIFOCTL_RXFLUSH_A > ; # [doc = "RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum MFIFOCTL_RXFLUSH_A { # [doc = "0: NOFLUSH"] MFIFOCTL_RXFLUSH_NOFLUSH = 0 , # [doc = "1: FLUSH"] MFIFOCTL_RXFLUSH_FLUSH = 1 , } impl From < MFIFOCTL_RXFLUSH_A > for bool { # [inline (always)] fn from (variant : MFIFOCTL_RXFLUSH_A) -> Self { variant as u8 != 0 } } impl MFIFOCTL_RXFLUSH_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> MFIFOCTL_RXFLUSH_A { match self . bits { false => MFIFOCTL_RXFLUSH_A :: MFIFOCTL_RXFLUSH_NOFLUSH , true => MFIFOCTL_RXFLUSH_A :: MFIFOCTL_RXFLUSH_FLUSH , } } # [doc = "NOFLUSH"] # [inline (always)] pub fn is_mfifoctl_rxflush_noflush (& self) -> bool { * self == MFIFOCTL_RXFLUSH_A :: MFIFOCTL_RXFLUSH_NOFLUSH } # [doc = "FLUSH"] # [inline (always)] pub fn is_mfifoctl_rxflush_flush (& self) -> bool { * self == MFIFOCTL_RXFLUSH_A :: MFIFOCTL_RXFLUSH_FLUSH } } # [doc = "Field `MFIFOCTL_RXFLUSH` writer - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."] pub type MFIFOCTL_RXFLUSH_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , MFIFOCTL_RXFLUSH_A > ; impl < 'a , REG , const O : u8 > MFIFOCTL_RXFLUSH_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NOFLUSH"] # [inline (always)] pub fn mfifoctl_rxflush_noflush (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_RXFLUSH_A :: MFIFOCTL_RXFLUSH_NOFLUSH) } # [doc = "FLUSH"] # [inline (always)] pub fn mfifoctl_rxflush_flush (self) -> & 'a mut crate :: W < REG > { self . variant (MFIFOCTL_RXFLUSH_A :: MFIFOCTL_RXFLUSH_FLUSH) } } impl R { # [doc = "Bits 0:2 - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."] # [inline (always)] pub fn mfifoctl_txtrig (& self) -> MFIFOCTL_TXTRIG_R { MFIFOCTL_TXTRIG_R :: new ((self . bits & 7) as u8) } # [doc = "Bit 7 - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."] # [inline (always)] pub fn mfifoctl_txflush (& self) -> MFIFOCTL_TXFLUSH_R { MFIFOCTL_TXFLUSH_R :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Bits 8:10 - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."] # [inline (always)] pub fn mfifoctl_rxtrig (& self) -> MFIFOCTL_RXTRIG_R { MFIFOCTL_RXTRIG_R :: new (((self . bits >> 8) & 7) as u8) } # [doc = "Bit 15 - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."] # [inline (always)] pub fn mfifoctl_rxflush (& self) -> MFIFOCTL_RXFLUSH_R { MFIFOCTL_RXFLUSH_R :: new (((self . bits >> 15) & 1) != 0) } } impl W { # [doc = "Bits 0:2 - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."] # [inline (always)] # [must_use] pub fn mfifoctl_txtrig (& mut self) -> MFIFOCTL_TXTRIG_W < MFIFOCTL_SPEC , 0 > { MFIFOCTL_TXTRIG_W :: new (self) } # [doc = "Bit 7 - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."] # [inline (always)] # [must_use] pub fn mfifoctl_txflush (& mut self) -> MFIFOCTL_TXFLUSH_W < MFIFOCTL_SPEC , 7 > { MFIFOCTL_TXFLUSH_W :: new (self) } # [doc = "Bits 8:10 - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."] # [inline (always)] # [must_use] pub fn mfifoctl_rxtrig (& mut self) -> MFIFOCTL_RXTRIG_W < MFIFOCTL_SPEC , 8 > { MFIFOCTL_RXTRIG_W :: new (self) } # [doc = "Bit 15 - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."] # [inline (always)] # [must_use] pub fn mfifoctl_rxflush (& mut self) -> MFIFOCTL_RXFLUSH_W < MFIFOCTL_SPEC , 15 > { MFIFOCTL_RXFLUSH_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "I2C Master FIFO Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfifoctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mfifoctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct MFIFOCTL_SPEC ; impl crate :: RegisterSpec for MFIFOCTL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`mfifoctl::R`](R) reader structure"] impl crate :: Readable for MFIFOCTL_SPEC { } # [doc = "`write(|w| ..)` method takes [`mfifoctl::W`](W) writer structure"] impl crate :: Writable for MFIFOCTL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets MFIFOCTL to value 0"] impl crate :: Resettable for MFIFOCTL_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }