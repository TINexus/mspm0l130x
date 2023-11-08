# [doc = "Register `CTL` reader"] pub type R = crate :: R < CTL_SPEC > ; # [doc = "Register `CTL` writer"] pub type W = crate :: W < CTL_SPEC > ; # [doc = "Field `CTL_PREFETCH` reader - Used to enable/disable instruction prefetch to Flash."] pub type CTL_PREFETCH_R = crate :: BitReader < CTL_PREFETCH_A > ; # [doc = "Used to enable/disable instruction prefetch to Flash.\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL_PREFETCH_A { # [doc = "0: DISABLE"] CTL_PREFETCH_DISABLE = 0 , # [doc = "1: ENABLE"] CTL_PREFETCH_ENABLE = 1 , } impl From < CTL_PREFETCH_A > for bool { # [inline (always)] fn from (variant : CTL_PREFETCH_A) -> Self { variant as u8 != 0 } } impl CTL_PREFETCH_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL_PREFETCH_A { match self . bits { false => CTL_PREFETCH_A :: CTL_PREFETCH_DISABLE , true => CTL_PREFETCH_A :: CTL_PREFETCH_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_ctl_prefetch_disable (& self) -> bool { * self == CTL_PREFETCH_A :: CTL_PREFETCH_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_ctl_prefetch_enable (& self) -> bool { * self == CTL_PREFETCH_A :: CTL_PREFETCH_ENABLE } } # [doc = "Field `CTL_PREFETCH` writer - Used to enable/disable instruction prefetch to Flash."] pub type CTL_PREFETCH_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL_PREFETCH_A > ; impl < 'a , REG , const O : u8 > CTL_PREFETCH_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn ctl_prefetch_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_PREFETCH_A :: CTL_PREFETCH_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn ctl_prefetch_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_PREFETCH_A :: CTL_PREFETCH_ENABLE) } } # [doc = "Field `CTL_ICACHE` reader - Used to enable/disable Instruction caching on flash access."] pub type CTL_ICACHE_R = crate :: BitReader < CTL_ICACHE_A > ; # [doc = "Used to enable/disable Instruction caching on flash access.\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL_ICACHE_A { # [doc = "0: DISABLE"] CTL_ICACHE_DISABLE = 0 , # [doc = "1: ENABLE"] CTL_ICACHE_ENABLE = 1 , } impl From < CTL_ICACHE_A > for bool { # [inline (always)] fn from (variant : CTL_ICACHE_A) -> Self { variant as u8 != 0 } } impl CTL_ICACHE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL_ICACHE_A { match self . bits { false => CTL_ICACHE_A :: CTL_ICACHE_DISABLE , true => CTL_ICACHE_A :: CTL_ICACHE_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_ctl_icache_disable (& self) -> bool { * self == CTL_ICACHE_A :: CTL_ICACHE_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_ctl_icache_enable (& self) -> bool { * self == CTL_ICACHE_A :: CTL_ICACHE_ENABLE } } # [doc = "Field `CTL_ICACHE` writer - Used to enable/disable Instruction caching on flash access."] pub type CTL_ICACHE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL_ICACHE_A > ; impl < 'a , REG , const O : u8 > CTL_ICACHE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn ctl_icache_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_ICACHE_A :: CTL_ICACHE_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn ctl_icache_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_ICACHE_A :: CTL_ICACHE_ENABLE) } } # [doc = "Field `CTL_LITEN` reader - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"] pub type CTL_LITEN_R = crate :: BitReader < CTL_LITEN_A > ; # [doc = "Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL_LITEN_A { # [doc = "0: DISABLE"] CTL_LITEN_DISABLE = 0 , # [doc = "1: ENABLE"] CTL_LITEN_ENABLE = 1 , } impl From < CTL_LITEN_A > for bool { # [inline (always)] fn from (variant : CTL_LITEN_A) -> Self { variant as u8 != 0 } } impl CTL_LITEN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL_LITEN_A { match self . bits { false => CTL_LITEN_A :: CTL_LITEN_DISABLE , true => CTL_LITEN_A :: CTL_LITEN_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_ctl_liten_disable (& self) -> bool { * self == CTL_LITEN_A :: CTL_LITEN_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_ctl_liten_enable (& self) -> bool { * self == CTL_LITEN_A :: CTL_LITEN_ENABLE } } # [doc = "Field `CTL_LITEN` writer - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"] pub type CTL_LITEN_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL_LITEN_A > ; impl < 'a , REG , const O : u8 > CTL_LITEN_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn ctl_liten_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_LITEN_A :: CTL_LITEN_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn ctl_liten_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_LITEN_A :: CTL_LITEN_ENABLE) } } impl R { # [doc = "Bit 0 - Used to enable/disable instruction prefetch to Flash."] # [inline (always)] pub fn ctl_prefetch (& self) -> CTL_PREFETCH_R { CTL_PREFETCH_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Used to enable/disable Instruction caching on flash access."] # [inline (always)] pub fn ctl_icache (& self) -> CTL_ICACHE_R { CTL_ICACHE_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"] # [inline (always)] pub fn ctl_liten (& self) -> CTL_LITEN_R { CTL_LITEN_R :: new (((self . bits >> 2) & 1) != 0) } } impl W { # [doc = "Bit 0 - Used to enable/disable instruction prefetch to Flash."] # [inline (always)] # [must_use] pub fn ctl_prefetch (& mut self) -> CTL_PREFETCH_W < CTL_SPEC , 0 > { CTL_PREFETCH_W :: new (self) } # [doc = "Bit 1 - Used to enable/disable Instruction caching on flash access."] # [inline (always)] # [must_use] pub fn ctl_icache (& mut self) -> CTL_ICACHE_W < CTL_SPEC , 1 > { CTL_ICACHE_W :: new (self) } # [doc = "Bit 2 - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"] # [inline (always)] # [must_use] pub fn ctl_liten (& mut self) -> CTL_LITEN_W < CTL_SPEC , 2 > { CTL_LITEN_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Prefetch/Cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CTL_SPEC ; impl crate :: RegisterSpec for CTL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`ctl::R`](R) reader structure"] impl crate :: Readable for CTL_SPEC { } # [doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"] impl crate :: Writable for CTL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CTL to value 0x07"] impl crate :: Resettable for CTL_SPEC { const RESET_VALUE : Self :: Ux = 0x07 ; }