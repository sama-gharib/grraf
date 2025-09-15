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

use grraf::backend::{ Graph, NodeId, IndexList};
use grraf::frontend::depth_first_search;
use grraf::node_id;

fn create_five_letters_as_node_ids() -> Vec<NodeId> {
    vec!["A", "B", "C", "D", "E"].iter().map(|letter| node_id!(name: **letter)).collect()
}

fn create_stable(ids: &[NodeId]) -> IndexList<usize, ()>  {
    let mut graph = IndexList::new();
    ids.iter().for_each(
        {
            let mut count = 0;
            let graph = &mut graph;
            move |id| {
                graph.add_vertex(count, id.clone());
                count += 1;
            }
        }
    );

    graph
}

fn create_exemple_line_index_list() -> IndexList<usize, ()> {    
    let ids: Vec<_> = create_five_letters_as_node_ids();
    let mut graph = create_stable(&ids);

    graph.connect_vertices(node_id!(name: "A"), () ,node_id!(name: "B")).unwrap();
    graph.connect_vertices(node_id!(name: "B"), () ,node_id!(name: "C")).unwrap();
    graph.connect_vertices(node_id!(name: "C"), () ,node_id!(name: "D")).unwrap();
    graph.connect_vertices(node_id!(name: "D"), () ,node_id!(name: "E")).unwrap();

    graph
}

#[test]
fn dfs_line_graph() {
    let graph = create_exemple_line_index_list();
    let ids = create_five_letters_as_node_ids();
    
    assert_eq!(
        depth_first_search(
            &graph, Some(node_id!(name: "A")), 4
        ),
        Some(ids)
    );
}

#[test]
fn dfs_random_graph() {
    let ids = create_five_letters_as_node_ids();
    let mut graph = create_stable(&ids);

    graph.connect_vertices(node_id!(name: "A"), (), node_id!(name: "C")).unwrap();
    graph.connect_vertices(node_id!(name: "A"), (), node_id!(name: "B")).unwrap();
    graph.connect_vertices(node_id!(name: "B"), (), node_id!(name: "C")).unwrap();
    graph.connect_vertices(node_id!(name: "C"), (), node_id!(name: "D")).unwrap();
    graph.connect_vertices(node_id!(name: "C"), (), node_id!(name: "E")).unwrap();
    graph.connect_vertices(node_id!(name: "E"), (), node_id!(name: "A")).unwrap();
    graph.connect_vertices(node_id!(name: "E"), (), node_id!(name: "B")).unwrap();

    assert_eq!(
        depth_first_search(
            &graph, Some(node_id!(name: "A")), 4
        ),
        Some(
            vec![node_id!(name: "A"), node_id!(name: "B"), node_id!(name: "C"), node_id!(name: "E")]
        )
    );
}
