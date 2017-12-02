mod vertex;
mod graph;

fn main() {
    let mut g = graph::Graph::new();
    for i in 0i64..10i64 {
        let mut v = vertex::Vertex::new(i as u64);
        for x in -2i64..3i64 {
            if x == 0 {
                continue;
            }
            if i + x >= 0 && i + x <= 9 {
                v.add_arc((i + x) as u64, 2i64 * ((x + 5i64) / 2i64));
            }
        }
        g.add_vertex(v);
    }
    print!("{:?}", g);
    print!("{:?}", g.shortest(0, 9));
}
