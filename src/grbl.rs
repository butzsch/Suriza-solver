//! Communication with a GRBL controller over a USB port

extern crate serialport;

use crate::{
    pyo3::prelude::*,
    std::{io, io::prelude::*, string::FromUtf8Error, time::Duration},
};

const BUFFER_SIZE: usize = 8 * 1024;

#[pyclass]
pub struct GRBL {
    port: Box<dyn serialport::SerialPort>,
    buffer: [u8; BUFFER_SIZE],
    pos: usize,
}

#[pymethods]
impl GRBL {
    #[new]
    fn __new__(obj: &PyRawObject, port: String) -> PyResult<()> {
        let mut handler = Self::try_connect(&port)?;
        handler.exchange_init_sequence()?;

        obj.init(move |_| handler)
    }

    /// Moves the pen to the specified position.
    fn move_to(&mut self, x: f64, y: f64) -> PyResult<()> {
        self.move_to_using("G1", x, y).map_err(Into::into)
    }

    /// Move the pen to the specified position at an increased rate.
    fn seek_to(&mut self, x: f64, y: f64) -> PyResult<()> {
        self.move_to_using("G0", x, y).map_err(Into::into)
    }

    /// Lowers the pen on the z-axis, preparing it for drawing a line.
    fn lower_pen(&mut self) -> PyResult<()> {
        self.exchange("M5\n").map_err(Into::into)
    }

    /// Raises the pen on the z-axis so it can be moved across the plane without
    /// leaving a line.
    fn raise_pen(&mut self) -> PyResult<()> {
        self.exchange("M3S30\n").map_err(Into::into)
    }
}

impl GRBL {
    /// Tries to open the connection using the given `port`.
    fn try_connect(port: &str) -> io::Result<Self> {
        let mut port = serialport::open(port)?;

        // Value taken from the official GRBL documentation.
        port.set_baud_rate(115_200)?;

        // The GRBL will not respond until the command is executed, which can
        // take a while, for example when it is moving to the home position.
        port.set_timeout(Duration::from_secs(30))?;

        Ok(Self {
            port,
            buffer: [0; BUFFER_SIZE],
            pos: 0,
        })
    }

    /// Sends initialization messages and sets some basic settings required when
    /// connecting to a GRBL.
    fn exchange_init_sequence(&mut self) -> io::Result<()> {
        // Ignore greeting messages
        loop {
            let line = self.read()?;
            if line == "['$H'|'$X' to unlock]\r\n" {
                break;
            }
        }

        // The GRBL always starts out in a "locked" state, we can unlock it by
        // homing.
        self.exchange("$H\n")?;

        // Set unit to mm
        self.exchange("G21\n")?;

        // Set distance mode to absolute
        self.exchange("G90\n")?;

        // Set the feedrate
        self.exchange("F2000\n")?;

        Ok(())
    }

    fn move_to_using(&mut self, command: &str, x: f64, y: f64) -> PyResult<()> {
        let gcode = format!("{} X{} Y{}\n", command, x, y);

        self.exchange(&gcode).map_err(Into::into)
    }

    /// Reads a single newline-delimited message from the given `port`. Returns
    /// an `io::Error` if the underlying read operation failed, which may be
    /// caused by a timeout on the port.
    fn read(&mut self) -> io::Result<String> {
        let mut start = 0;
        loop {
            if let Some(line) = self.get_line_from_buffer(start) {
                return line.map_err(|error| {
                    io::Error::new(io::ErrorKind::InvalidData, error)
                });
            }

            start = self.pos;
            self.pos += self.port.read(&mut self.buffer[self.pos..])?;
        }
    }

    /// Inspects the internal buffer starting at `offset`. It stops at the first
    /// newline character it finds, removing all characters leading up to it
    /// from the buffer and returning them. Returns `None` if no newline
    /// character was found.
    fn get_line_from_buffer(
        &mut self,
        start: usize,
    ) -> Option<Result<String, FromUtf8Error>> {
        let found = self.buffer[start..self.pos]
            .iter()
            .position(|&byte| byte == b'\n');

        found.map(|found| {
            let line_length = start + found;
            let (left, right) = self.buffer.split_at_mut(line_length + 1);
            let line = String::from_utf8(left.to_vec())?;

            left.copy_from_slice(&right[..=line_length]);
            self.pos -= line_length;

            Ok(line)
        })
    }

    /// Writes a single newline-delimited message to the given `port`. Returns
    /// an `io::Error` if the underlying write operation failed.
    fn write(&mut self, message: &str) -> io::Result<()> {
        assert!(message.ends_with('\n'));

        self.port.write_all(&message.as_bytes())
    }

    /// Writes a single newline-delimited message to the given `port` and waits
    /// for it to be acknowledged with an 'ok' response. Returns a `io::Error`
    /// if either the read or write threw an error.
    fn exchange(&mut self, message: &str) -> io::Result<()> {
        self.write(message)?;

        loop {
            let line = self.read()?;
            if line.ends_with("ok\r\n") {
                return Ok(());
            }
        }
    }
}
