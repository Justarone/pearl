use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct KeyType(Vec<u8>);

impl Key for KeyType {
    const LEN: u16 = 8;
}

impl From<Vec<u8>> for KeyType {
    fn from(mut v: Vec<u8>) -> Self {
        v.resize(KeyType::LEN as usize, 0);
        Self(v)
    }
}

impl AsRef<[u8]> for KeyType {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Default for KeyType {
    fn default() -> Self {
        Self(vec![0_u8; Self::LEN as usize])
    }
}

impl From<usize> for KeyType {
    fn from(i: usize) -> Self {
        let mut v = serialize(&i).unwrap();
        v.resize(KeyType::LEN as usize, 0);
        Self(v)
    }
}

impl Into<usize> for KeyType {
    fn into(self) -> usize {
        deserialize(&self.0).unwrap()
    }
}
