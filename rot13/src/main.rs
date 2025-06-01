use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
impl<R: Read> Read for RotDecoder<R>{
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        let len = self.input.read(buffer)?;

        for c in &mut buffer[0..len]{
            if c.is_ascii_lowercase(){
                *c += self.rot;
                if !c.is_ascii_lowercase(){
                    *c = 'a' as u8 + (*c - 'a' as u8) % 26;
                }
            }

            if c.is_ascii_uppercase(){
                *c += self.rot;
                if !c.is_ascii_uppercase(){
                    *c = 'A' as u8 + (*c - 'A' as u8) % 26;
                }
            }
        }

        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_slice(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}