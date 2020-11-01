use image::Rgb;
use raytracer::{scene::*, view::*};

fn main() {
    let view = View::new(
        1920,
        1080,
        [0.0, 0.0, 0.0], //no other positions supported w/ reflections gauranteed
        90.0,
        [0.0, 0.0, 1.0],
        12,
        Rgb([50, 100, 200]),
        1e-3,
    );
    let mut scene = Scene::default();
    scene.addLight(Light::new([0.0, 1.0, 7.0], 20.0));
    scene.addLight(Light::new([2.0, 0.5, 2.0], 40.0));

    scene.addObject(Sphere::new(
        [0.0, -0.3, 3.0],
        Rgb([255, 0, 0]),
        0.2,
        0.9,
        0.0,
    ));
    scene.addObject(Sphere::new(
        [1.0, -0.3, 5.0],
        Rgb([0, 0, 255]),
        0.3,
        0.9,
        0.3,
    ));
    scene.addObject(Plane::new(
        Rgb([0, 255, 0]),
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        0.6,
        0.0,
    ));
    scene.addObject(Plane::new(
        Rgb([0, 0, 255]),
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        0.6,
        0.0,
    ));
    scene.addObject(Plane::new(
        Rgb([255; 3]),
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 8.0],
        0.05,
        1.0,
    ));
    scene.addObject(Plane::new(
        Rgb([255; 3]),
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -3.0],
        0.05,
        1.0,
    ));
    scene.addObject(Plane::new(
        Rgb([100, 0, 100]),
        [1.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
        0.6,
        0.0,
    ));
    scene.addObject(Plane::new(
        Rgb([255; 3]),
        [0.0, 1.0, 0.0],
        [0.0, 2.0, 0.0],
        0.6,
        0.0,
    ));
    let img = view.render(&scene);
    img.save("trace.png").unwrap();
}
