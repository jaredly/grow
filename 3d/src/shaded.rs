use std::ptr;
use gl;
use gl::types::*;
use na::{Pnt3, Pnt2, Vec3, Mat3, Mat4, Iso3};
use na;
use kiss3d::resource::Material;
use kiss3d::scene::ObjectData;
use kiss3d::light::Light;
use kiss3d::camera::Camera;
use kiss3d::resource::{Mesh, Shader, ShaderAttribute, ShaderUniform};

#[path = "./error.rs"]
mod error;

/// A material that draws normals of an object.
pub struct ShaderMaterial {
    shader:    Shader,
    position:  ShaderAttribute<Pnt3<f32>>,
    uvs:       ShaderAttribute<Pnt2<f32>>,
    view:      ShaderUniform<Mat4<f32>>,
    transform: ShaderUniform<Mat4<f32>>,
    scale:     ShaderUniform<Mat3<f32>>,
    //time:      ShaderUniform<f32>,
    time_local: i32,
}

impl ShaderMaterial {
    /// Creates a new ShaderMaterial.
    pub fn new(vertex_src: &str, fragment_src: &str) -> ShaderMaterial {
        let mut shader = Shader::new_from_str(vertex_src, fragment_src);

        shader.use_program();

        ShaderMaterial {
            position:  shader.get_attrib("position").unwrap(),
            uvs:       shader.get_attrib("uvs").unwrap(),
            transform: shader.get_uniform("transform").unwrap(),
            scale:     shader.get_uniform("scale").unwrap(),
            view:      shader.get_uniform("view").unwrap(),
            //time:      shader.get_uniform("time").unwrap(),
            time_local:0,
            shader:    shader
        }
    }

    pub fn default() -> ShaderMaterial {
        ShaderMaterial::new(UVS_VERTEX_SRC, UVS_FRAGMENT_SRC)
    }

    pub fn inc_time(&mut self) {
        self.time_local += 1;
    }

    pub fn set_time(&mut self, time: i32) {
        self.time_local = time;
    }
}

impl Material for ShaderMaterial {
    fn render(&mut self,
              pass:      usize,
              transform: &Iso3<f32>,
              scale:     &Vec3<f32>,
              camera:    &mut Camera,
              _:         &Light,
              data:      &ObjectData,
              mesh:      &mut Mesh) {
        if !data.surface_rendering_active() {
            return
        }
        // enable/disable culling.
        if data.backface_culling_enabled() {
            verify!(gl::Enable(gl::CULL_FACE));
        }
        else {
            verify!(gl::Disable(gl::CULL_FACE));
        }


        self.shader.use_program();
        self.position.enable();
        self.uvs.enable();

        /*
         *
         * Setup camera and light.
         *
         */
        camera.upload(pass, &mut self.view);

        /*
         *
         * Setup object-related stuffs.
         *
         */
        let formated_transform: Mat4<f32> = na::to_homogeneous(transform);
        // FIXME: add a function `na::diagonal(scale)` to nalgebra.
        let formated_scale:     Mat3<f32> = Mat3::new(scale.x, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, scale.z);

        self.transform.upload(&formated_transform);
        self.scale.upload(&formated_scale);
        self.inc_time();
        //self.time.upload(&(self.time_local as f32));

        mesh.bind_coords(&mut self.position);
        mesh.bind_uvs(&mut self.uvs);
        mesh.bind_faces();

        unsafe {
            gl::DrawElements(gl::TRIANGLES,
                             mesh.num_pts() as GLint,
                             gl::UNSIGNED_INT,
                             ptr::null());
        }

        mesh.unbind();

        self.position.disable();
        self.uvs.disable();
    }
}

pub static UVS_VERTEX_SRC: &'static str = A_VERY_LONG_STRING;

pub static UVS_FRAGMENT_SRC: &'static str = ANOTHER_VERY_LONG_STRING;

const A_VERY_LONG_STRING: &'static str =
"#version 120
attribute vec3 position;
attribute vec3 uvs;
uniform float time;
uniform mat4 view;
uniform mat4 transform;
uniform mat3 scale;
varying vec3 uv_as_a_color;
void main() {
    // float vtime = sin(time / 25.0) * 0.4 + 0.5;
    uv_as_a_color  = vec3(uvs.y * 0.6, 1.0 - uvs.y * 0.6, uvs.y * .2);
    // uv_as_a_color  = vec3(uvs.x, 0.1, uvs.x / 2.0 + 0.5);
    gl_Position = view * transform * mat4(scale) * vec4(position, 1.0);
}
";

const ANOTHER_VERY_LONG_STRING: &'static str =
"#version 120
varying vec3 uv_as_a_color;
void main() {
    gl_FragColor = vec4(uv_as_a_color, 0.9);
}
";
