use std::collections::HashMap;
use crate::graph::node::Node;
use crate::graph::edge::Edge;
use crate::graph::traits::edge_traits::{EdgeTraits, EdgeType};
use crate::graph::traits::node_traits::{NodeTraits, NodeType};

pub struct Graph {
    nodes: HashMap<String, Node>,
    base_id: u64,
    root: Option<Node>,
}

impl Graph {
    pub fn new() -> Self {
        let root_id = format!("{}_root", 0);
        let root_node = Node::new(NodeType::Primary, root_id);
        Graph {
            nodes: HashMap::new(),
            base_id: 1,
            root: Some(root_node),
        }
    }

    /// Node Functions

    pub fn create_node(&mut self, node_type: NodeType) -> (Node, String) {
        let id = format!("{}_{}", self.base_id, node_type.to_string() );
        self.base_id += 1;
        let node = Node::new(node_type, id.clone());
        (node, id)
    }
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.get_id().to_string(), node);
    }

    pub fn get_node_by_id(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_node_by_id_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn add_text_data(&mut self, node_id: &str, data: &str)
    {
        if let Some(node) = self.get_node_by_id_mut(node_id)
        {
            node.add_data(data.to_string());
        }
    }

    pub fn add_binary_data(&mut self, node_id: &str, data: Vec<u8>)
    {
        if let Some(node) = self.get_node_by_id_mut(node_id)
        {
            node.add_data(data);
        }
    }

    pub fn delete_node(&mut self, node_id: &str){
        self.nodes.remove(node_id);
    }



    /// Edge Functions
    pub fn get_edge_by_id(&self, edge_id: &str) -> Option<&Edge>
    {
        self.nodes.values().find_map(|node| node.get_edge(edge_id))
    }

    pub fn get_edge_by_id_mut(&mut self, edge_id: &str) -> Option<&mut Edge>
    {
        self.nodes.values_mut().find_map(|node| node.get_edge_mut(edge_id))
    }

    pub fn get_all_edges_of_node(&self, node_id: &str) -> Option<Vec<&Edge>>
    {
        self.nodes.get(node_id).and_then(|node| {node.get_all_edges()})
    }

    pub fn get_all_edges_of_node_mut(&mut self, node_id: &str) -> Option<Vec<&Edge>>
    {
        self.nodes.get_mut(node_id).and_then(|node| {node.get_all_edges()})
    }

    pub fn add_undirected_edge(&mut self, start_node_id: &str, end_node_id: &str)
    {
        let edge_id = format!("{}_{}_undirected", start_node_id, end_node_id);
        let edge= Edge::new(edge_id, start_node_id.to_string(), end_node_id.to_string());
        let reversed_edge = edge.reverse();

        if let Some(start_node) = self.get_node_by_id_mut(start_node_id)
        {
            start_node.add_edge(edge);
        }
        if let Some(end_node) = self.get_node_by_id_mut(end_node_id)
        {
            end_node.add_edge(reversed_edge);
        }
    }
    pub fn add_directed_edge(&mut self, start_node_id: &str, end_node_id: &str)
    {
        let edge_id = format!("{}_{}_directed", start_node_id, end_node_id);
        let edge= Edge::new(edge_id, start_node_id.to_string(), end_node_id.to_string());

        if let Some(start_node) = self.get_node_by_id_mut(start_node_id)
        {
            start_node.add_edge(edge);
        }
    }

    pub fn set_edge_type(&mut self, edge_id: &str, edge_type: EdgeType)
    {
        if let Some(edge) = self.get_edge_by_id_mut(edge_id)
        {
            edge.set_type(edge_type);
        }
    }

    pub fn set_edge_weight(&mut self, edge_id: &str, weight: f64)
    {
        if let Some(edge) = self.get_edge_by_id_mut(edge_id)
        {
            edge.set_weight(weight);
        }
    }
    pub fn set_ege_label(&mut self, edge_id: &str, label: &str)
    {
        if let Some(edge) = self.get_edge_by_id_mut(edge_id)
        {
            edge.set_label(label);
        }
    }
    pub fn set_edge_blocked(&mut self, edge_id: &str, blocked: bool)
    {
        if let Some(edge) = self.get_edge_by_id_mut(edge_id)
        {
            edge.set_blocked(blocked);
        }
    }

    fn delete_edge(&mut self, edge_id: &str)
    {
        if let Some(edge) = self.get_edge_by_id_mut(edge_id){
            match edge.get_type()
            {
                EdgeType::Undirected => self.delete_undirected_edge(edge_id),
                EdgeType::Directed => self.delete_directed_edge(edge_id),
                EdgeType::None => {}
            }
        }

    }
    fn delete_undirected_edge(&mut self, edge_id: &str)
    {
        if let Some(edge) = self.get_edge_by_id_mut(edge_id) {
            let start_node_id = edge.get_start_node_id().to_string();
            let end_node_id = edge.get_end_node_id().to_string();

            if let Some(start_node) = self.get_node_by_id_mut(&start_node_id) {
                start_node.remove_edge(edge_id);
            }
            if let Some(end_node) = self.get_node_by_id_mut(&end_node_id) {
                end_node.remove_edge(edge_id);
            }
        }
    }
    fn delete_directed_edge(&mut self, edge_id: &str)
    {
        if let Some(edge) = self.get_edge_by_id_mut(edge_id) {
            let start_node_id = edge.get_start_node_id().to_string();
            if let Some(start_node) = self.get_node_by_id_mut(&start_node_id) {
                start_node.remove_edge(edge_id);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::graph::traits::node_traits::NodeType;
    use super::*;
    #[test]
    fn test_add_and_get_node_and_remove(){
        let mut graph = Graph::new();

        let node1 = graph.create_node(NodeType::Primary);
        let  node2 =   graph.create_node(NodeType::Primary);

        let node1_id = &node1.1;
        let node2_id = &node2.1;

        graph.add_node(node1.0);
        graph.add_node(node2.0);

        let retrieved_node = graph.get_node_by_id(node1_id);
        let retrieved_node2 = graph.get_node_by_id(node2_id);
        assert!(retrieved_node.is_some());
        assert_eq!(retrieved_node.unwrap().get_id(), node1_id);
        assert!(retrieved_node2.is_some());
        assert_eq!(retrieved_node2.unwrap().get_id(), node2_id);

        graph.delete_node(node1_id);

        let retrieved_node = graph.get_node_by_id(node1_id);
        let retrieved_node2 = graph.get_node_by_id(node2_id);
        assert!(retrieved_node.is_none());
        assert!(retrieved_node2.is_some());
        assert_eq!(retrieved_node2.unwrap().get_id(), node2_id);

    }

    #[test]
    fn test_add_and_get_edge_and_remove(){
        let mut graph = Graph::new();
        let node1 = graph.create_node(NodeType::Primary);
        let node2 = graph.create_node(NodeType::Primary);
        let node3 = graph.create_node(NodeType::Primary);

        let node1_id = &node1.1;
        let node2_id = &node2.1;
        let node3_id = &node3.1;

        graph.add_node(node1.0);
        graph.add_node(node2.0);
        graph.add_node(node3.0);



        graph.add_undirected_edge(node1_id, node2_id);
        graph.add_directed_edge(node1_id, node3_id);

        let edge1_id = format!("{}_{}_undirected", node1_id, node2_id);
        let edge2_id = format!("{}_{}_directed", node1_id, node3_id);

        let retrieved_undirected_edge = graph.get_edge_by_id(edge1_id.as_str());
        let retrieved_directed_edge = graph.get_edge_by_id(edge2_id.as_str());

        assert!(retrieved_undirected_edge.is_some());
        assert!(retrieved_directed_edge.is_some());
        assert_eq!(retrieved_undirected_edge.unwrap().get_id(), edge1_id);
        assert_eq!(retrieved_directed_edge.unwrap().get_id(), edge2_id);

        graph.delete_edge(edge1_id.as_str());
        graph.delete_edge(edge2_id.as_str());

        let retrieved_undirected_edge = graph.get_edge_by_id(edge1_id.as_str());
        let retrieved_directed_edge = graph.get_edge_by_id(edge2_id.as_str());
        let retrieved_undirected_edge2 = graph.get_node_by_id(node2_id).unwrap().get_edge(edge1_id.as_str());

        assert!(retrieved_undirected_edge.is_none());
        assert!(retrieved_directed_edge.is_none());
        assert!(retrieved_undirected_edge2.is_none());
    }

}