use types::Triangulation3;

use std::io::{Write, BufWriter};
use std::fs::File;

fn write_3d_to_abaqus_format_impl<W: Write>(buf: BufWriter<W>, triangulation: &Triangulation3) {
    AbaqusWriter {
        writer: buf,
        triangulation: triangulation
    }.write();
}

pub fn write_3d_to_abaqus_format(path_to_file: &str, triangulation: &Triangulation3) {
    let f = File::create(path_to_file).expect(&format!("write_to_abaqus_format failed while opening file: {}", path_to_file));

    let buf = BufWriter::new(f);
    write_3d_to_abaqus_format_impl(buf, triangulation);
}


struct AbaqusWriter<'a, W: Write> {
    writer: BufWriter<W>,
    triangulation: &'a Triangulation3,
}

//todo: remove duplication from here for sure!
impl<'a, W: Write> AbaqusWriter<'a, W> {
    fn write(mut self) {
        self.write_header();
        self.write_nodes();
        self.write_elements();
        self.write_elset();
        self.write_footer();
    }

    fn write_header(&mut self) {
        let _ = self.writer.write("*Part, name=PART-1\n".as_bytes());
    }

    fn write_nodes(&mut self) {
        let _ = self.writer.write("*Node\n".as_bytes());
        for i in 0..self.triangulation.nodes().len() {
            let node = &self.triangulation.nodes()[i];
            let _ = self.writer.write(format!("{},\t{},\t{},\t{}\n", i + 1, node.x, node.y, node.z).as_bytes());
        }
    }

    fn write_elements(&mut self) {
        let _ = self.writer.write("*Element, type=C3D4\n".as_bytes());
        for i in 0..self.triangulation.elements().len() {
            let element = &self.triangulation.elements()[i];
            //abaqus uses ccw order instead of cw, writing nodes in order [cab] is required.
            let _ = self.writer.write(format!("{},\t{},\t{},\t{},\t{}\n", i + 1, element.index_d().0 + 1,element.index_c().0 + 1, element.index_b().0 + 1, element.index_a().0 + 1).as_bytes());
        }
    }

    fn write_elset(&mut self) {
        let _ = self.writer.write("*Elset, elset=M_1\n".as_bytes());

        let eles = &self.triangulation.elements();
        let mut written = 0;
        while written < eles.len() {
            if written > 0 {
                let _ = self.writer.write(",".as_bytes());
            }
            if written % 10 == 0 && written != 0 {
                let _ = self.writer.write("\n".as_bytes());
            }

            let _ = self.writer.write((written + 1).to_string().as_bytes());
            written += 1;
        }

        let _ = self.writer.write("\n".as_bytes());
    }

    fn write_footer(&mut self) {
        let _ = self.writer.write("*Solid Section, elset=M_1, material=M_1
1.,
*End Part
**
**
** ASSEMBLY
**
*Assembly, name=Assembly
**
*Instance, name=PART-1-1, part=PART-1
*End Instance
**
*End Assembly\n".as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;
    use std::io::{BufWriter};

    #[test]
    fn testing_bufwriter_and_string() {
        let nodes = vec![Point3::new(0., 0., 0.), Point3::new(100., 0.,0.), Point3::new(0., 100., 0.), Point3::new(0., 0., 100.), Point3::new(0., 0., -100.)];
        let eles = vec![Tetrahedron::new(&nodes, N3Index(0), N3Index(1), N3Index(2), N3Index(3)), Tetrahedron::new(&nodes, N3Index(0), N3Index(1), N3Index(2), N3Index(4))];
        let triangulation = Triangulation3::new_from_prebuilt_triangulation(nodes, eles);
        //let tr
        let mut s = String::new();

        write_3d_to_abaqus_format_impl(BufWriter::new(unsafe { s.as_mut_vec() }), &triangulation);

        let expected_file = "*Part, name=PART-1
*Node
1,	0,	0,	0
2,	100,	0,	0
3,	0,	100,	0
4,	0,	0,	100
5,	0,	0,	-100
*Element, type=C3D4
1,	2,	3,	4,	1
2,	5,	3,	2,	1
*Elset, elset=M_1
1,2
*Solid Section, elset=M_1, material=M_1
1.,
*End Part
**
**
** ASSEMBLY
**
*Assembly, name=Assembly
**
*Instance, name=PART-1-1, part=PART-1
*End Instance
**
*End Assembly
";

        assert_eq!(expected_file, s);
    }
}