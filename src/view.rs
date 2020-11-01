use crate::{scene::Object, scene::Scene, Color, Vecf};
use image::{Pixel, Rgb, RgbImage};
use std::f32::consts::PI;
use vecmath::{
    vec3_add, vec3_cross, vec3_dot, vec3_len, vec3_neg, vec3_normalized, vec3_scale, vec3_sub,
};

pub struct Ray {
    pub direction: Vecf,
    pub origin: Vecf,
}

impl Ray {
    pub fn new(origin: Vecf, direction: Vecf) -> Ray {
        let direction = vec3_normalized(direction);

        Ray { origin, direction }
    }
}

pub struct View {
    image_width: u32,
    image_height: u32,
    cam_position: Vecf,
    fov_rad: f32,
    direction: Vecf,
    max_depth: u32,
    background: Color,
    shadow_bias: f32,
}

impl View {
    pub fn new(
        image_width: u32,
        image_height: u32,
        cam_position: Vecf,
        fov: f32,
        direction: Vecf,
        max_depth: u32,
        background: Color,
        shadow_bias: f32,
    ) -> View {
        let fov_rad = (fov * PI) / 180.0;
        let direction = vec3_normalized(direction);
        View {
            image_width,
            image_height,
            cam_position,
            fov_rad,
            direction,
            max_depth,
            background,
            shadow_bias,
        }
    }
    pub fn render(&self, scene: &Scene) -> RgbImage {
        let mut img_buffer = RgbImage::new(self.image_width, self.image_height);
        let img_height = self.image_height as f32;
        let img_width = self.image_width as f32;
        let cam_right = vec3_normalized(vec3_cross([0.0, 1.0, 0.0], self.direction));
        let cam_up = vec3_normalized(vec3_cross(cam_right, self.direction));
        let cam_half_width = (self.fov_rad / 2.0).tan() as f32;
        let cam_half_height = cam_half_width * (img_height / img_width);
        let pixel_width = cam_half_width * 2.0 / img_width;
        let pixel_height = cam_half_height * 2.0 / img_height;

        for x in 0..self.image_width {
            for y in 0..self.image_height {
                let vec_x_pixel = vec3_scale(cam_right, pixel_width * x as f32 - cam_half_width);
                let vec_y_pixel = vec3_scale(cam_up, pixel_height * y as f32 - cam_half_height);
                let vec_translate = vec3_add(vec_x_pixel, vec_y_pixel);
                let mut ray = Ray::new(
                    self.cam_position,
                    vec3_normalized(vec3_add(self.direction, vec_translate)),
                );
                let mut pixel_color: [f32; 3] = [0.0; 3];
                let mut depth = 0;
                let mut reflection_coef = 1.0;
                while depth < self.max_depth && reflection_coef > 0.0 {
                    if !self.color_trace(scene, &mut reflection_coef, &mut ray, &mut pixel_color) {
                        break;
                    }
                    depth += 1;
                }
                let mut color = [0; 3];
                for c in 0..color.len() {
                    color[c] = (pixel_color[c] * 255.0) as u8;
                }
                img_buffer.put_pixel(x, y, Rgb(color));
            }
        }
        img_buffer
    }

    fn color_trace(
        &self,
        scene: &Scene,
        reflection_coef: &mut f32,
        ray: &mut Ray,
        current_color: &mut [f32; 3],
    ) -> bool {
        if let Some((hit_point, dist, hit_object)) = self.trace(scene, &ray) {
            let object_color = hit_object.get_color().0;
            let light = self.lambert_shade(scene, hit_object.as_ref(), hit_point);
            *ray = hit_object.reflect_ray(ray, hit_point);

            for i in 0..current_color.len() {
                current_color[i] += (object_color[i] as f32 / 255.0)
                    * light
                    * hit_object.get_lambert()
                    * *reflection_coef;
            }
            *reflection_coef *= hit_object.get_specular();
            true
        } else {
            false
        }
    }

    fn trace(&self, scene: &Scene, ray: &Ray) -> Option<(Vecf, f32, Box<dyn Object>)> {
        let mut min_dist = f32::INFINITY;
        let mut closest_object: Option<(Vecf, f32, Box<dyn Object>)> = None;
        for object in &scene.objects {
            let (distance, hit_point) = object.intersect(&ray);
            if distance < min_dist && distance > 0.0 {
                min_dist = distance;
                closest_object = Some((hit_point, min_dist, object.clone())); //OK??????
            }
        }
        closest_object
    }

    fn all_intersects(&self, scene: &Scene, ray: &Ray) -> Vec<f32> {
        let mut intersects = Vec::new();
        for object in &scene.objects {
            let (distance, hit_point) = object.intersect(&ray);
            if distance > 0.0 && distance != f32::INFINITY {
                intersects.push(distance);
            }
        }
        intersects.sort_by(|a, b| a.partial_cmp(b).unwrap());
        intersects
    }

    fn lambert_shade(&self, scene: &Scene, object: &dyn Object, point: Vecf) -> f32 {
        let mut lambert_amount = 0.0;
        for light in &scene.lights {
            let dist_to_light = vec3_sub(light.position, point);
            let dir_to_light = vec3_normalized(dist_to_light);
            let dist_to_light = vec3_len(dist_to_light);
            let shadow_point = vec3_add(point, vec3_scale(dir_to_light, self.shadow_bias));
            let mut blocked = false;
            for intersect in self.all_intersects(scene, &Ray::new(shadow_point, dir_to_light)) {
                if intersect < dist_to_light {
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                let contribution = vec3_dot(
                    dir_to_light,
                    object.normal_to(&Ray::new(point, vec3_neg(dir_to_light))),
                );
                if contribution > 0.0 {
                    lambert_amount +=
                        contribution * (light.intensity / (4.0 * PI * dist_to_light.powi(2)));
                }
            }
        }
        lambert_amount.min(1.0)
    }
}
