#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(llvm_asm)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_unsafe)]

/// Library for ATmega2560P chip.
#[cfg(feature = "atmega2560p")]
pub mod atmega2560p {
    /// Hardware Abstraction Library (HAL).
    #[cfg(feature = "atmega2560p-hal")]
    pub mod hal {
        pub mod power;

        pub mod watchdog;

        pub mod sleep_mode;

        pub mod port;

        pub mod interrupts;

        pub mod pin;

        #[cfg(feature = "analog")]
        pub mod analog;

        #[cfg(feature = "analog")]
        pub mod digital;
    }

    /// Communication Protocols
    #[cfg(feature = "com")]
    pub mod com {
        pub mod serial;

        pub mod usart;

        pub mod usart_transmit;

        pub mod usart_initialize;

        pub mod usart_recieve;

        pub mod i2c;
    }
}

#[cfg(feature = "atmega2560p")]
pub use atmega2560p::*;

#[cfg(feature = "atmega328p")]
pub mod atmega328p {
    // Hardware Abstraction Library (HAL).
    #[cfg(feature = "atmega328p-hal")]
    pub mod hal {
        pub mod port;

        pub mod pin;

        pub mod watchdog;

        pub mod interrupts;

        pub mod sleep_mode;

        pub mod power;

        #[cfg(feature = "analog")]
        pub mod analog;

        #[cfg(feature = "analog")]
        pub mod digital;
    }

    #[cfg(feature = "com")]
    pub mod com {
        pub mod i2c;
    }
}

#[cfg(feature = "atmega328p")]
pub use atmega328p::*;

#[cfg(feature = "sensors")]
pub mod sensors {
    pub mod mpu6050;

    pub mod aht10;
}

pub mod avr;
pub mod config;
pub mod delay;
pub mod math;
