
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        kind: "adc",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA27",
                signal: "0",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "1",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "2",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "3",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "7",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "8",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "ADC1",
        kind: "adc",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "0",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "1",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "3",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "6",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "7",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "8",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "AES",
        kind: "aes",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "COMP0",
        kind: "comp",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "IN0+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "IN0-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "IN1+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN3+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "OUT",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                pf: Some(6u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "COMP1",
        kind: "comp",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "IN1-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN3+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "OUT",
                pf: Some(6u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "COMP2",
        kind: "comp",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PB19",
                signal: "IN1+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "IN1-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "OUT",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "OUT",
                pf: Some(2u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "CPUSS",
        kind: "cpuss",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "CRC",
        kind: "crc",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "DAC0",
        kind: "dac",
        version: None,
        pins: &[PeripheralPin {
            pin: "PA15",
            signal: "OUT",
            pf: Some(0u8),
        }],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "DEBUGSS",
        kind: "debugss",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "SWCLK",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SWDIO",
                pf: Some(2u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "DMA",
        kind: "dma",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "EVENT",
        kind: "event",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "FLASHCTL",
        kind: "flashctl",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "GPAMP",
        kind: "gpamp",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "IN+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "IN-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "OUT",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "GPIOA",
        kind: "gpio",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "PA0",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "PA1",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "PA10",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "PA11",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "PA15",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "PA16",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "PA18",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "PA19",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "PA2",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "PA20",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "PA21",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "PA22",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "PA23",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "PA24",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "PA25",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "PA26",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "PA27",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "PA3",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "PA4",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "PA5",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PA6",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "PA9",
                pf: Some(1u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "GPIOB",
        kind: "gpio",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PB19",
                signal: "PB19",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "PB23",
                pf: Some(1u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "I2C0",
        kind: "i2c",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SCL",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "SDA",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                pf: Some(4u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(8usize),
    },
    Peripheral {
        name: "I2C1",
        kind: "i2c",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SCL",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SDA",
                pf: Some(9u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(8usize),
    },
    Peripheral {
        name: "IOMUX",
        kind: "iomux",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "MATHACL",
        kind: "mathacl",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "OPA0",
        kind: "opa",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "IN0+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "IN0-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "IN1+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "IN1-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN2+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "OUT",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "OPA1",
        kind: "opa",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PB19",
                signal: "IN0+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "IN1+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN2+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "OUT",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "RTC",
        kind: "rtc",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA27",
                signal: "RTC_OUT",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTC_OUT",
                pf: Some(6u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "SPI0",
        kind: "spi",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SCLK",
                pf: Some(3u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "SPI1",
        kind: "spi",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CS0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "SCLK",
                pf: Some(2u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "TIMA0",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP0_CMPL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP0_CMPL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP1_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1_CMPL",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP2",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CCP2",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CCP2_CMPL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CCP2_CMPL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP2_CMPL",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP3",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP3_CMPL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP3_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "FAULT0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "FAULT0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "FAULT0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FAULT1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "FAULT1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "FAULT2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "FAULT2",
                pf: Some(5u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMA1",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CCP0_CMPL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CCP1_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP1_CMPL",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "FAULT0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "FAULT0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "FAULT0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FAULT1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "FAULT1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "FAULT2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "FAULT2",
                pf: Some(5u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG0",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP1",
                pf: Some(5u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG12",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP1",
                pf: Some(4u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG6",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(8u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG7",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1",
                pf: Some(7u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG8",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "CCP0",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CCP0",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IDX",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IDX",
                pf: Some(6u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TRNG",
        kind: "trng",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "UART0",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CTS",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "TX",
                pf: Some(2u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "UART1",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA21",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RX",
                pf: Some(2u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "UART2",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TX",
                pf: Some(2u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "UART3",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "CTS",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RTS",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "TX",
                pf: Some(2u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "VREF",
        kind: "vref",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "VREF+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "VREF-",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "WUC",
        kind: "wuc",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "WWDT0",
        kind: "wwdt",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "WWDT1",
        kind: "wwdt",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "GROUP0",
        number: 0,
    },
    Interrupt {
        name: "GROUP1",
        number: 1,
    },
    Interrupt {
        name: "TIMG8",
        number: 2,
    },
    Interrupt {
        name: "UART3",
        number: 3,
    },
    Interrupt {
        name: "ADC0",
        number: 4,
    },
    Interrupt {
        name: "ADC1",
        number: 5,
    },
    Interrupt {
        name: "DAC0",
        number: 7,
    },
    Interrupt {
        name: "SPI0",
        number: 9,
    },
    Interrupt {
        name: "SPI1",
        number: 10,
    },
    Interrupt {
        name: "UART1",
        number: 13,
    },
    Interrupt {
        name: "UART2",
        number: 14,
    },
    Interrupt {
        name: "UART0",
        number: 15,
    },
    Interrupt {
        name: "TIMG0",
        number: 16,
    },
    Interrupt {
        name: "TIMG6",
        number: 17,
    },
    Interrupt {
        name: "TIMA0",
        number: 18,
    },
    Interrupt {
        name: "TIMA1",
        number: 19,
    },
    Interrupt {
        name: "TIMG7",
        number: 20,
    },
    Interrupt {
        name: "TIMG12",
        number: 21,
    },
    Interrupt {
        name: "I2C0",
        number: 24,
    },
    Interrupt {
        name: "I2C1",
        number: 25,
    },
    Interrupt {
        name: "AES",
        number: 28,
    },
    Interrupt {
        name: "RTC",
        number: 30,
    },
    Interrupt {
        name: "DMA",
        number: 31,
    },
];
pub(crate) static INTERRUPT_GROUPS: &[InterruptGroup] = &[
    InterruptGroup {
        name: "GROUP0",
        number: 0,
        interrupts: &[
            GroupInterrupt {
                name: "WWDT0",
                number: 0,
            },
            GroupInterrupt {
                name: "WWDT1",
                number: 1,
            },
            GroupInterrupt {
                name: "DEBUGSS",
                number: 2,
            },
            GroupInterrupt {
                name: "FLASHCTL",
                number: 3,
            },
            GroupInterrupt {
                name: "SYSCTL",
                number: 6,
            },
        ],
    },
    InterruptGroup {
        name: "GROUP1",
        number: 1,
        interrupts: &[
            GroupInterrupt {
                name: "GPIOA",
                number: 0,
            },
            GroupInterrupt {
                name: "GPIOB",
                number: 1,
            },
            GroupInterrupt {
                name: "COMP0",
                number: 2,
            },
            GroupInterrupt {
                name: "COMP1",
                number: 3,
            },
            GroupInterrupt {
                name: "COMP2",
                number: 4,
            },
            GroupInterrupt {
                name: "TRNG",
                number: 5,
            },
        ],
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        number: 0,
        full: true,
    },
    DmaChannel {
        number: 1,
        full: true,
    },
    DmaChannel {
        number: 2,
        full: true,
    },
    DmaChannel {
        number: 3,
        full: false,
    },
    DmaChannel {
        number: 4,
        full: false,
    },
    DmaChannel {
        number: 5,
        full: false,
    },
    DmaChannel {
        number: 6,
        full: false,
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin {
        pin: "PB19",
        pincm: 45u8,
    },
    Pin {
        pin: "PA23",
        pincm: 53u8,
    },
    Pin {
        pin: "PA24",
        pincm: 54u8,
    },
    Pin {
        pin: "PA18",
        pincm: 40u8,
    },
    Pin {
        pin: "PB23",
        pincm: 51u8,
    },
    Pin {
        pin: "PA26",
        pincm: 59u8,
    },
    Pin {
        pin: "PA19",
        pincm: 41u8,
    },
    Pin {
        pin: "PA22",
        pincm: 47u8,
    },
    Pin {
        pin: "PA27",
        pincm: 60u8,
    },
    Pin {
        pin: "PA20",
        pincm: 42u8,
    },
    Pin {
        pin: "PA21",
        pincm: 46u8,
    },
    Pin {
        pin: "PA0",
        pincm: 1u8,
    },
    Pin {
        pin: "PA16",
        pincm: 38u8,
    },
    Pin {
        pin: "PA25",
        pincm: 55u8,
    },
    Pin {
        pin: "PA1",
        pincm: 2u8,
    },
    Pin {
        pin: "PA2",
        pincm: 7u8,
    },
    Pin {
        pin: "PA15",
        pincm: 37u8,
    },
    Pin {
        pin: "PA9",
        pincm: 20u8,
    },
    Pin {
        pin: "PA5",
        pincm: 10u8,
    },
    Pin {
        pin: "PA3",
        pincm: 8u8,
    },
    Pin {
        pin: "PA10",
        pincm: 21u8,
    },
    Pin {
        pin: "PA11",
        pincm: 22u8,
    },
    Pin {
        pin: "PA6",
        pincm: 11u8,
    },
    Pin {
        pin: "PA4",
        pincm: 9u8,
    },
];
