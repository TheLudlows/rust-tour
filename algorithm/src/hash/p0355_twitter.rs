use std::collections::{HashMap, HashSet, VecDeque};

struct Twitter {
    map: HashMap<i32, HashSet<i32>>,
    vec: VecDeque<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self { map: Default::default(), vec: VecDeque::new() }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.vec.push_front((user_id, tweet_id))
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut ret = Vec::new();
        let set = self.map.get(&user_id);

        let mut new_set = HashSet::new();
        new_set.insert(user_id);
        match set {
            None => {}
            Some(starts) => {
                for i in starts {
                    new_set.insert(*i);
                }
            }
        }
        for (uid, tid) in self.vec.iter() {
            if new_set.contains(&uid) {
                ret.push(*tid);
            }
            if ret.len() == 10 {
                break;
            }
        }
        ret
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let mut set = self.map.get_mut(&follower_id);
        match set {
            None => {
                let mut set = HashSet::new();
                set.insert(followee_id);
                self.map.insert(follower_id, set);
            }
            Some(start) => {
                start.insert(followee_id);
            }
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        let mut set = self.map.get_mut(&follower_id);
        match set {
            None => {}
            Some(start) => {
                start.remove(&followee_id);
            }
        }
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
#[test]
fn test_add() {
    let mut t = Twitter::new();
    t.post_tweet(1, 5);
    let ret = t.get_news_feed(1);

    println!("{:?}", ret)
}