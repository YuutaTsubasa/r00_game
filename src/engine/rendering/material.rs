pub struct Material {
    pub color: Vec<f32>,
    pub texture_id: Option<u32>,
    pub shader_program: u32,
}

impl Material {
    pub fn new(
        color: Vec<f32>,
        texture_id: Option<u32>,
        vertex_shader: &str,
        fragment_shader: &str) -> Self {
        let shader_program = create_shader_program(vertex_shader, fragment_shader);
        Self {
            color,
            texture_id,
            shader_program,
        }
    }
}

fn create_shader_program(vertex_shader_src: &str, fragment_shader_src: &str) -> u32 {
    let vertex_shader = compile_shader(vertex_shader_src, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(fragment_shader_src, gl::FRAGMENT_SHADER);

    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // 檢查連接錯誤
        let mut success: gl::types::GLint = 1;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut log: [gl::types::GLchar; 512] = [0; 512];
            gl::GetProgramInfoLog(shader_program, 512, std::ptr::null_mut(), log.as_mut_ptr());

            // 將 &[i8] 轉換為 &[u8]
            let log_u8 = unsafe {
                std::slice::from_raw_parts(log.as_ptr() as *const u8, log.len())
            };
            panic!("Shader linking failed: {:?}", std::str::from_utf8(&log_u8).unwrap());
        }
    }

    shader_program
}

fn compile_shader(source: &str, shader_type: u32) -> u32 {
    let shader = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // 檢查編譯錯誤
        let mut success: gl::types::GLint = 1;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log: [gl::types::GLchar; 512] = [0; 512];
            gl::GetShaderInfoLog(shader, 512, std::ptr::null_mut(), log.as_mut_ptr());

            // 將 &[i8] 轉換為 &[u8]
            let log_u8 = unsafe {
                std::slice::from_raw_parts(log.as_ptr() as *const u8, log.len())
            };
            panic!("Shader compilation failed: {:?}", std::str::from_utf8(&log_u8).unwrap());
        }
    }

    shader
}