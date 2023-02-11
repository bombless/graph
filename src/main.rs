#![feature(const_trait_impl)]

use naive_force_graph::{ForceGraph, Node, NodeData, NodeId, Parameters};
use macroquad::prelude::*;


#[allow(unused)]
fn from_array<const M: usize, const N: usize>(arr: [[i32; N]; M]) -> Vec<Vec<i32>> {
    IntoIterator::into_iter(arr).map(Vec::from).collect()
}

#[allow(unused)]
fn get_data() -> Vec<Vec<i32>> {
    from_array([[1,2],[3,4]])
}

fn init_graph() -> ForceGraph::<usize> {

    // create a force graph with default parameters
    let mut graph = ForceGraph::<usize>::new(Default::default());

    // create nodes
    let n1_idx = graph.add_node(NodeData {
        x: screen_width() / 4.0,
        y: screen_height() / 4.0,
        user_data: 1,
        ..Default::default()
    });
    let n2_idx = graph.add_node(NodeData {
        x: 3.0 * screen_width() / 4.0,
        y: screen_height() / 4.0,
        user_data: 2,
        ..Default::default()
    });
    let n3_idx = graph.add_node(NodeData {
        x: 3.0 * screen_width() / 4.0,
        y: 3.0 * screen_height() / 4.0,
        user_data: 3,
        ..Default::default()
    });

    // set up links between nodes
    graph.add_edge(n1_idx, n2_idx, Default::default());
    graph.add_edge(n2_idx, n3_idx, Default::default());

    graph
}

#[macroquad::main("Demo")]
async fn main() {

    // let graph = init_graph();
    let matrix = get_data();
    // println!("result {:?}", Solution::matrix_rank_transform(matrix));
    let mut uf = Solution::union_find(&matrix);
    let graph = Solution::force_graph(&matrix, &mut uf);
    run_graph(graph).await;
}

async fn run_graph<T: ToString>(mut graph: ForceGraph::<T>) {

    const NODE_RADIUS: f32 = 15.0;

    fn node_overlaps_mouse_position<T>(node: &Node<T>) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        ((node.x() - mouse_x) * (node.x() - mouse_x) + (node.y() - mouse_y) * (node.y() - mouse_y))
            < NODE_RADIUS * NODE_RADIUS
    }

    fn draw_arrow(node1: Vec2, node2: Vec2) {
        const ANGLE: f32 = 0.6;
        fn get_value<F: Fn(Vec2)->f32>(f: F, dis: f32, node1: Vec2, node2: Vec2) -> f32 {
            f(node2) + dis * (f(node1) - f(node2)) /
                ((node1.x - node2.x).powi(2) + (node1.y - node2.y).powi(2)).sqrt()
        }
        
        let v1 = Vec2::new(
            get_value(|x| x.x, NODE_RADIUS, node1, node2),
            get_value(|x| x.y, NODE_RADIUS, node1, node2)
        );
        let point_x = Vec2::new(
            get_value(|x| x.x, 2.0 * NODE_RADIUS, node1, node2),
            get_value(|x| x.y, 2.0 * NODE_RADIUS, node1, node2)
        );

        let x = v1.x + (point_x.x - v1.x) * ANGLE.cos() - (point_x.y - v1.y) * ANGLE.sin();
        let y = v1.y + (point_x.y - v1.y) * ANGLE.cos() + (point_x.x - v1.x) * ANGLE.sin();
        let v2 = Vec2::new(x, y);

        let x = v1.x + (point_x.x - v1.x) * (-ANGLE).cos() - (point_x.y - v1.y) * (-ANGLE).sin();
        let y = v1.y + (point_x.y - v1.y) * (-ANGLE).cos() + (point_x.x - v1.x) * (-ANGLE).sin();
        let v3 = Vec2::new(x, y);

        draw_line(node1.x, node1.y, node2.x, node2.y, 2.0, GRAY);
        draw_triangle(v1, v2, v3, GRAY);
    }

    let mut dragging_node_idx = None;

    let mut i = 0;

    loop {
        clear_background(BLACK);

        // draw edges
        graph.visit_edges(|_, node1, node2, _edge| {            
            draw_arrow(Vec2::new(node1.x(), node1.y()), Vec2::new(node2.x(), node2.y()));
        });

        // draw nodes
        graph.visit_nodes(|id, node| {

            if i < 100 {
                // println!("{:?} {:?}", id, (node.x(), node.y()));
            }

            draw_circle(node.x(), node.y(), NODE_RADIUS, WHITE);
            draw_text(
                &node.user_data().to_string(),
                node.x() + NODE_RADIUS,
                node.y() + NODE_RADIUS / 2.0,
                25.0,
                RED,
            );

            // highlight hovered or dragged node
            if node_overlaps_mouse_position(node)
                || dragging_node_idx
                    .filter(|idx| *idx == node.index())
                    .is_some()
            {
                draw_circle_lines(node.x(), node.y(), NODE_RADIUS, 2.0, RED);
            }
        });

        // drag nodes with the mouse
        if is_mouse_button_down(MouseButton::Left) {
            graph.visit_nodes_mut(|_, node| {
                if let Some(idx) = dragging_node_idx {
                    if idx == node.index() {
                        let (mouse_x, mouse_y) = mouse_position();
                        node.x = mouse_x;
                        node.y = mouse_y;
                    }
                } else if node_overlaps_mouse_position(node) {
                    dragging_node_idx = Some(node.index());
                }
            });
        } else {
            dragging_node_idx = None;
        }

        if i < 1000 { i += 1; }

        graph.update(get_frame_time());

        next_frame().await
    }

}

#[allow(unused)]
struct Solution;

mod solution;
