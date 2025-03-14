// This file is autogenerated by mspm0-metapac-gen: do not edit by hand.
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PERIPHERALREGION."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iomux {
    ptr: *mut u8,
}
unsafe impl Send for Iomux {}
unsafe impl Sync for Iomux {}
impl Iomux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Pin Control Management Register in SECCFG region."]
    #[inline(always)]
    pub const fn pincm(self, n: usize) -> crate::common::Reg<regs::Pincm, crate::common::RW> {
        assert!(n < 251usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Pin Control Management Register in SECCFG region."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pincm(pub u32);
    impl Pincm {
        #[doc = "Peripheral Function selection bits."]
        #[inline(always)]
        pub const fn pf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Peripheral Function selection bits."]
        #[inline(always)]
        pub fn set_pf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Peripheral is Connected."]
        #[inline(always)]
        pub const fn pc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral is Connected."]
        #[inline(always)]
        pub fn set_pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "This has the IOPAD WAKEUP signal as status bit."]
        #[inline(always)]
        pub const fn wakestat(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "This has the IOPAD WAKEUP signal as status bit."]
        #[inline(always)]
        pub fn set_wakestat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Pull Down control selection."]
        #[inline(always)]
        pub const fn pipd(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Pull Down control selection."]
        #[inline(always)]
        pub fn set_pipd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Pull Up control selection."]
        #[inline(always)]
        pub const fn pipu(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Pull Up control selection."]
        #[inline(always)]
        pub fn set_pipu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Input Enable Control Selection."]
        #[inline(always)]
        pub const fn inena(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Input Enable Control Selection."]
        #[inline(always)]
        pub fn set_inena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Hysteresis Enable Control Selection."]
        #[inline(always)]
        pub const fn hysten(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Hysteresis Enable Control Selection."]
        #[inline(always)]
        pub fn set_hysten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Drive strength control selection, for HS IOCELL only."]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Drive strength control selection, for HS IOCELL only."]
        #[inline(always)]
        pub fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "High output value will tri-state the output when this bit is enabled."]
        #[inline(always)]
        pub const fn hiz1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "High output value will tri-state the output when this bit is enabled."]
        #[inline(always)]
        pub fn set_hiz1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Data inversion selection."]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Data inversion selection."]
        #[inline(always)]
        pub fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Wakeup Enable bit."]
        #[inline(always)]
        pub const fn wuen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup Enable bit."]
        #[inline(always)]
        pub fn set_wuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Wakeup Compare Value bit."]
        #[inline(always)]
        pub const fn wcomp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup Compare Value bit."]
        #[inline(always)]
        pub fn set_wcomp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Pincm {
        #[inline(always)]
        fn default() -> Pincm {
            Pincm(0)
        }
    }
}
