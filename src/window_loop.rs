/*
* @Author: otae
* @Date:   2017-07-20 16:17:26
* @Last Modified by:   otae
* @Last Modified time: 2017-07-23 17:22:59
*/

use graphic_object::*;
use ncurses::*;
use Fragment::*;
use frame_buffer::*;
use std::thread::sleep;
use std::time::*;
use mesh::*;
use transform::*;
use camera_controler::*;

pub fn main_loop (mut meshList: &Vec<Mesh>, mut gObjList: &mut Vec<GraphicObject>) {

	let mut height: i32 = 0;
	let mut width: i32 = 0;
	let mut cam: CameraControler = CameraControler::new();

	loop {

		getmaxyx(stdscr(), &mut height, &mut width);

		let mut frameBuffer: FrameBuffer = FrameBuffer{frame: Vec::new(), depth: Vec::new()};

		frameBuffer.allocate(width, height);
		cam.update();

		for i in 0 .. gObjList.len() {
			for x in gObjList[i].to_fragments(cam.transform) {
				x.rasterize(&mut frameBuffer);
			}
		//	camera.direction.y = a.cos();
		//	camera.direction.z = a.sin();

//			camera.direction.x = a.cos();
		//	camera.direction.y = a.cos();
//			camera.direction.z = a.sin();

//			camera.direction.normalize();
			//println!("{:?} {:?} {:?}", camera.direction.x, camera.direction.y, camera.direction.z);
//			gObjList[i].transform.position.y += 0.001;
		}
		for i in 0 .. width {
			for j in 0 .. height {
				let mut c: String = String::new();
				c.push(frameBuffer.frame[j as usize][i as usize]);
				mvprintw(j,i, &c);
			}
		}
		refresh();
		//sleep(Duration::from_millis(100));
	}
}
