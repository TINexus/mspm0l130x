# [doc = "Register `INT_EVENT2_IMASK` reader"] pub type R = crate :: R < INT_EVENT2_IMASK_SPEC > ; # [doc = "Register `INT_EVENT2_IMASK` writer"] pub type W = crate :: W < INT_EVENT2_IMASK_SPEC > ; # [doc = "Field `INT_EVENT2_IMASK_TX` reader - Transmit FIFO event mask."] pub type INT_EVENT2_IMASK_TX_R = crate :: BitReader < INT_EVENT2_IMASK_TX_A > ; # [doc = "Transmit FIFO event mask.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_IMASK_TX_A { # [doc = "0: CLR"] INT_EVENT2_IMASK_TX_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_IMASK_TX_SET = 1 , } impl From < INT_EVENT2_IMASK_TX_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_IMASK_TX_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_IMASK_TX_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_IMASK_TX_A { match self . bits { false => INT_EVENT2_IMASK_TX_A :: INT_EVENT2_IMASK_TX_CLR , true => INT_EVENT2_IMASK_TX_A :: INT_EVENT2_IMASK_TX_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_imask_tx_clr (& self) -> bool { * self == INT_EVENT2_IMASK_TX_A :: INT_EVENT2_IMASK_TX_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_imask_tx_set (& self) -> bool { * self == INT_EVENT2_IMASK_TX_A :: INT_EVENT2_IMASK_TX_SET } } # [doc = "Field `INT_EVENT2_IMASK_TX` writer - Transmit FIFO event mask."] pub type INT_EVENT2_IMASK_TX_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_IMASK_TX_A > ; impl < 'a , REG , const O : u8 > INT_EVENT2_IMASK_TX_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event2_imask_tx_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_TX_A :: INT_EVENT2_IMASK_TX_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event2_imask_tx_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_TX_A :: INT_EVENT2_IMASK_TX_SET) } } impl R { # [doc = "Bit 4 - Transmit FIFO event mask."] # [inline (always)] pub fn int_event2_imask_tx (& self) -> INT_EVENT2_IMASK_TX_R { INT_EVENT2_IMASK_TX_R :: new (((self . bits >> 4) & 1) != 0) } } impl W { # [doc = "Bit 4 - Transmit FIFO event mask."] # [inline (always)] # [must_use] pub fn int_event2_imask_tx (& mut self) -> INT_EVENT2_IMASK_TX_W < INT_EVENT2_IMASK_SPEC , 4 > { INT_EVENT2_IMASK_TX_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event2_imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event2_imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT2_IMASK_SPEC ; impl crate :: RegisterSpec for INT_EVENT2_IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event2_imask::R`](R) reader structure"] impl crate :: Readable for INT_EVENT2_IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`int_event2_imask::W`](W) writer structure"] impl crate :: Writable for INT_EVENT2_IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT2_IMASK to value 0"] impl crate :: Resettable for INT_EVENT2_IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }