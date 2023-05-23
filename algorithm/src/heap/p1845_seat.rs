use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

struct SeatManager {
    seat_pool:BinaryHeap<Reverse<i32>>,
    booked_seats:HashSet<i32>
}

impl SeatManager {

    fn new(n: i32) -> Self {
        let mut pool:BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for i in 0..n {
            pool.push(Reverse(i));
        }
        Self {
            seat_pool: pool,
            booked_seats: Default::default(),
        }

    }

    fn reserve(&mut self) -> i32 {
        let r = self.seat_pool.pop().map_or(-1, |e| e.0);
        if r != -1 {
            self.booked_seats.insert(r);
        }
        r
    }

    fn unreserve(&mut self, seat_number: i32) {
        match self.booked_seats.remove(&seat_number) {
            true => {
                self.seat_pool.push(Reverse(seat_number));
            }
            false => {}
        }
    }
}
#[test]
fn test() {

}