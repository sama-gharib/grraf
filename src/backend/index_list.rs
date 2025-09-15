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

use super::{Error, Graph, NodeId};
use std::collections::HashMap;

pub struct IndexList<Vertex: PartialEq, Edge> {
    successors: HashMap<NodeId, Vec<(Edge, NodeId)>>,
    vertices_map: HashMap<NodeId, Vertex>,
}

impl<Vertex: PartialEq, Edge> IndexList<Vertex, Edge> {
    pub fn new() -> Self {
        Self {
            successors: HashMap::new(),
            vertices_map: HashMap::new(),
        }
    }
}

impl<Vertex: PartialEq, Edge> Graph<Vertex, Edge> for IndexList<Vertex, Edge> {
    fn get_vertex_count(&self) -> usize {
        self.successors.len()
    }

    fn get_vertex(&self, id: NodeId) -> Option<&Vertex> {
        self.vertices_map.get(&id)
    }
    fn get_any_vertex(&self) -> Option<&Vertex> {
        self.vertices_map.values().next()
    }

    fn get_any_id(&self) -> Option<NodeId> {
        Some(self.vertices_map.keys().next()?.clone())
    }

    fn get_successors(&self, id: NodeId) -> Option<Vec<(NodeId, &Vertex)>> {
        let s = self.successors.get(&id)?;
        Some(
            s.iter()
                .map(|edge| (edge.1.clone(), self.get_vertex(edge.1.clone()).unwrap()))
                .collect::<Vec<_>>(),
        )
    }

    fn add_vertex(&mut self, v: Vertex, id: NodeId) {
        self.successors.insert(id.clone(), Vec::new());
        self.vertices_map.insert(id, v);
    }
    fn remove_vertex(&mut self, id: NodeId) {
        self.successors.retain(|node, _| node != &id);
        for successors in self.successors.values_mut() {
            successors.retain(|x| x.1 != id);
        }
        self.vertices_map.retain(|node, _| node != &id);
    }

    fn connect_vertices(
        &mut self,
        source: NodeId,
        edge: Edge,
        target: NodeId,
    ) -> Result<(), Error> {
        if let None = self.vertices_map.get(&target) {
            Err(Error::VertexNotFound { id: target })
        } else {
            let succ = self
                .successors
                .get_mut(&source)
                .ok_or(Error::VertexNotFound { id: source.clone() })?;

            if succ.iter().map(|x| &x.1).find(|x| x == &&target).is_some() {
                return Err(Error::AlreadyConnected { source, target });
            }

            succ.push((edge, target));

            Ok(())
        }
    }
}
