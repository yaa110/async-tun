use std::convert::TryFrom;

#[derive(Clone, Copy)]
pub struct EthernetAddr([u8; 6]);

impl std::fmt::Debug for EthernetAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EthernetAddr {{ {} }}", self)
    }
}

impl std::fmt::Display for EthernetAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{:02x}", v)?;
            } else {
                write!(f, ":{:02x}", v)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum EthernetAddrError {
    WrongLength(usize),
}

impl std::fmt::Display for EthernetAddrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongLength(length) => write!(f, "Address too short: {}", length),
        }
    }
}

impl std::error::Error for EthernetAddrError {}

impl From<[u8; 6]> for EthernetAddr {
    fn from(addr: [u8; 6]) -> Self {
        Self(addr)
    }
}

impl TryFrom<&[u8]> for EthernetAddr {
    type Error = EthernetAddrError;

    fn try_from(addr: &[u8]) -> Result<Self, Self::Error> {
        if addr.len() != 6 {
            return Err(EthernetAddrError::WrongLength(addr.len()));
        }

        let mut new_addr: [u8; 6] = [0; 6];
        new_addr.copy_from_slice(addr);
        Ok(Self(new_addr))
    }
}

impl AsRef<[u8]> for EthernetAddr {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl EthernetAddr {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
