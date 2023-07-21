mod vector;
mod matrix;

use matrix::Mat4;
use vector::Vec4;
use anyhow::{anyhow, Result};

type Float = vector::Float;

enum CastResult {
    Hit(Float),
    Miss
}

fn translate(position: &Vec4, v: &Vec4) -> Vec4 {
    position - v
}

fn sphere(radius: Float, p: &Vec4) -> Float {
    p.mag() - radius
}

fn cuboid(dimensions: &Vec4, p: &Vec4) -> Float {
    let q = &p.abs() - dimensions;

    let v = Vec4::position(
        Float::max(q.x, 0.0),
        Float::max(q.y, 0.0),
        Float::max(q.z, 0.0)
    );

    v.mag() - 0.1
}

// As used by Media Molecule, apparently
fn smooth_union(d1: Float, d2: Float, k: Float) -> Float {
    let h = Float::max( k - Float::abs(d1 - d2), 0.0) / k;
    Float::min(d1, d2) - h * h * k * (1.0 / 4.0)
}

fn union(d1: Float, d2: Float) -> Float {
    Float::min(d1, d2)
}

fn sdf(position: &Vec4) -> Float {
    union(
        union(
            smooth_union(
                sphere(1.0, &translate(position, &Vec4::position(-3.0, -1.0, 0.0))),
                sphere(1.0, &translate(position, &Vec4::position(-2.0, 1.0, 0.0))),
                1.5
            ),
            smooth_union(
                sphere(1.0, &translate(position, &Vec4::position(2.0, -1.0, 0.0))),
                sphere(1.0, &translate(position, &Vec4::position(3.0, 1.0, 0.0))),
                2.0
            )
        ),
        union(
            sphere(500.0, &translate(position, &Vec4::position(0.0, -505.0, 0.0))),
            smooth_union(
                cuboid(&Vec4::direction(0.5, 0.5, 0.5), position),
                cuboid(&Vec4::direction(1.0, 1.0, 1.0), &translate(position, &Vec4::direction(1.0, 1.0, 1.0))),
                1.0
            )
        )
    )
}

// iquilez-derived, I'm sure I could do this myself, but...
fn calc_normal(position: &Vec4) -> Vec4 {
    let tiny = 0.5773 * 0.005;

    let xyy = Vec4::position(tiny, -tiny, -tiny);
    let yyx = Vec4::position(-tiny, -tiny, tiny);
    let yxy = Vec4::position(-tiny, tiny, -tiny);
    let xxx = Vec4::position(tiny, tiny, tiny);

    let v1 = xyy.scale(sdf(&(position + &xyy)));
    let v2 = yyx.scale(sdf(&(position + &yyx)));
    let v3 = yxy.scale(sdf(&(position + &yxy)));
    let v4 = xxx.scale(sdf(&(position + &xxx)));

    (&(&(&v1 + &v2) + &v3) + &v4).normalized().as_direction()
}

fn cast_ray(position: &Vec4, ray: &Vec4) -> CastResult {

    let t_min: Float = 1.0;
    let t_max: Float = 200.0;
    let iter_max = 50;

    let mut t = t_min;
    for _ in 1..iter_max {
        let dist = sdf(&(position + &ray.scale(t)));
        if dist < 0.0001 * t {
            return CastResult::Hit(t);
        }
        t += dist;
        if t > t_max {
            return CastResult::Miss;
        }
    }

    CastResult::Hit(t)
}

fn illuminate(position: &Vec4, normal: &Vec4) -> Float {
    let light_pos = Vec4::position(300.0, 500.0, -300.0);
    let min = 0.1;

    let light_dir = (&light_pos - position).normalized();
    let shadow = cast_ray(position, &light_dir);
    match shadow {
        CastResult::Hit(_) => min,
        CastResult::Miss => light_dir.dot_product(normal).clamp(min, 1.0)
    }
}

fn main() -> Result<()> {

    let (xsize, ysize) = (1920, 1080);
    //let (xsize, ysize) = (10,10);
    let mut image = image::RgbImage::new(xsize, ysize);

    let res = Vec4::position(xsize as Float, ysize as Float, 0.0);
    let scale = 1.0 / ysize as Float;

    let position = Vec4::position(5.0, 5.0, -10.0);
    let look_at = Vec4::position(0.0, 0.0, 0.0);
    let camera = Mat4::look(&position, &look_at);

    for y in 0..ysize {
        for x in 0..xsize {
            // Convert render coord to (-1, -1) -> (1,1)

            let render_pos = Vec4::position(x as Float, y as Float, 0.0);
            let mut pos = (&render_pos.scale(2.0) - &res).scale(scale);
            pos.z = 2.5;

            // And normalise for direction from (0,0)
            let normal_pos = pos.normalized();

            let view_ray = &camera * &normal_pos;
            let result = cast_ray(&position, &view_ray);

            if let CastResult::Hit(t) = result {
                let pos = &position + &view_ray.scale(t);
                let normal = calc_normal(&pos);
                let light = illuminate(&pos, &normal);

                let brightness = (255 as Float * light) as u8;

                image.put_pixel(x, (ysize-1) -y, image::Rgb([brightness, brightness, brightness]));
            }

        }
    }
    image.save("output.png")?;
    Ok(())
}

#[cfg(test)]
mod tests
{

    use super::*;

    fn near_enough(v1: Float, v2: Float) -> bool{
        let tolerance = 0.000001;
        Float::abs(v2-v1) < tolerance
    }

    fn vec_near_enough(v1: Vec4, v2: Vec4) -> bool {
        near_enough(v1.x, v2.x) &&
            near_enough(v1.y, v2.y) &&
            near_enough(v1.z, v2.z) &&
            near_enough(v1.w, v2.w)
    }

    #[test]
    fn check()
    {
        let p = Vec4::position(0.0, 0.0, -10.0);
        let d = Vec4::direction(0.0, 0.0, 1.0);
        let pt = cast_ray(&p, &d);

        match pt {
            CastResult::Hit(t) => {
                let point = &p + &(d.scale(t));
                let norm = calc_normal(&point);
                let expected = Vec4::direction(0.0, 0.0, -1.0);
                assert!(vec_near_enough(norm, expected));
            },
            CastResult::Miss => {
                assert!(false, "Expected hit");
            }
        }
    }
}