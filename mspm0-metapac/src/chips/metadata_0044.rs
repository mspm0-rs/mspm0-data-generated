
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        kind: "adc",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "0",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "10",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "11",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "12",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "13",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "14",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "15",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "16",
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
                pin: "PB24",
                signal: "5",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "6",
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
            PeripheralPin {
                pin: "PB19",
                signal: "9",
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
                pin: "PA11",
                signal: "DAC_OUT",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "IN1+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "IN1-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "IN2+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "IN2-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN3+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "OUT",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "OUT",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "OUT",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "OUT",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "OUT",
                pf: Some(8u8),
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
        name: "CRCP0",
        kind: "crc",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
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
                pin: "PA11",
                signal: "PA11",
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
                pin: "PA14",
                signal: "PA14",
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
                pin: "PA17",
                signal: "PA17",
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
                pin: "PA28",
                signal: "PA28",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "PA3",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "PA31",
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
                pin: "PA7",
                signal: "PA7",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "PA8",
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
            PeripheralPin {
                pin: "PB17",
                signal: "PB17",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "PB18",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "PB19",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "PB2",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "PB20",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "PB24",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "PB3",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "PB6",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "PB7",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "PB8",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "PB9",
                pf: Some(1u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "I2S0",
        kind: "i2s",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "AD0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "AD0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "AD0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "AD0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "AD1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "AD1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "AD1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "AD1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "BCLK",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "BCLK",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BCLK",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MCLK",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "MCLK",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "MCLK",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "WCLK",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "WCLK",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "WCLK",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "WCLK",
                pf: Some(9u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
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
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTC_OUT",
                pf: Some(8u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "NPU",
        kind: "npu",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
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
                pin: "PA2",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CCP0",
                pf: Some(8u8),
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
                pin: "PB9",
                signal: "CCP0_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CCP1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB9",
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
                pin: "PB20",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CCP1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CCP1_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CCP2",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP2",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB18",
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
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CCP3",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP3",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "CCP3_CMPL",
                pf: Some(4u8),
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
                pin: "PB3",
                signal: "CCP3_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP3_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "FAULT0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "FAULT0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "FAULT0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "FAULT0",
                pf: Some(8u8),
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
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "FAULT1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "FAULT1",
                pf: Some(8u8),
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
                pin: "PA1",
                signal: "FAULT2",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "FAULT2",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "FAULT2",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "FAULT2",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "FAULT2",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
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
        name: "TIMG0",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PB17",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CCP0",
                pf: Some(5u8),
            },
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
                pin: "PA23",
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
                pin: "PB18",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(5u8),
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
                pin: "PA24",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CCP1",
                pf: Some(7u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG6",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP1",
                pf: Some(7u8),
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
                pin: "PB15",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA23",
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
                pin: "PB16",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA24",
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
        name: "UC0",
        kind: "unicomm",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CTS",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "RX",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "SCL",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "SCL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "SDA",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "SDA",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "SDA",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "TX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "TX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "TX",
                pf: Some(7u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "UC1",
        kind: "unicomm",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CTS",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCL",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                pf: Some(4u8),
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
                pin: "PA19",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "TX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "TX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "TX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "TX",
                pf: Some(4u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "UC2",
        kind: "unicomm",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CS0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CS0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CS0",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CS1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CS1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CS1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CS1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CS1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CS2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CS2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CS2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CS2",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CS2",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CS3",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CS3",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CS3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CS3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CS3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CS3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "CS3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CS3",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CS3",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CS3",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "PICO",
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
                pin: "PB17",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "PICO",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA7",
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
                pin: "PB18",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCLK",
                pf: Some(7u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "UC3",
        kind: "unicomm",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "CS0",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CS0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CS0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CS0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CS0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CTS",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "PICO",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "PICO",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "PICO",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "PICO",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "PICO",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "PICO",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "PICO",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PICO",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "POCI",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "POCI",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "POCI",
                pf: Some(3u8),
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
                pin: "PB16",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "POCI",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "POCI",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "POCI",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "RX",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RX",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "SCLK",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "SCLK",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCLK",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SCLK",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "SCLK",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "SCLK",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCLK",
                pf: Some(10u8),
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
                pin: "PB2",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "TX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "TX",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "TX",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "TX",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "USBFS0",
        kind: "usbfs",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "DM",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "DP",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
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
    Peripheral {
        name: "WWDT1",
        kind: "wwdt",
        version: Some("v1"),
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
        name: "TIMB0",
        number: 2,
    },
    Interrupt {
        name: "ADC0",
        number: 4,
    },
    Interrupt {
        name: "USBFS0",
        number: 6,
    },
    Interrupt {
        name: "UC0",
        number: 9,
    },
    Interrupt {
        name: "UC1",
        number: 10,
    },
    Interrupt {
        name: "I2S0",
        number: 11,
    },
    Interrupt {
        name: "NPU",
        number: 12,
    },
    Interrupt {
        name: "UC2",
        number: 13,
    },
    Interrupt {
        name: "UC3",
        number: 14,
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
        name: "TIMG7",
        number: 20,
    },
    Interrupt {
        name: "AESADV",
        number: 28,
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
        full: true,
    },
    DmaChannel {
        number: 4,
        full: true,
    },
    DmaChannel {
        number: 5,
        full: true,
    },
    DmaChannel {
        number: 6,
        full: false,
    },
    DmaChannel {
        number: 7,
        full: false,
    },
    DmaChannel {
        number: 8,
        full: false,
    },
    DmaChannel {
        number: 9,
        full: false,
    },
    DmaChannel {
        number: 10,
        full: false,
    },
    DmaChannel {
        number: 11,
        full: false,
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin {
        pin: "PA0",
        pincm: 1u8,
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
        pin: "PA7",
        pincm: 14u8,
    },
    Pin {
        pin: "PB2",
        pincm: 15u8,
    },
    Pin {
        pin: "PB3",
        pincm: 16u8,
    },
    Pin {
        pin: "PA8",
        pincm: 19u8,
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
        pin: "PA11",
        pincm: 22u8,
    },
    Pin {
        pin: "PA1",
        pincm: 2u8,
    },
    Pin {
        pin: "PB6",
        pincm: 23u8,
    },
    Pin {
        pin: "PB7",
        pincm: 24u8,
    },
    Pin {
        pin: "PB8",
        pincm: 25u8,
    },
    Pin {
        pin: "PB9",
        pincm: 26u8,
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
        pin: "PA13",
        pincm: 35u8,
    },
    Pin {
        pin: "PA14",
        pincm: 36u8,
    },
    Pin {
        pin: "PA28",
        pincm: 3u8,
    },
    Pin {
        pin: "PA15",
        pincm: 37u8,
    },
    Pin {
        pin: "PA16",
        pincm: 38u8,
    },
    Pin {
        pin: "PA17",
        pincm: 39u8,
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
        pin: "PB17",
        pincm: 43u8,
    },
    Pin {
        pin: "PB18",
        pincm: 44u8,
    },
    Pin {
        pin: "PB19",
        pincm: 45u8,
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
        pin: "PB20",
        pincm: 48u8,
    },
    Pin {
        pin: "PB24",
        pincm: 52u8,
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
        pin: "PA26",
        pincm: 58u8,
    },
    Pin {
        pin: "PA27",
        pincm: 59u8,
    },
    Pin {
        pin: "PA31",
        pincm: 6u8,
    },
    Pin {
        pin: "PA2",
        pincm: 7u8,
    },
    Pin {
        pin: "PA3",
        pincm: 8u8,
    },
];
