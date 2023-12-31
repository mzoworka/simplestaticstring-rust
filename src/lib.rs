#![no_std]
#![feature(min_specialization)]
#![feature(fmt_internals)]

use core::slice;

use simplestaticvec::{StaticVec, StaticVecError};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StaticStringError {
    CapacityExceeded,
}

impl From<StaticVecError> for StaticStringError {
    fn from(value: StaticVecError) -> Self {
        match value {
            StaticVecError::CapacityExceeded => Self::CapacityExceeded,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct StaticString<const N: usize> {
    data: StaticVec<u8, N>,
}

impl<const N: usize> StaticString<N> {
    pub fn new(len: usize) -> Result<Self, StaticStringError>
    {
        Ok(Self {
            data: StaticVec::new(len).map_err(|e: StaticVecError| -> StaticStringError {e.into()})?,
        })
    }

    pub fn format(args: core::fmt::Arguments<'_>) -> Result<Self, StaticStringError> {
        args.to_static_string()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data.as_slice()
    }

    pub fn iter(&self) -> slice::Iter<'_, u8> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, u8> {
        self.data.iter_mut()
    }

    pub fn resize(&mut self, new_len: usize) -> Result<(), StaticStringError> {
        self.data.resize(new_len).map_err(|e| e.into())
    }

    pub fn try_extend_from_slice(&mut self, other: &[u8]) -> Result<(), StaticStringError>
    {
        self.data.try_extend_from_slice(other).map_err(|e| e.into())
    }

    pub fn try_extend_from_iter<I: Iterator<Item = u8>>(&mut self, iter: I) -> Result<(), StaticStringError>
    {
        self.data.try_extend_from_iter(iter).map_err(|e| e.into())
    }

    pub fn try_extend_from_iter_ref<'a, I: Iterator<Item = &'a u8>>(&mut self, iter: I) -> Result<(), StaticStringError>
    {
        self.data.try_extend_from_iter(iter.cloned()).map_err(|e| e.into())
    }
}

impl<'a, const N: usize> IntoIterator for &'a StaticString<N> {
    type Item = &'a u8;

    type IntoIter = slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize> Default for StaticString<N> {
    fn default() -> Self {
        Self { data: StaticVec::default() }
    }
}

impl<'a, const N: usize> From<&'a [u8;N]> for StaticString<N> {
    fn from(value: &'a[u8;N]) -> Self {
        let this = Self {
            data: value.into()
        };

        this
    }
}

impl<const N: usize> From<[u8;N]> for StaticString<N> {
    fn from(value: [u8;N]) -> Self {
        Self {
            data: value.into()
        }
    }
}

impl<const N: usize> core::fmt::Write for StaticString<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.try_extend_from_slice(s.as_bytes()).map_err(|_e| core::fmt::Error)
    }
}

impl<const N: usize> core::fmt::Display for StaticString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&**self, f)
    }
}

impl<const N: usize> core::fmt::Debug for StaticString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&**self, f)
    }
}

pub trait ToStaticString {
    fn to_static_string<const N: usize>(&self) -> Result<StaticString<N>, StaticStringError>;
}

impl<T: core::fmt::Display + ?Sized> ToStaticString for T {
    default fn to_static_string<const N: usize>(&self) -> Result<StaticString<N>, StaticStringError> {
        let mut buf = StaticString::<N>::new(0)?;
        let mut formatter = core::fmt::Formatter::new(&mut buf);
        core::fmt::Display::fmt(self, &mut formatter).map_err(|_e| StaticStringError::CapacityExceeded)?; //CapacityExceeded is the only possible error from Write
        Ok(buf)
    }
}

impl<const N: usize> core::ops::Deref for StaticString<N> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.data.as_slice()) }
    }
}
