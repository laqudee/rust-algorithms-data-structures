use std::collections::HashMap;
/// Rust HashMap实现邻接表图
/// Vertex
/// connects
/// 对Vertex，需要的操作有新建点、获取点自身的值、添加邻接点、获取所有邻接点
/// 获取到邻接点的权重
use std::hash::Hash;

// 点定义
#[derive(Debug, Clone)]
pub struct Vertex<T> {
    key: T,
    connects: Vec<(T, i32)>, // 邻接集合
}

impl<T: Clone + PartialEq> Vertex<T> {
    pub fn new(key: T) -> Self {
        Self {
            key,
            connects: Vec::new(),
        }
    }

    // 判断与当前点是否相邻
    pub fn adjacent_key(&self, key: &T) -> bool {
        for (nbr, _wt) in self.connects.iter() {
            if nbr == key {
                return true;
            }
        }

        false
    }

    pub fn add_neighbor(&mut self, nbr: T, wt: i32) {
        self.connects.push((nbr, wt));
    }

    // 获取相邻的点集合
    pub fn get_connects(&self) -> Vec<&T> {
        let mut connects = Vec::new();
        for (nbr, _wt) in self.connects.iter() {
            connects.push(nbr);
        }

        connects
    }

    // 返回到邻点的边权重
    pub fn get_nbr_weight(&self, key: &T) -> &i32 {
        for (nbr, wt) in self.connects.iter() {
            if nbr == key {
                return wt;
            }
        }

        &0
    }
}

// 图定义
#[derive(Debug, Clone)]
pub struct Graph<T> {
    vertnums: u32,                   // 点数
    edgenums: u32,                   // 边数
    vertices: HashMap<T, Vertex<T>>, // 点集合
}

impl<T: Hash + Eq + PartialEq + Clone> Graph<T> {
    pub fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertnums == 0
    }

    pub fn vertex_num(&self) -> u32 {
        self.vertnums
    }

    pub fn edge_num(&self) -> u32 {
        self.edgenums
    }

    pub fn contains(&self, key: &T) -> bool {
        for (nbr, _vertex) in self.vertices.iter() {
            if nbr == key {
                return true;
            }
        }

        false
    }

    pub fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    pub fn get_vertex(&self, key: &T) -> Option<&Vertex<T>> {
        if let Some(vertex) = self.vertices.get(key) {
            Some(&vertex)
        } else {
            None
        }
    }

    // 获取所有节点的key
    pub fn vertex_keys(&self) -> Vec<T> {
        let mut keys = Vec::new();
        for key in self.vertices.keys() {
            keys.push(key.clone());
        }

        keys
    }

    // 删除点 （同时要删除边）
    pub fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let old_vertex = self.vertices.remove(key);
        self.vertnums -= 1;

        // 删除从当前点出发的边
        self.edgenums -= old_vertex.clone().unwrap().get_connects().len() as u32;

        // 删除到当前点的边
        for vertex in self.vertex_keys() {
            if let Some(vt) = self.vertices.get_mut(&vertex) {
                if vt.adjacent_key(key) {
                    vt.connects.retain(|(k, _)| k != key);
                    self.edgenums -= 1;
                }
            }
        }

        old_vertex
    }

    pub fn add_edge(&mut self, from: &T, to: &T, wt: i32) {
        // 若点不存在要先添加点
        if !self.contains(from) {
            let _fvert = self.add_vertex(from);
        }

        if !self.contains(to) {
            let _tvert = self.add_vertex(to);
        }

        // 添加边
        self.edgenums += 1;
        self.vertices
            .get_mut(from)
            .unwrap()
            .add_neighbor(to.clone(), wt);
    }

    // 判断两个点是否相邻
    pub fn adjacent(&self, from: &T, to: &T) -> bool {
        self.vertices.get(from).unwrap().adjacent_key(to)
    }
}

pub fn it_work() {
    let mut g = Graph::new();

    for i in 0..6 {
        g.add_vertex(&i);
    }
    println!("graph empty: {}", g.is_empty());

    let vertices = g.vertex_keys();
    for vertex in vertices {
        println!("vertex: {:#?}", vertex);
    }

    g.add_edge(&0, &1, 5);
    g.add_edge(&0, &5, 2);
    g.add_edge(&1, &2, 3);
    g.add_edge(&2, &3, 4);
    g.add_edge(&3, &4, 7);
    g.add_edge(&3, &5, 3);
    g.add_edge(&4, &0, 1);
    g.add_edge(&4, &4, 8);
    println!("vert nums: {}", g.vertex_num());
    println!("edge nums: {}", g.edge_num());
    println!("contains 0 : {}", g.contains(&0));

    let vertex = g.get_vertex(&0).unwrap();
    println!(
        "key: {}, to nbr 1 weight: {}",
        vertex.key,
        vertex.get_nbr_weight(&1)
    );

    let keys = vertex.get_connects();
    for nbr in keys {
        print!("nighbor: {nbr}");
    }

    for (nbr, wt) in vertex.connects.iter() {
        println!("0 nighbor: {nbr}, weight: {wt}");
    }

    let res = g.adjacent(&0, &1);
    println!("0 adjacent to 1: {res}");
    let res = g.adjacent(&3, &2);
    println!("3 adjacent to 2: {res}");

    let rm = g.remove_vertex(&0).unwrap();
    println!("remove vertex: {}", rm.key);
    println!("left vert nums: {}", g.vertex_num());
    println!("left edge nums: {}", g.edge_num());
    println!("contains 0 : {}", g.contains(&0));
}
