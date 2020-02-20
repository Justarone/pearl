use super::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Bloom {
    inner: BitVec,
    hashers: Vec<AHasher>,
}

fn false_positive_rate(k: f64, n: f64, m: f64) -> f64 {
    (1f64 - 1f64.exp().powf(-k * n / m)).powi(k as i32)
}

impl Bloom {
    pub fn new(elements: usize) -> Self {
        let elements = elements as f64;
        debug!("bloom filter for {} elements", elements);
        let max_bit_count = 4194304usize; // 1Mb
        debug!("max bit count: {}", max_bit_count);
        error!("@TODO config limits, hashers count");
        let k = 2usize; // hashers count
        let mut bits_count = (elements * k as f64 / 2f64.ln()) as usize;
        let init_bits_count = (elements * k as f64 / 2f64.ln()) as usize;
        let bits_step = 8196usize;
        let mut fpr; // false positive rate
        error!("@TODO configurable fpr threshold");
        loop {
            if bits_count >= max_bit_count {
                trace!("bits count EQ or GREATER max bit count");
                fpr = false_positive_rate(k as f64, elements, bits_count as f64);
                trace!("false positive: {:.6}", fpr,);
                bits_count = init_bits_count;
            } else {
                trace!("bits count LESSER max bit count");
                fpr = false_positive_rate(k as f64, elements, bits_count as f64);
                trace!("bloom false positive rate: {:.6}", fpr,);
                bits_count = max_bit_count.min(bits_step + bits_count);
                trace!("increased bits count to: {}", bits_count);
            }
            if fpr < 0.001 {
                break;
            }
        }
        debug!(
            "result fpr: {:.6}, k: {}, m: {}, n: {}",
            false_positive_rate(k as f64, elements, bits_count as f64),
            k,
            bits_count,
            elements
        );
        Self {
            inner: bitvec![0; bits_count as usize],
            hashers: Self::hashers(k),
        }
    }

    pub fn hashers(k: usize) -> Vec<AHasher> {
        error!("@TODO create configurable hashers");
        (0..k)
            .map(|i| {
                AHasher::new_with_keys((i + 1).try_into().unwrap(), (i + 2).try_into().unwrap())
            })
            .collect()
    }

    pub fn from_raw(buf: &[u8], bits: usize) -> Self {
        let hashers = Self::hashers(2);
        debug!("deserialize filter from buf, len = {}", buf.len());
        let buf = bincode::deserialize(buf).unwrap();
        let mut bit_vec = BitVec::from_vec(buf);
        bit_vec.truncate(bits);
        Self {
            inner: bit_vec,
            hashers,
        }
    }

    pub fn add(&mut self, item: impl AsRef<[u8]>) {
        let mut hashers = self.hashers.clone();
        let len = self.inner.len() as u64;
        for h in hashers.iter_mut().map(|hasher| {
            hasher.write(item.as_ref());
            trace!("hasher: {:?}", hasher);
            hasher.finish() % len
        }) {
            *self
                .inner
                .get_mut(h as usize)
                .expect("impossible due to mod by len") = true;
            trace!("set true to {}", h);
        }
        trace!("filter add: {:#?}", self.inner);
    }

    pub fn contains(&self, item: impl AsRef<[u8]>) -> bool {
        trace!("filter: {:#?}", self.inner);
        let mut hashers = self.hashers.clone();
        let len = self.inner.len() as u64;
        let res = hashers
            .iter_mut()
            .map(|hasher| {
                hasher.write(item.as_ref());
                trace!("hasher: {:?}", hasher);
                hasher.finish() % len
            })
            .all(|i| *self.inner.get(i as usize).expect("unreachable"));
        trace!("item definitely missed: {}", !res);
        res
    }

    pub fn size(&self) -> u64 {
        bincode::serialized_size(self.inner.as_slice()).unwrap()
    }

    pub fn bits(&self) -> usize {
        self.inner.len()
    }

    pub fn as_slice(&self) -> &[usize] {
        self.inner.as_slice()
    }
}
