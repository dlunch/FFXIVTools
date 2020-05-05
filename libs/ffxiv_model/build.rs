use std::fs;
use std::io;
use std::path::Path;
use std::str;

fn main() -> io::Result<()> {
    let shaders = Path::new("./shaders");
    for entry in shaders.read_dir()? {
        let path = entry?.path();
        let ext = path.extension().unwrap().to_str().unwrap();
        if ext == "frag" || ext == "vert" {
            let compiled_filename = format!("{}.spv", path.to_str().unwrap());
            let compilation = compile_shader(&path, ext)?;
            save_shader(Path::new(&compiled_filename), compilation)?;

            print!("cargo:rerun-if-changed={}", path.to_str().unwrap());
            print!("cargo:rerun-if-changed={}", compiled_filename);
        }
    }

    Ok(())
}

fn compile_shader(path: &Path, ext: &str) -> io::Result<shaderc::CompilationArtifact> {
    let code = fs::read(path)?;

    let stage = match ext {
        "vert" => shaderc::ShaderKind::Vertex,
        "frag" => shaderc::ShaderKind::Fragment,
        _ => panic!(),
    };

    let mut compiler = shaderc::Compiler::new().unwrap();
    Ok(compiler
        .compile_into_spirv(str::from_utf8(&code).unwrap(), stage, "shader.glsl", "main", None)
        .unwrap())
}

fn save_shader(path: &Path, compilation: shaderc::CompilationArtifact) -> io::Result<()> {
    let code = compilation.as_binary_u8();

    std::fs::write(path, &code)
}
