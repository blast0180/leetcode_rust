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