//! Provides information about the standard output streams (standard output and standard error).

#![warn(missing_docs, clippy::missing_docs_in_private_items)]

use std::{
    io::{Stderr, Stdout},
    marker::PhantomData,
    sync::atomic::{AtomicU16, Ordering},
};

use terminal_size::terminal_size_of;

/// Default number of columns in a line.
///
/// This value is returned by [`OutputInfo::get_line_width`] if the stream does not refer to a
/// terminal, or the terminal width cannot be determined.
pub const DEFAULT_LINE_WIDTH: u16 = 80;

/// Information about standard output.
pub static STDOUT_INFO: OutputInfo<Stdout> = OutputInfo {
    phantom: PhantomData,
};

/// Information about standard error.
pub static STDERR_INFO: OutputInfo<Stderr> = OutputInfo {
    phantom: PhantomData,
};

/// Information about a standard output streams (standard output or standard error).
pub struct OutputInfo<S: private::StdStream> {
    /// Phantom data to mark type `S` as used.
    phantom: PhantomData<S>,
}

impl<S: private::StdStream> OutputInfo<S> {
    /// Returns the number of columns in a line.
    ///
    /// If the stream refers to a terminal, the terminal width is returned. If the stream does not
    /// refer to a terminal, or the terminal width cannot be determined, [`DEFAULT_LINE_WIDTH`] is
    /// returned.
    pub fn get_line_width() -> u16 {
        static LINE_WIDTH: AtomicU16 = AtomicU16::new(0);
        let mut line_width = LINE_WIDTH.load(Ordering::Relaxed);
        if line_width == 0 {
            line_width = if let Some((width, _)) = terminal_size_of(S::get_stream()) {
                if width.0 != 0 {
                    width.0
                } else {
                    DEFAULT_LINE_WIDTH
                }
            } else {
                DEFAULT_LINE_WIDTH
            };
            LINE_WIDTH.store(line_width, Ordering::Relaxed);
        }
        line_width
    }
}

/// Private module containing implementation details for [this crate](crate).
mod private {
    use std::{
        io::{self, Stderr, Stdout},
        os::fd::AsFd,
    };

    /// Provides access to a standard stream instance.
    pub trait StdStream {
        /// The standard stream type.
        type Stream: AsFd;

        /// Returns the standard stream instance.
        #[must_use]
        fn get_stream() -> Self::Stream;
    }

    impl StdStream for Stdout {
        type Stream = Stdout;

        #[inline]
        fn get_stream() -> Stdout {
            io::stdout()
        }
    }

    impl StdStream for Stderr {
        type Stream = Stderr;

        #[inline]
        fn get_stream() -> Stderr {
            io::stderr()
        }
    }
}
