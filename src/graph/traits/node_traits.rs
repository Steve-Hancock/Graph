use std::fmt::Display;
use crate::graph::edge::Edge;

/// Represents the various types of a node that can be categorized into.
#[derive(Clone, Debug,)]
pub enum NodeType {
    /// A node that Represents a central or important role within a system
    Primary,
    /// A node used for holding and managing data of both string and binary types.
    Data,
    /// A node used for holding data of String and text
    Text,
    /// A node used for holding data of images and videos or other binary type data
    Binary,
    /// A default state used when a node does not belong to a specific category
    None,
}

impl Default for NodeType
{
    fn default() -> Self {
        NodeType::None // Default to None if no specific type is needed
    }
}

impl Display for NodeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NodeType::Primary => write!(f, "Primary"),
            NodeType::Data => write!(f, "Data"),
            NodeType::Text => write!(f, "Text"),
            NodeType::Binary => write!(f, "Binary"),
            NodeType::None => write!(f, "None"),
        }
    }
}

/// Represents different types of data that can be held by nodes.
#[derive(Clone)]
pub enum NodeData {
    /// For storing multiple text entries.
    Text(Vec<String>),
    /// For storing binary data such as images or videos.
    Binary(Vec<Vec<u8>>),
    /// A composite of multiple data types, allowing both text and binary data.
    Composite {
        text: Vec<String>,
        binary: Vec<Vec<u8>>,
    },
    /// A state for when there is no data associated with the node.
    None,
}





pub trait NodeTraits {

    fn get_type(&self) ->&NodeType;
    fn get_id(&self) -> &str;
    fn get_edge(&self, node_id: &str) -> Option<&Edge>;
    fn get_edge_mut(&mut self, node_id: &str) -> Option<&mut Edge>;
    fn get_all_edges(&self) -> Option<Vec<&Edge>>;
    fn add_edge(&mut self, edge: Edge);
    fn remove_edge(&mut self, edge_id: &str);
    fn get_data(&self) -> &NodeData;
    fn set_data(&mut self, data: NodeData);
    fn set_type(&mut self, node_type: NodeType);
    fn set_id(&mut self, id: String);
    fn get_creation_time(&self) -> u64;
    fn get_modification_time(&self) -> u64;
    fn set_modification_time(&mut self, modification_time: u64);

}