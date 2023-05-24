///  Rust 实现 Trie

// 字典树定义
#[derive(Default)]
pub struct Trie {
    root: Node,
}

// 节点
#[derive(Default)]
pub struct Node {
    children: [Option<Box<Node>>; 26], // 字符节点列表
    end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    // 单词插入
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        // 逐个字符插入
        for c in word.as_bytes() {
            let index = (c - b'a') as usize;
            let next = &mut node.children[index];
            node = next.get_or_insert_with(Box::<Node>::default);
        }

        node.end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        self.word_node(word).map_or(false, |node| node.end)
    }

    // 判断是否存在以某个前缀开头的单词
    pub fn start_with(&self, prefix: &str) -> bool {
        self.word_node(prefix).is_some()
    }

    // 前缀字符串
    // wps: word_prefix_string
    pub fn word_node(&self, wps: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in wps.as_bytes() {
            let index = (c - b'a') as usize;
            match &node.children[index] {
                None => return None,
                Some(next) => node = next.as_ref(),
            }
        }

        Some(node)
    }
}

pub fn it_work() {
    let mut trie = Trie::new();
    trie.insert("box");
    trie.insert("insert");
    trie.insert("apple");
    trie.insert("appeal");

    let res1 = trie.search("apple");
    let res2 = trie.search("apples");
    let res3 = trie.start_with("ins");
    let res4 = trie.start_with("ina");
    println!("word 'apple' in Trie: {res1}");
    println!("word 'apples' in Trie: {res2}");
    println!("prefix 'ins' in Trie: {res3}");
    println!("prefix 'ina' in Trie: {res4}");
}
