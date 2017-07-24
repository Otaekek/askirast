/*
* @Author: otae
* @Date:   2017-07-21 10:14:39
* @Last Modified by:   otae
* @Last Modified time: 2017-07-23 20:45:10
*/

use float3::*;

pub struct Mesh {
	pub vertices: Vec<(Float3, f32, f32, Float3)>,
}

impl Mesh {

	fn pushVertice(&mut self, position: Float3, uv: (f32, f32), normal: Float3) {
		self.vertices.push((position * 0.15, uv.0, uv.1, normal));
	}

	pub fn load(&mut self, path: String) {

	}

	pub fn new() -> Mesh {
		Mesh {
			vertices: Vec::new()
		}
	}
	pub fn loadCube(&mut self) {

		self.pushVertice(Float3{x: -1.0, y: -1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:-1.0});
		self.pushVertice(Float3{x: 1.0, y: -1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:-1.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:-1.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:-1.0});
		self.pushVertice(Float3{x: -1.0, y: 1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:-1.0});
		self.pushVertice(Float3{x: -1.0, y: -1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:-1.0});
	
		self.pushVertice(Float3{x: -1.0, y: -1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:1.0});
		self.pushVertice(Float3{x: 1.0, y: -1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:1.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:1.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:1.0});
		self.pushVertice(Float3{x: -1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:1.0});
		self.pushVertice(Float3{x: -1.0, y: -1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: 0.0, z:1.0});

		self.pushVertice(Float3{x: -1.0, y: 1.0, z: -1.0}, (0.0, 0.0), Float3{x: 0.0, y: 1.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: 1.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: 1.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: 1.0, z:0.0});
		self.pushVertice(Float3{x: -1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: 1.0, z:0.0});
		self.pushVertice(Float3{x: -1.0, y: 1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: 1.0, z:0.0});

		self.pushVertice(Float3{x: -1.0, y: -1.0, z: -1.0}, (0.0, 0.0), Float3{x: 0.0, y: -1.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: -1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: -1.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: -1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: -1.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: -1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: -1.0, z:0.0});
		self.pushVertice(Float3{x: -1.0, y: -1.0, z:1.0}, (0.0, 0.0), Float3{x: 0.0, y: -1.0, z:0.0});
		self.pushVertice(Float3{x: -1.0, y: -1.0, z:-1.0}, (0.0, 0.0), Float3{x: 0.0, y: -1.0, z:0.0});

		self.pushVertice(Float3{x: 1.0, y: -1.0, z:-1.0}, (0.0, 0.0), Float3{x: 1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: -1.0, z:1.0}, (0.0, 0.0), Float3{x:1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: 1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: 1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: 1.0, z:-1.0}, (0.0, 0.0), Float3{x: 1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: 1.0, y: -1.0, z:-1.0}, (0.0, 0.0), Float3{x: 1.0, y: 0.0, z:0.0});

		self.pushVertice(Float3{x: -1.0, y: -1.0, z:-1.0}, (0.0, 0.0), Float3{x: -1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: -1.0, y: -1.0, z:1.0}, (0.0, 0.0), Float3{x: -1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: -1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: -1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: -1.0, y: 1.0, z:1.0}, (0.0, 0.0), Float3{x: -1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: -1.0, y: 1.0, z:-1.0}, (0.0, 0.0), Float3{x: -1.0, y: 0.0, z:0.0});
		self.pushVertice(Float3{x: -1.0, y: -1.0, z:-1.0}, (0.0, 0.0), Float3{x: -1.0, y: 0.0, z:0.0});
	}
}
