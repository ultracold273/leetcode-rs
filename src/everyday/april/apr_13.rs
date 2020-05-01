
use std::collections::HashMap;

struct Twitter {
    time: usize,
    messages: HashMap<i32, Vec<(usize, i32)>>,
    following: HashMap<i32, Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Twitter { time: 0, messages: HashMap::new(), following: HashMap::new() }
    }
    
    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let twitters = self.messages.entry(user_id).or_insert(Vec::new());
        twitters.push((self.time, tweet_id));
        self.time += 1;
    }
    
    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut time_feeds = Vec::<(usize, i32)>::new();
        if let Some(msg) = self.messages.get(&user_id) {
            time_feeds.extend(msg.iter());
        }
        if let Some(users) = self.following.get(&user_id) {
            for user in users {
                if let Some(msg) = self.messages.get(user) {
                    time_feeds.extend(msg.iter());
                }
            }
        }
        time_feeds.sort_by(|a, b| b.0.cmp(&a.0));
        time_feeds.iter().map(|a| a.1).take(10).collect()
    }
    
    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let following = self.following.entry(follower_id).or_insert(Vec::new());
        if !following.contains(&followee_id) && followee_id != follower_id { following.push(followee_id); }
    }
    
    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        let following = self.following.entry(follower_id).or_insert(Vec::new());
        if let Some(pos) = following.iter().position(|x| *x == followee_id) {
            following.remove(pos);
        }
    }
}

// 
// Your Twitter object will be instantiated and called as such:
// let obj = Twitter::new();
// obj.post_tweet(userId, tweetId);
// let ret_2: Vec<i32> = obj.get_news_feed(userId);
// obj.follow(followerId, followeeId);
// obj.unfollow(followerId, followeeId);
// 