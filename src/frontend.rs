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

use crate::backend::{ Graph, NodeId };

pub fn depth_first_search<Vertex: PartialEq, Edge>(
    graph: &impl Graph<Vertex, Edge>,
    start: Option<NodeId>,
    target: Vertex
) -> Option<Vec<NodeId>> { 

    let vertex_count = graph.get_vertex_count();
    let mut stack: Vec<NodeId> = Vec::with_capacity(vertex_count);
    stack.push( if let Some(id) = start { id } else { graph.get_any_id()? } );
    let mut path = stack.clone();
    let mut path_cursor = 0;

    while !stack.is_empty() {
        let next = stack.pop().unwrap();
        println!("Next in stack: {next:?}");

        for (index, id) in path.iter().enumerate() {
            if &next == id {
                path_cursor = index;
            }
        }
        
        if let Some(slot) = path.get_mut(path_cursor) {
            *slot = next.clone();
        } else {
            path.push(next.clone());
        }
        path_cursor += 1;
        
        let successors = graph.get_successors(next).unwrap();
        println!("\tSuccessors: {:?}", successors.iter().map(|x| x.0.clone()).collect::<Vec::<_>>());

        for (id, vertex) in successors.iter() {
            stack.push(id.clone());
        
            if vertex == &&target {
                path.push(id.clone());
                path.shrink_to(path_cursor);
                
                return Some(path);
            }
        }   
    }

    None
}
