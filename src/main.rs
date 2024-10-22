#![feature(autodiff)]

use glam::Vec3;


#[autodiff(autodiff_sdf, Forward, Dual, Dual)]
fn sdf(at: Vec3, res: &mut f32) {
    let sphere_one = at.length() - 1.0;
    let sphere_two = (at - Vec3::X).length() - 1.0;
    let un = sphere_one.min(sphere_two);
    *res = un;
}

fn main() {

    let at = Vec3::splat(2.0);

    let x: Vec3 = Vec3::from(at.clone());
    let mut y = 0.0;

    //wrt x
    let dx0 = Vec3::new(1.0, 0.0, 0.0);
    let mut dy0 = 0.0;
    autodiff_sdf(x, dx0, &mut y, &mut dy0);

    //wrt y
    let dx1 = Vec3::new(0.0, 1.0, 0.0);
    let mut dy1 = 0.0;
    autodiff_sdf(x, dx1, &mut y, &mut dy1);

    //wrt z
    let dx2 = Vec3::new(0.0, 0.0, 1.0);
    let mut dy2 = 0.0;
    autodiff_sdf(x, dx2, &mut y, &mut dy2);

    println!("x={at}, dy=[{dy0}, {dy1}, {dy2}]");
}
