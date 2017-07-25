/*
* @Author: otae
* @Date:   2017-07-20 16:17:29
* @Last Modified by:   otae
* @Last Modified time: 2017-07-25 01:15:34
*/

use float3::*;
use Fragment::*;
use mesh::*;
use transform::*;

#[derive(Clone, Copy)]
pub struct GraphicObject<'a> {
	mesh: &'a Mesh,
	pub transform: Transform
}

impl<'a> GraphicObject<'a> {

	pub fn new (mesh: &'a Mesh, transform: Transform) -> GraphicObject {
		GraphicObject {
			transform: transform,
			mesh: mesh
		}
	}

	pub fn to_fragments(&self, camera: Transform) -> Vec<Fragment> {

		let mut fragments: Vec<Fragment>  = Vec::new();

		let mut i: usize = 0;
		while (i < self.mesh.vertices.len()) {
			let mut v: [(Float3, f32, f32, Float3); 3] = [self.mesh.vertices[i],
				self.mesh.vertices[i + 1],
				self.mesh.vertices[i + 2]];
			for i in 0 .. 3 {
				v[i].0 = v[i].0 + self.transform.position;
				v[i].0 = v[i].0 - camera.position;
				v[i].0.rotateFromDir(camera.direction);
				v[i].0.x /= v[i].0.length() * 1.3;
				v[i].0.y /= v[i].0.length();

			}
			fragments.push(Fragment{v1: v[0], v2: v[1], v3: v[2]});
			i += 3;
		}
		return fragments;
	}
}
