mod bfs;
mod dfs;
mod graph_adjlist;
mod graph_matrix;
mod dijkstra;

fn main() {
    // Test 邻接矩阵实现的图
    graph_matrix::it_work();

    // Test 邻接表实现的图
    graph_adjlist::it_work();

    // Test 广度优先遍历
    bfs::it_work();

    // Test 深度优先遍历
    dfs::it_work();

    // Test 最短路径算法Dijkstra
    dijkstra::it_work();
}
