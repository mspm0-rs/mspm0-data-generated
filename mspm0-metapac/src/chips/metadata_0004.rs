
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        kind: "adc",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA27",
                signal: "0",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "3",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "6",
                pf: Some(0u8),
            },
        ],
    },
    Peripheral {
        name: "CPUSS",
        kind: "cpuss",
        version: Some("v1"),
        pins: &[],
    },
    Peripheral {
        name: "CRC",
        kind: "crc",
        version: None,
        pins: &[],
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
    },
    Peripheral {
        name: "DMA",
        kind: "dma",
        version: Some("v1"),
        pins: &[],
    },
    Peripheral {
        name: "EVENT",
        kind: "event",
        version: None,
        pins: &[],
    },
    Peripheral {
        name: "FLASHCTL",
        kind: "flashctl",
        version: None,
        pins: &[],
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
                pin: "PA1",
                signal: "PA1",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "PA1",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "PA19",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "PA20",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "PA24",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "PA27",
                pf: Some(1u8),
            },
        ],
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
                pin: "PA0",
                signal: "SDA",
                pf: Some(3u8),
            },
        ],
    },
    Peripheral {
        name: "IOMUX",
        kind: "iomux",
        version: Some("v1"),
        pins: &[],
    },
    Peripheral {
        name: "SPI0",
        kind: "spi",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CS1_POCI1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CS2_POCI2",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "PICO",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "POCI",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "POCI",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SCLK",
                pf: Some(3u8),
            },
        ],
    },
    Peripheral {
        name: "TIMA0",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CCP0_CMPL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP3_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "FAULT1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "FAULT1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "FAULT2",
                pf: Some(7u8),
            },
        ],
    },
    Peripheral {
        name: "TIMG14",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP2",
                pf: Some(5u8),
            },
        ],
    },
    Peripheral {
        name: "TIMG8",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(4u8),
            },
        ],
    },
    Peripheral {
        name: "UART0",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "CTS",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RTS",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RX",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "TX",
                pf: Some(5u8),
            },
        ],
    },
    Peripheral {
        name: "VREF",
        kind: "vref",
        version: None,
        pins: &[],
    },
    Peripheral {
        name: "WUC",
        kind: "wuc",
        version: None,
        pins: &[],
    },
    Peripheral {
        name: "WWDT0",
        kind: "wwdt",
        version: None,
        pins: &[],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "GROUP0",
        number: 0,
    },
    Interrupt {
        name: "GPIOA",
        number: 1,
    },
    Interrupt {
        name: "TIMG8",
        number: 2,
    },
    Interrupt {
        name: "ADC0",
        number: 4,
    },
    Interrupt {
        name: "SPI0",
        number: 9,
    },
    Interrupt {
        name: "UART0",
        number: 15,
    },
    Interrupt {
        name: "TIMG14",
        number: 16,
    },
    Interrupt {
        name: "TIMA0",
        number: 18,
    },
    Interrupt {
        name: "I2C0",
        number: 24,
    },
    Interrupt {
        name: "DMA",
        number: 31,
    },
];
pub(crate) static INTERRUPT_GROUPS: &[InterruptGroup] = &[InterruptGroup {
    name: "GROUP0",
    number: 0,
    interrupts: &[
        GroupInterrupt {
            name: "WWDT0",
            number: 0,
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
}];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[DmaChannel {
    number: 0,
    full: false,
}];
pub(crate) static PINS: &[Pin] = &[
    Pin {
        pin: "PA24",
        pincm: 25u8,
    },
    Pin {
        pin: "PA0",
        pincm: 1u8,
    },
    Pin {
        pin: "PA27",
        pincm: 28u8,
    },
    Pin {
        pin: "PA1",
        pincm: 2u8,
    },
    Pin {
        pin: "PA20",
        pincm: 21u8,
    },
    Pin {
        pin: "PA19",
        pincm: 20u8,
    },
];
