// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - renderer.rs

use crate::vector::*;
use crate::camera::*;
use crate::ray::*;

use rand::Rng;

struct Pixel
{
    color: Vector,
    depth: f32,
}

struct Material
{
    diffuse: Vector,
    specular: Vector,
    emissive: Vector,
    metallic: f32,
    roughness: f32,
}

struct Sphere
{
    position: Vector,
    radius: f32,
    material: Material,
}

struct Intersection
{
    color: Vector,
    position: Vector,
    normal: Vector,
    distance: f32,
    exit: f32,
    object: *const Sphere,
}

struct Scene
{
    objects: Vec<Sphere>,
    sun: Vector,
}

pub struct Renderer
{
    frametime: f32,
    frame_count: f32,

    accumulate: bool,
    accumulated_data: Vec<Vector>,
    framebuffer: Vec<Vector>,

    camera: Camera,

    closest: Intersection,

    scene: Scene,
}

impl Renderer
{
    pub fn new(width: usize, height: usize) -> Renderer
    {
        let mut renderer = Renderer
        {
            frametime: 0.0,
            frame_count: 1.0, // avoid division by zero
            accumulate: true,
            accumulated_data: vec![Vector::zero(); width * height],
            framebuffer: vec![Vector::zero(); width * height],
            camera: Camera::new(90.0, 0.1, 1000.0, width, height),
            closest: Intersection{ color: Vector::zero(), position: Vector::zero(), normal: Vector::zero(), distance: 0.0, exit: 0.0, object: std::ptr::null() },
            scene: Scene{ objects: Vec::new(), sun: Vector::zero() },
        };

        renderer.scene = Scene
        {
            objects: vec![
                Sphere
                {
                    position: Vector::new(0.0, 0.0, 5.0, 1.0),
                    radius: 1.0,
                    material: Material
                    {
                        diffuse: Vector::new(1.0, 0.0, 0.0, 1.0),
                        specular: Vector::new(1.0, 0.3, 0.3, 1.0),
                        emissive: Vector::zero(),
                        metallic: 0.9,
                        roughness: 0.5,
                    },
                },
                Sphere
                {
                    position: Vector::new(3.0, 0.0, 5.0, 1.0),
                    radius: 1.5,
                    material: Material
                    {
                        diffuse: Vector::new(0.0, 1.0, 0.0, 1.0),
                        specular: Vector::new(0.3, 1.0, 0.3, 1.0),
                        emissive: Vector::zero(),
                        metallic: 0.7,
                        roughness: 0.0,
                    },
                },
                Sphere
                {
                    position: Vector::new(0.0, 1003.0, 0.0, 1.0),
                    radius: 1000.0,
                    material: Material
                    {
                        diffuse: Vector::new(0.85, 0.85, 1.0, 1.0),
                        specular: Vector::new(0.4, 0.4, 1.0, 1.0),
                        emissive: Vector::zero(),
                        metallic: 0.0,
                        roughness: 0.0,
                    },
                },
            ],

            sun: Vector::new(10.0, -10.0, -10.0, 1.0),
        };

        return renderer;
    }

    pub fn framebuffer(&self) -> &Vec<Vector>
    {
        return &self.framebuffer;
    }

    pub fn frametime(&self) -> f32
    {
        return self.frametime;
    }

    pub fn position(&self) -> Vector
    {
        return self.camera.position();
    }

    pub fn rotation(&self) -> Vector
    {
        return self.camera.direction();
    }

    pub fn bitmap(&self) -> Vec<u8>
    {
        let mut bitmap = vec![0; self.framebuffer.len() * 4];

        for (i, pixel) in self.framebuffer.iter().enumerate()
        {
            let index = i * 4;

            bitmap[index + 0] = (pixel.x() * 255.0) as u8;
            bitmap[index + 1] = (pixel.y() * 255.0) as u8;
            bitmap[index + 2] = (pixel.z() * 255.0) as u8;
            bitmap[index + 3] = (pixel.w() * 255.0) as u8;
        }

        return bitmap;
    }

    pub fn size(&self) -> [usize; 2]
    {
        return [self.camera.width(), self.camera.height()];
    }

    fn jitter(vec1: &Vector, noise: f32) -> Vector
    {
        let mut rng = rand::thread_rng();

        return Vector::new
        (
            vec1.x() + rng.gen::<f32>() * noise,
            vec1.y() + rng.gen::<f32>() * noise,
            vec1.z() + rng.gen::<f32>() * noise,
            vec1.w(),
        );
    }

    fn fresnel(intersection: &Intersection, ray: &Ray) -> f32
    {
        let direction = Vector::scale(&ray.direction, -1.0);

        let mut metallic: f32 = 0.0;

        if !intersection.object.is_null()
        {   
            unsafe
            {
                metallic = (*intersection.object).material.metallic;
            }
        }

        // fresnel's law
        let cos_incident = Vector::dot(&intersection.normal, &direction);
        let coefficient = metallic;
        let sin_theta_squared = coefficient * coefficient * (1.0 - cos_incident * cos_incident);

        if sin_theta_squared > 1.0
        {
            // total internal reflection
            return 1.0;
        }

        let cos_theta = f32::sqrt(1.0 - sin_theta_squared);

        // schlick approximation
        let r0 = (1.0 - coefficient) / (1.0 + coefficient);
        let r0_squared = r0 * r0;

        let partial = 1.0 - cos_theta;
        let power = partial * partial * partial * partial * partial;

        let r = r0_squared + (1.0 - r0_squared) * power;

        let fresnel = 1.0 - f32::clamp(r, 0.0, 1.0);
        return fresnel;
    }

    fn direct(intersection: &Intersection) -> Vector
    {
        return Vector::zero();
    }

    fn indirect(intersection: &Intersection) -> Vector
    {
        return Vector::zero();
    }

    fn reflect(intersection: &Intersection, ray: &Ray) -> Ray
    {
        const EPSILON: f32 = 0.001;

        // ensure the reflection Ray doesn't re-hit the same object due to floating-point inaccuracy
        let extruded = Vector::scale(&intersection.normal, EPSILON);
        let position = Vector::add(&intersection.position, &extruded);

        // then reflect across the normal
        let direction = Vector::reflect(&ray.direction, &intersection.normal);
        return Ray::new(position, direction);
    }

    fn miss() -> Intersection
    {
        return Intersection
        {
            color: Vector::zero(),
            position: Vector::zero(),
            normal: Vector::zero(),
            distance: f32::MAX,
            exit: f32::MAX,
            object: std::ptr::null(),
        };
    }

    // TODO: refraction

    fn trace(&mut self, ray: &Ray) -> Intersection
    {
        let mut distance = f32::MAX;

        let mut intersection = Self::miss();

        for object in &self.scene.objects 
        {
            let difference = Vector::sub(&ray.origin, &object.position);

            let a = Vector::dot(&ray.direction, &ray.direction);
            let b = 2.0 * Vector::dot(&ray.direction, &difference);
            let c = Vector::dot(&difference, &difference) - object.radius * object.radius;

            let d = b * b - 4.0 * a * c;

            if d > 0.0 
            {
                let t1 = (-b - f32::sqrt(d)) / (2.0 * a);
                let t2 = (-b + f32::sqrt(d)) / (2.0 * a);

                if t1 > 0.0
                {
                    let progress = Vector::scale(&ray.direction, t1);
                    let hit = Vector::add(&ray.origin, &progress);

                    let toward = Vector::sub(&hit, &object.position);
                    let normal = Vector::normalize(&toward);

                    if t1 < distance 
                    {
                        distance = t1;
                        intersection = Intersection
                        {
                            color: object.material.diffuse,
                            position: hit,
                            normal: normal,
                            distance: t1,
                            exit: t2,
                            object: object as *const Sphere,
                        };
                    }
                }
            }
        }

        if distance == f32::MAX
        {
            return Self::miss();
        }

        return intersection;
    }

    fn gamma_correct(color: Vector, gamma: f32) -> Vector
    {
        return Vector::new
        (
            f32::powf(color.x(), 1.0 / gamma),
            f32::powf(color.y(), 1.0 / gamma),
            f32::powf(color.z(), 1.0 / gamma),
            color.w(),
        );
    }

    fn tonemap(color: Vector) -> Vector
    {
        const EXPOSURE: f32 = 1.0;

        let mapped = Vector::new
        (
            1.0 - f32::exp(-color.x() * EXPOSURE),
            1.0 - f32::exp(-color.y() * EXPOSURE),
            1.0 - f32::exp(-color.z() * EXPOSURE),
            color.w(),
        );

        return mapped;
    }

    fn shade(&mut self, x: usize, y: usize, bounces: u32, contribution: &mut Vector, ray: &mut Ray) -> Pixel
    {
        if bounces == 0
        {
            return Pixel
            {
                color: Vector::broadcast(1.0),
                depth: f32::MAX,
            };
        }

        let mut depth = f32::MAX;

        let intersection = self.trace(&ray);

        if intersection.object.is_null()
        {
            // no intersection, so cast to sky

            let top_sky_color = Vector::new(0.529, 0.808, 0.922, 1.0);
            let bottom_sky_color = Vector::new(0.106, 0.275, 0.711, 1.0);

            let clamped = f32::clamp(ray.direction.y(), -1.0, 1.0);
            let adjusted = (clamped + 1.0) * 0.5;

            let sky = Vector::lerp(&bottom_sky_color, &top_sky_color, adjusted);

            *contribution = Vector::mul( contribution, &sky);
        }

        else
        {
            let direction_jittered = Vector::normalize(&Self::jitter(&ray.direction, 0.01));
            *ray = Ray::new(ray.origin, direction_jittered);

            let object = unsafe { &*intersection.object };
            let material= &object.material;

            let metallic = material.metallic;

            let inverted = Vector::scale(&direction_jittered, -1.0);
            let cos_theta = Vector::dot(&intersection.normal, &inverted);

            let fresnel = Self::fresnel(&intersection, &ray);

            let base_color = &material.diffuse;
            
            let specular = self.shade(x, y, bounces - 1, contribution, ray);
            let specular_color = specular.color;

            let specular_blend = Vector::lerp(&specular_color, &base_color, metallic);

            let diffuse_contribution = Vector::scale(base_color, (1.0 - metallic) * cos_theta.max(0.0));
            let specular_contribution = Vector::scale(&specular_blend, fresnel);

            let total_contribution = Vector::add(&diffuse_contribution, &specular_contribution);

            *contribution = Vector::mul(contribution, &total_contribution);

            ray.direction = Self::reflect(&intersection, &ray).direction;
            ray.direction = Self::jitter(&ray.direction, 0.5 * material.roughness);
        }

        return Pixel 
        {
            color: *contribution,
            depth: depth,
        };
    }

    pub fn update(&mut self, ctx: &egui::Context)
    {
        self.camera.update(self.frametime, ctx);
    }

    pub fn render(&mut self, bounces: u32) 
    {
        let now = std::time::Instant::now();

        let width = self.camera.width();
        let height = self.camera.height();

        if self.camera.moved
        {
            self.frame_count = 1.0;

            self.accumulated_data.clear();
            self.accumulated_data.resize(width * height, Vector::zero());

            self.camera.moved = false;
        }

        for y in 0..height
        {
            for x in 0..width
            {
                let mut contribution = Vector::broadcast(1.0);
                let mut ray = self.camera.rays()[y * self.camera.width() + x];
                let pixel = self.shade(x, y, bounces, &mut contribution, &mut ray);

                let corrected = Self::gamma_correct(pixel.color, 2.2);
                let renormalized = Self::tonemap(corrected);

                let index = y * width + x;

                if self.accumulate
                {
                    self.accumulated_data[index] = Vector::add(&self.accumulated_data[index], &renormalized);
                    self.framebuffer[index] = Vector::scale(&self.accumulated_data[index], 1.0 / self.frame_count);
                }

                else
                {
                    // no need to write to the accumulation buffer
                    self.framebuffer[index] = renormalized;
                }
            }
        }

        let elapsed = now.elapsed();

        self.frametime = elapsed.as_secs_f32();
        self.frame_count += 1.0;
    }

}
