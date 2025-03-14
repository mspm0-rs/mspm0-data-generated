// This file is autogenerated by mspm0-metapac-gen: do not edit by hand.
pub static METADATA: Metadata = Metadata {
    name: "mspm0l110x",
    peripherals: &[
        Peripheral {
            name: "CPUSS",
            kind: "cpuss",
            pins: &[],
        },
        Peripheral {
            name: "DMA",
            kind: "dma",
            pins: &[],
        },
        Peripheral {
            name: "GPIOA",
            kind: "gpio",
            pins: &[],
        },
        Peripheral {
            name: "IOMUX",
            kind: "iomux",
            pins: &[],
        },
        Peripheral {
            name: "TIMG0",
            kind: "tim",
            pins: &[],
        },
        Peripheral {
            name: "TIMG1",
            kind: "tim",
            pins: &[],
        },
        Peripheral {
            name: "TIMG2",
            kind: "tim",
            pins: &[],
        },
        Peripheral {
            name: "TIMG4",
            kind: "tim",
            pins: &[],
        },
        Peripheral {
            name: "UART0",
            kind: "uart",
            pins: &[],
        },
        Peripheral {
            name: "UART1",
            kind: "uart",
            pins: &[],
        },
    ],
    pincm_mappings: &[
        PinCmMapping {
            pin: "PA0",
            pincm: 1,
        },
        PinCmMapping {
            pin: "PA1",
            pincm: 2,
        },
        PinCmMapping {
            pin: "PA10",
            pincm: 11,
        },
        PinCmMapping {
            pin: "PA11",
            pincm: 12,
        },
        PinCmMapping {
            pin: "PA12",
            pincm: 13,
        },
        PinCmMapping {
            pin: "PA13",
            pincm: 14,
        },
        PinCmMapping {
            pin: "PA14",
            pincm: 15,
        },
        PinCmMapping {
            pin: "PA15",
            pincm: 16,
        },
        PinCmMapping {
            pin: "PA16",
            pincm: 17,
        },
        PinCmMapping {
            pin: "PA17",
            pincm: 18,
        },
        PinCmMapping {
            pin: "PA18",
            pincm: 19,
        },
        PinCmMapping {
            pin: "PA19",
            pincm: 20,
        },
        PinCmMapping {
            pin: "PA2",
            pincm: 3,
        },
        PinCmMapping {
            pin: "PA20",
            pincm: 21,
        },
        PinCmMapping {
            pin: "PA21",
            pincm: 22,
        },
        PinCmMapping {
            pin: "PA22",
            pincm: 23,
        },
        PinCmMapping {
            pin: "PA23",
            pincm: 24,
        },
        PinCmMapping {
            pin: "PA24",
            pincm: 25,
        },
        PinCmMapping {
            pin: "PA25",
            pincm: 26,
        },
        PinCmMapping {
            pin: "PA26",
            pincm: 27,
        },
        PinCmMapping {
            pin: "PA27",
            pincm: 28,
        },
        PinCmMapping {
            pin: "PA3",
            pincm: 4,
        },
        PinCmMapping {
            pin: "PA4",
            pincm: 5,
        },
        PinCmMapping {
            pin: "PA5",
            pincm: 6,
        },
        PinCmMapping {
            pin: "PA6",
            pincm: 7,
        },
        PinCmMapping {
            pin: "PA7",
            pincm: 8,
        },
        PinCmMapping {
            pin: "PA8",
            pincm: 9,
        },
        PinCmMapping {
            pin: "PA9",
            pincm: 10,
        },
    ],
    interrupts: &[
        Interrupt {
            name: "GROUP0",
            number: 0,
        },
        Interrupt {
            name: "GPIOA",
            number: 1,
        },
        Interrupt {
            name: "TIMG1",
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
            name: "UART1",
            number: 13,
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
            name: "TIMG2",
            number: 18,
        },
        Interrupt {
            name: "TIMG4",
            number: 20,
        },
        Interrupt {
            name: "I2C0",
            number: 24,
        },
        Interrupt {
            name: "DMA",
            number: 31,
        },
    ],
    dma_channels: &[
        DmaChannel {
            number: 0,
            full: true,
        },
        DmaChannel {
            number: 1,
            full: false,
        },
        DmaChannel {
            number: 2,
            full: false,
        },
    ],
};
