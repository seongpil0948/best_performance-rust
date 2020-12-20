/// A compact graph representation. Edges are numbered in order of insertion.
/// Each adjacency list consists of all edges pointing out from a given vertex.
pub struct Graph {
    /// Maps a vertex id to the first edge in its adjacency list.
    first: Vec<Option<usize>>,
    /// Maps an edge id to the next edge in the same adjacency list.
    next: Vec<Option<usize>>,
    /// Maps an edge id to the vertex that it points to.
    endp: Vec<usize>,
}

impl Graph {
    /// init Graph for Reduce
    /// vmax: num of vertex, amax: num of Arcs
    pub fn new(vmax: usize, amax: usize) -> Self {
        Self {
            first: vec![None; vmax],
            next: Vec::with_capacity(amax),
            last: Vec::with_capacity(amax),
        }
    }
    /// Return Number of Vertices
    pub fn num_v(&self) -> usize {
        self.first.len()
    } 
    /// Return Number of Arcs
    pub fn num_arcs(&self) -> usize {
        self.last.len()
    }

    // direct arc from u to v
    pub fn add_arc(&mut self, u: usize, v: usize) {
        self.next.push(self.first[u]);
        self.first[u] = Some(self.num_arcs());
        self.endp(v)
    }

    /// all arcs are added via this func
    pub fn add_undirected_arc(&mut self, u: usize, v: usize) {
        self.add_arc(u, v);
        self.add_arc(v, u);
    }
    
    /// 2-SAT(2-SATisfiability)
    /// https://sevity.tistory.com/152
    /// If we think of each even-numbered vertex as a variable, and its
    /// odd-numbered successor as its negation, then we can build the
    /// implication graph corresponding to any 2-CNF formula.
    /// Note that u||v == !u -> v == !v -> u./// 
    pub fn add_two_sat_clause(&mut self, u: usize, v: usize) {
        self.add_edge(u ^ 1, v);
        self.add_edge(v ^ 1, u);
    }/// 
}