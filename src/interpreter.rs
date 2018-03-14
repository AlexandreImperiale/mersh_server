extern crate std;
extern crate mersh;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec::*;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Definition of data structures.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Definition of resources.
#[derive(Serialize, Deserialize)]
pub enum Resource {
    /// Unsigned integer.
    UInt(usize),
    /// Vector of floats.
    VecFloat(Vec<f64>),
    /// Mesh3d.
    Mesh3d(Box<mersh::mesh::Mesh3d>)
}

/// Definition of commands.
#[derive(Serialize, Deserialize)]
pub enum Cmd {
    /// Creating a new unsigned integer.
    NewUInt{ input: usize, output_id: String },
    /// Creating a new vector of floats.
    NewVecFloat{ input: Vec<f64>, output_id: String },
    /// Creating a new 3d mesh.
    NewMesh3d{ output_id: String },
    /// Pushing vertex into 3d mesh.
    PushVertex3d{ mesh_id: String, coords_id: String },
    /// Accessing vertex in 3d mesh.
    GetVertex3d{ mesh_id: String, idx_id: String, output_id: String }
}

/// Definition of an interpreter for mersh exposed commands.
#[derive(Default, Serialize, Deserialize)]
pub struct Interpreter {
    /// Mapping between resource id and resources.
    pub resources: HashMap<String, Resource>,
    /// Associated command history.
    pub cmd_history: VecDeque<Cmd>,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Interpreter {
    /// Applying a command. Once applied, commands are moved to interpreter history.
    ///
    /// * `cmd` - a command to be applied on resources.
    ///
    pub fn apply_cmd(&mut self, cmd: Cmd)
    {
        match cmd {
            Cmd::NewUInt { input, ref output_id } => self.new_uint(input, output_id),
            Cmd::NewVecFloat { ref input, ref output_id } => self.new_vec_float(input, output_id),
            Cmd::NewMesh3d { ref output_id } => self.new_mesh3d(output_id),
            Cmd::PushVertex3d { ref mesh_id, ref coords_id } => self.push_vertex3d(mesh_id, coords_id),
            Cmd::GetVertex3d { ref mesh_id, ref idx_id, ref output_id } => self.get_vertex3d(mesh_id, idx_id, output_id),
        }
        self.cmd_history.push_back(cmd);
    }

    /// Adding an unsigned integer in resources.
    /// This function panics if output id is already defined.
    ///
    /// * `input` - input unsigned integer to be added to interpreter resources.
    /// * `output_id` - associated (unique) output id.
    ///
    pub fn new_uint(&mut self, input: usize, output_id: &str)
    {
        self.panic_if_defined(output_id);
        self.resources.insert(String::from(output_id), Resource::UInt(input));
    }

    /// Adding a vector of floats in resources.
    ///
    /// * `input` - input slice of floats, to be copied and added to interpreter resources.
    /// * `output_id` - associated (unique) output id.
    ///
    pub fn new_vec_float(&mut self, input: &[f64], output_id: &str)
    {
        self.panic_if_defined(output_id);
        self.resources.insert(String::from(output_id), Resource::VecFloat(input.to_vec()));
    }

    /// Adding a new 3d mesh in resources.
    ///
    /// * `output_id` - associated (unique) id of the 3d mesh.
    ///
    pub fn new_mesh3d(&mut self, output_id: &str)
    {
        self.panic_if_defined(output_id);
        self.resources.insert(String::from(output_id), Resource::Mesh3d(Box::new(mersh::mesh::Mesh3d::default())));
    }

    /// Pushing a 3d vertex into a mesh.
    ///
    /// * `mesh_id` - id associated to the mesh.
    /// * `coords_id` - id associated to the coordinates of the vertex.
    ///
    /// This function panics if neither the `mesh_id` nor the `coords_id` are ids associated to
    /// actual resources in the interpreter.
    ///
    pub fn push_vertex3d(&mut self, mesh_id: &str, coords_id: &str)
    {
        let pnt;
        {
            let vec = self.get_mut_rsrc(coords_id).as_mut_vec_float();
            pnt = mersh::base::Pnt3d::new([vec[0], vec[1], vec[2]]);
        }
        self.get_mut_rsrc(mesh_id).as_mut_mesh3d().vertices.push(pnt);
    }

    /// Accessing a 3d vertex in a mesh.
    ///
    /// * `mesh_id` - id associated to the mesh.
    /// * `idx_id` - id associated to the index of the vertex.
    ///
    /// This function panics if neither the `mesh_id` nor the `idx_id` are ids associated to
    /// actual resources in the interpreter.
    ///
    pub fn get_vertex3d(&mut self, mesh_id: &str, idx_id: &str, output_id: &str)
    {
        let coords;
        {
            let idx = self.get_mut_rsrc(idx_id).as_mut_uint();
            let pnt = &self.get_mut_rsrc(mesh_id).as_mut_mesh3d().vertices[idx];
            coords = [pnt.coords.x, pnt.coords.y, pnt.coords.z];
        }
        self.new_vec_float(&coords, output_id);
    }
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// private Implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Resource {

    // Transforming mutable reference to resource into a mutable reference to an unsigned integer.
    // This function panics if resource does not reference an unsigned integer.
    fn as_mut_uint(&mut self) -> usize
    {
        if let Resource::UInt(uint) = *self { uint }
        else { panic!(Resource::wrong_conversion_err()) }
    }

    // Transforming mutable reference to resource into a mutable reference to a slice of floats.
    // This function panics if resource does not reference a vector of floats.
    fn as_mut_vec_float(&mut self) -> &mut [f64]
    {
        if let Resource::VecFloat(ref mut vec) = *self { vec }
        else { panic!(Resource::wrong_conversion_err()) }
    }

    // Transforming mutable reference to resource into a mutable reference to a 3d mesh.
    // This function panics if resource does not reference a 3d mesh.
    fn as_mut_mesh3d(&mut self) -> &mut mersh::mesh::Mesh3d
    {
        if let Resource::Mesh3d(ref mut mesh) = *self { mesh }
        else { panic!(Resource::wrong_conversion_err()) }
    }

    // Returning error message for wrong conversion.
    fn wrong_conversion_err() -> &'static str { "Wrong resource derived type conversion." }
}

impl Interpreter {

    // Checking if id is already defined. This function panics if id is already defined
    //  in the interpreter resources.
    fn panic_if_defined(&self, id: &str)
    {
        if self.resources.get(id).is_some() { panic!("Id already defined.") }
    }

    // Accessing mutable reference to a resource from its id.
    // This function panics if the input id does not corresponds to present resources in interpreter.
    fn get_mut_rsrc(&mut self, id: &str) -> &mut Resource
    {
        if let Some(rsrc) = self.resources.get_mut(id) { rsrc }
        else { panic!("Undefined id.") }
    }
}