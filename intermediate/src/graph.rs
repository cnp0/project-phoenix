// TODO: not root-based, isolate structs Vertex/Node and Arc/Edge
struct GraphNode<'t> {
    pub value: &'static str,
    pub children: Vec<&'t GraphNode<'t>>,
}

struct Graph<'t> {
    pub root: &'t mut GraphNode<'t>,
}

impl<'t> Graph<'t> {
    pub fn new(root: &'t mut GraphNode<'t>) -> Graph<'t> {
        Graph::<'t> { root: root }
    }

    pub fn add(&mut self, child: &'t GraphNode<'t>) {
        self.root.children.push(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let test_value = "root";
        let mut root = GraphNode {
            value: test_value,
            children: vec![],
        };
        let graph = Graph::new(&mut root);

        assert_eq!(test_value, graph.root.value);
    }
}
