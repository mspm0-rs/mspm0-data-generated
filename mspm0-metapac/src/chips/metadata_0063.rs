
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
                pin: "PA18",
                signal: "12",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "17",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "18",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "19",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "2",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "20",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "21",
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
        name: "AESADV",
        kind: "aesadv",
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
                pin: "PA13",
                signal: "IN2-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "OUT",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "OUT",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "OUT",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "OUT",
                pf: Some(11u8),
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
        name: "CRC0",
        kind: "crc",
        version: None,
        pins: &[],
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
        pins: &[],
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
                pin: "PA12",
                signal: "PA12",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "PA13",
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
                pin: "PB14",
                signal: "PB14",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "PB15",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "PB16",
                pf: Some(1u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
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
        name: "IWDT",
        kind: "iwdt",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "KEYSTORECTL",
        kind: "keystorectl",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "LFSS",
        kind: "lfss",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "RTC_OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "RTC_OUT",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTC_OUT",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RTC_OUT",
                pf: Some(8u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "RTC_B",
        kind: "rtc",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "SYSCTL",
        kind: "sysctl",
        version: Some("l122x_l222x"),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "BEEPER",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CLK_OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CLK_OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CLK_OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CLK_OUT",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "CLK_OUT",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "FCC_IN",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "FCC_IN",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "FCC_IN",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FCC_IN",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "HFCLKIN",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "HFXIN",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "HFXOUT",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "LFCLKIN",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "LFXIN",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "LFXOUT",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "NRST",
                signal: "NRST",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
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
                pin: "PB14",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP0",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP0_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP0_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP1_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP2",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CCP2",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CCP2_CMPL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP2_CMPL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP2_CMPL",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP3",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CCP3_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP3_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP3_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP3_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP3_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CCP3_CMPL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "FAULT0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FAULT0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "FAULT0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "FAULT1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FAULT1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FAULT1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "FAULT1",
                pf: Some(10u8),
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
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMB0",
        kind: "tim",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMB1",
        kind: "tim",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMB2",
        kind: "tim",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMB3",
        kind: "tim",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG1",
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
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP1",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG14",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CCP0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CCP1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP2",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CCP2",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP2",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP2",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CCP2",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP2",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CCP2",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP3",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP3",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP3",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CCP3",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP3",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP3",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP3",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP3",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG2",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PB15",
                signal: "CCP0",
                pf: Some(6u8),
            },
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
                pin: "PA9",
                signal: "CCP0",
                pf: Some(9u8),
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
                pin: "PB16",
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
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "UC11",
        kind: "unicomm",
        version: None,
        pins: &[
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
                pin: "PB16",
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
            PeripheralPin {
                pin: "PB15",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "TX",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "UC4",
        kind: "unicomm",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "CS0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CS0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CS0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CS0",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CS0",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CTS",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CTS",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "PICO",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "PICO",
                pf: Some(2u8),
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
                pin: "PB16",
                signal: "PICO",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "POCI",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "POCI",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "POCI",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCLK",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SCLK",
                pf: Some(3u8),
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
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "TX",
                pf: Some(7u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "UC6",
        kind: "unicomm",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SCL",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "SCL",
                pf: Some(9u8),
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
        sys_fentries: None,
    },
    Peripheral {
        name: "UC7",
        kind: "unicomm",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCL",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCL",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "SDA",
                pf: Some(11u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "UC8",
        kind: "unicomm",
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
                pin: "PA13",
                signal: "CS0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CS0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "PICO",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "PICO",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "POCI",
                pf: Some(9u8),
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
            PeripheralPin {
                pin: "PB14",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RX",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RX",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SCLK",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCLK",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SCLK",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SCLK",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SCLK",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "TX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "TX",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                pf: Some(11u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
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
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "SYSCTL",
        number: 0,
    },
    Interrupt {
        name: "DEBUGSS",
        number: 1,
    },
    Interrupt {
        name: "UC6",
        number: 3,
    },
    Interrupt {
        name: "ADC0",
        number: 4,
    },
    Interrupt {
        name: "TIMB0",
        number: 5,
    },
    Interrupt {
        name: "TIMB1",
        number: 6,
    },
    Interrupt {
        name: "COMP0",
        number: 7,
    },
    Interrupt {
        name: "UC7",
        number: 8,
    },
    Interrupt {
        name: "UC8",
        number: 9,
    },
    Interrupt {
        name: "TIMB2",
        number: 13,
    },
    Interrupt {
        name: "TIMB3",
        number: 14,
    },
    Interrupt {
        name: "UC4",
        number: 15,
    },
    Interrupt {
        name: "TIMG14",
        number: 16,
    },
    Interrupt {
        name: "TIMG2",
        number: 17,
    },
    Interrupt {
        name: "TIMA0",
        number: 18,
    },
    Interrupt {
        name: "TIMG1",
        number: 19,
    },
    Interrupt {
        name: "GPIOA",
        number: 22,
    },
    Interrupt {
        name: "GPIOB",
        number: 23,
    },
    Interrupt {
        name: "UC11",
        number: 24,
    },
    Interrupt {
        name: "AESADV",
        number: 26,
    },
    Interrupt {
        name: "FLASHCTL",
        number: 27,
    },
    Interrupt {
        name: "WWDT0",
        number: 29,
    },
    Interrupt {
        name: "LFSS",
        number: 30,
    },
    Interrupt {
        name: "DMA",
        number: 31,
    },
];
pub(crate) static INTERRUPT_GROUPS: &[InterruptGroup] = &[];
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
        full: false,
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin {
        pin: "PA26",
        pincm: 59u8,
    },
    Pin {
        pin: "PA3",
        pincm: 8u8,
    },
    Pin {
        pin: "PA4",
        pincm: 9u8,
    },
    Pin {
        pin: "PA5",
        pincm: 10u8,
    },
    Pin {
        pin: "PA6",
        pincm: 11u8,
    },
    Pin {
        pin: "PA9",
        pincm: 20u8,
    },
    Pin {
        pin: "PA10",
        pincm: 21u8,
    },
    Pin {
        pin: "PB14",
        pincm: 31u8,
    },
    Pin {
        pin: "PB15",
        pincm: 32u8,
    },
    Pin {
        pin: "PB16",
        pincm: 33u8,
    },
    Pin {
        pin: "PA12",
        pincm: 34u8,
    },
    Pin {
        pin: "PA27",
        pincm: 60u8,
    },
    Pin {
        pin: "PA13",
        pincm: 35u8,
    },
    Pin {
        pin: "PA18",
        pincm: 40u8,
    },
    Pin {
        pin: "PA19",
        pincm: 41u8,
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
        pin: "PA22",
        pincm: 47u8,
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
        pin: "PA25",
        pincm: 55u8,
    },
    Pin {
        pin: "PA0",
        pincm: 1u8,
    },
    Pin {
        pin: "PA1",
        pincm: 2u8,
    },
    Pin {
        pin: "PA2",
        pincm: 7u8,
    },
];
