/*
* @Author: otae
* @Date:   2017-07-22 10:57:53
* @Last Modified by:   otae
* @Last Modified time: 2017-07-23 00:56:52
*/

use float3::*;

#[derive(Clone, Copy)]
pub struct Transform {
	pub position: Float3,
	pub direction: Float3 
}

impl Transform {
	pub fn new () -> Transform {
		Transform {
			position: Float3 {x: 0.0, y: 0.0, z: 0.0},
			direction: Float3 {x: 0.0, y: 0.0, z: 1.0}
		}
	}
}
