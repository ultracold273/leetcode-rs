
use std::collections::HashMap;

struct TweetCounts {
    tweets: HashMap<String, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TweetCounts {

    fn new() -> Self {
        TweetCounts { tweets: HashMap::new() }
    }
    
    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        let tweet = self.tweets.entry(tweet_name).or_insert(Vec::new());
        tweet.push(time);
    }
    
    fn get_tweet_counts_per_frequency(&self, freq: String, tweet_name: String, start_time: i32, end_time: i32) -> Vec<i32> {
        let freq_index = match freq.as_ref() {
            "minute" => {0},
            "hour" => {1},
            "day" => {2},
            _ => {0},
        };
        let delta = vec![60, 60*60, 60*60*24];
        let nbin = ((end_time + 1 - start_time) + delta[freq_index] - 1) / delta[freq_index];
        let mut res = vec![0; nbin as usize];
        if let Some(arr) = self.tweets.get(&tweet_name) {
            for &t in arr {
                if t >= start_time && t < end_time + 1 {
                    let bin_index = ((t - start_time) / delta[freq_index]) as usize;
                    res[bin_index] += 1;
                }
            }
        }
        res
    }
}

