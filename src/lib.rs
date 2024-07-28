//! 自作のアルゴリズムライブラリ
//! 競技プログラミング用

#![allow(unused_imports)]
use std::{cmp::Reverse, collections::BinaryHeap, fmt::{Debug, Display}};

// use ac_library::{Dsu, Segtree};
use itertools::Itertools;
// use proconio::{
//     fastout, input,
//     marker::{Bytes, Chars, Isize1, Usize1},
// };
// use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
// use std::{cmp, fmt::Display};
// use superslice::Ext;

/// # ダイクストラのアルゴリズム  
/// 木(辺とその重みのリスト)と始点を受け取り、始点から各頂点までの最短距離を返す  
/// リストのn番目は，n番目の節点に隣接する節点mとその辺の重みを表す  
/// m番目にもnの情報を入れる
/// 
/// # Example
/// ```rust
/// use algo_rs::dijkstra;
/// 
/// let graph = vec![
///     vec![(1, 100), (2, 3)],
///     vec![(0, 100), (3, 3), (4, 4)],
///     vec![(0, 3), (3, 4)],
///     vec![(2, 4), (1, 3)],
///     vec![(1, 4)],
/// ];
/// 
/// let distances = dijkstra(&graph, 0);
/// assert_eq!(distances, vec![0, 10, 3, 7, 14]);
/// 
/// let distances = dijkstra(&graph, 3);
/// assert_eq!(distances, vec![7, 3, 4, 0, 7]);
/// ```
pub fn dijkstra(graph: &[Vec<(usize, usize)>], start: usize) -> Vec<usize> {
    // 各頂点から始点への最短距離
    let mut distances = vec![usize::MAX; graph.len()];
    distances[start] = 0;

    // min-heap
    //(距離, 頂点番号)
    let mut queue: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    queue.push((Reverse(0), start));

    while !queue.is_empty() {
        // 距離が最小の節点を取り出す
        let (dist, vertex) = queue.pop().unwrap();
        //
        if dist.0 > distances[vertex] {
            continue;
        }

        // 隣接するすべての節点の最短距離を更新する
        for &(u, weight) in &graph[vertex] {
            let new_dist = dist.0 + weight;
            if distances[u] > new_dist {
                distances[u] = new_dist;
                queue.push((Reverse(new_dist), u));
            }
        }
    }

    distances
}

/// スライスを空白区切りで文字列に変換する
pub fn string_slice<T: ToString>(slice: &[T]) -> String {
    slice.iter()
        .map(std::string::ToString::to_string)
        .join(" ")
}

pub fn dbg_two_dim_vec_string<T: Debug>(slice: &[Vec<T>]) -> String {
    slice.iter()
        .map(|s| format!("{:?}", s))
        .join("\n")
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn two_dim_vec_string_test() {
        let slice = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(dbg_two_dim_vec_string(&slice), "[1, 2, 3]\n[4, 5, 6]");

        let slice = vec![vec![Some(1), Some(2), Some(3)], vec![Some(4), Some(5), Some(6)]];
        assert_eq!(dbg_two_dim_vec_string(&slice), "[Some(1), Some(2), Some(3)]\n[Some(4), Some(5), Some(6)]");
    }
}
