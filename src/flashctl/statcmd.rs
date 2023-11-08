# [doc = "Register `STATCMD` reader"] pub type R = crate :: R < STATCMD_SPEC > ; # [doc = "Field `STATCMD_CMDDONE` reader - Command Done"] pub type STATCMD_CMDDONE_R = crate :: BitReader < STATCMD_CMDDONE_A > ; # [doc = "Command Done\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STATCMD_CMDDONE_A { # [doc = "0: STATNOTDONE"] STATCMD_CMDDONE_STATNOTDONE = 0 , # [doc = "1: STATDONE"] STATCMD_CMDDONE_STATDONE = 1 , } impl From < STATCMD_CMDDONE_A > for bool { # [inline (always)] fn from (variant : STATCMD_CMDDONE_A) -> Self { variant as u8 != 0 } } impl STATCMD_CMDDONE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STATCMD_CMDDONE_A { match self . bits { false => STATCMD_CMDDONE_A :: STATCMD_CMDDONE_STATNOTDONE , true => STATCMD_CMDDONE_A :: STATCMD_CMDDONE_STATDONE , } } # [doc = "STATNOTDONE"] # [inline (always)] pub fn is_statcmd_cmddone_statnotdone (& self) -> bool { * self == STATCMD_CMDDONE_A :: STATCMD_CMDDONE_STATNOTDONE } # [doc = "STATDONE"] # [inline (always)] pub fn is_statcmd_cmddone_statdone (& self) -> bool { * self == STATCMD_CMDDONE_A :: STATCMD_CMDDONE_STATDONE } } # [doc = "Field `STATCMD_CMDPASS` reader - Command Pass - valid when CMD_DONE field is 1"] pub type STATCMD_CMDPASS_R = crate :: BitReader < STATCMD_CMDPASS_A > ; # [doc = "Command Pass - valid when CMD_DONE field is 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STATCMD_CMDPASS_A { # [doc = "0: STATFAIL"] STATCMD_CMDPASS_STATFAIL = 0 , # [doc = "1: STATPASS"] STATCMD_CMDPASS_STATPASS = 1 , } impl From < STATCMD_CMDPASS_A > for bool { # [inline (always)] fn from (variant : STATCMD_CMDPASS_A) -> Self { variant as u8 != 0 } } impl STATCMD_CMDPASS_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STATCMD_CMDPASS_A { match self . bits { false => STATCMD_CMDPASS_A :: STATCMD_CMDPASS_STATFAIL , true => STATCMD_CMDPASS_A :: STATCMD_CMDPASS_STATPASS , } } # [doc = "STATFAIL"] # [inline (always)] pub fn is_statcmd_cmdpass_statfail (& self) -> bool { * self == STATCMD_CMDPASS_A :: STATCMD_CMDPASS_STATFAIL } # [doc = "STATPASS"] # [inline (always)] pub fn is_statcmd_cmdpass_statpass (& self) -> bool { * self == STATCMD_CMDPASS_A :: STATCMD_CMDPASS_STATPASS } } # [doc = "Field `STATCMD_CMDINPROGRESS` reader - Command In Progress"] pub type STATCMD_CMDINPROGRESS_R = crate :: BitReader < STATCMD_CMDINPROGRESS_A > ; # [doc = "Command In Progress\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STATCMD_CMDINPROGRESS_A { # [doc = "0: STATCOMPLETE"] STATCMD_CMDINPROGRESS_STATCOMPLETE = 0 , # [doc = "1: STATINPROGRESS"] STATCMD_CMDINPROGRESS_STATINPROGRESS = 1 , } impl From < STATCMD_CMDINPROGRESS_A > for bool { # [inline (always)] fn from (variant : STATCMD_CMDINPROGRESS_A) -> Self { variant as u8 != 0 } } impl STATCMD_CMDINPROGRESS_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STATCMD_CMDINPROGRESS_A { match self . bits { false => STATCMD_CMDINPROGRESS_A :: STATCMD_CMDINPROGRESS_STATCOMPLETE , true => STATCMD_CMDINPROGRESS_A :: STATCMD_CMDINPROGRESS_STATINPROGRESS , } } # [doc = "STATCOMPLETE"] # [inline (always)] pub fn is_statcmd_cmdinprogress_statcomplete (& self) -> bool { * self == STATCMD_CMDINPROGRESS_A :: STATCMD_CMDINPROGRESS_STATCOMPLETE } # [doc = "STATINPROGRESS"] # [inline (always)] pub fn is_statcmd_cmdinprogress_statinprogress (& self) -> bool { * self == STATCMD_CMDINPROGRESS_A :: STATCMD_CMDINPROGRESS_STATINPROGRESS } } # [doc = "Field `STATCMD_FAILWEPROT` reader - Command failed due to Write/Erase Protect Sector Violation"] pub type STATCMD_FAILWEPROT_R = crate :: BitReader < STATCMD_FAILWEPROT_A > ; # [doc = "Command failed due to Write/Erase Protect Sector Violation\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STATCMD_FAILWEPROT_A { # [doc = "0: STATNOFAIL"] STATCMD_FAILWEPROT_STATNOFAIL = 0 , # [doc = "1: STATFAIL"] STATCMD_FAILWEPROT_STATFAIL = 1 , } impl From < STATCMD_FAILWEPROT_A > for bool { # [inline (always)] fn from (variant : STATCMD_FAILWEPROT_A) -> Self { variant as u8 != 0 } } impl STATCMD_FAILWEPROT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STATCMD_FAILWEPROT_A { match self . bits { false => STATCMD_FAILWEPROT_A :: STATCMD_FAILWEPROT_STATNOFAIL , true => STATCMD_FAILWEPROT_A :: STATCMD_FAILWEPROT_STATFAIL , } } # [doc = "STATNOFAIL"] # [inline (always)] pub fn is_statcmd_failweprot_statnofail (& self) -> bool { * self == STATCMD_FAILWEPROT_A :: STATCMD_FAILWEPROT_STATNOFAIL } # [doc = "STATFAIL"] # [inline (always)] pub fn is_statcmd_failweprot_statfail (& self) -> bool { * self == STATCMD_FAILWEPROT_A :: STATCMD_FAILWEPROT_STATFAIL } } # [doc = "Field `STATCMD_FAILVERIFY` reader - Command failed due to verify error"] pub type STATCMD_FAILVERIFY_R = crate :: BitReader < STATCMD_FAILVERIFY_A > ; # [doc = "Command failed due to verify error\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STATCMD_FAILVERIFY_A { # [doc = "0: STATNOFAIL"] STATCMD_FAILVERIFY_STATNOFAIL = 0 , # [doc = "1: STATFAIL"] STATCMD_FAILVERIFY_STATFAIL = 1 , } impl From < STATCMD_FAILVERIFY_A > for bool { # [inline (always)] fn from (variant : STATCMD_FAILVERIFY_A) -> Self { variant as u8 != 0 } } impl STATCMD_FAILVERIFY_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STATCMD_FAILVERIFY_A { match self . bits { false => STATCMD_FAILVERIFY_A :: STATCMD_FAILVERIFY_STATNOFAIL , true => STATCMD_FAILVERIFY_A :: STATCMD_FAILVERIFY_STATFAIL , } } # [doc = "STATNOFAIL"] # [inline (always)] pub fn is_statcmd_failverify_statnofail (& self) -> bool { * self == STATCMD_FAILVERIFY_A :: STATCMD_FAILVERIFY_STATNOFAIL } # [doc = "STATFAIL"] # [inline (always)] pub fn is_statcmd_failverify_statfail (& self) -> bool { * self == STATCMD_FAILVERIFY_A :: STATCMD_FAILVERIFY_STATFAIL } } # [doc = "Field `STATCMD_FAILILLADDR` reader - Command failed due to the use of an illegal address"] pub type STATCMD_FAILILLADDR_R = crate :: BitReader < STATCMD_FAILILLADDR_A > ; # [doc = "Command failed due to the use of an illegal address\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STATCMD_FAILILLADDR_A { # [doc = "0: STATNOFAIL"] STATCMD_FAILILLADDR_STATNOFAIL = 0 , # [doc = "1: STATFAIL"] STATCMD_FAILILLADDR_STATFAIL = 1 , } impl From < STATCMD_FAILILLADDR_A > for bool { # [inline (always)] fn from (variant : STATCMD_FAILILLADDR_A) -> Self { variant as u8 != 0 } } impl STATCMD_FAILILLADDR_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STATCMD_FAILILLADDR_A { match self . bits { false => STATCMD_FAILILLADDR_A :: STATCMD_FAILILLADDR_STATNOFAIL , true => STATCMD_FAILILLADDR_A :: STATCMD_FAILILLADDR_STATFAIL , } } # [doc = "STATNOFAIL"] # [inline (always)] pub fn is_statcmd_faililladdr_statnofail (& self) -> bool { * self == STATCMD_FAILILLADDR_A :: STATCMD_FAILILLADDR_STATNOFAIL } # [doc = "STATFAIL"] # [inline (always)] pub fn is_statcmd_faililladdr_statfail (& self) -> bool { * self == STATCMD_FAILILLADDR_A :: STATCMD_FAILILLADDR_STATFAIL } } # [doc = "Field `STATCMD_FAILMODE` reader - Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode."] pub type STATCMD_FAILMODE_R = crate :: BitReader < STATCMD_FAILMODE_A > ; # [doc = "Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STATCMD_FAILMODE_A { # [doc = "0: STATNOFAIL"] STATCMD_FAILMODE_STATNOFAIL = 0 , # [doc = "1: STATFAIL"] STATCMD_FAILMODE_STATFAIL = 1 , } impl From < STATCMD_FAILMODE_A > for bool { # [inline (always)] fn from (variant : STATCMD_FAILMODE_A) -> Self { variant as u8 != 0 } } impl STATCMD_FAILMODE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STATCMD_FAILMODE_A { match self . bits { false => STATCMD_FAILMODE_A :: STATCMD_FAILMODE_STATNOFAIL , true => STATCMD_FAILMODE_A :: STATCMD_FAILMODE_STATFAIL , } } # [doc = "STATNOFAIL"] # [inline (always)] pub fn is_statcmd_failmode_statnofail (& self) -> bool { * self == STATCMD_FAILMODE_A :: STATCMD_FAILMODE_STATNOFAIL } # [doc = "STATFAIL"] # [inline (always)] pub fn is_statcmd_failmode_statfail (& self) -> bool { * self == STATCMD_FAILMODE_A :: STATCMD_FAILMODE_STATFAIL } } # [doc = "Field `STATCMD_FAILINVDATA` reader - Program command failed because an attempt was made to program a stored 0 value to a 1."] pub type STATCMD_FAILINVDATA_R = crate :: BitReader < STATCMD_FAILINVDATA_A > ; # [doc = "Program command failed because an attempt was made to program a stored 0 value to a 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STATCMD_FAILINVDATA_A { # [doc = "0: STATNOFAIL"] STATCMD_FAILINVDATA_STATNOFAIL = 0 , # [doc = "1: STATFAIL"] STATCMD_FAILINVDATA_STATFAIL = 1 , } impl From < STATCMD_FAILINVDATA_A > for bool { # [inline (always)] fn from (variant : STATCMD_FAILINVDATA_A) -> Self { variant as u8 != 0 } } impl STATCMD_FAILINVDATA_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STATCMD_FAILINVDATA_A { match self . bits { false => STATCMD_FAILINVDATA_A :: STATCMD_FAILINVDATA_STATNOFAIL , true => STATCMD_FAILINVDATA_A :: STATCMD_FAILINVDATA_STATFAIL , } } # [doc = "STATNOFAIL"] # [inline (always)] pub fn is_statcmd_failinvdata_statnofail (& self) -> bool { * self == STATCMD_FAILINVDATA_A :: STATCMD_FAILINVDATA_STATNOFAIL } # [doc = "STATFAIL"] # [inline (always)] pub fn is_statcmd_failinvdata_statfail (& self) -> bool { * self == STATCMD_FAILINVDATA_A :: STATCMD_FAILINVDATA_STATFAIL } } # [doc = "Field `STATCMD_FAILMISC` reader - Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit."] pub type STATCMD_FAILMISC_R = crate :: BitReader < STATCMD_FAILMISC_A > ; # [doc = "Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STATCMD_FAILMISC_A { # [doc = "0: STATNOFAIL"] STATCMD_FAILMISC_STATNOFAIL = 0 , # [doc = "1: STATFAIL"] STATCMD_FAILMISC_STATFAIL = 1 , } impl From < STATCMD_FAILMISC_A > for bool { # [inline (always)] fn from (variant : STATCMD_FAILMISC_A) -> Self { variant as u8 != 0 } } impl STATCMD_FAILMISC_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STATCMD_FAILMISC_A { match self . bits { false => STATCMD_FAILMISC_A :: STATCMD_FAILMISC_STATNOFAIL , true => STATCMD_FAILMISC_A :: STATCMD_FAILMISC_STATFAIL , } } # [doc = "STATNOFAIL"] # [inline (always)] pub fn is_statcmd_failmisc_statnofail (& self) -> bool { * self == STATCMD_FAILMISC_A :: STATCMD_FAILMISC_STATNOFAIL } # [doc = "STATFAIL"] # [inline (always)] pub fn is_statcmd_failmisc_statfail (& self) -> bool { * self == STATCMD_FAILMISC_A :: STATCMD_FAILMISC_STATFAIL } } impl R { # [doc = "Bit 0 - Command Done"] # [inline (always)] pub fn statcmd_cmddone (& self) -> STATCMD_CMDDONE_R { STATCMD_CMDDONE_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Command Pass - valid when CMD_DONE field is 1"] # [inline (always)] pub fn statcmd_cmdpass (& self) -> STATCMD_CMDPASS_R { STATCMD_CMDPASS_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Command In Progress"] # [inline (always)] pub fn statcmd_cmdinprogress (& self) -> STATCMD_CMDINPROGRESS_R { STATCMD_CMDINPROGRESS_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 4 - Command failed due to Write/Erase Protect Sector Violation"] # [inline (always)] pub fn statcmd_failweprot (& self) -> STATCMD_FAILWEPROT_R { STATCMD_FAILWEPROT_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - Command failed due to verify error"] # [inline (always)] pub fn statcmd_failverify (& self) -> STATCMD_FAILVERIFY_R { STATCMD_FAILVERIFY_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - Command failed due to the use of an illegal address"] # [inline (always)] pub fn statcmd_faililladdr (& self) -> STATCMD_FAILILLADDR_R { STATCMD_FAILILLADDR_R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 7 - Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode."] # [inline (always)] pub fn statcmd_failmode (& self) -> STATCMD_FAILMODE_R { STATCMD_FAILMODE_R :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Bit 8 - Program command failed because an attempt was made to program a stored 0 value to a 1."] # [inline (always)] pub fn statcmd_failinvdata (& self) -> STATCMD_FAILINVDATA_R { STATCMD_FAILINVDATA_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 12 - Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit."] # [inline (always)] pub fn statcmd_failmisc (& self) -> STATCMD_FAILMISC_R { STATCMD_FAILMISC_R :: new (((self . bits >> 12) & 1) != 0) } } # [doc = "Command Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statcmd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct STATCMD_SPEC ; impl crate :: RegisterSpec for STATCMD_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`statcmd::R`](R) reader structure"] impl crate :: Readable for STATCMD_SPEC { } # [doc = "`reset()` method sets STATCMD to value 0"] impl crate :: Resettable for STATCMD_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }