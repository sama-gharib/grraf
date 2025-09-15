/*
    Program name: grraf
    Brief: grraf is a library providing a generic API for several graph data structures. Please see the `README.md` file for more details.
    Contact: sama.gharib-ali-barura@proton.me

    Please see the `LICENSE` file for this program complete license.

    Copyright (C) 2025  GHARIB ALI BARURA Sama.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
