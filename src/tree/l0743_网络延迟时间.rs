/*
 * @lc app=leetcode.cn id=743 lang=rust
 *
 * [743] 网络延迟时间
 */
struct Solution;
// @lc code=start
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dist: Vec<_> = (0..=n).map(|_| usize::MAX).collect();
        let mut heap = BinaryHeap::new();
        dist[k] = 0;
        heap.push(State {
            cost: 0,
            position: k,
        });
        while let Some(State { cost, position }) = heap.pop() {
            if cost > dist[position] {
                continue;
            }
            for edge in times.iter().filter(|v| v[0] as usize == position) {
                let next = State {
                    cost: cost + edge[2] as usize,
                    position: edge[1] as usize,
                };
                //println!("cost: {}, edge: {:?}, next: {:?}", cost, edge, next);
                if next.cost < dist[next.position] {
                    heap.push(next);
                    dist[next.position] = next.cost;
                }
            }
        }
        let m = dist.iter().skip(1).max().unwrap();
        if *m == usize::MAX {
            -1
        } else {
            *m as i32
        }
    }
}
// @lc code=end

mod tests {
    #[test]
    fn test() {
        use super::*;
        let r =
            Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2);
        dbg!(r);
    }
}
