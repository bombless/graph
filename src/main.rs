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

    loop {
        clear_background(BLACK);

        // draw edges
        graph.visit_edges(|node1, node2, _edge| {            
            draw_arrow(Vec2::new(node1.x(), node1.y()), Vec2::new(node2.x(), node2.y()));
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

        graph.update(get_frame_time());

        next_frame().await
    }

}

#[allow(unused)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    #[allow(unused)]
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut uf = UnionFind::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {

            }
        }
        unimplemented!()
    }
}

struct UnionFind(HashMap<(usize, usize), (usize, usize)>);

impl UnionFind {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn union(&mut self, l: (usize, usize), r: (usize, usize)) {
        if !self.0.contains_key(&l) {
            self.0.insert(l, l);
        }
        if !self.0.contains_key(&r) {
            self.0.insert(r, r);
        }
        let pl = self.find(l);
        let pr = self.find(r);
        if pl != pr {
            self.0.insert(pl, pr);
        }
    }
    fn find(&mut self, v: (usize, usize)) -> (usize, usize) {
        if !self.0.contains_key(&v) {
            self.0.insert(v, v);
        }
        *self.0.get(&v).unwrap()
    }
    fn groups(&mut self) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
        let mut ret = HashMap::new();
        let keys = self.0.keys().cloned().collect::<Vec<_>>();
        for g in keys {
            let p = self.find(g);
            if !ret.contains_key(&p) {
                ret.insert(p, Vec::new());
            }
            ret.get_mut(&p).unwrap().push(g);
        }
        ret
    }
}