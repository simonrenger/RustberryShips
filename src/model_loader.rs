use engine_resources::MeshResource;
use std::path::Path;
use ::tobj;
use ::cgmath::Vector3;


pub fn load_model_from(path: &Path) -> MeshResource {
    let tobj_var = tobj::load_obj(path);
    if tobj_var.is_err() {
        println!("Could not load: {:?}", path);

        return MeshResource::new();
    }

    let mut mesh_resource = MeshResource::new();

    let (models, materials) = tobj_var.unwrap();

    // println!("# of models: {}", models.len());
    // println!("# of materials: {}", materials.len());
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        // println!("model[{}].name = \'{}\'", i, m.name);
        // println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        // println!("Size of model[{}].indices: {}", i, mesh.indices.len());
        for i in &mesh.indices {
            // println!("    idx[{}] = {}, {}, {}.", f, mesh.indices[3 * f],
            //     mesh.indices[3 * f + 1], mesh.indices[3 * f + 2]);
            let idx = *i as usize;
            mesh_resource.raw_vertices.push(Vector3{x: mesh.positions[idx*3], y: mesh.positions[idx*3+1], z: mesh.positions[idx*3+2]});
        }

        // Normals and texture coordinates are also loaded, but not printed in this example
        // println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        assert!(mesh.positions.len() % 3 == 0);
        for v in 0..mesh.positions.len() / 3 {
            // println!("    v[{}] = ({}, {}, {})", v, mesh.positions[3 * v],
            //     mesh.positions[3 * v + 1], mesh.positions[3 * v + 2]);
        }
    }
    // for (i, m) in materials.iter().enumerate() {
    //     println!("material[{}].name = \'{}\'", i, m.name);
    //     println!("    material.Ka = ({}, {}, {})", m.ambient[0], m.ambient[1], m.ambient[2]);
    //     println!("    material.Kd = ({}, {}, {})", m.diffuse[0], m.diffuse[1], m.diffuse[2]);
    //     println!("    material.Ks = ({}, {}, {})", m.specular[0], m.specular[1], m.specular[2]);
    //     println!("    material.Ns = {}", m.shininess);
    //     println!("    material.d = {}", m.dissolve);
    //     println!("    material.map_Ka = {}", m.ambient_texture);
    //     println!("    material.map_Kd = {}", m.diffuse_texture);
    //     println!("    material.map_Ks = {}", m.specular_texture);
    //     println!("    material.map_Ns = {}", m.normal_texture);
    //     println!("    material.map_d = {}", m.dissolve_texture);
    //     for (k, v) in &m.unknown_param {
    //         println!("    material.{} = {}", k, v);
    //     }
    // }

    mesh_resource
}
