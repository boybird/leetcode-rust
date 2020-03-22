/*
 * @lc app=leetcode.cn id=146 lang=rust
 *
 * [146] LRU缓存机制
 */

// @lc code=start
use std::collections::VecDeque;

#[derive(Debug)]
struct LRUCache {
    data: Vec<(i32, i32)>,
    used: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            data: vec![(-1, -1); capacity as usize],
            used: VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        for (k, v) in self.data.iter() {
            if k == &key {
                self.used.retain(|x| *x != key);
                self.used.push_back(key);
                return *v;
            }
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.get(key) != -1 {
            for elem in self.data.iter_mut() {
                if elem.0 == key {
                    elem.1 = value;
                    break;
                }
            }
            return;
        }
        if self.used.len() >= self.data.capacity() {
            let k = self.used.pop_front();
            let pos = self.data.iter().position(|(k0, _v)| *k0 == k.unwrap());
            self.data[pos.unwrap()] = (key, value);
        } else {
            self.data[self.used.len()] = (key, value);
        }
        self.used.push_back(key);
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        let mut cache: LRUCache = LRUCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        dbg!(cache.get(1)); // 返回  1
        cache.put(3, 3); // 该操作会使得密钥 2 作废
        dbg!(cache.get(2)); // 返回 -1 (未找到)
        cache.put(4, 4); // 该操作会使得密钥 1 作废
        dbg!(cache.get(1)); // 返回 -1 (未找到)
        dbg!(cache.get(3)); // 返回  3
        dbg!(cache.get(4)); // 返回  4
    }
}
