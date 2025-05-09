use anyhow::{Context, Result};
use std::{
    io::{self, Read, Write},
    os::unix::io::{AsRawFd, RawFd},
};
use nix::{
    pty::{openpty, Winsize},
    unistd::{close, read, write},
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// A pair of master and slave PTY file descriptors
#[cfg(unix)]
pub struct PtyPair {
    pub master: PtyMaster,
    pub slave: PtySlave,
}

#[cfg(unix)]
pub struct PtyMaster {
    fd: RawFd,
}

#[cfg(unix)]
pub struct PtySlave {
    fd: RawFd,
}

#[cfg(unix)]
impl PtyPair {
    pub fn new() -> Result<Self> {
        // Default terminal size
        let ws = Winsize {
            ws_row: 24,
            ws_col: 80,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let pty = openpty(&ws, None).context("Failed to open PTY")?;

        Ok(Self {
            master: PtyMaster { fd: pty.master },
            slave: PtySlave { fd: pty.slave },
        })
    }

    /// Create a new PTY pair with specified dimensions
    pub fn with_size(row: u16, cols: u16) -> Result<Self> {
        let ws = Winsize {
            ws_row: row,
            ws_col: cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let pty = openpty(&ws, None).context("Failed to open PTY")?;

        Ok(Self {
            master: PtyMaster { fd: pty.master },
            slave: PtySlave { fd: pty.slave },
        })
    }
}

#[cfg(unix)]
impl PtyMaster {
    /// Resize the PTY
    pub fn resize(&self, rows: u16, cols: u16) -> Result<()> {
        let ws = Winsize {
            ws_row: rows,
            ws_col: cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let res = unsafe { libc::ioctl(self.fd, libc::TIOCSWINSZ, &ws) };

        if res < 0 {
            return Err(std::io::Error::last_os_error()).context("Failed to resize PTY");
        }

        Ok(())
    }
}

#[cfg(unix)]
impl AsRawFd for PtyMaster {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[cfg(unix)]
impl AsRawFd for PtySlave {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[cfg(unix)]
impl Read for PtyMaster {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match read(self.fd, buf) {
            Ok(n) => Ok(n),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}

#[cfg(unix)]
impl Write for PtyMaster {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match write(self.fd, buf) {
            Ok(n) => Ok(n),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // PTY doesn't buffer, so this is a no-op
        Ok(())
    }
}

#[cfg(unix)]
impl Drop for PtyMaster {
    fn drop(&mut self) {
        let _ = close(self.fd);
    }
}

#[cfg(unix)]
impl Drop for PtySlave {
    fn drop(&mut self) {
        let _ = close(self.fd);
    }
}

#[cfg(unix)]
impl AsyncRead for PtyMaster {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match read(self.fd, buf.initialize_unfilled()) {
            Ok(n) => {
                buf.advance(n);
                std::task::Poll::Ready(Ok(()))
            }
            Err(nix::errno::Errno::EAGAIN) => std::task::Poll::Pending,
            Err(e) => {
                std::task::Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, e)))
            }
        }
    }
}

#[cfg(unix)]
impl AsyncWrite for PtyMaster {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        match write(self.fd, buf) {
            Ok(n) => std::task::Poll::Ready(Ok(n)),
            Err(nix::errno::Errno::EAGAIN) => std::task::Poll::Pending,
            Err(e) => {
                std::task::Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, e)))
            }
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        // Closing is handled by drop
        std::task::Poll::Ready(Ok(()))
    }
}

#[cfg(windows)]
pub struct PtyPair {
    // Windows-specific fields
}

#[cfg(windows)]
impl PtyPair {
    pub fn new() -> Result<Self> {
        unimplemented!("Windows PTY support not implemented yet");
    }
}
