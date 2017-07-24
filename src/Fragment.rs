/*
* @Author: otae
* @Date:   2017-07-20 18:48:46
* @Last Modified by:   otae
* @Last Modified time: 2017-07-23 19:35:39
*/

use float3::*;
use frame_buffer::*;
use float2::*;

fn min (x1: f32, x2: f32) -> i32 {
	if (x1 < x2) {
		return x1 as i32;
	}
	return x2 as i32;
}

fn max (x1: f32, x2: f32) -> i32 {
	if (x1 > x2) {
		return x1 as i32;
	}
	return x2 as i32;
}

pub struct Fragment {
	pub v1: (Float3, f32, f32, Float3),
	pub v2: (Float3, f32, f32, Float3),
	pub v3: (Float3, f32, f32, Float3)
}

fn verticeToScreenSpace (x: Float3, width: usize, height: usize) -> Float2 {

	let ret: Float2 = Float2{x: (0.5 + x.x) * width as f32, y: (0.5 + x.y) * height as f32};

	ret
}

impl Fragment {
	pub fn rasterize (self, fb: &mut FrameBuffer) {

		let vr1 = self.v1.0;
		let vr2 = self.v2.0;
		let vr3 = self.v3.0;

		let v1 = verticeToScreenSpace(vr1, fb.frame[0].len(), fb.frame.len());
		let v2 = verticeToScreenSpace(vr2, fb.frame[0].len(), fb.frame.len());
		let v3 = verticeToScreenSpace(vr3, fb.frame[0].len(), fb.frame.len());

		let maxX = max(v1.x, max(v2.x, v3.x) as f32);
		let minX = min(v1.x, min(v2.x, v3.x) as f32);
		let maxY = max(v1.y, max(v2.y, v3.y) as f32);
		let minY = min(v1.y, min(v2.y, v3.y) as f32);

		//println!("{:?} {:?} {:?} {:?} {:?} {:?}", v1.x, v1.y, v2.x, v2.y, v3.x, v3.y);
		let vs1 = Float2{x: v2.x - v1.x, y: v2.y - v1.y};
		let vs2 = Float2{x: v3.x - v1.x, y: v3.y - v1.y};
		for x in minX .. maxX {
			for y in minY .. maxY {
				if !(x > 0 && y > 0 && x < fb.frame[0].len() as i32 && y < fb.frame.len() as i32) {
					continue ;
				}

				let q: Float2 = Float2 {x: x as f32 - v1.x, y: y as f32 - v1.y};

				let s: f32 = q.GetCrossProduct(vs2) / vs1.GetCrossProduct(vs2);
				let t: f32 = vs1.GetCrossProduct(q) / vs1.GetCrossProduct(vs2);
				let d: f32 = vr1.z * t + (1.0 - t) * vr3.z + vr2.z * s + (1.0 - s) * vr3.z;
				if (d < fb.depth[y as usize][x as usize] && d > 0.0 && (s >= 0.0) && (t >= 0.0) && (s + t <= 1.0)) {
					if self.v1.3.z != 0.0 { 
						fb.frame[y as usize][x as usize] = 'X';
					}
					else {
						fb.frame[y as usize][x as usize] = 'O';
					}
					fb.depth[y as usize][x as usize] = d;
				}
			}
		}
	}
}

pub struct Fragments {
	pub fragments: Vec<Fragment>
}
