//! 自作のアルゴリズムライブラリ
//! 競技プログラミング用

#![allow(unused_imports)]
use std::{fmt::Display, cmp::*};
use std::collections::*;
// use superslice::*;
use itertools::*;
// use ac_library::{Dsu, Segtree};
// use proconio::{input, fastout, marker::*};

/// ダイクストラのアルゴリズム
/// グラフ(辺のリスト)と始点を受け取り、始点から各頂点までの最短距離を返す
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
pub fn string_slice<T: Display>(slice: &[T]) -> String {
    slice.iter()
        .map(std::string::ToString::to_string)
        .join(" ")
}
