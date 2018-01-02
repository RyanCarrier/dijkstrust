
use std::fmt;
use std::collections::LinkedList;
use vertex;
use std;
use std::fs::File;
use std::io::Read;
use std::io;
use std::num;
use std::error;
use std::cmp::Ordering;
use std::collections::BinaryHeap;


#[derive(Debug)]
pub enum ImportError {
    Io(io::Error),
    Fmt(fmt::Error),
    ParseF(num::ParseFloatError),
    ParseI(num::ParseIntError),
    FileFormat,
}


impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            ImportError::Io(ref err) => write!(f, "IO error: {}", err),
            ImportError::Fmt(ref err) => write!(f, "Fmt error: {}", err),
            ImportError::ParseF(ref err) => write!(f, "Parse error: {}", err),
            ImportError::ParseI(ref err) => write!(f, "Parse error: {}", err),
            ImportError::FileFormat => {
                write!(f, "Format error: {}", "Issue with import file format")
            }
        }
    }
}

impl error::Error for ImportError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            ImportError::Io(ref err) => err.description(),
            ImportError::Fmt(ref err) => err.description(),
            ImportError::ParseF(ref err) => err.description(),
            ImportError::ParseI(ref err) => err.description(),
            ImportError::FileFormat => "Format error: Issue with import file format",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ImportError::Io(ref err) => Some(err),
            ImportError::Fmt(ref err) => Some(err),
            ImportError::ParseF(ref err) => Some(err),
            ImportError::ParseI(ref err) => Some(err),
            ImportError::FileFormat => None,
        }
    }
}



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

    pub fn export(&self) -> String {
        return (&self.verticies)
            .into_iter()
            .map(|v| v.export())
            .collect::<Vec<String>>()
            .join("\r\n");
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
        //let mut visiting = LinkedList::new();
        let destination = destination as usize;
        let mut visiting = BinaryHeap::new();
        let mut touch_dest = false;
        self.setup(source);
        visiting.push(source);
        while !visiting.is_empty() {
            //for each node we are visiting, get it's current id, distance and arcs to other nodes
            let visitingid = visiting.pop().unwrap() as usize;
            let distance = self.verticies[visitingid as usize].best_distance;

            //let arcs = self.verticies[visitingid as usize].arcs.clone();
            //for each arc to other nodes, check if that path is the new best path to it
            //(from our visiting node)
            let mut i = 0;
            let l = self.verticies[visitingid as usize].arcs.len();
            while i < l {
                //arc in arcs {
                let id = self.verticies[visitingid as usize].arcs[i].to as usize;
                let dist = self.verticies[visitingid as usize].arcs[i].distance;
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
                vertex.best_verticie = visitingid as u64;
                visiting.push(id as u64);
            }
        }
        if !touch_dest {
            return None;
        }
        return Some(self.best_path(source, destination as u64, touch_dest));
    }

    pub fn import(source: &str) -> Result<Graph, ImportError> {
        /*
        NODE to,dist to,dist
        NODE
        NODE
        */
        let mut g = Graph::new();
        for line in source.split("\n") {
            let mut items = line.trim().split(" ");
            let n = items.next();
            if n == None || n.unwrap().trim().is_empty() {
                continue;
            }
            let n = try!(n.unwrap().parse::<u64>().map_err(ImportError::ParseI));
            let mut v = vertex::Vertex::new(n);
            for item in items {
                if item.trim().is_empty() {
                    continue;
                }
                //first item should be consumed by first next()
                let mut arc_raw = item.trim().split(",");
                let to = arc_raw.next();
                let distance = arc_raw.next();
                if (to == None || to.unwrap().trim().is_empty()) ||
                    (distance == None || distance.unwrap().trim().is_empty())
                {
                    return Err(ImportError::FileFormat);
                }
                let t = try!(to.unwrap().parse::<u64>().map_err(ImportError::ParseI));
                let d = try!(distance.unwrap().parse::<i64>().map_err(
                    ImportError::ParseI,
                ));
                v.add_arc(t as u64, d as i64)
            }
            g.add_vertex(v);
        }
        Ok(g)
    }

    pub fn import_file(file: &str) -> Result<Graph, ImportError> {
        let mut data = String::new();
        let mut f = try!(File::open(file).map_err(ImportError::Io));

        try!(f.read_to_string(&mut data).map_err(ImportError::Io));
        return Graph::import(&*data);
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
    }

    #[test]
    #[should_panic(expected = r#"FileFormat"#)]
    fn test_import_fail1() {
        let fail_string1 = "0 1,
        1 2,1 4,1000
        2 5,1 4,10
        4 0,5 5,0
        5";
        Graph::import(fail_string1).unwrap();
    }

    #[test]
    #[should_panic(expected = r#"FileFormat"#)]
    fn test_import_fail2() {
        let fail_string2 = "0 ,1
        1 2,1 4,1000
        2 5,1 4,10
        4 0,5 5,0
        5";
        Graph::import(fail_string2).unwrap();
    }

    #[test]
    #[should_panic(expected = r#"InvalidDigit"#)]
    fn test_import_fail3() {
        let fail_string3 = "0 1,1
        a 2,1 4,1000
        2 5,1 4,10
        4 0,5 5,0
        5";
        Graph::import(fail_string3).unwrap();
    }



    #[test]
    fn test_shortest_nopath() {}

    #[test]
    fn test_export() {
        let success_strings = [
            "0 1,4\r
1 2,1 4,1000\r
2 5,1 4,10\r
4 0,5 5,0\r
5",
            "0 2,1\r
1 2,1 4,1000\r
2 5,1 4,10\r
4 0,5 5,0\r
5",
            "0 1,1\r
6 2,1 4,1000\r
2 5,1 4,10\r
4 0,5 5,0\r
5",
        ];
        for s in success_strings.iter() {
            assert_eq!(
                Graph::import(s).unwrap().export().trim().replace(" \r", ""),
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
    use std::io;

    #[bench]
    fn setup_node_4(b: &mut Bencher) {
        bench_setup_n(4, b);
    }
    #[bench]
    fn setup_node_256(b: &mut Bencher) {
        bench_setup_n(256, b);
    }
    #[bench]
    fn setup_node_4096(b: &mut Bencher) {
        bench_setup_n(4096, b);
    }


    #[bench]
    fn bench_0_node_4(b: &mut Bencher) {
        benchn(4, b);
    }
    #[bench]
    fn bench_1_node_16(b: &mut Bencher) {
        benchn(16, b);
    }
    #[bench]
    fn bench_2_node_64(b: &mut Bencher) {
        benchn(64, b);
    }
    #[bench]
    fn bench_3_node_256(b: &mut Bencher) {
        benchn(256, b);
    }
    #[bench]
    fn bench_4_node_1024(b: &mut Bencher) {
        benchn(1024, b);
    }
    #[bench]
    fn bench_5_node_4096(b: &mut Bencher) {
        benchn(4096, b);
    }

    fn bench_setup_n(n: i64, b: &mut Bencher) {
        let base = "./test_files/".to_owned();
        let filename = generate(n, base);
        let mut g = match Graph::import_file(&filename) {
            Ok(r) => r,
            Err(e) => {
                println!("Error importing: {} {}", filename, e);
                return;
            }
        };
        b.iter(|| g.setup(0));
    }

    fn benchn(n: i64, b: &mut Bencher) {
        let base = "./test_files/".to_owned();
        let filename = generate(n, base);
        let mut g = match Graph::import_file(&filename) {
            Ok(r) => r,
            Err(e) => {
                println!("Error importing: {} {}", filename, e);
                return;
            }
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
            //fs::remove_file(Path::new(&filename));
            //println!("File {} already exists, removing.", filename);
            //println!("File {} already exists, leaving.", filename);
            return filename;
        }
        print!("\nGenerating...");
        io::stdout().flush();
        let mut g = Graph::new();
        for i in 0..nodes as i64 {
            let mut v = Vertex::new(i as u64);
            for j in 0..nodes as i64 {
                if j == i {
                    continue;
                }
                //u32 to i64, to ensure >0
                let r = rand::random::<u32>() as i64 % ((nodes * (nodes - j + 1)) as i64);
                v.add_arc(j as u64, ((2 * nodes) - j) as i64 + (r));
            }
            g.add_vertex(v);
        }
        print!("✓\nExporting...");
        io::stdout().flush();
        let x = &*g.export();
        println!("✓");
        write_to_file(x, &filename);
        return filename;
    }

    fn write_to_file(source: &str, file: &str) {
        print!("Creating file...");
        io::stdout().flush();
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut f = match File::create(&file) {
            Err(why) => panic!("couldn't create {}: {}", file, why.description()),
            Ok(f) => f,
        };
        print!("✓\nWriting...");
        io::stdout().flush();
        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match f.write_all(source.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", file, why.description()),
            Ok(_) => println!("✓"),
        }
        io::stdout().flush();
    }
}
