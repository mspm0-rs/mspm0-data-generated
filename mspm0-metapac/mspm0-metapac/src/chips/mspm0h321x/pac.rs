// This file is autogenerated by mspm0-metapac-gen: do not edit by hand.
# [cfg (feature = "rt")] pub const NVIC_PRIO_BITS : u32 = 2 ; # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum Interrupt { SYSCTL = 0 , DEBUGSS = 1 , TIMG8 = 2 , UART1 = 3 , ADC0 = 4 , UART2 = 8 , SPI0 = 9 , GENSUB0 = 11 , GENSUB1 = 12 , UART0 = 15 , TIMG14 = 16 , TIMG2 = 17 , TIMA0 = 18 , TIMG1 = 19 , GPIOA = 22 , GPIOB = 23 , I2C0 = 24 , I2C1 = 25 , FLASHCTL = 27 , WWDT0 = 29 , GROUP30 = 30 , DMA = 31 } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)] fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")] mod _vectors { extern "C" { fn NonMaskableInt () ; fn HardFault () ; fn SVCall () ; fn PendSV () ; fn SysTick () ; fn SYSCTL () ; fn DEBUGSS () ; fn TIMG8 () ; fn UART1 () ; fn ADC0 () ; fn UART2 () ; fn SPI0 () ; fn GENSUB0 () ; fn GENSUB1 () ; fn UART0 () ; fn TIMG14 () ; fn TIMG2 () ; fn TIMA0 () ; fn TIMG1 () ; fn GPIOA () ; fn GPIOB () ; fn I2C0 () ; fn I2C1 () ; fn FLASHCTL () ; fn WWDT0 () ; fn GROUP30 () ; fn DMA () ; } # [repr (C)] pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [link_section = ".vector_table.interrupts"] # [no_mangle] pub static __INTERRUPTS : [Vector ; 32] = [Vector { _handler : SYSCTL } , Vector { _handler : DEBUGSS } , Vector { _handler : TIMG8 } , Vector { _handler : UART1 } , Vector { _handler : ADC0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : UART2 } , Vector { _handler : SPI0 } , Vector { _reserved : 0 } , Vector { _handler : GENSUB0 } , Vector { _handler : GENSUB1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : UART0 } , Vector { _handler : TIMG14 } , Vector { _handler : TIMG2 } , Vector { _handler : TIMA0 } , Vector { _handler : TIMG1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : GPIOA } , Vector { _handler : GPIOB } , Vector { _handler : I2C0 } , Vector { _handler : I2C1 } , Vector { _reserved : 0 } , Vector { _handler : FLASHCTL } , Vector { _reserved : 0 } , Vector { _handler : WWDT0 } , Vector { _handler : GROUP30 } , Vector { _handler : DMA }] ; } # [cfg (feature = "rt")] pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")] pub use Interrupt as interrupt ;
# [path = "../../peripherals/cpuss.rs"] pub mod cpuss ; # [path = "../../peripherals/gpio.rs"] pub mod gpio ; # [path = "../../peripherals/iomux.rs"] pub mod iomux ; # [path = "../../peripherals/sysctl_h321x.rs"] pub mod sysctl ; # [path = "../../peripherals/tim.rs"] pub mod tim ; # [doc = "Address: 1073758208"] pub const ADC0 : () = () ; pub const CPUSS : cpuss :: Cpuss = unsafe { cpuss :: Cpuss :: from_ptr (1077936128 as * mut _) } ; # [doc = "Address: 1074556928"] pub const DEBUGSS : () = () ; pub const GPIOA : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (1074397184 as * mut _) } ; pub const GPIOB : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (1074405376 as * mut _) } ; # [doc = "Address: 1074724864"] pub const I2C0 : () = () ; # [doc = "Address: 1074733056"] pub const I2C1 : () = () ; pub const IOMUX : iomux :: Iomux = unsafe { iomux :: Iomux :: from_ptr (1078099968 as * mut _) } ; # [doc = "Address: 1074348032"] pub const LFSS : () = () ; # [doc = "Address: 1078362112"] pub const SPI0 : () = () ; pub const SYSCTL : sysctl :: Sysctl = unsafe { sysctl :: Sysctl :: from_ptr (1074458624 as * mut _) } ; pub const TIMA0 : tim :: Tim = unsafe { tim :: Tim :: from_ptr (1082523648 as * mut _) } ; pub const TIMG1 : tim :: Tim = unsafe { tim :: Tim :: from_ptr (1074290688 as * mut _) } ; pub const TIMG14 : tim :: Tim = unsafe { tim :: Tim :: from_ptr (1074282496 as * mut _) } ; pub const TIMG2 : tim :: Tim = unsafe { tim :: Tim :: from_ptr (1074298880 as * mut _) } ; pub const TIMG8 : tim :: Tim = unsafe { tim :: Tim :: from_ptr (1074331648 as * mut _) } ; # [doc = "Address: 1074823168"] pub const UART0 : () = () ; # [doc = "Address: 1074790400"] pub const UART1 : () = () ; # [doc = "Address: 1074798592"] pub const UART2 : () = () ;
