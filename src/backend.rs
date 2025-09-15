mod index_list;

pub use index_list::IndexList;

#[derive(Debug)]
pub enum Error {
    VertexNotFound { id: NodeId },
    AlreadyConnected { source: NodeId, target: NodeId }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NodeId {
    UInt ( usize ),
    Name (String)
}

#[macro_export]
macro_rules! node_id {
    (name : $v: expr) => {
        NodeId::Name($v.into())
    };
    (uint : &v: expr) => {
        NodeId::UInt($v);
    };
}

pub trait Graph<Vertex: PartialEq, Edge> {
    fn get_vertex_count(&self) -> usize;
    fn get_vertex(&self, id: NodeId) -> Option<&Vertex>;
    fn get_any_vertex(&self) -> Option<&Vertex>;
    
    fn get_any_id(&self) -> Option<NodeId>;
    
    fn get_successors(&self, id: NodeId) -> Option<Vec<(NodeId, &Vertex)>>;
    
    fn add_vertex(&mut self, v: Vertex, id: NodeId);
    fn remove_vertex(&mut self, id: NodeId);
    fn connect_vertices(&mut self, source: NodeId, edge: Edge, target: NodeId) -> Result<(), Error>;
}
