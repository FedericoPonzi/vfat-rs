//! I/O traits. With std it uses the std io modules structs, otherwise uses the custom implementation which is copied from rust std to ease compatibility.

pub type Result<T> = core::result::Result<T, Error>;
#[cfg(not(feature = "std"))]
pub use nonstd_io::*;

#[cfg(feature = "std")]
pub use std_io::*;

#[cfg(feature = "std")]
mod std_io {
    pub use std::io::Error;
    pub use std::io::ErrorKind;
    pub use std::io::Read;
    pub use std::io::Seek;
    pub use std::io::SeekFrom;
    pub use std::io::Write;
}

#[cfg(not(feature = "std"))]
mod nonstd_io {
    use crate::VfatRsError;
    use core::cmp;
    use core::fmt;
    use core::mem;
    use snafu::Snafu;

    type Result<T> = core::result::Result<T, Error>;

    pub trait Read {
        fn read(&mut self, buf: &mut [u8]) -> crate::Result<usize>;
    }
    /// A trait for objects which are byte-oriented sinks.
    pub trait Write {
        /// Write a buffer into this writer, returning how many bytes were written.
        fn write(&mut self, buf: &[u8]) -> Result<usize>;

        /// Flush this output stream, ensuring that all intermediately buffered
        /// contents reach their destination.
        fn flush(&mut self) -> Result<()>;

        /// Attempts to write an entire buffer into this writer.
        fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
            while !buf.is_empty() {
                match self.write(buf) {
                    Ok(0) => {
                        return Err(Error::new(
                            ErrorKind::WriteZero,
                            "failed to write whole buffer",
                        ));
                    }
                    Ok(n) => buf = &buf[n..],
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        }

        /// Writes a formatted string into this writer, returning any error
        /// encountered.
        fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<()> {
            // Create a shim which translates a Write to a fmt::Write and saves
            // off I/O errors. instead of discarding them
            struct Adaptor<'a, T: ?Sized> {
                inner: &'a mut T,
                error: Result<()>,
            }

            impl<T: Write + ?Sized> fmt::Write for Adaptor<'_, T> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    match self.inner.write_all(s.as_bytes()) {
                        Ok(()) => Ok(()),
                        Err(e) => {
                            self.error = Err(e);
                            Err(fmt::Error)
                        }
                    }
                }
            }

            let mut output = Adaptor {
                inner: self,
                error: Ok(()),
            };
            match fmt::write(&mut output, fmt) {
                Ok(()) => Ok(()),
                Err(..) => {
                    // check if the error came from the underlying `Write` or not
                    if output.error.is_err() {
                        output.error
                    } else {
                        Err(Error::new(ErrorKind::Other, "formatter error"))
                    }
                }
            }
        }

        /// Creates a "by reference" adaptor for this instance of `Write`.
        fn by_ref(&mut self) -> &mut Self
        where
            Self: Sized,
        {
            self
        }
    }

    impl Write for &mut [u8] {
        #[inline]
        fn write(&mut self, data: &[u8]) -> Result<usize> {
            let amt = cmp::min(data.len(), self.len());
            let (a, b) = mem::take(self).split_at_mut(amt);
            a.copy_from_slice(&data[..amt]);
            *self = b;
            Ok(amt)
        }

        #[inline]
        fn write_all(&mut self, data: &[u8]) -> Result<()> {
            if self.write(data)? == data.len() {
                Ok(())
            } else {
                Err(Error::new(
                    ErrorKind::WriteZero,
                    &"failed to write whole buffer",
                ))
            }
        }

        #[inline]
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    /// Enumeration of possible methods to seek within an I/O object.
    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)]
    pub enum SeekFrom {
        /// Sets the offset to the provided number of bytes.
        Start(u64),
        /// Sets the offset to the size of this object plus the specified number of
        /// bytes.
        End(i64),
        /// Sets the offset to the current position plus the specified number of
        /// bytes.
        Current(i64),
    }

    /// The `Seek` trait provides a cursor which can be moved within a stream of
    /// bytes.
    pub trait Seek {
        /// Seek to an offset, in bytes, in a stream.
        fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
        /// Returns the current seek position from the start of the stream.
        ///
        /// This is equivalent to `self.seek(SeekFrom::Current(0))`.
        fn stream_position(&mut self) -> Result<u64> {
            self.seek(SeekFrom::Current(0))
        }
    }

    impl<S: Seek + ?Sized> Seek for &mut S {
        #[inline]
        fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
            (**self).seek(pos)
        }
    }

    #[derive(Snafu)]
    #[snafu(visibility(pub(crate)))]
    /// The error type for I/O operations of the [`Read`], [`Write`], [`Seek`], and
    /// associated traits.
    ///
    /// [`Read`]: super::Read
    /// [`Write`]: super::Write
    /// [`Seek`]: super::Seek
    pub struct Error {
        repr: Repr,
    }
    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.repr, f)
        }
    }

    #[derive(Debug)]
    pub enum Repr {
        Simple(ErrorKind),
    }

    /// A list specifying general categories of I/O error.
    #[non_exhaustive]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[allow(dead_code)]
    pub enum ErrorKind {
        /// An entity was not found, often a file.
        NotFound,
        /// The operation lacked the necessary privileges to complete.
        PermissionDenied,
        /// The connection was refused by the remote server.
        ConnectionRefused,
        /// The connection was reset by the remote server.
        ConnectionReset,
        /// The connection was aborted (terminated) by the remote server.
        ConnectionAborted,
        /// The network operation failed because it was not connected yet.
        NotConnected,
        /// A socket address could not be bound because the address is already in
        /// use elsewhere.
        AddrInUse,
        /// A nonexistent interface was requested or the requested address was not
        /// local.
        AddrNotAvailable,
        /// The operation failed because a pipe was closed.
        BrokenPipe,
        /// An entity already exists, often a file.
        AlreadyExists,
        /// The operation needs to block to complete, but the blocking operation was
        /// requested to not occur.
        WouldBlock,
        /// A parameter was incorrect.
        InvalidInput,
        /// Data not valid for the operation were encountered.
        InvalidData,
        /// The I/O operation's timeout expired, causing it to be canceled.
        TimedOut,
        /// An error returned when an operation could not be completed because a
        /// call to [`write`] returned [`Ok(0)`].
        WriteZero,
        /// This operation was interrupted.
        Interrupted,
        /// Any I/O error not part of this list.
        Other,
        /// An error returned when an operation could not be completed because an
        /// "end of file" was reached prematurely.
        UnexpectedEof,
    }
    impl Error {
        /// Creates a new I/O error from a known kind of error as well as an
        /// arbitrary error payload.
        #[must_use]
        pub fn new<A>(kind: ErrorKind, _: A) -> Self {
            Self {
                repr: Repr::Simple(kind),
            }
        }

        /// Returns the corresponding [`ErrorKind`] for this error.
        #[must_use]
        pub fn kind(&self) -> ErrorKind {
            match self.repr {
                Repr::Simple(kind) => kind,
            }
        }
    }
    impl From<ErrorKind> for Error {
        fn from(kind: ErrorKind) -> Self {
            Self {
                repr: Repr::Simple(kind),
            }
        }
    }
    impl From<VfatRsError> for Error {
        fn from(err: VfatRsError) -> Self {
            Self::new(ErrorKind::Other, err)
        }
    }
}
