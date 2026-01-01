#![allow(unused)]

use std::io::Read;

pub(crate) struct Reader<'a> {
    data: &'a str,
    position: usize,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        Self { data, position: 0 }
    }

    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn position(&self) -> usize {
        self.position
    }

    pub(crate) fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    pub(crate) fn advance(&mut self, n: usize) -> std::io::Result<()> {
        self.position = self
            .position
            .checked_add(n)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
        Ok(())
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.position >= self.data.len()
    }

    pub(crate) fn peek(&self) -> Option<u8> {
        self.data.as_bytes().get(self.position).copied()
    }

    pub(crate) fn consume_whitespace(&mut self) -> std::io::Result<()> {
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
        let remaining = self
            .data
            .len()
            .checked_sub(self.position)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
        let to_read = buf.len().min(remaining);
        let to_position = self
            .position
            .checked_add(to_read)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;

        let to_slice = buf
            .get_mut(..to_read)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
        let from_slice = &self
            .data
            .as_bytes()
            .get(self.position..to_position)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
        to_slice.copy_from_slice(from_slice);

        self.position = to_position;
        Ok(to_read)
    }
}
