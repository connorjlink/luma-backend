// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - camera.rs

use crate::matrix::*;
use crate::vector::*;
use crate::ray::*;

pub struct Camera
{
    // camera matrices 
    projection: Matrix,
    projection_inverse: Matrix,
    view: Matrix,
    view_inverse: Matrix,

    // camera configuration
    fov: f32,
    near: f32,
    far: f32,
    width: usize,
    height: usize,

    // camera look
    pub moved: bool,
    pitch: f32,
    yaw: f32,

    // depth-of-field effect
    depth: f32,
    show_depth: bool,

    // camera position
    position: Vector,
    direction: Vector,
    right: Vector,

    // camera view
    rays: Vec<Ray>,
}

impl Camera
{
    pub fn new(fov: f32, near: f32, far: f32, width: usize, height: usize) -> Camera
    {
        let mut camera = Camera
        {
            projection: Matrix::identity(),
            projection_inverse: Matrix::identity(),
            view: Matrix::identity(),
            view_inverse: Matrix::identity(),
            fov: fov,
            near: near,
            far: far,
            width: width,
            height: height,
            moved: false,
            pitch: 0.0,
            yaw: 0.0,
            depth: 10.0,
            show_depth: false,
            position: Vector::zero(),
            direction: Vector::new(0.0, 0.0, 1.0, 1.0),
            right: Vector::zero(),
            rays: vec![],
        };

        camera.recompute_projection();
        camera.recompute_view();
        camera.recompute_rays();

        return camera;
    }

    pub fn rays(&self) -> &Vec<Ray>
    {
        return &self.rays;
    }

    pub fn view(&self) -> &Matrix
    {
        return &self.view;
    }

    pub fn view_inverse(&self) -> &Matrix
    {
        return &self.view_inverse;
    }

    pub fn position(&self) -> Vector
    {
        return self.position;
    }
    
    pub fn direction(&self) -> Vector
    {
        return self.direction;
    }

    pub fn width(&self) -> usize
    {
        return self.width;
    }

    pub fn height(&self) -> usize
    {
        return self.height;
    }

    pub fn recompute_view(&mut self)
    {
        let at = Vector::add(&self.position, &self.direction);
        let up = Vector::new(0.0, 1.0, 0.0, 1.0);

        self.view = Matrix::lookat(&self.position, &at, &up);
        self.view_inverse = Matrix::inverse(&self.view);
    }

    pub fn recompute_projection(&mut self)
    {
        let aspect_ratio = self.width as f32 / self.height as f32;

        let fov_radians = f32::to_radians(self.fov);

        self.projection = Matrix::perspective(fov_radians, aspect_ratio, self.near, self.far);
        self.projection_inverse = Matrix::inverse(&self.projection);
    }

    pub fn recompute_rays(&mut self)
    {
        self.rays.resize(self.width * self.height, Ray::zero());

        for y in 0..self.height
        {
            for x in 0..self.width
            {
                let x_coord = (x as f32 / self.width  as f32) * 2.0 - 1.0;
                let y_coord = (y as f32 / self.height as f32) * 2.0 - 1.0;

                let extended: Vector = Vector::new(x_coord, y_coord, 1.0, 1.0);
                let target = Matrix::apply(&self.projection_inverse, &extended);

                let homogenized = Vector::scale(&target, 1.0 / target.w());
                
                let mut normalized = Vector::normalize(&homogenized);
                normalized.set_w(0.0);

                let direction = Matrix::apply(&self.view_inverse, &normalized);
                let position = self.position;

                let ray = Ray::new(position, direction);
                self.rays[y * self.width + x] = ray;
            }
        }
    }

    pub fn update(&mut self, delta: f32, ctx: &egui::Context)
    {
        const MOVEMENT_SPEED: f32 = 0.75;
        const ROTATION_SPEED: f32 = 1.0;

        self.moved = false;

        let up = Vector::new(0.0, 1.0, 0.0, 1.0);
        let dir = Vector::new(self.direction.x(), 0.0, -self.direction.z(), 1.0);
        let forward = Vector::normalize(&dir);
        let right = Vector::cross(&up, &forward);

        if ctx.input(|i| i.key_pressed(egui::Key::W))
        {
            self.position = Vector::add(&self.position, &Vector::scale(&forward, MOVEMENT_SPEED * delta));
            self.moved = true;
        }

        else if ctx.input(|i| i.key_pressed(egui::Key::S))
        {
            self.position = Vector::sub(&self.position, &Vector::scale(&forward, MOVEMENT_SPEED * delta));
            self.moved = true;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::A))
        {
            self.position = Vector::sub(&self.position, &Vector::scale(&right, MOVEMENT_SPEED * delta));
            self.moved = true;
        }

        else if ctx.input(|i| i.key_pressed(egui::Key::D))
        {
            self.position = Vector::add(&self.position, &Vector::scale(&right, MOVEMENT_SPEED * delta));
            self.moved = true;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Q))
        {
            self.position = Vector::add(&self.position, &Vector::scale(&up, MOVEMENT_SPEED * delta));
            self.moved = true;
        }

        else if ctx.input(|i| i.key_pressed(egui::Key::E))
        {
            self.position = Vector::sub(&self.position, &Vector::scale(&up, MOVEMENT_SPEED * delta));
            self.moved = true;
        }

        if ctx.input(|i| i.key_down(egui::Key::ArrowLeft))
        {
            self.yaw -= ROTATION_SPEED * delta;
            self.moved = true;
        }

        else if ctx.input(|i| i.key_down(egui::Key::ArrowRight))
        {
            self.yaw += ROTATION_SPEED * delta;
            self.moved = true;
        }

        if ctx.input(|i| i.key_down(egui::Key::ArrowUp))
        {
            self.pitch += ROTATION_SPEED * delta;
            self.moved = true;
        }

        else if ctx.input(|i| i.key_down(egui::Key::ArrowDown))
        {
            self.pitch -= ROTATION_SPEED * delta;
            self.moved = true;
        }

        // handles mouse interaction to provide click and pan functionality
        let mouse = ctx.input(|i| i.pointer.press_origin());
        if mouse.is_some()
        {
            let mouse_pos = mouse.unwrap();
            let delta_x = mouse_pos.x - ctx.screen_rect().center().x;
            let delta_y = mouse_pos.y - ctx.screen_rect().center().y;

            self.yaw = delta_x * ROTATION_SPEED * delta;
            self.pitch = delta_y * ROTATION_SPEED * delta;

            self.moved = true;
        }

        const PI: f32 = 3.14159265358979323846;

        self.pitch = f32::clamp(self.pitch, -PI / 4.0, PI / 4.0);

        let cos_pitch = f32::cos(self.pitch);
        let sin_pitch = f32::sin(self.pitch);

        let cos_yaw = f32::cos(self.yaw);
        let sin_yaw = f32::sin(self.yaw);

        self.direction = Vector::new
        (
            cos_pitch * sin_yaw,
            -sin_pitch,
            cos_pitch * cos_yaw,
            0.0,
        );

        self.direction = Vector::normalize(&self.direction);

        if self.moved
        {
            self.recompute_view();
            self.recompute_rays();
        }

    }
}