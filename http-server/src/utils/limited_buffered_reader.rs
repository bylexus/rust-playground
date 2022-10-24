use std::{
    io::{BufReader, Read, Result, Error, ErrorKind},
    net::TcpStream,
};

pub trait BufReaderExt: Read {
    fn read_max_until(&mut self, byte: u8, max_bytes: usize) -> Result<Vec<u8>>;
}

impl BufReaderExt for BufReader<&TcpStream> {
    fn read_max_until(&mut self, byte: u8, max_bytes: usize) -> Result<Vec<u8>> {
        let mut remaining_bytes = max_bytes;
        let mut byte_buf = [0u8];
        let mut res_buf = Vec::new();

        // because we read from a buffered reader, we can read byte-by-byte,
        // without too much performance loss (I guess...): This allows to
        // check for the final byte boundary.
        while remaining_bytes > 0 {
            if self.read(&mut byte_buf)? == 1 {
                remaining_bytes -= 1;
                res_buf.push(byte_buf[0]);
                if byte_buf[0] == byte {
                    return Ok(res_buf);
                }
            }
        }

        Err(Error::new(ErrorKind::UnexpectedEof, "Input too long"))
    }
}

// pub struct LimitedBufferedReader<'a> {
//     stream: &'a mut BufReader<&'a TcpStream>,
//     buffer: Box<[u8]>,
//     buffer_preread: Vec<u8>,
// }

// impl<'a> LimitedBufferedReader<'a> {
//     pub fn new(stream: &'a mut BufReader<&'a TcpStream>) -> LimitedBufferedReader<'a> {
//         let reader = LimitedBufferedReader {
//             stream: stream,
//             buffer: Box::new([0; 1024]),
//             buffer_preread: Vec::new(),
//         };

//         reader
//     }

//     pub fn read_max_until(
//         &mut self,
//         byte: u8,
//         mut max_bytes: usize,
//         buffer: &mut Vec<u8>,
//     ) -> Result<usize> {
//         let mut read_bytes = 0;

//         while max_bytes > 0 {
//             // do we have some pre-read buffer content? process it first:
//             if self.buffer_preread.len() > 0 {
//                 let (found, returned_bytes, new_preread_buf) = self.process_read_max_until_buffer(
//                     byte,
//                     max_bytes,
//                     buffer,
//                     &self.buffer_preread,
//                 )?;
//                 self.buffer_preread = new_preread_buf;
//                 max_bytes -= returned_bytes;
// 				read_bytes += returned_bytes
//                 if found {
//                     return Ok(read_bytes);
//                 }
//             }
//             // now the preread-buffer must be empty, else we did something wrong:
//             assert!(self.buffer_preread.len() == 0);

//             let read_bytes = self.stream.read(&mut self.buffer)?;
//             if read_bytes > 0 {
//                 let (found, returned_bytes, new_preread_buf) = self.process_read_max_until_buffer(
//                     byte,
//                     max_bytes,
//                     buffer,
//                     &self.buffer.to_vec()
//                 )?;
//                 self.buffer_preread = new_preread_buf;
//                 max_bytes -= returned_bytes;
// 				read_bytes += returned_bytes;
//                 if found {
//                     return Ok(read_bytes);
//                 }
//             } else {
// 				return Ok(read_bytes);
// 			}
//         }

//         Ok(read_bytes)
//     }

//     fn process_read_max_until_buffer(
//         &self,
//         byte: u8,
//         max_bytes: usize,
//         buffer: &mut Vec<u8>,
//         read_buffer: &Vec<u8>,
//     ) -> Result<(bool, usize, Vec<u8>)> {
//         let mut append_to_index = 0;
//         let mut found = false;
//         let mut read_bytes: usize = 0;
//         if let Some(idx) = read_buffer.iter().position(|el| *el == byte) {
//             // found stop char:
//             found = true;
//             append_to_index = idx;
//         } else {
//             append_to_index = read_buffer.len() - 1;
//         }

//         if append_to_index < max_bytes {
//             buffer.extend_from_slice(&read_buffer[0..=append_to_index]);
//             read_bytes = append_to_index + 1;
//             let buffer_preread = match append_to_index < read_buffer.len() - 1 {
//                 true => Vec::from(&read_buffer[(append_to_index + 1)..]),
//                 false => Vec::new(),
//             };
//             return Ok((found, read_bytes, buffer_preread));
//         } else {
//             return Err(Error::new(ErrorKind::Other, "Input too long"));
//         }
//     }
// }
