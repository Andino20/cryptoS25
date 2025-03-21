#[derive(Clone)]
pub struct Bits<I: Iterator<Item = u8>> {
    iter: I,
    current_bit: usize,
    byte: u8,
}

impl<I: Iterator<Item = u8>> Bits<I> {
    fn new(iter: I) -> Self {
        Bits {
            iter,
            current_bit: 0,
            byte: 0,
        }
    }
}

impl<I> Iterator for Bits<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_bit == 0 {
            self.byte = self.iter.next()?;
        }
        let result = (self.byte >> self.current_bit) & 0x01;
        self.current_bit = (self.current_bit + 1) % 8;
        Some(result)
    }
}

pub trait BitsIterator: Iterator<Item = u8> + Sized {
    fn bits(self) -> Bits<Self> {
        Bits::new(self)
    }
}

impl<T: Iterator<Item = u8>> BitsIterator for T {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bits() {
        let data: [u8; 2] = [0xAA, 0x55];
        let mut iter = data.iter().cloned().bits();

        assert_eq!(iter.next(), Some(0x00));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), Some(0x00));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), Some(0x00));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), Some(0x00));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), Some(0x00));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), Some(0x00));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), Some(0x00));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), Some(0x00));
        assert_eq!(iter.next(), None);
    }
}
