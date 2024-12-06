use std::collections::{HashMap};
use std::any::Any;
use std::time::SystemTime;
use crate::graph::edge::Edge;
use crate::graph::traits::edge_traits::EdgeTraits;
use crate::graph::traits::node_traits::*;



/// A struct representing a node in a network, with associated type, ID, connections, and data.
pub struct Node {
    id: String,
    node_type: NodeType,
    edges: HashMap<String, Edge>,
    data: NodeData,
    creation_time: u64,
    modification_time: u64,
}

impl Node {
    pub fn new(node_type: NodeType, id: String) -> Self {
        let data = match node_type {
            NodeType::Primary => NodeData::None,
            NodeType::Data => NodeData::Composite {text: vec![], binary: vec![]},
            NodeType::Text => NodeData::Text(vec![]),
            NodeType::Binary => NodeData::Binary(vec![]),
            NodeType::None => NodeData::None,
        };

        let creation_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        Node {
            node_type,
            id,
            edges: HashMap::new(),
            data,
            creation_time,
            modification_time: creation_time,
        }
    }



    pub fn add_data<T: Any>(&mut self, new_data: T) {
        let any_data = &new_data as &dyn Any;
        match &mut self.data {
            NodeData::Text(text) => {
                if let Some(data) = any_data.downcast_ref::<String>() {
                    text.push(data.clone());
                } else {
                    eprintln!("Cannot add data: new_data is not a String");
                }
            }

            NodeData::Binary(binary) => {
                if let Some(data) = any_data.downcast_ref::<Vec<u8>>() {
                    binary.push(data.clone());
                } else {
                    eprintln!("Cannot add data: new_data is not binary data in a Vec<u8> format");
                }
            }

            NodeData::Composite { text, binary } => {
                if let Some(data) = any_data.downcast_ref::<String>() {
                    text.push(data.clone());
                } else if let Some(data) = any_data.downcast_ref::<Vec<u8>>() {
                    binary.push(data.clone());
                } else {
                    eprintln!("Cannot add data: new_data is not a String or binary data in a Vec<u8> format");
                }
            }

            NodeData::None => {
                eprintln!("Cannot add data: NodeData is None");
            }
        }
    }
}

impl NodeTraits for Node {

    fn get_type(&self) -> &NodeType {
        &self.node_type
    }
    fn get_id(&self) -> &str {
        &self.id
    }
    fn get_edge(&self, edge_id: &str) -> Option<&Edge> { self.edges.get(edge_id) }
    fn get_edge_mut(&mut self, edge_id: &str) -> Option<&mut Edge> { self.edges.get_mut(edge_id) }
    fn get_all_edges(&self) -> Option<Vec<&Edge>>
    {
        if self.edges.is_empty() {
            None
        } else {
            Some(self.edges.values().collect())
        }
    }
    fn add_edge(&mut self, edge: Edge) {
        let edge_id = edge.get_id().to_string();
        self.edges.insert(edge_id, edge);
    }
    fn remove_edge(&mut self, edge_id: &str) {
        self.edges.remove(edge_id);
    }
    fn get_data(&self) -> &NodeData {
        &self.data
    }
    fn set_data(&mut self, data: NodeData) {
        self.data = data;
    }
    fn set_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }
    fn set_id(&mut self, id: String) {
        self.id = id;
    }
    fn get_creation_time(&self) -> u64
    {
        self.creation_time
    }
    fn get_modification_time(&self) -> u64
    {
        self.modification_time
    }
    fn set_modification_time(&mut self, modification_time: u64)
    {
        self.modification_time = modification_time;
    }
}