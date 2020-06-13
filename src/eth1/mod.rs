use std::fmt;

pub use self::rlp::*;

pub mod rlp;

struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
        where T: ?Sized + AsRef<[u8]> + 'a {
        HexSlice(data.as_ref())
    }

    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for HexSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

trait HexDisplayExt {
    fn hex_display(&self) -> HexSlice<'_>;
}

impl<T> HexDisplayExt for T
    where T: ?Sized + AsRef<[u8]> {
    fn hex_display(&self) -> HexSlice<'_> {
        HexSlice::new(self)
    }
}


