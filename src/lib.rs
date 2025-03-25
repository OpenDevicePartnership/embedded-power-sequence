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

/// A representation for a device's power sequence
pub trait PowerSequence: ErrorType {
    #[power_state]
    /// Power the device on.
    async fn power_on(&mut self) -> Result<(), Self::Error>;

    #[power_state]
    /// Power the device off. Usually a direct reverse of [`PowerSequence::power_on`].
    async fn power_off(&mut self) -> Result<(), Self::Error>;

    #[power_state]
    /// Put device in idle state. The Windows operating system refers
    /// to this as Modern Standby
    async fn idle(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[power_state]
    /// The reverse operation of [`PowerSequence::idle`].
    async fn wake_up(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[power_state]
    /// Commonly referred to as "Suspend To RAM".
    async fn suspend(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[power_state]
    /// Wake from Suspend to RAM state.
    async fn resume(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[power_state]
    /// Commonly refered to as "Suspend to Disk".
    async fn hibernate(&mut self) -> Result<(), Self::Error> {
        self.power_off().await
    }

    #[power_state]
    /// Wake from Suspend to Disk. In many cases, this is the same as
    /// [`PowerSequence::power_on`].
    async fn activate(&mut self) -> Result<(), Self::Error> {
        self.power_on().await
    }
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

    #[inline]
    #[power_state]
    async fn idle(&mut self) -> Result<(), Self::Error> {
        T::idle(self).await
    }

    #[inline]
    #[power_state]
    async fn wake_up(&mut self) -> Result<(), Self::Error> {
        T::wake_up(self).await
    }

    #[inline]
    #[power_state]
    async fn suspend(&mut self) -> Result<(), Self::Error> {
        T::suspend(self).await
    }

    #[inline]
    #[power_state]
    async fn resume(&mut self) -> Result<(), Self::Error> {
        T::resume(self).await
    }

    #[inline]
    #[power_state]
    async fn hibernate(&mut self) -> Result<(), Self::Error> {
        T::hibernate(self).await
    }

    #[inline]
    #[power_state]
    async fn activate(&mut self) -> Result<(), Self::Error> {
        T::activate(self).await
    }
}
