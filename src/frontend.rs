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
