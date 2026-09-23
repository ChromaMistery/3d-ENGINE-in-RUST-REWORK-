use crate::obj::Triangle;
use glam::Vec3;
use tobj::{self, LoadOptions};

//carga el objeto solo con el path absoluto
pub fn load_obj(path: &str) -> Vec<Triangle> {
    // la carpeta tiene que estar fuera de src
    let models = tobj::load_obj(
        path,
        &LoadOptions {
            triangulate: true,    //triangular por si acaso
            ..Default::default()  //dejar todo default
        },
    )
    .expect("Error al cargar el objeto");

    let mut triangles: Vec<Triangle> = Vec::new();

    //por cada dato crudo iterar 3 veces en una vuelta para conseguir xyz y meterlo
    // al triangulo
    for m in models.0.iter().enumerate() {
        let mesh = &m.1.mesh;

        //nama cojemo un punto
        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize; //base 0
            let i1 = mesh.indices[i + 1] as usize; //base 3
            let i2 = mesh.indices[i + 2] as usize; //base 6
            //es como si los i definieran los limites de una sub lista
            // ejemplo, i0(x,y,z)0,1,2 i marca el punto cero, y sumando movemos ese punto
            // con (0 * 3) limitamos el tamano se la sub matriz para que tenga solo ese largo

            let p0 = Vec3::new(
                mesh.positions[i0 * 3],
                mesh.positions[i0 * 3 + 1],
                mesh.positions[i0 * 3 + 2],
            );

            let p2 = Vec3::new(
                mesh.positions[i1 * 3],
                mesh.positions[i1 * 3 + 1],
                mesh.positions[i1 * 3 + 2],
            );

            let p1 = Vec3::new(
                mesh.positions[i2 * 3],
                mesh.positions[i2 * 3 + 1],
                mesh.positions[i2 * 3 + 2],
            );

            triangles.push(Triangle::new(p0, p1, p2));
        }
    }

    triangles
}
