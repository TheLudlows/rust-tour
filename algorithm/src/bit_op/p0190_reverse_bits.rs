use crate::Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
       // private static final int M1 = 0x55555555; // 01010101010101010101010101010101
       // private static final int M2 = 0x33333333; // 00110011001100110011001100110011
        //private static final int M4 = 0x0f0f0f0f; // 00001111000011110000111100001111
        //private static final int M8 = 0x00ff00ff; // 00000000111111110000000011111111

        x = (x >> 16) + (x << 16);
        x = ((x & 0x00ff00ff) << 8) + ((x >> 8 )& 0x00ff00ff);
        x = ((x & 0x0f0f0f0f) << 4) +  ((x >> 4) & 0x0f0f0f0f );
        x = ((x & 0x33333333) << 2) +  ((x >> 2) & 0x33333333 );
        x = ((x & 0x55555555) << 1) +  ((x >> 1) & 0x55555555 );
        x
    }
}

#[test]
fn test() {
    Solution::reverse_bits(43261596);
}