extern crate mersh;

mod interpreter {

    use mersh::interpreter::*;

    #[test]
    fn apply_cmd() {

        let mut interpreter = Interpreter::default();

        interpreter.apply_cmd(Cmd::NewUInt{ input: 12, output_id: "MyInt".to_string() });
        if interpreter.resources.get(&"MyInt".to_string()).is_none() { assert!(false); }
    }

    #[test]
    fn new_uint_0() {

        let mut interpreter = Interpreter::default();
        let id = String::from("MyInt");

        interpreter.new_uint(12, &id);
        if let Some(rsrc) = interpreter.resources.get(&id) {
            match rsrc {
                &Resource::UInt(uint) => assert_eq!(uint, 12),
                _ => assert!(false)
                }
        } else { assert!(false) }
    }

    #[test]
    #[should_panic]
    fn new_uint_1() {

        let mut interpreter = Interpreter::default();

        let id = String::from("MyInt");
        interpreter.new_uint(12, &id);
        interpreter.new_uint(223, &id); // => panics here !
    }

    #[test]
    fn new_vec_float_0() {

        let mut interpreter = Interpreter::default();
        let id = String::from("MyId");

        interpreter.new_vec_float(&vec![0., 25.0, 6.0], &id);
        if let Some(rsrc) = interpreter.resources.get(&id) {
            match rsrc {
                &Resource::VecFloat(ref vec_float) => assert_eq!(vec_float[1], 25.0),
                _ => assert!(false)
            }
        } else { assert!(false) }
    }

    #[test]
    #[should_panic]
    fn new_vec_float_1() {

        let mut interpreter = Interpreter::default();

        let id = String::from("MyInt");
        interpreter.new_vec_float(&vec![0., 25.0, 6.0], &id);
        interpreter.new_vec_float(&vec![0., 9.0], &id); // => panics here !
    }

    #[test]
    fn new_mesh3d_0() {

        let mut interpreter = Interpreter::default();
        let id = String::from("MyMesh");

        interpreter.new_mesh3d(&id);
        if let Some(rsrc) = interpreter.resources.get(&id) {
            match rsrc {
                &Resource::Mesh3d(_) => assert!(true),
                _ => assert!(false)
            }
        } else { assert!(false) }
    }

    #[test]
    #[should_panic]
    fn new_mesh3d_1() {

        let mut interpreter = Interpreter::default();

        let id = String::from("MyMesh");
        interpreter.new_mesh3d(&id);
        interpreter.new_mesh3d(&id); // => panics here !
    }

    #[test]
    fn push_vertex3d() {

        let mut interpreter = Interpreter::default();

        let coords_id = String::from("MyCoords");
        interpreter.new_vec_float(&vec![0.0, 1.0, 2.0], &coords_id);

        let mesh_id = String::from("MyMesh");
        interpreter.new_mesh3d(&mesh_id);

        interpreter.push_vertex3d(&mesh_id, &coords_id);

        if let Some(rsrc) = interpreter.resources.get(&mesh_id) {
            match rsrc {
                &Resource::Mesh3d(ref mesh) => {
                        assert_eq!(mesh.vertices[0].coords.x, 0.0);
                        assert_eq!(mesh.vertices[0].coords.y, 1.0);
                        assert_eq!(mesh.vertices[0].coords.z, 2.0);
                    },
                _ => assert!(false)
            }
        } else { assert!(false) }
    }

    #[test]
    fn get_vertex3d() {

        let mut interpreter = Interpreter::default();

        let mesh_id = String::from("MyMesh");
        interpreter.new_mesh3d(&mesh_id);

        let coords_id = String::from("MyCoords");
        interpreter.new_vec_float(&vec![6.0, 1.0, 2.0], &coords_id);

        interpreter.push_vertex3d(&mesh_id, &coords_id);

        let idx_id = String::from("MyId");
        interpreter.new_uint(0, &idx_id);

        let output_id = String::from("MyVertex");
        interpreter.get_vertex3d(&mesh_id, &idx_id, &output_id);

        if let Some(rsrc) = interpreter.resources.get(&output_id) {
            match rsrc {
                &Resource::VecFloat(ref vec) => {
                        assert_eq!(vec[0], 6.0);
                        assert_eq!(vec[1], 1.0);
                        assert_eq!(vec[2], 2.0);
                    },
                _ => assert!(false)
            }
        } else { assert!(false) }
    }
}
