// dfs
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut cnt: i32 = 0;
        let mut vis: Vec<bool> = vec![false; is_connected.len()];

        for node in 0..is_connected.len() {
            if !vis[node] {
                cnt += 1;
                Self::dfs(&is_connected, &mut vis, node);
            }
        }
        cnt
    }

    fn dfs(is_connected: &[Vec<i32>], vis: &mut [bool], node: usize) {
        vis[node] = true;

        for adj_node in 0..is_connected.len() {
            if is_connected[node][adj_node] == 1 && !vis[adj_node] {
                Self::dfs(is_connected, vis, adj_node);
            }
        }
    }
}

// // bfs
// use std::collections::VecDeque;

// impl Solution {
//     pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
//         let mut cnt: i32 = 0;
//         let mut vis: Vec<bool> = vec![false; is_connected.len()];

//         for node in 0..is_connected.len() {
//             if !vis[node] {
//                 cnt += 1;
//                 Self::bfs(&is_connected, &mut vis, node);
//             }
//         }
//         cnt
//     }

//     fn bfs(is_connected: &[Vec<i32>], vis: &mut [bool], node: usize) {
//         let mut q: VecDeque<usize> = VecDeque::new();
//         q.push_back(node);
//         vis[node] = true;
        
//         while !q.is_empty() {
//             let node: usize = q.pop_front().unwrap();
            
//             for adj_node in 0..is_connected.len() {
//                 if is_connected[node][adj_node] == 1 && !vis[adj_node] {
//                     q.push_back(adj_node);
//                     vis[adj_node] = true;
//                 }
//             }
//         }
//     }
// }



















