#![feature(const_trait_impl)]

use force_graph::{ForceGraph, Node, NodeData};
use macroquad::prelude::*;


#[allow(unused)]
fn from_array<const M: usize, const N: usize>(arr: [[i32; N]; M]) -> Vec<Vec<i32>> {
    IntoIterator::into_iter(arr).map(Vec::from).collect()
}

#[allow(unused)]
fn get_data() -> Vec<Vec<i32>> {
    from_array([[1,2],[3,4]])
}

#[allow(unused)]
fn test_graph() -> ForceGraph::<usize> {

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
        x: screen_width() / 4.0,
        y: 3.0 * screen_height() / 4.0,
        user_data: 3,
        ..Default::default()
    });
    let n4_idx = graph.add_node(NodeData {
        x: 3.0 * screen_width() / 4.0,
        y: 3.0 * screen_height() / 4.0,
        user_data: 4,
        ..Default::default()
    });
    let n5_idx = graph.add_node(NodeData {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        user_data: 5,
        is_anchor: true,
        ..Default::default()
    });

    // set up links between nodes
    graph.add_edge(n1_idx, n5_idx, Default::default());
    graph.add_edge(n2_idx, n5_idx, Default::default());
    graph.add_edge(n3_idx, n5_idx, Default::default());
    graph.add_edge(n4_idx, n5_idx, Default::default());

    graph
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

    let graph = init_graph();
    run_graph(graph).await;
}

async fn run_graph(mut graph: ForceGraph::<usize>) {

    const NODE_RADIUS: f32 = 15.0;

    fn node_overlaps_mouse_position(node: &Node<usize>) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        ((node.x() - mouse_x) * (node.x() - mouse_x) + (node.y() - mouse_y) * (node.y() - mouse_y))
            < NODE_RADIUS * NODE_RADIUS
    }

    fn draw_arrow(node1: Vec2, node2: Vec2) -> String {
        const ANGLE: f32 = 0.6;
        fn get_value<F: Fn(Vec2)->f32>(f: F, node1: Vec2, node2: Vec2) -> f32 {
            f(node2) + NODE_RADIUS * (f(node1) - f(node2)) /
                ((node1.x - node2.x).powi(2) + (node1.y - node2.y).powi(2)).sqrt()
        }
        fn get_value2<F: Fn(Vec2)->f32>(f: F, node1: Vec2, node2: Vec2) -> f32 {
            f(node2) + 2. * NODE_RADIUS * (f(node1) - f(node2)) /
                ((node1.x - node2.x).powi(2) + (node1.y - node2.y).powi(2)).sqrt()
        }
        // fn get_point2_x(node1: Vec2, node2: Vec2, point1: Vec2) -> f32 {
        //     point1.x -
        //     (
        //         4. / 3. * NODE_RADIUS
        //         -
        //         (
        //             (node2.x - point1.x) / (node2.y - point1.y) * ()
        //         ).powi(2)
        //     ).sqrt()
        // }
        let point1 = Vec2::new(get_value(|x| x.x, node1, node2), get_value(|x| x.y, node1, node2));
        let v1 = point1;
        draw_circle(point1.x, point1.y, NODE_RADIUS / 6., BLUE);
        let point_x = Vec2::new(get_value2(|x| x.x, node1, node2), get_value2(|x| x.y, node1, node2));
        draw_circle(point_x.x, point_x.y, NODE_RADIUS / 6., BLUE);

        let x = point1.x + (point_x.x - point1.x) * ANGLE.cos() - (point_x.y - point1.y) * ANGLE.sin();
        let y = point1.y + (point_x.y - point1.y) * ANGLE.cos() + (point_x.x - point1.x) * ANGLE.sin();
        let v2 = Vec2::new(x, y);


        draw_circle(x, y, NODE_RADIUS / 6., RED);

        let x = point1.x + (point_x.x - point1.x) * (-ANGLE).cos() - (point_x.y - point1.y) * (-ANGLE).sin();
        let y = point1.y + (point_x.y - point1.y) * (-ANGLE).cos() + (point_x.x - point1.x) * (-ANGLE).sin();
        let v3 = Vec2::new(x, y);

        draw_circle(x, y, NODE_RADIUS / 6., RED);

        draw_triangle(v1, v2, v3, GRAY);

        format!("point_x = {:?}\npoint1 = {:?}\nx, y = {} {}", point_x, point1, x, y)
    }

    let mut dragging_node_idx = None;

    let mut count = 0;

    loop {
        clear_background(BLACK);

        // draw edges
        graph.visit_edges(|node1, node2, _edge| {
            if count == 100 {
                println!("{:?}", Vec2::new(node2.x(), node2.y()));
            }
            draw_line(node1.x(), node1.y(), node2.x(), node2.y(), 2.0, GRAY);
        });

        // draw nodes
        graph.visit_nodes(|node| {

            draw_circle(node.x(), node.y(), NODE_RADIUS, WHITE);
            draw_text(
                &node.data.user_data.to_string(),
                node.x() - NODE_RADIUS / 2.0,
                node.y() + NODE_RADIUS / 2.0,
                25.0,
                BLACK,
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

        graph.visit_edges(|node1, node2, _edge| {
            let x = node2.x();
            let y = node2.y();
            let v1 = Vec2::new(x - NODE_RADIUS, y);
            let v2 = Vec2::new(x - NODE_RADIUS - NODE_RADIUS, y - NODE_RADIUS / 2.);
            let v3 = Vec2::new(x - NODE_RADIUS - NODE_RADIUS, y + NODE_RADIUS / 2.);
            draw_triangle(v1, v3, v2, GRAY);
            let v = draw_arrow(Vec2::new(node1.x(), node1.y()), Vec2::new(node2.x(), node2.y()));
            // draw_circle(x - NODE_RADIUS, y, NODE_RADIUS, RED);
            // draw_circle(x - NODE_RADIUS, y - NODE_RADIUS, NODE_RADIUS, YELLOW);
            // draw_circle(x - NODE_RADIUS, y + NODE_RADIUS, NODE_RADIUS, GREEN);
            if count == 100 {
                println!("{:?}", [v1, v2, v3]);
                println!("{}", v);
            }
        });

        // drag nodes with the mouse
        if is_mouse_button_down(MouseButton::Left) {
            graph.visit_nodes_mut(|node| {
                if let Some(idx) = dragging_node_idx {
                    if idx == node.index() {
                        let (mouse_x, mouse_y) = mouse_position();
                        node.data.x = mouse_x;
                        node.data.y = mouse_y;
                    }
                } else if node_overlaps_mouse_position(node) {
                    dragging_node_idx = Some(node.index());
                }
            });
        } else {
            dragging_node_idx = None;
        }

        if count <= 1000 {
            count += 1;
        }

        graph.update(get_frame_time());

        next_frame().await
    }

}

#[allow(unused)]
struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        matrix
    }
}
