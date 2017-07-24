/*
* @Author: otae
* @Date:   2017-07-23 14:13:00
* @Last Modified by:   otae
* @Last Modified time: 2017-07-24 21:39:44
*/

use transform::*;
use ncurses::*;
use transform::*;

pub struct CameraControler {
	pub transform: Transform,
	pub eulerX: f32,
	pub eulerY: f32
}

impl CameraControler {
	pub fn new() -> CameraControler {
		let ret = CameraControler{transform: Transform::new(), eulerX: 0.0, eulerY: 0.0};
		ret
	}

	pub fn update(&mut self) {

		let mut c: i32 = getch();
		while c != -1 { 
			match c {
				258 => self.eulerX += 0.1,
				259 => self.eulerX -= 0.1,
				260 => self.eulerY += 0.1,
				261 => self.eulerY -= 0.1,
				97 => self.transform.position.y += 0.01,
				101 => self.transform.position.y -= 0.01,
				122 => self.transform.position.z += 0.01,
				115 => self.transform.position.z -= 0.01,
				113 => self.transform.position.x += 0.01,
				100 => self.transform.position.x -= 0.01,
				_ => ()
			}
			c = getch();
		}
		self.transform.direction.rotateFromAngle(self.eulerX, self.eulerY);
		self.eulerX = 0.0;
		self.eulerY = 0.0;
		self.transform.direction.normalize();
}
}