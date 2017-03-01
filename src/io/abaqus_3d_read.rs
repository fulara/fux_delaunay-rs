use types::*;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;


pub fn load_3d_from_abaqus_format(path_to_file: &str) -> Result<Triangulation3, String> {
    let f = match File::open(path_to_file) {
        Ok(file) => file,
        Err(_) => return Err("failed to open file".to_owned())
    };

    let fbuf = BufReader::new(f);
    read_buff(fbuf)
}

fn read_buff<R: Read>(reader: BufReader<R>) -> Result<Triangulation3, String> {
    enum ParserState {
        BeforeNode,
        Nodes,
        Elements,
        AfterElements,
    };

    let mut state = ParserState::BeforeNode;

    let mut nodes: Vec<Point3> = vec![];
    let mut elements: Vec<Tetrahedron> = vec![];

    for line in reader.lines() {
        match line {
            Ok(line) =>
                match state {
                    ParserState::BeforeNode => {
                        if line.contains("*Node") {
                            state = ParserState::Nodes;
                        }
                    },
                    ParserState::Nodes => {
                        if line.contains("Element") {
                            state = ParserState::Elements
                        } else {
                            let split_line: Vec<_> = line.trim().split(',').collect();

                            if split_line.len() != 4 {
                                return Err(format!("line '{}' is not valid for a Node line", line))
                            } else {
                                let x: f64 = split_line[1].trim().parse().unwrap();
                                let y: f64 = split_line[2].trim().parse().unwrap();
                                let z: f64 = split_line[3].trim().parse().unwrap();

                                nodes.push(Point3::new(x, y, z));
                            }
                        }
                    }

                    ParserState::Elements => {
                        if line.contains("*") {
                            state = ParserState::AfterElements
                        } else {
                            let split_line: Vec<_> = line.trim().split(',').collect();

                            if split_line.len() != 5 {
                                return Err(format!("line '{}' is not valid for a elementLine line. has len of '{}'", line, split_line.len()))
                            } else {
                                let n1 = split_line[1].trim().parse::<usize>().unwrap() - 1usize;
                                let n2 = split_line[2].trim().parse::<usize>().unwrap() - 1usize;
                                let n3 = split_line[3].trim().parse::<usize>().unwrap() - 1usize;
                                let n4 = split_line[4].trim().parse::<usize>().unwrap() - 1usize;

                                elements.push(Tetrahedron::new(&nodes, N3Index(n1), N3Index(n2), N3Index(n3),N3Index(n4)));
                            }
                        }
                    }
                    _ => ()
                },
            _ => ()
        }
    }

    match state {
        ParserState::AfterElements | ParserState::Elements => Ok(Triangulation3::new_from_prebuilt_triangulation(nodes, elements)),
        _ => Err("failed to parse file".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::read_buff;
    use std::io::BufReader;
    use types::N3Index;

    #[test]
    fn test() {
        let s =
            "*Heading
** Job name: 3G-V1 Model name: 3G-V1
** Generated by: Abaqus/CAE Version 6.8-1
*Preprint, echo=NO, model=NO, history=NO, contact=NO
**
** PARTS
**
*Part, name=PART-1
*Node
    1,	0,	    0,    0
    2,	100,    0,      0
    3,	0,	    100,    0
    4,	0,	    0,    100
*Element, type=C3D4
    1,  1,  2,  3,  4
";
        let reader = BufReader::new(s.as_bytes());
        match read_buff(reader) {
            Ok(tr) => {
                assert_eq!(4, tr.nodes().len());

                assert_eq!(0., tr.nodes()[0].x);
                assert_eq!(0., tr.nodes()[0].y);
                assert_eq!(0., tr.nodes()[0].z);

                assert_eq!(100., tr.nodes()[1].x);
                assert_eq!(0., tr.nodes()[1].y);
                assert_eq!(0., tr.nodes()[1].z);

                assert_eq!(0., tr.nodes()[2].x);
                assert_eq!(100., tr.nodes()[2].y);
                assert_eq!(0., tr.nodes()[2].z);

                assert_eq!(0., tr.nodes()[3].x);
                assert_eq!(0., tr.nodes()[3].y);
                assert_eq!(100., tr.nodes()[3].z);

                assert_eq!(1, tr.elements().len());
                assert_eq!(N3Index(0), tr.elements()[0].index_a());
                assert_eq!(N3Index(1), tr.elements()[0].index_b());
                assert_eq!(N3Index(2), tr.elements()[0].index_c());
                assert_eq!(N3Index(3), tr.elements()[0].index_d());
            }
            Err(str) => panic!("Expected success here got err: '{}'", str)
        }
    }
}