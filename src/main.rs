#![feature(core, io, std_misc, plugin)]

extern crate glutin;
#[macro_use] extern crate glium;
#[plugin] extern crate glium_macros;
extern crate "nalgebra" as na;

fn main() {
    use glium::{Surface, DisplayBuild};

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium().unwrap();

    let vertex_buffer = {
        #[vertex_format]
        #[derive(Copy)]
        struct Vertex {
            position: [f32; 3],
            color: [f32; 3],
        }

        glium::VertexBuffer::new(&display, vec![
            Vertex { position: [-1.0, 0.0, 0.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [ 1.0, 0.0, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0, 1.7320508075688772, 0.0], color: [0.0, 0.0, 1.0] },
        ])
    };

    let index_buffer = glium::IndexBuffer::new(&display, glium::index::TrianglesList(vec![0u16, 1, 2]));

    let program = glium::Program::from_source(&display,
        // vertex shader
        "   #version 110

            uniform mat4 matrix;

            attribute vec3 position;
            attribute vec3 color;

            varying vec3 v_color;

            void main() {
                gl_Position = vec4(position, 1.0) * matrix;
                v_color = color;
            }
        ",

        // fragment shader
        "   #version 110
            varying vec3 v_color;

            void main() {
                gl_FragColor = vec4(v_color, 1.0);
            }
        ",

        // optional geometry shader
        None
    ).unwrap();

    // the main loop
    // each cycle will draw once
    'main: loop {
        use std::old_io::timer;
        use std::time::Duration;
        use na::*;

        let uniforms = uniform! {
            matrix: one::<Mat4<f32>>()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &std::default::Default::default()).unwrap();
        target.finish();

        // sleeping for some time in order not to use up too much CPU
        timer::sleep(Duration::milliseconds(17));

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }
    }
}
