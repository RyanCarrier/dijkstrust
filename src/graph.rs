
use std::fmt;
use std::collections::LinkedList;
use vertex;
use std;
use std::fs::File;
use std::io::Read;

#[derive(PartialEq)]
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
        //TODO: implement anti looping (on arc direct and on secondary to self.)
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

    pub fn import(source: &str) -> Option<Graph> {
        /*
        NODE to,dist to,dist
        NODE
        NODE
        */
        let mut g = Graph::new();
        for line in source.split("\n") {
            let mut items = line.trim().split(" ");
            let n = items.next();
            if n == None {
                continue;
            }
            let n = n.unwrap().parse::<u64>();
            if n.is_err() {
                return None;
            }
            let mut v = vertex::Vertex::new(n.unwrap());
            for item in items {
                //first item should be consumed by first next()
                let mut arc_raw = item.trim().split(",");
                let to = arc_raw.next();
                let distance = arc_raw.next();
                if to == None || distance == None {
                    return None;
                }
                let t = to.unwrap().parse::<u64>();
                let d = distance.unwrap().parse::<i64>();
                if t.is_err() || d.is_err() {
                    return None;
                }
                v.add_arc(t.unwrap() as u64, d.unwrap() as i64)
            }
            g.add_vertex(v);
        }
        Some(g)
    }

    pub fn import_file(file: &str) -> Option<Graph> {
        let mut data = String::new();
        let mut f = match File::open(file) {
            Ok(f) => f,
            Err(_f) => return None,
        };
        f.read_to_string(&mut data);
        return Graph::import(&*data);
    }

    pub fn export(self) -> String {
        let mut s = String::new();
        for v in self.verticies {
            s = format!("{}{}", s, v.id);
            for a in v.arcs {
                s = format!("{} {},{}", s, a.to, a.distance);
            }
            s = format!("{}\n", s);
        }
        s
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Graph::new(), Graph { verticies: Vec::new() });
    }

    #[test]
    fn test_add_remove_vertex() {
        let mut g = Graph::new();
        g.add_vertex(vertex::Vertex::new(1234));
        assert!(g != Graph::new());
        let mut v2 = vertex::Vertex::new(1);
        v2.add_arc(1234, 5);
        g.add_vertex(v2);
        assert_eq!(g.verticies[0], vertex::Vertex::new(1234));
        let arc = vertex::Arc {
            to: 1234,
            distance: 5,
        };
        assert_eq!(g.verticies[1].arcs[0], arc);
        g.remove_vertex(1234);
        assert_eq!(g.verticies.len(), 1);
        assert_eq!(g.verticies[0].arcs[0], arc);
    }

    #[test]
    fn test_import() {
        let a = Graph::import(
            "0 1,1
            1 2,1 4,1000
            2 5,1 4,10
            4 0,5 5,0
            5",
        ).unwrap();
        assert_eq!(a.verticies.len(), 5);
        assert_eq!(a.verticies[0].id, 0);
        assert_eq!(a.verticies[0].arcs.len(), 1);
        assert_eq!(a.verticies[0].arcs[0], vertex::Arc { to: 1, distance: 1 });
        assert_eq!(a.verticies[4].id, 5);
        assert_eq!(a.verticies[4].arcs.len(), 0);
        assert_eq!(a.verticies[2].arcs.len(), 2);
        assert_eq!(
            a.verticies[2].arcs[1],
            vertex::Arc {
                to: 4,
                distance: 10,
            }
        );
        let fail_strings = [
            "0 1,
1 2,1 4,1000
2 5,1 4,10
4 0,5 5,0
5",
            "0 ,1
1 2,1 4,1000
2 5,1 4,10
4 0,5 5,0
5",
            "0 1,1
a 2,1 4,1000
2 5,1 4,10
4 0,5 5,0
5",
        ];
        for f in fail_strings.iter() {
            assert_eq!(Graph::import(f), None);
        }
    }

    #[test]
    fn test_shortest_nopath() {}

    #[test]
    fn test_export() {
        let success_strings = [
            "0 1,4
1 2,1 4,1000
2 5,1 4,10
4 0,5 5,0
5",
            "0 2,1
1 2,1 4,1000
2 5,1 4,10
4 0,5 5,0
5",
            "0 1,1
6 2,1 4,1000
2 5,1 4,10
4 0,5 5,0
5",
        ];
        for s in success_strings.iter() {
            assert_eq!(
                Graph::import(s).unwrap().export().trim(),
                s.to_string().trim()
            );
        }
    }
}



#[cfg(all(test, feature = "unstable"))]
mod bench {
    //extern crate test;
    use test::Bencher;

    use graph::Graph;
    use vertex::Vertex;
    use rand;
    use std::fs::File;
    use std::fs;
    use std::io::Write;
    use std::path::Path;
    use std::error::Error;

    #[bench]
    fn bench_node_16(b: &mut Bencher) {
        benchn(16, b);
    }

    fn benchn(n: i64, b: &mut Bencher) {
        let base = "./test_files/".to_owned();
        let filename = generate(n, base);
        let mut g = match Graph::import_file(&filename).unwrap() {
            Ok(r) => r,
            Err(e) => println!("Error importing: {} {}", filename, e.description()),
        };
        b.iter(|| g.shortest(0, (n - 1) as u64));
    }

    fn generate(nodes: i64, mut path: String) -> String {
        if !path.ends_with("/") {
            path = format!("{}/", path);
        }
        fs::create_dir_all(&path);
        let filename = format!("{}graph{}nodes.txt", path, nodes);
        if Path::new(&filename).exists() {
            println!("File {} already exists, leaving.", filename);
            return filename;
        }
        let mut g = Graph::new();
        for i in 0..nodes as i64 {
            let mut v = Vertex::new(i as u64);
            for j in 0..nodes as i64 {
                if j == i {
                    continue;
                }
                let r = rand::random::<i64>();
                v.add_arc(
                    j as u64,
                    (2 * nodes - j) as i64 + (r / ((nodes * (nodes - j + 1)) as i64)),
                );
            }
            g.add_vertex(v);
        }
        write_to_file(&*g.export(), &filename);
        return filename;
    }

    fn write_to_file(source: &str, file: &str) {
        /*
        let mut f = match File::create(file) {
            Err(f) => {
                println!("Error writing to file '{}' {}", file, f);
                f;
            }
            Ok(f) => f,

            _ => println!("Nothing"),
        };
        let _r = match f.write_fmt("{}", source) {
            Ok(r) => r,
            Err(r) => {
                println!("error writing to file {} {}", file, r);
                return;
            }
        };*/

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut f = match File::create(&file) {
            Err(why) => panic!("couldn't create {}: {}", file, why.description()),
            Ok(f) => f,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match f.write_all(source.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", file, why.description()),
            Ok(_) => println!("successfully wrote to {}", file),
        }
    }
}
