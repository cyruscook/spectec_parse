#![allow(unused)]

use std::io::Read;

pub struct Reader<'a> {
    data: &'a str,
    position: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data, position: 0 }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    pub fn advance(&mut self, n: usize) -> std::io::Result<()> {
        self.position = self
            .position
            .checked_add(n)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
        Ok(())
    }

    pub fn is_eof(&self) -> bool {
        self.position >= self.data.len()
    }

    pub fn peek(&self) -> Option<u8> {
        self.data.as_bytes().get(self.position).copied()
    }

    pub fn consume_whitespace(&mut self) -> std::io::Result<()> {
        loop {
            if let Some(c) = self.peek()
                && c.is_ascii_whitespace()
            {
                self.position = self
                    .position
                    .checked_add(1)
                    .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
            } else {
                break;
            }
        }
        Ok(())
    }
}

impl Read for Reader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = self.data.len() - self.position;
        let to_read = buf.len().min(remaining);
        let to_position = self.position + to_read;

        let to_slice = buf
            .get_mut(..to_read)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
        let from_slice = &self
            .data
            .as_bytes()
            .get(self.position..to_position)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
        to_slice.copy_from_slice(from_slice);

        self.position += to_read;
        Ok(to_read)
    }
}
