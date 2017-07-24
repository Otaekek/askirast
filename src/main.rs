/*
* @Author: otae
* @Date:   2017-07-20 16:17:30
* @Last Modified by:   otae
* @Last Modified time: 2017-07-23 19:34:11
*/

extern crate ncurses;

mod mesh;
mod window_loop;
mod graphic_object;
mod float3;
mod Fragment;
mod float2;
mod frame_buffer;
mod transform;
mod camera_controler;

use ncurses::*;
use window_loop::*;
use graphic_object::*;
use mesh::*;
use transform::*;
use float3::*;

fn init () {
	initscr();
	keypad(stdscr(), true);
	noecho();
	timeout(1);
}

fn main() {

	let mut meshList: Vec<Mesh> = Vec::new();
	let mut gObjList: Vec<GraphicObject> = Vec::new();

	let mut cube: Mesh = Mesh::new();

	cube.loadCube();
	meshList.push(cube);

	for i in 0 .. 10 {
		for j in 0 .. 10 {
			gObjList.push(GraphicObject::new(&meshList[0], Transform{position: Float3{x: 0.5 * j as f32, y: 0.5, z: 0.5 * i as f32}, direction: Float3::new()}));
		}
	}

	init();

	main_loop(&meshList, &mut gObjList);
	endwin();
}
