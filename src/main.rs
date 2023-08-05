use miniquad::*;

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
    uv: [f32; 2],
}

struct Stage {
    pipeline: Pipeline,
    //bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
    white_texture: TextureId,
    text_bitmap: Vec<u8>,
    king_bitmap: Vec<u8>,
    king_texture: TextureId,
    last_char: char,
    font: fontdue::Font,
    show_king: bool,
    king_dim: (u16, u16),
    keyb_shown: bool,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let vertices: [Vertex; 4] = [
            Vertex {
                pos: [-0.5, 0.5],
                color: [1., 1., 1., 1.],
                uv: [0., 0.],
            },
            Vertex {
                pos: [0.5, 0.5],
                color: [1., 1., 1., 1.],
                uv: [1., 0.],
            },
            Vertex {
                pos: [-0.5, -0.5],
                color: [1., 1., 1., 1.],
                uv: [0., 1.],
            },
            Vertex {
                pos: [0.5, -0.5],
                color: [1., 1., 1., 1.],
                uv: [1., 1.],
            },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 1, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        /*
        let pixels: [u8; 4 * 4 * 4] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];
        let texture = ctx.new_texture_from_rgba8(4, 4, &pixels);
        */

        let white_texture = ctx.new_texture_from_rgba8(1, 1, &[255, 255, 255, 255]);

        let font = include_bytes!("../ProggyClean.ttf") as &[u8];
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
        let (metrics, text_bitmap) = font.rasterize('b', 256.0);
        let text_bitmap: Vec<_> = text_bitmap
                    .iter()
                    .flat_map(|coverage| vec![255, 255, 255, *coverage])
                    .collect();
        let texture = ctx.new_texture_from_rgba8(metrics.width as u16, metrics.height as u16, &text_bitmap);

        let img = image::load_from_memory(
            include_bytes!("../king.png")
        ).unwrap().to_rgba8();
        let width = img.width() as u16;
        let height = img.height() as u16;
        let king_bitmap = img.into_raw();
        let king_texture = ctx.new_texture_from_rgba8(width, height, &king_bitmap);

        //let bindings = Bindings {
        //    vertex_buffers: vec![vertex_buffer],
        //    index_buffer: index_buffer,
        //    images: vec![texture],
        //};

        let shader = ctx
            .new_shader(
                ShaderSource {
                    glsl_vertex: Some(shader::GL_VERTEX),
                    glsl_fragment: Some(shader::GL_FRAGMENT),
                    metal_shader: Some(shader::METAL),
                },
                shader::meta(),
            )
            .unwrap();

        let params = PipelineParams {
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            ..Default::default()
        };

        let pipeline = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float4),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            shader,
            params
        );

        Stage {
            pipeline,
            //bindings,
            ctx,
            white_texture,
            text_bitmap,
            king_bitmap,
            king_texture,
            last_char: 'a',
            font,
            show_king: true,
            king_dim: (width, height),
            keyb_shown: false,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        if !self.keyb_shown && self.last_char == 'g' {
            window::show_keyboard(true);
            self.keyb_shown = true;
        }
    }

    fn draw(&mut self) {
        let vertices: [Vertex; 4] = 
            if self.last_char == ' ' && !self.show_king {
                [
                Vertex {
                    pos: [-0.5, 0.5],
                    color: [1., 0., 1., 1.],
                    uv: [0., 0.],
                },
                Vertex {
                    pos: [0.5, 0.5],
                    color: [1., 1., 0., 1.],
                    uv: [1., 0.],
                },
                Vertex {
                    pos: [-0.5, -0.5],
                    color: [0., 0., 0.8, 1.],
                    uv: [0., 1.],
                },
                Vertex {
                    pos: [0.5, -0.5],
                    color: [1., 1., 0., 1.],
                    uv: [1., 1.],
                },
            ]
            } else {
                [
                Vertex {
                    pos: [-0.5, 0.5],
                    color: [1., 1., 1., 1.],
                    uv: [0., 0.],
                },
                Vertex {
                    pos: [0.5, 0.5],
                    color: [1., 1., 1., 1.],
                    uv: [1., 0.],
                },
                Vertex {
                    pos: [-0.5, -0.5],
                    color: [1., 1., 1., 1.],
                    uv: [0., 1.],
                },
                Vertex {
                    pos: [0.5, -0.5],
                    color: [1., 1., 1., 1.],
                    uv: [1., 1.],
                },
            ]
        };
        let vertex_buffer = self.ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 1, 2, 3];
        let index_buffer = self.ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let (metrics, text_bitmap) = self.font.rasterize(self.last_char, 256.0);
        let text_bitmap: Vec<_> = text_bitmap
                    .iter()
                    .flat_map(|coverage| vec![255, 255, 255, *coverage])
                    .collect();

        let texture = if self.last_char == ' ' {
            if self.show_king {
                let (width, height) = self.king_dim;
                self.king_texture
            } else {
                self.white_texture
            }
        } else {
            self.ctx.new_texture_from_rgba8(metrics.width as u16, metrics.height as u16, &text_bitmap)
        };

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        };

        let clear = PassAction::clear_color(0., 1., 0., 1.);
        self.ctx.begin_default_pass(clear);
        self.ctx.end_render_pass();

        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&bindings);
        self.ctx.draw(0, 6, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }

    fn key_down_event(&mut self, keycode: KeyCode, modifiers: KeyMods, repeat: bool) {
        if repeat {
            return;
        }
        match keycode {
            KeyCode::A => if modifiers.shift { self.last_char = 'a' } else { self.last_char = 'A' },
            KeyCode::B => if modifiers.shift { self.last_char = 'b' } else { self.last_char = 'B' },
            KeyCode::C => if modifiers.shift { self.last_char = 'c' } else { self.last_char = 'C' },
            KeyCode::D => if modifiers.shift { self.last_char = 'd' } else { self.last_char = 'D' },
            KeyCode::E => if modifiers.shift { self.last_char = 'e' } else { self.last_char = 'E' },
            KeyCode::F => if modifiers.shift { self.last_char = 'f' } else { self.last_char = 'F' },
            KeyCode::G => { if modifiers.shift { self.last_char = 'g' } else { self.last_char = 'G' }
                self.keyb_shown = false;
            }
            KeyCode::H => if modifiers.shift { self.last_char = 'h' } else { self.last_char = 'H' },
            KeyCode::I => if modifiers.shift { self.last_char = 'i' } else { self.last_char = 'I' },
            KeyCode::J => if modifiers.shift { self.last_char = 'j' } else { self.last_char = 'J' },
            KeyCode::K => if modifiers.shift { self.last_char = 'k' } else { self.last_char = 'K' },
            KeyCode::L => if modifiers.shift { self.last_char = 'l' } else { self.last_char = 'L' },
            KeyCode::M => if modifiers.shift { self.last_char = 'm' } else { self.last_char = 'M' },
            KeyCode::N => if modifiers.shift { self.last_char = 'n' } else { self.last_char = 'N' },
            KeyCode::O => if modifiers.shift { self.last_char = 'o' } else { self.last_char = 'O' },
            KeyCode::P => if modifiers.shift { self.last_char = 'p' } else { self.last_char = 'P' },
            KeyCode::Q => if modifiers.shift { self.last_char = 'q' } else { self.last_char = 'Q' },
            KeyCode::R => if modifiers.shift { self.last_char = 'r' } else { self.last_char = 'R' },
            KeyCode::S => if modifiers.shift { self.last_char = 's' } else { self.last_char = 'S' },
            KeyCode::T => if modifiers.shift { self.last_char = 't' } else { self.last_char = 'T' },
            KeyCode::U => if modifiers.shift { self.last_char = 'u' } else { self.last_char = 'U' },
            KeyCode::V => if modifiers.shift { self.last_char = 'v' } else { self.last_char = 'V' },
            KeyCode::W => if modifiers.shift { self.last_char = 'w' } else { self.last_char = 'W' },
            KeyCode::X => if modifiers.shift { self.last_char = 'x' } else { self.last_char = 'X' },
            KeyCode::Y => if modifiers.shift { self.last_char = 'y' } else { self.last_char = 'Y' },
            KeyCode::Z => if modifiers.shift { self.last_char = 'z' } else { self.last_char = 'Z' },
            KeyCode::Space => { self.last_char = ' '; self.show_king = true; },
            KeyCode::Enter => { self.last_char = ' '; self.show_king = false; },
            _ => {}
        }
        println!("{:?}", keycode);
    }
    //fn mouse_motion_event(&mut self, x: f32, y: f32) {
    //    //println!("{} {}", x, y);
    //}
    //fn mouse_wheel_event(&mut self, x: f32, y: f32) {
    //    println!("{} {}", x, y);
    //}
    //fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
    //    //println!("{:?} {} {}", button, x, y);
    //}
    //fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
    //    //println!("{:?} {} {}", button, x, y);
    //}
}

fn main() {
    /*
    let mut conf = conf::Conf::default();
    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        conf::AppleGfxApi::Metal
    } else {
        conf::AppleGfxApi::OpenGl
    };

    miniquad::start(conf, move || Box::new(Stage::new()));
    */
    miniquad::start(
        miniquad::conf::Conf {
            window_resizable: true,
            platform: miniquad::conf::Platform {
                linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
                wayland_use_fallback_decorations: false,
                ..Default::default()
            },
            ..Default::default()
        },
        || {
            window::show_keyboard(true);
            Box::new(Stage::new())
        },
    );
}

mod shader {
    use miniquad::*;

    pub const GL_VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec4 in_color;
    attribute vec2 in_uv;

    varying lowp vec4 color;
    varying lowp vec2 uv;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        color = in_color;
        uv = in_uv;
    }"#;

    pub const GL_FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;
    varying lowp vec2 uv;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = color * texture2D(tex, uv);
    }"#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Vertex
    {
        float2 in_pos   [[attribute(0)]];
        float4 in_color [[attribute(1)]];
        float2 in_uv    [[attribute(2)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
        float4 color [[user(locn0)]];
        float2 uv [[user(locn1)]];
    };

    vertex RasterizerData vertexShader(Vertex v [[stage_in]])
    {
        RasterizerData out;

        out.position = float4(v.in_pos.xy, 0.0, 1.0);
        out.color = v.in_color;
        out.uv = v.texcoord;

        return out;
    }

    fragment float4 fragmentShader(RasterizerData in [[stage_in]], texture2d<float> tex [[texture(0)]], sampler texSmplr [[sampler(0)]])
    {
        return in.color * tex.sample(texSmplr, in.uv);
    }

    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}

