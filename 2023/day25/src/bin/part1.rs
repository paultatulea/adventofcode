use std::collections::{HashMap, VecDeque};
use std::cmp::min;

#[allow(dead_code)]
const INPUT: &'static str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

#[allow(dead_code)]
const EXPECTED: &'static str = "54";


fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

// Implementation of Edmonds Karp to determine the maximum flow / minimum cut
fn bfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<usize>, capacity: &mut Vec<Vec<i64>>, flow: &mut Vec<Vec<i64>>, source: usize, sink: usize, visited_token: usize) -> i64 {
    let mut q = VecDeque::with_capacity(graph.len());
    q.push_back(source);
    visited[source] = visited_token;
    // Store previous node to recreate the path
    let mut prev = vec![None; graph.len()];
    while let Some(node) = q.pop_front() {
        if node == sink {
            break;
        }
        for &neighbor in graph[node].iter() {
            let cap = capacity[node][neighbor] - flow[node][neighbor];
            if cap > 0 && visited[neighbor] != visited_token {
                visited[neighbor] = visited_token;
                prev[neighbor] = Some(node);
                q.push_back(neighbor);
            }
        }
    }

    // Base case: unreachable sink
    if prev[sink].is_none() {
        return 0;
    }

    let mut bottle_neck = i64::MAX;
    let mut curr = sink;
    while let Some(prev_node) = prev[curr] {
        bottle_neck = min(bottle_neck, capacity[prev_node][curr] - flow[prev_node][curr]);
        curr = prev_node;
    }
    let mut curr = sink;
    while let Some(prev_node) = prev[curr] {
        flow[prev_node][curr] += bottle_neck;
        flow[curr][prev_node] -= bottle_neck;
        curr = prev_node;
    }
    return bottle_neck;
}

fn solve(graph: &Vec<Vec<usize>>, source: usize, sink: usize) -> (i64, Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let num_nodes = graph.len();
    let mut capacity: Vec<Vec<i64>> = Vec::new();
    for node in graph.iter() {
        capacity.push(vec![1; node.len()]);
    }
    let mut visited_token = 0;  // Can increment to mark nodes as unvisited in current iteration.
    let mut visited: Vec<usize> = vec![visited_token; graph.len()];
    let mut flow_matrix = vec![vec![0; num_nodes]; num_nodes];
    let mut capacity_matrix = vec![vec![0; num_nodes]; num_nodes];
    for (node_id, neighbors) in graph.iter().enumerate() {
        for (&neighbor, &cap) in neighbors.iter().zip(capacity[node_id].iter()) {
            capacity_matrix[node_id][neighbor] = cap;
        }
    }

    let mut max_flow = 0i64;
    let mut flow = -1i64;
    while flow != 0 {
        visited_token += 1;
        flow = bfs(&graph, &mut visited, &mut capacity_matrix, &mut flow_matrix, source, sink, visited_token);
        max_flow += flow;
    }
    (max_flow, flow_matrix, capacity_matrix)
}

fn min_cut(graph: &Vec<Vec<usize>>, saturated_edges: &Vec<(usize, usize)>, source: usize, sink: usize) -> Vec<(usize, usize)> {
    // We know that we must cut three edges because all edges have capacity 1.
    // Determine three edge cuts that results in 0 flow.
    for i in 2..saturated_edges.len() {
        for j in 1..i {
            for k in 0..j {
                let edge1 = saturated_edges[i];
                let edge2 = saturated_edges[j];
                let edge3 = saturated_edges[k];
                let mut new_graph = graph.clone();
                let idx1 = new_graph[edge1.0].iter().position(|&x| x == edge1.1).unwrap();
                let idx2 = new_graph[edge2.0].iter().position(|&x| x == edge2.1).unwrap();
                let idx3 = new_graph[edge3.0].iter().position(|&x| x == edge3.1).unwrap();
                new_graph[edge1.0].swap_remove(idx1);
                new_graph[edge2.0].swap_remove(idx2);
                new_graph[edge3.0].swap_remove(idx3);
                let (flow, _, _) = solve(&new_graph, source, sink);
                if flow == 0 {
                    return vec![edge1, edge2, edge3];
                }
            }
        }
    }
    return vec![];
}

fn count_nodes(graph: &Vec<Vec<usize>>, start: usize) -> usize {
    let mut visited = vec![0; graph.len()];
    let mut q = VecDeque::new();
    q.push_front(start);
    while let Some(node) = q.pop_front() {
        if visited[node] == 1 {
            continue;
        }
        visited[node] = 1;
        for &neighbor in graph[node].iter() {
            q.push_back(neighbor);
        }
    }
    return visited.iter().sum();
}

fn solution(s: &str) -> String {
    let mut node_map: HashMap<&str, usize> = HashMap::new();
    let graph: Vec<Vec<usize>> = s.lines().fold(Vec::new(), |mut acc, line| {
        // Map all nodes to a number for adjacency matrices
        let mut parts = line.split(": ");
        let left = parts.next().unwrap();
        let left_id: usize;
        if let Some(&id) = node_map.get(left) {
            left_id = id;
        } else {
            left_id = node_map.len();
            node_map.insert(left, left_id);
            acc.push(Vec::new());
        }
        let right: Vec<_> = parts.next().unwrap().split_whitespace().map(|node| {
            let node_id: usize;
            if let Some(&id) = node_map.get(node) {
                node_id = id;
            } else {
                node_id = node_map.len();
                node_map.insert(node, node_id);
                acc.push(Vec::new());
            }
            node_id
        }).collect();
        for right_id in right {
            acc[right_id].push(left_id);
            acc[left_id].push(right_id);
        }
        acc
    });

    let inv_node_map = node_map.iter().fold(HashMap::new(), |mut acc, (node, node_id)| {
        acc.insert(node_id, node);
        acc
    });

    let num_nodes = graph.len();
    println!("{num_nodes}");

    let mut ans = 0;
    let source = 0;
    for sink in 1..graph.len() {
        let (max_flow, cap_mat, flow_mat) = solve(&graph, source, sink);
        if max_flow == 3 {
            // Determine the saturated edges
            let mut saturated_edges = Vec::new();
            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    if cap_mat[i][j] > 0 && cap_mat[i][j] == flow_mat[i][j] {
                        saturated_edges.push((i, j));
                    }
                }
            }
            let min_cut_edges = min_cut(&graph, &saturated_edges, source, sink);
            // Determine the size of each disconnected group by counting nodes reachable from source & sink
            let mut new_graph = graph.clone();
            for (from, to) in min_cut_edges {
                println!("Cutting edge {} <-> {}", inv_node_map.get(&from).unwrap(), inv_node_map.get(&to).unwrap());
                let idx = new_graph[from].iter().position(|&x| x == to).unwrap();
                new_graph[from].swap_remove(idx);
                let idx2 = new_graph[to].iter().position(|&x| x == from).unwrap();
                new_graph[to].swap_remove(idx2);
            }
            let n1 = count_nodes(&new_graph, source);
            let n2 = count_nodes(&new_graph, sink);
            ans = n1 * n2;
            println!("n1 = {n1}, n2 = {n2}, ans = {ans}");
            break;
        }
    }
    return ans.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
