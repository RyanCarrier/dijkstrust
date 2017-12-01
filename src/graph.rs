use std::fmt;
use std::collections::LinkedList;
use vertex;
use std;

pub struct Graph {
    verticies: Vec<vertex::Vertex>,
}
impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "verticies:\n{:?}\n",
            self.verticies,
        )
    }
}

#[derive(Debug)]
pub struct BestPath {
    success: bool,
    distance: i64,
    path: Vec<u64>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { verticies: Vec::new() }
    }

    pub fn add_vertex(&mut self, v: vertex::Vertex) {
        self.verticies.push(v);
    }

    pub fn remove_vertex(&mut self, id: u64) {
        self.verticies.retain(|v| v.id != id)
    }

    fn setup(&mut self, source: u64) {
        for vertex in &mut self.verticies {
            vertex.best_distance = std::i64::MAX;
            vertex.best_verticie = 0;
        }
        self.verticies[source as usize].best_distance = 0;
        //set best distance to i64max
        //reset all best distance and vertex
    }

    fn best_path(&self, source: u64, destination: u64, touch_dest: bool) -> BestPath {
        let mut b = BestPath {
            success: touch_dest,
            distance: self.verticies[destination as usize].best_distance,
            path: Vec::new(),
        };
        //you can just fuck this stuff right off if you need to
        let mut vid = destination;
        b.path.push(vid);
        while vid != source {
            vid = self.verticies[vid as usize].best_verticie.clone();
            b.path.push(vid);
        }
        b.path.reverse();
        b
    }

    pub fn shortest(&mut self, source: u64, destination: u64) -> Option<BestPath> {
        let mut visiting = LinkedList::new();
        let mut touch_dest = false;
        self.setup(source);
        visiting.push_back(source);
        while !visiting.is_empty() {
            //for each node we are visiting, get it's current id, distance and arcs to other nodes
            let visitingid = visiting.pop_front().unwrap();
            let distance = self.verticies[visitingid as usize].best_distance;

            //let arcs = self.verticies[visitingid as usize].arcs.clone();
            //for each arc to other nodes, check if that path is the new best path to it
            //(from our visiting node)
            let mut i = 0;
            let l = self.verticies[visitingid as usize].arcs.len();
            while i < l {
                //arc in arcs {
                let id = self.verticies[visitingid as usize].arcs[i].to.clone();
                let dist = self.verticies[visitingid as usize].arcs[i].distance.clone();
                i += 1;
                let vertex = &mut self.verticies[id as usize];
                if id == visitingid || distance >= vertex.best_distance ||
                    distance + dist >= vertex.best_distance
                {
                    continue;
                }
                if id == destination {
                    touch_dest = true;
                }
                //if it is the new best node, make sure we update it's distance and best node, then
                //set it to be (re)visited later
                vertex.best_distance = distance + dist;
                vertex.best_verticie = visitingid;
                visiting.push_back(id);
            }
        }
        if !touch_dest {
            return None;
        }
        return Some(self.best_path(source, destination, touch_dest));
    }
}
