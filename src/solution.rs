use super::Solution;
use naive_force_graph::{NodeData, ForceGraph, NodeId, Parameters};

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
pub struct Pos((i32, (usize, usize)));

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
    pub fn force_graph(matrix: &Vec<Vec<i32>>, uf: &mut UnionFind) -> ForceGraph<Pos> {
        fn cmp(x: &(i32, (usize, usize)), y: &(i32, (usize, usize))) -> bool {
            x.0 > y.0
        }
        let m = matrix.len();
        let n = matrix[0].len();

        let mut graph = ForceGraph::new(Parameters { ideal_distance: 200., ..Parameters::default() });

        let mut map = HashMap::new();

        for i in 0..m {
            let mut v = OrderedVec::new(cmp);
            for j in 0..n {
                v.insert((matrix[i][j], uf.find((i, j))));
            }
            let mut last_id = None;

            for &e in v.iter() {
                let id = graph.add_node(NodeData {
                    x: 100.,
                    y: 100.,
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
    pub fn union_find(matrix: &Vec<Vec<i32>>) -> UnionFind {
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
        //println!("(ret, remaining) {:?}", (&ret, &remaining));
        (ret, remaining)
    }
}

pub struct UnionFind(HashMap<(usize, usize), (usize, usize)>);

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
