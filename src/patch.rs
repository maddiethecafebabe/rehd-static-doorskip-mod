use std::io::{self, Cursor};

#[derive(Debug)]
pub enum PatchError {
    Io(io::Error),
    NoMatchFound,
}

#[derive(Debug, Clone, Copy)]
pub struct Patch {
    name: &'static str,
    orig: &'static [u8],
    modded: &'static [u8]
}

impl Patch {
    pub const fn new(name: &'static str, orig: &'static [u8], modded: &'static [u8]) -> Self {
        if orig.len() != modded.len() {
            panic!()
        }

        Self { name, orig, modded }
    }

    pub fn apply(&self, stream: &mut [u8]) -> Result<(), PatchError> {
        let patch_size = self.orig.len();

        for i in 0..(stream.len() - patch_size) {
            let cur_needle = &stream[i..(i + patch_size)];

            if cur_needle == self.orig {
                println!("found match for {} @ 0x{:x}", self.name, i);
                
                io::copy(&mut Cursor::new(self.modded), &mut &mut stream[i..(i + patch_size)]).unwrap();

                println!("applied!");
                return Ok(())
            }

            if cur_needle == self.modded {
                println!("found already modded bytes, this could be a false positive (hence just ignored) but is this executable maybe already patched?");
            }
        }

        Err(PatchError::NoMatchFound)
    }

    pub fn revert(&self, stream: &mut [u8]) -> Result<(), PatchError> {
        let this = Self { name: "reverting-patch", orig: self.modded, modded: self.orig };
        this.apply(stream)
    }
}
