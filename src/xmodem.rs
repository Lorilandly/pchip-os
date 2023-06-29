use crate::uart;
use alloc::{boxed::Box, vec, vec::Vec};
use core::fmt;
use crc::{Crc, CRC_16_XMODEM};

const SOH: u8 = 0x01;
const STX: u8 = 0x02;
const EOT: u8 = 0x04;
const ACK: u8 = 0x06;
const NAK: u8 = 0x15;
const CAN: u8 = 0x18;
const CPMEOF: u8 = 0x1A;
const CRC: u8 = 0x43;

const CRC16: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);

/// Configuration for the XMODEM transfer.
#[derive(Copy, Clone, Debug)]
pub struct Xmodem {
    /// The number of errors that can occur before the communication is
    /// considered a failure. Errors include unexpected bytes and timeouts waiting for bytes.
    pub max_errors: u32,

    /// The number of errors that can occur before the communication is
    /// considered a failure. Errors include unexpected bytes and timeouts waiting for bytes.
    ///
    /// This only applies to the initial packet
    pub max_initial_errors: u32,

    /// The byte used to pad the last block. XMODEM can only send blocks of a certain size,
    /// so if the message is not a multiple of that size the last block needs to be padded.
    pub pad_byte: u8,
}

impl Xmodem {
    /// Creates the XMODEM config with default parameters.
    pub fn new() -> Self {
        Xmodem {
            max_errors: 16,
            max_initial_errors: 16,
            pad_byte: 0x1a,
        }
    }

    // TODO:
    // Subroutines
    // The character-receive subroutine:
    //      called with a parameter
    //      specifying the number of seconds to wait. The receiver should first
    //      call it with a time of 10, then <nak> and try again, 10 times.
    //
    // 1st bit subroutine:
    //
    // packet receive subroutine:
    //      Arg: packet size
    //      Ret: packet
    //
    // "PURGE" subroutine:
    //      before calling <nak>

    /// Receive an XMODEM transmission.
    ///
    /// `dev` should be the serial communication channel (e.g. the serial device).
    /// The received data will be returned as `Vec<u8>`.
    /// The CRC mode is always used
    ///
    /// # Timeouts
    /// This method has no way of setting the timeout of `dev`, so it's up to the caller
    /// to set the timeout of the device before calling this method. Timeouts on receiving
    /// bytes will be counted against `max_errors`, but timeouts on transmitting bytes
    /// will be considered a fatal error.
    pub fn recv<D: uart::Read + fmt::Write>(&self, dev: &mut D) -> Result<Vec<u8>, Error> {
        let mut errors = 0;
        let mut initial_errors = 0;
        let mut file: Vec<u8> = vec![];
        let mut handled_first_packet = false;
        //dbg!("Starting XMODEM receive");

        let first_char;
        loop {
            (dev.write_char(CRC.into())?);

            match dev.read() {
                Ok(bt @ (SOH | STX)) => {
                    first_char = bt;
                    break;
                }
                // TODO
                //Err(uart::Error::Timeout) => {}
                _ => {
                    initial_errors += 1;
                    if initial_errors > self.max_initial_errors {
                        // eprint!(
                        // "Exhausted max retries ({}) while waiting for SOH or STX",
                        // self.max_initial_errors
                        // );
                        return Err(Error::ExhaustedRetries);
                    }
                }
            }
        }
        //dbg!("NCG sent. Receiving stream.");
        let mut packet_cnt: u8 = 0;
        loop {
            match if handled_first_packet {
                dev.read()
            } else {
                Ok(first_char)
            } {
                Ok(bt @ (SOH | STX)) => {
                    handled_first_packet = true;
                    // Handle next packet
                    let packet_size = match bt {
                        SOH => 128,
                        STX => 1024,
                        _ => 0, // Why does the compiler need this?
                    };
                    let pnum = dev.read()?; // specified packet number
                    let pnum255 = dev.read()? + pnum; // 1's complemented `pnum`. Sum must equal to 255
                    let data: Box<[u8]> = (0..packet_size)
                        .map(|_| dev.read())
                        .collect::<Result<_, _>>()?;

                    let chk_crc = {
                        let recv_checksum = ((dev.read()? as u16) << 8) | dev.read()? as u16;
                        CRC16.checksum(&data) == recv_checksum
                    };

                    // if pnum = packet_num: ignore
                    // if pnum = packet_num + 1: continue
                    // else: cancel transmission
                    if packet_cnt == pnum {
                        // ignore packet if `pnum` is repeated
                        dev.write_char(ACK.into())?;
                    } else if pnum255 != 255 {
                        // Respond with `CAN` if `pnum` is wrong
                        dev.write_char(CAN.into())?;
                        dev.write_char(CAN.into())?;
                        return Err(Error::Canceled);
                    } else if packet_cnt.wrapping_add(1) == pnum && chk_crc {
                        // Accept packet if `pnum` is correct and crc check passed
                        packet_cnt = packet_cnt.wrapping_add(1);
                        dev.write_char(ACK.into())?;
                        file.extend_from_slice(&data);
                    } else {
                        // Otherwise clear uart buffer and respond with `NAK`
                        loop {
                            match dev.read() {
                                Err(uart::Error::Timeout) => break,
                                _ => (),
                            }
                        }
                        dev.write_char(NAK.into())?;
                        errors += 1;
                    }
                }
                Ok(EOT) => {
                    // End of file, truncate the filler characters
                    dev.write_char(ACK.into())?;
                    if let Some(len) = file.iter().rposition(|x| *x != CPMEOF) {
                        file.truncate(len + 1);
                    }
                    break;
                }
                Ok(_) => {
                    // warn!("Unrecognized symbol!");
                }
                Err(_) => {
                    if !handled_first_packet {
                        errors = self.max_errors;
                    } else {
                        errors += 1;
                    }
                    // warn!("Timeout!")
                }
            }
            if errors >= self.max_errors {
                // eprint!(
                // "Exhausted max retries ({}) while waiting for ACK for EOT",
                // self.max_errors
                // );
                return Err(Error::ExhaustedRetries);
            }
        }
        Ok(file)
    }
}

#[derive(Debug)]
pub enum Error {
    Fmt(fmt::Error),

    ///
    Timeout,

    /// The number of communications errors exceeded `max_errors` in a single
    /// transmission.
    ExhaustedRetries,

    /// The transmission was canceled by the other end of the channel.
    Canceled,
}

impl From<fmt::Error> for Error {
    fn from(value: fmt::Error) -> Self {
        Self::Fmt(value)
    }
}

impl From<uart::Error> for Error {
    fn from(_value: uart::Error) -> Self {
        Self::Timeout
    }
}
