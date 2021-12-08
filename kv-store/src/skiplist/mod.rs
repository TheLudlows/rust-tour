mod arena;
mod list;
mod key;
mod inline_list;

pub use arena::*;
pub use key::*;

const MAX_HEIGHT: usize = 20;
const HEIGHT_INCREASE: u32 = u32::MAX / 3;

pub trait Allocator {
    fn alloc(&self, alain: usize, size: usize) -> u32;
    fn len(&self) -> u32;
    fn capacity(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::sync::atomic::AtomicPtr;
    use bytes::Bytes;

    use crate::skiplist::{FixedLengthSuffixComparator, inline_list::InlineSkipList};

    use super::{list::Skiplist};

    #[test]
    fn test_find_near() {
        let comp = FixedLengthSuffixComparator::new(8);
        let list = Skiplist::with_capacity(comp, 1 << 20);
        for i in 0..1000 {
            let key = Bytes::from(format!("{:05}{:08}", i * 10 + 5, 0));
            let value = Bytes::from(format!("{:05}", i));
            list.put(key, value);
        }
        let mut cases = vec![
            ("00001", false, false, Some("00005")),
            ("00001", false, true, Some("00005")),
            ("00001", true, false, None),
            ("00001", true, true, None),
            ("00005", false, false, Some("00015")),
            ("00005", false, true, Some("00005")),
            ("00005", true, false, None),
            ("00005", true, true, Some("00005")),
            ("05555", false, false, Some("05565")),
            ("05555", false, true, Some("05555")),
            ("05555", true, false, Some("05545")),
            ("05555", true, true, Some("05555")),
            ("05558", false, false, Some("05565")),
            ("05558", false, true, Some("05565")),
            ("05558", true, false, Some("05555")),
            ("05558", true, true, Some("05555")),
            ("09995", false, false, None),
            ("09995", false, true, Some("09995")),
            ("09995", true, false, Some("09985")),
            ("09995", true, true, Some("09995")),
            ("59995", false, false, None),
            ("59995", false, true, None),
            ("59995", true, false, Some("09995")),
            ("59995", true, true, Some("09995")),
        ];
        for (i, (key, less, allow_equal, exp)) in cases.drain(..).enumerate() {
            let seek_key = Bytes::from(format!("{}{:08}", key, 0));
            let res = unsafe { list.find_near(&seek_key, less, allow_equal) };
            if exp.is_none() {
                assert!(res.is_null(), "{}", i);
                continue;
            }
            let e = format!("{}{:08}", exp.unwrap(), 0);
            assert_eq!(&unsafe { &*res }.key, e.as_bytes(), "{}", i);
        }
    }

    #[test]
    fn test_skl() {
        let comp = FixedLengthSuffixComparator::new(1);
        println!("start");
        let skl = Skiplist::with_capacity(comp, 1024);
        println!("new");
        let r = skl.put("a", "a");
        println!("put");
        println!("{:?}", r);
        assert_eq!(skl.get(&Bytes::from("a")), Some(&Bytes::from("a")));
    }

    #[test]
    fn test_inline_skl() {
        let comp = FixedLengthSuffixComparator::new(1);
        println!("start");
        let skl = InlineSkipList::with_capacity(comp, 1024);
        println!("new");
        skl.put(Bytes::from("a"), Bytes::from("a"));
        println!("put");
        assert_eq!(skl.get(&Bytes::from("a")), Some(&Bytes::from("a")));
    }
    #[test]
    fn test_bytes() {
        println!("{}", mem::size_of::<Bytes>());
        println!("{}", mem::size_of::<AtomicPtr<u8>>());
    }
}