//! This crate provides a Hardware Abstraction Layer for SoC Power Sequencing.

#![doc(html_root_url = "https://docs.rs/embedded-keyboard/latest")]
#![cfg_attr(not(test), no_std)]
#![allow(async_fn_in_trait)]

use macros::power_state;

/// Power Sequence error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic Power Sequence error kind.
    ///
    /// By using this method, Power Sequence errors freely defined by HAL
    /// implementations can be converted to a set of generic Power Sequence
    /// errors upon which generic code can act.
    fn kind(&self) -> ErrorKind;
}

/// Power Sequence error kind.
///
/// This represents a common set of errors. HAL implementations are free to
/// define more specific or additional error types. However, by providing a
/// mapping to these common Power Sequence errors, generic code can still react
/// to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// A different error occurred. The original error may contain more
    /// information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Power Sequence error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

pub trait PowerSequence: ErrorType {
    #[power_state]
    async fn power_on(&mut self) -> Result<(), Self::Error>;

    #[power_state]
    async fn power_off(&mut self) -> Result<(), Self::Error>;
}

impl<T: PowerSequence + ?Sized> PowerSequence for &mut T {
    #[inline]
    #[power_state]
    async fn power_on(&mut self) -> Result<(), Self::Error> {
        T::power_on(self).await
    }

    #[inline]
    #[power_state]
    async fn power_off(&mut self) -> Result<(), Self::Error> {
        T::power_off(self).await
    }
}
