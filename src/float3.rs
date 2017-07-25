/*
* @Author: otae
* @Date:   2017-07-20 16:21:10
* @Last Modified by:   otae
* @Last Modified time: 2017-07-24 21:53:14
*/

use std::ops;

#[derive(Clone, Copy)]
pub struct Float3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}


impl Float3 {
	pub fn new() -> Float3 {
		Float3{x: 0.0, y: 0.0, z: 0.0}
	}

	pub fn rotateFromAngle(&mut self, rotX: f32, rotY: f32) {

		let mut tmp: Float3 = *self;

		tmp.x = 	self.x * rotY.cos()
						+ self.z * rotY.sin();
	
		tmp.y = 	self.y * rotX.cos()
						- self.z * rotX.sin();

		tmp.z = 	-self.x * rotY.sin()
						+ self.y * rotX.sin()
							+ self.z * rotY.cos();

		self.x = tmp.x;
		self.y = tmp.y;
		self.z = tmp.z;
	}

	pub fn rotateFromDir(&mut self, dir: Float3) {
		self.rotateFromAngle(dir.y.atan2(dir.z), dir.x.atan2(dir.z));
	}

	pub fn length(&self) -> f32 {
		return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
	}

	pub fn normalize(&mut self) {
		let l = self.length();

		self.x /= l;
		self.y /= l;
		self.z /= l; 
	}
}

impl ops::Add<Float3> for Float3 {

	type Output = Float3;

	fn add(self, _rhs: Float3) -> Float3 {
	
		let mut ret: Float3 = Float3::new();

		ret.x = self.x + _rhs.x;
		ret.y = self.y + _rhs.y;
		ret.z = self.z + _rhs.z;
		ret
	}
}

impl ops::Sub<Float3> for Float3 {

	type Output = Float3;

	fn sub(self, _rhs: Float3) -> Float3 {
	
		let mut ret: Float3 = Float3::new();

		ret.x = self.x - _rhs.x;
		ret.y = self.y - _rhs.y;
		ret.z = self.z - _rhs.z;
		ret
	}
}

impl ops::Add<Float3> for f32 {

	type Output = Float3;

	fn add(self, _rhs: Float3) -> Float3 {
	
		let mut ret: Float3 = Float3::new();

		ret.x = self + _rhs.x;
		ret.y = self + _rhs.y;
		ret.z = self + _rhs.z;
		ret
	}
}

impl ops::Mul<f32> for Float3 {

	type Output = Float3;

	fn mul(self, _rhs: f32) -> Float3 {
	
		let mut ret: Float3 = Float3::new();

		ret.x = self.x * _rhs;
		ret.y = self.y * _rhs;
		ret.z = self.z * _rhs;
		ret
	}
}