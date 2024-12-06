
#[derive(Clone)]
pub enum EdgeType {
    Directed,
    Undirected,
    None,
}


pub trait EdgeTraits {
    fn set_start_node_id(&mut self, start_node_id: &str);
    fn set_end_node_id(&mut self, end_node_id: &str);
    fn get_start_node_id(&self) -> &str;
    fn get_end_node_id(&self) -> &str;
    fn get_type(&self) -> EdgeType;
    fn set_type(&mut self, edge_type: EdgeType);
    fn get_weight(&self) -> Option<f64>;
    fn set_weight(&mut self, weight: f64);
    fn get_id(&self) -> &str;
    fn set_id(&mut self, id: &str);
    fn set_label(&mut self, label: &str);
    fn get_label(&self) -> Option<&str>;
    fn is_blocked(&self) -> bool;
    fn set_blocked(&mut self, blocked: bool);
    fn get_creation_time(&self) -> u64;
    fn get_modification_time(&self) -> u64;
    fn set_modification_time(&mut self);
}