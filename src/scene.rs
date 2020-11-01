use vecmath::{vec3_add, vec3_cross, vec3_dot, vec3_len, vec3_normalized, vec3_scale, vec3_sub};

use crate::{view::Ray, Color, Vecf};

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn addObject<T: Object + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn addLight(&mut self, light: Light) {
        self.lights.push(light);
    }
}

pub struct Light {
    pub position: Vecf,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vecf, intensity: f32) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

pub trait Object: CloneObject {
    fn intersect(&self, ray: &Ray) -> (f32, Vecf);

    fn get_position(&self) -> Vecf;

    fn get_color(&self) -> Color;

    fn normal_to(&self, hit_ray: &Ray) -> Vecf;

    fn get_lambert(&self) -> f32;

    fn get_specular(&self) -> f32;

    fn reflect_ray(&self, ray: &Ray, point: Vecf) -> Ray;
}

pub trait CloneObject {
    fn clone_object(&self) -> Box<dyn Object>;
}

impl<T> CloneObject for T
where
    T: Object + Clone + 'static,
{
    fn clone_object(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.clone_object()
    }
}

#[derive(Clone)]
pub struct Sphere {
    position: Vecf,
    color: Color,
    radius: f32,
    sq_radius: f32,
    lambert: f32,
    specular: f32,
}

impl Sphere {
    pub fn new(position: Vecf, color: Color, radius: f32, lambert: f32, specular: f32) -> Sphere {
        let sq_radius = radius * radius;
        Sphere {
            position,
            color,
            radius,
            sq_radius,
            lambert,
            specular,
        }
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> (f32, Vecf) {
        let mut distance = f32::INFINITY;
        let from_ray_origin = vecmath::vec3_sub(self.position, ray.origin);
        let on_ray_midpoint = vecmath::vec3_dot(from_ray_origin, ray.direction);
        if on_ray_midpoint > 0.0 {
            let c_center_to_midpoint =
                vecmath::vec3_square_len(from_ray_origin) - (on_ray_midpoint * on_ray_midpoint);
            if c_center_to_midpoint < self.sq_radius {
                let midpoint_to_intersect = (self.sq_radius - c_center_to_midpoint).sqrt();
                distance = on_ray_midpoint - midpoint_to_intersect;
            }
        }
        let hit_position = vec3_add(ray.origin, vec3_scale(ray.direction, distance));
        (distance, hit_position)
    }

    fn get_position(&self) -> Vecf {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn normal_to(&self, hit_ray: &Ray) -> Vecf {
        vec3_normalized(vec3_sub(hit_ray.origin, self.position))
    }

    fn get_lambert(&self) -> f32 {
        self.lambert
    }

    fn get_specular(&self) -> f32 {
        self.specular
    }

    fn reflect_ray(&self, ray: &Ray, point: Vecf) -> Ray {
        let temp_ray = Ray::new(point, ray.direction);
        let reflection = 2.0 * vec3_dot(ray.direction, self.normal_to(&temp_ray));
        let mut reflected_ray = vec3_scale(self.normal_to(&temp_ray), reflection);
        reflected_ray = vec3_sub(ray.direction, reflected_ray);
        Ray::new(point, reflected_ray)
    }
}

#[derive(Clone)]
pub struct Plane {
    point: Vecf,
    color: Color,
    normal: Vecf,
    width: f32,
    height: f32,
    lambert: f32,
    specular: f32,
}

impl Plane {
    pub fn new(color: Color, normal: Vecf, point: Vecf, lambert: f32, specular: f32) -> Plane {
        let height = f32::INFINITY;
        let width = f32::INFINITY;
        let normal = vec3_normalized(normal);
        Plane {
            color,
            normal,
            width,
            height,
            point,
            lambert,
            specular,
        }
    }
    pub fn from_points(
        color: Color,
        top_right: Vecf,
        bottom_right: Vecf,
        bottom_left: Vecf,
        lambert: f32,
        specular: f32,
    ) -> Plane {
        let height_vec = vec3_sub(top_right, bottom_right);
        let width_vec = vec3_sub(bottom_right, bottom_left);
        let center_position = vec3_scale(vec3_add(top_right, bottom_left), 0.5);
        let point = top_right;
        let normal = vec3_normalized(vec3_cross(width_vec, height_vec));
        let height = vec3_len(height_vec);
        let width = vec3_len(width_vec);

        Plane {
            color,
            normal,
            width,
            height,
            point,
            lambert,
            specular,
        }
    }
}
//TODO: FIX!
impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> (f32, Vecf) {
        let mut distance = f32::INFINITY;
        let norm_ray_dot = vec3_dot(ray.direction, self.normal);
        if norm_ray_dot > 1e-6 {
            let to_center = vec3_sub(self.point, ray.origin);
            let new_distance = vec3_dot(to_center, self.normal) / norm_ray_dot;
            // TODO: Limit plane by checking width&height
            // let hit_pos = vec3_add(ray.origin, vec3_scale(ray.direction, new_distance));
            // let hit_from_center = vec3_sub(hit_pos, self.center_position);
            if new_distance > 0.0 {
                distance = new_distance;
            }
        }
        let hit_position = vec3_add(ray.origin, vec3_scale(ray.direction, distance));
        (distance, hit_position)
    }

    fn get_position(&self) -> Vecf {
        self.point
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn normal_to(&self, hit_ray: &Ray) -> Vecf {
        if vec3_dot(hit_ray.direction, self.normal) < 0.0 {
            self.normal
        } else {
            vecmath::vec3_neg(self.normal)
        }
    }

    fn get_lambert(&self) -> f32 {
        self.lambert
    }

    fn get_specular(&self) -> f32 {
        self.specular
    }

    fn reflect_ray(&self, ray: &Ray, point: Vecf) -> Ray {
        let reflection = 2.0 * vec3_dot(ray.direction, self.normal_to(ray));
        let mut reflected_ray = vec3_scale(self.normal_to(ray), reflection);
        reflected_ray = vec3_sub(ray.direction, reflected_ray);
        Ray::new(point, reflected_ray)
    }
}
