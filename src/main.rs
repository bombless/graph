#![feature(const_trait_impl)]

use naive_force_graph::{ForceGraph, Node, NodeData, NodeId};
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
    println!("result {:?}", Solution::matrix_rank_transform(matrix));
    // let mut uf = Solution::union_find(&matrix);
    // let graph = Solution::force_graph(&matrix, &mut uf);
    // run_graph(graph).await;
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

    loop {
        clear_background(BLACK);

        // draw edges
        graph.visit_edges(|_, node1, node2, _edge| {            
            draw_arrow(Vec2::new(node1.x(), node1.y()), Vec2::new(node2.x(), node2.y()));
        });

        // draw nodes
        graph.visit_nodes(|_, node| {

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

        graph.update(get_frame_time());

        next_frame().await
    }

}

#[allow(unused)]
struct Solution;

use std::collections::HashMap;

struct OrderedVec<T, F>(Vec<T>, F);

impl<T: Eq, F: Fn(&T, &T)->bool> OrderedVec<T, F> {
    fn new(f: F) -> Self {
        Self(Vec::new(), f)
    }
    fn insert(&mut self, v: T) {
        for i in 0 .. self.0.len() {
            if v == self.0[i] {
                return;
            }
            if self.1(&self.0[i], &v) {
                self.0.insert(i, v);
                return;
            }
        }
        self.0.insert(self.0.len(), v);
    }
    fn iter(&self) -> impl Iterator<Item=&T> {
        self.0.iter()
    }
}

#[derive(Default)]
struct Pos((i32, (usize, usize)));

impl From<(i32, (usize, usize))> for Pos {
    fn from(x: (i32, (usize, usize))) -> Self {
        Self(x)
    }
}
impl ToString for Pos {
    fn to_string(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl Solution {
    #[allow(unused)]
    pub fn matrix_rank_transform(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut uf = Self::union_find(&matrix);
        let mut graph = Self::force_graph(&matrix, &mut uf);
        let groups = uf.groups();
        let (ranking, remaining) = Self::rank_groups(&matrix, &mut graph, &mut uf);
        let mut curr_rank = 1;
        for e in ranking {
            for &(i, j) in groups.get(&e).unwrap() {
                matrix[i][j] = curr_rank;
            }
            curr_rank += 1;
        }
        for (i, j) in remaining {
            matrix[i][j] = curr_rank;
        }
        matrix
    }
    fn force_graph(matrix: &Vec<Vec<i32>>, uf: &mut UnionFind) -> ForceGraph<Pos> {
        fn cmp(x: &(i32, (usize, usize)), y: &(i32, (usize, usize))) -> bool {
            x.0 > y.0
        }
        let m = matrix.len();
        let n = matrix[0].len();

        let mut graph = ForceGraph::new(Default::default());

        let mut map = HashMap::new();

        for i in 0..m {
            let mut v = OrderedVec::new(cmp);
            for j in 0..n {
                v.insert((matrix[i][j], uf.find((i, j))));
            }
            let mut last_id = None;

            for &e in v.iter() {
                let id = graph.add_node(NodeData {
                    x: screen_width() / 4.0,
                    y: screen_height() / 4.0,
                    user_data: e.into(),
                    ..Default::default()
                });
                map.insert(e.1, id);

                if let Some(last_id) = last_id {
                    graph.add_edge(last_id, id, Default::default());
                }

                last_id = Some(id);
            }
        }

        for j in 0..n {
            let mut v = OrderedVec::new(cmp);
            for i in 0..m {
                v.insert((matrix[i][j], uf.find((i, j))));
            }
            let mut last_id = None;

            for &e in v.iter() {

                let id = *map.get(&e.1).unwrap();

                if let Some(last_id) = last_id {
                    graph.add_edge(last_id, id, Default::default());
                }

                last_id = Some(id);
            }
        }
        graph
    }
    fn union_find(matrix: &Vec<Vec<i32>>) -> UnionFind {
        let mut uf = UnionFind::new();
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m {
            for j in 0..n {
                let value = matrix[i][j];
                for k in 0..m {
                    if value == matrix[k][j] {
                        uf.union((i, j), (k, j));
                    }
                }
                for k in 0..n {
                    if value == matrix[i][k] {
                        uf.union((i, j), (i, k));
                    }
                }

            }
        }
        uf
    }
    fn rank_groups(matrix: &Vec<Vec<i32>>, graph: &mut ForceGraph<Pos>, uf: &mut UnionFind)
    -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        fn find_min<'a>(matrix: &Vec<Vec<i32>>, graph: &ForceGraph<Pos>) -> Option<(NodeId, (usize, usize))> {
            if graph.edge_count() == 0 {
                return None;
            }
            let mut nodes = std::collections::HashMap::new();
            graph.visit_nodes(|id, node| {
                nodes.insert(node.user_data().0.1, id);
            });
            graph.visit_edges(|_, _node1, node2, _edge| {
                nodes.remove(&node2.user_data().0.1);
            });
            let mut ret = None;
            let mut min = std::i32::MAX;
            for (pair, id) in &nodes {
                let i = matrix[pair.0][pair.1];
                if min > i {
                    min = i;
                    ret = Some((*id, *pair))
                }
            }
            ret
        }
        fn query_node(head: (usize, usize), graph: &ForceGraph<Pos>) -> (NodeId, Vec<(usize, usize)>) {
            let mut id = None;
            let mut next = Vec::new();
            graph.visit_nodes(|_, node| {
                if node.user_data().0.1 == head {
                    id = Some(node.index());
                    next.push(node.user_data().0.1);
                }
            });
            (id.unwrap(), next)
        }
        let v = uf.values().collect::<Vec<_>>();
        let mut ret = Vec::new();

        while let Some((id, head)) = find_min(matrix, graph) {
            graph.remove_node(id);
            ret.push(head);
        }
        
        let mut remaining = Vec::new();
        graph.visit_nodes(|_, node| {
            remaining.push(node.user_data().0.1);
        });
        println!("(ret, remaining) {:?}", (&ret, &remaining));
        (ret, remaining)
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
    fn values(&self) -> impl Iterator<Item=&(usize, usize)> {
        self.0.values()
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