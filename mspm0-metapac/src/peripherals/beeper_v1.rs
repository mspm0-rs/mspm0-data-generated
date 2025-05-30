// This file is autogenerated by mspm0-metapac-gen: do not edit by hand.
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "The MSPM0C110x/MSPS003 beeper. This technically exists in the SYSCTL block, but is separated."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Beeper {
    ptr: *mut u8,
}
unsafe impl Send for Beeper {}
unsafe impl Sync for Beeper {}
impl Beeper {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "BEEPER Configuration"]
    #[inline(always)]
    pub const fn beepcfg(self) -> crate::common::Reg<regs::Beepcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1190usize) as _) }
    }
}
pub mod regs {
    #[doc = "BEEPER Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Beepcfg(pub u32);
    impl Beepcfg {
        #[doc = "Beeper output enable"]
        #[inline(always)]
        pub const fn en(&self) -> super::vals::En {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::En::from_bits(val as u8)
        }
        #[doc = "Beeper output enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: super::vals::En) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Beeper Output Frequency Configuration"]
        #[inline(always)]
        pub const fn freq(&self) -> super::vals::Freq {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Freq::from_bits(val as u8)
        }
        #[doc = "Beeper Output Frequency Configuration"]
        #[inline(always)]
        pub fn set_freq(&mut self, val: super::vals::Freq) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Beepcfg {
        #[inline(always)]
        fn default() -> Beepcfg {
            Beepcfg(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum En {
        #[doc = "Beeper Output Disabled"]
        DISABLE = 0x0,
        #[doc = "Beeper Output Enabled"]
        ENABLE = 0x01,
    }
    impl En {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> En {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for En {
        #[inline(always)]
        fn from(val: u8) -> En {
            En::from_bits(val)
        }
    }
    impl From<En> for u8 {
        #[inline(always)]
        fn from(val: En) -> u8 {
            En::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Freq {
        #[doc = "Beeper runs at 8KHz"]
        _8KHZ = 0x0,
        #[doc = "Beeper runs at 4KHz"]
        _4KHZ = 0x01,
        #[doc = "Beeper runs at 2KHz"]
        _2KHZ = 0x02,
        #[doc = "Beeper runs at 1KHz"]
        _1KHZ = 0x03,
    }
    impl Freq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Freq {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Freq {
        #[inline(always)]
        fn from(val: u8) -> Freq {
            Freq::from_bits(val)
        }
    }
    impl From<Freq> for u8 {
        #[inline(always)]
        fn from(val: Freq) -> u8 {
            Freq::to_bits(val)
        }
    }
}
