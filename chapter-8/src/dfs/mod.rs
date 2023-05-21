/// Rust 实现深度优先遍历
use crate::bfs::graph::Graph;

// 根据data构建图
fn create_graph(data: [[usize; 2]; 20]) -> Vec<(Graph, usize)> {
    let mut arr: Vec<(Graph, usize)> = Vec::new();

    for _ in 0..9 {
        arr.push((Graph::new(), 0));
    }

    for i in 1..9 {
        for j in 0..data.len() {
            if data[j][0] == i {
                arr[i].0.insert(data[j][1]);
            }
        }
        print!("[{i}]->");
        arr[i].0.print_node();
    }

    arr
}

// 深度优先遍历
pub fn dfs(graph: Vec<(Graph, usize)>) {
    let mut gp = graph;
    let mut nodes: Vec<usize> = Vec::new();
    let mut temp: Vec<usize> = Vec::new();

    gp[1].1 = 1;
    let mut curr = gp[1].0.get_first().clone();

    // 打印图
    print!("{}->", 1);
    while let Some(val) = curr {
        nodes.insert(0, val.borrow().data);
        curr = val.borrow().next.clone();
    }

    // 打印深度优先图
    loop {
        if 0 == nodes.len() {
            break;
        } else {
            let data = nodes.pop().unwrap();
            if 0 == gp[data].1 {
                gp[data].1 = 1;
                print!("{data}->");

                // 节点加入 temp
                let mut curr = gp[data].0.get_first().clone();
                while let Some(val) = curr {
                    temp.push(val.borrow().data);
                    curr = val.borrow().next.clone();
                }

                while !temp.is_empty() {
                    nodes.push(temp.pop().unwrap());
                }
            }
        }
    }
    println!("");
}

pub fn it_work() {
    let data = [
        [1, 2],
        [2, 1],
        [1, 3],
        [3, 1],
        [2, 4],
        [4, 2],
        [2, 5],
        [5, 2],
        [3, 6],
        [6, 3],
        [3, 7],
        [7, 3],
        [4, 5],
        [5, 4],
        [6, 7],
        [7, 6],
        [5, 8],
        [8, 5],
        [6, 8],
        [8, 6],
    ];
    let gp = create_graph(data);
    dfs(gp);
}
