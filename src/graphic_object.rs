/*
* @Author: otae
* @Date:   2017-07-20 16:17:29
* @Last Modified by:   otae
* @Last Modified time: 2017-07-20 19:04:52
*/

use float3::*;
use Fragment::*;

pub struct Mesh {
	pub positions: Vec<Float3>,
	pub uvs: Vec<(f32, f32)>,
	pub normals: Vec<Float3>
}

pub struct GraphicObject<'a> {
	mesh: &'a Mesh,
	position: Float3
}

impl<'a> GraphicObject<'a> {
	pub fn to_fragments(&self) -> Fragments{
		return Fragments{fragments: Vec::new()};
	}
}