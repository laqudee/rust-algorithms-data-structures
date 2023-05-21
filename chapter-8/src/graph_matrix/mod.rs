/// 基于邻接矩阵的图
/// 点 Vertex
/// 边 Edge

// 点定义
#[derive(Debug)]
pub struct Vertex<'a> {
    id: usize,
    name: &'a str,
}

impl Vertex<'_> {
    pub fn new(id: usize, name: &'static str) -> Self {
        Self { id, name }
    }
}

// 边定义
#[derive(Debug, Clone)]
pub struct Edge {
    edge: bool, // 表示是否有边，并不需要构造一个边实体
}

impl Edge {
    pub fn new() -> Self {
        Self { edge: false }
    }

    pub fn set_edge() -> Self {
        Self { edge: true }
    }
}

// 图定义
#[derive(Debug)]
pub struct Graph {
    nodes: usize,
    graph: Vec<Vec<Edge>>, // 每个点的边放一个vec
}

impl Graph {
    pub fn new(nodes: usize) -> Self {
        Self {
            nodes,
            graph: vec![vec![Edge::new(); nodes]; nodes],
        }
    }

    pub fn len(&self) -> usize {
        self.nodes
    }

    pub fn is_empty(&self) -> bool {
        self.nodes == 0
    }

    // 添加边，设置边属性为true
    pub fn add_edge(&mut self, n1: &Vertex, n2: &Vertex) {
        if n1.id < self.nodes && n2.id < self.nodes {
            self.graph[n1.id][n2.id] = Edge::set_edge();
        } else {
            panic!("error");
        }
    }
}

pub fn it_work() {
    let mut g = Graph::new(4);
    let n1 = Vertex::new(0, "n1");
    let n2 = Vertex::new(1, "n2");
    let n3 = Vertex::new(2, "n3");
    let n4 = Vertex::new(3, "n4");

    g.add_edge(&n1, &n2);
    g.add_edge(&n1, &n3);
    g.add_edge(&n2, &n3);
    g.add_edge(&n2, &n4);
    g.add_edge(&n3, &n4);
    g.add_edge(&n3, &n1);

    println!("{:#?}", g);
    println!("graph empty: {}", g.is_empty());
    println!("graph nodes: {}", g.len());
}
