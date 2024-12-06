use std::time::SystemTime;
use crate::graph::traits::edge_traits::*;
#[derive(Clone)]
pub struct Edge {
    edge_id: String,
    edge_type: EdgeType,
    weight: Option<f64>,
    start_node_id: String,
    end_node_id: String,
    label: Option<String>,
    created_at: u64,
    updated_at: u64,
    blocked: bool,
}

impl Edge {
    pub fn new(edge_id: String, start_node_id: String, end_node_id: String) -> Self {
        Self {
            edge_id,
            start_node_id,
            end_node_id,
            edge_type: EdgeType::Undirected,
            label: None,
            weight: None,
            created_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            updated_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            blocked: false,
        }
    }

    pub fn reverse(&self) -> Edge
    {
        Edge {
            start_node_id: self.get_start_node_id().to_string(),
            end_node_id:  self.get_end_node_id().to_string(),
            edge_id: self.edge_id.clone(),
            edge_type: self.edge_type.clone(),
            label: self.label.clone(),
            weight: self.weight.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.get_modification_time().clone(),
            blocked: self.blocked.clone(),
        }
    }

}


impl EdgeTraits for Edge {

    fn set_start_node_id(&mut self, start_node_id: &str)
    {
        self.start_node_id = start_node_id.to_string();
    }
    fn set_end_node_id(&mut self, end_node_id: &str)
    {
        self.end_node_id = end_node_id.to_string();
    }
    fn get_start_node_id(&self) -> &str
    {
        &self.start_node_id
    }
    fn get_end_node_id(&self) -> &str
    {
        &self.end_node_id
    }
    fn get_type(&self) -> EdgeType
    {
        self.edge_type.clone()
    }
    fn set_type(&mut self, edge_type: EdgeType)
    {
        self.edge_type = edge_type;
    }
    fn get_weight(&self) -> Option<f64>
    {
        self.weight
    }
    fn set_weight(&mut self, weight: f64)
    {
        self.weight = Some(weight);
    }
    fn get_id(&self) -> &str
    {
        &self.edge_id
    }
    fn set_id(&mut self, id: &str)
    {
        self.edge_id = id.to_string();
    }
    fn set_label(&mut self, label: &str)
    {
        self.label = Some(label.to_string());
    }
    fn get_label(&self) -> Option<&str>
    {
        self.label.as_deref()
    }
    fn is_blocked(&self) -> bool
    {
        self.blocked
    }
    fn set_blocked(&mut self, blocked: bool)
    {
        self.blocked = blocked;
    }
    fn get_creation_time(&self) -> u64
    {
        self.created_at
    }
    fn get_modification_time(&self) -> u64
    {
        self.updated_at
    }
    fn set_modification_time(&mut self)
    {
        self.updated_at = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    }
}