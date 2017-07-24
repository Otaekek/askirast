/*
* @Author: otae
* @Date:   2017-07-20 19:07:13
* @Last Modified by:   otae
* @Last Modified time: 2017-07-22 21:55:17
*/

#[derive(Clone, Copy)]
pub struct Float2 {
	pub x: f32,
	pub y: f32
}

impl Float2 {
	pub fn GetCrossProduct(self, _rhs: Float2) -> f32 {
		self.x * _rhs.y - self.y * _rhs.x
	}
}