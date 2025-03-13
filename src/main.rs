use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn get_directory_size(path: &Path) -> u64 {
    let mut total_size = 0;
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                total_size += entry.metadata().unwrap().len();
            } else if path.is_dir() {
                total_size += get_directory_size(&path);
            }
        }
    }
    total_size
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}

fn get_font_directories() -> Vec<(PathBuf, u64)> {
    let current_dir = std::env::current_dir().expect("No se pudo obtener el directorio actual");
    println!("Escaneando directorio: {:?}", current_dir);
    let mut dirs = Vec::new();

    match fs::read_dir(&current_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(dirname) = path.file_name() {
                            if dirname.to_string_lossy().starts_with("2") {
                                println!("Encontrado directorio: {:?}", dirname);
                                let size = get_directory_size(&path);
                                println!("Tamaño calculado: {}", format_size(size));
                                dirs.push((path, size));
                            }
                        }
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error al leer el directorio: {}", e);
        }
    }
    
    if dirs.is_empty() {
        println!("⚠️ No se encontraron directorios que empiecen con '2'");
    }
    
    dirs.sort_by(|a, b| a.0.file_name().unwrap().cmp(b.0.file_name().unwrap()));
    dirs
}

fn update_readme(directories: &[(PathBuf, u64)]) -> std::io::Result<()> {
    let readme_path = "README.md";
    println!("Intentando abrir: {}", readme_path);
    
    let mut content = String::new();
    match File::open(readme_path) {
        Ok(mut file) => {
            file.read_to_string(&mut content)?;
            println!("✓ README.md leído correctamente");
        },
        Err(e) => {
            eprintln!("❌ Error al abrir README.md: {}", e);
            return Err(e);
        }
    }

    let mut new_structure = String::from("\n## Directory Structure\n\nThe fonts are organized in directories with the following naming convention:\n\n");
    
    for (path, size) in directories {
        let dirname = path.file_name().unwrap().to_string_lossy();
        let size_str = format_size(*size);
        let description = match dirname.as_ref() {
            "2word" => "Fonts optimized for Microsoft Word and Office applications",
            "2linux" => "Fonts compatible with Linux systems",
            "2mac" => "Fonts for macOS",
            "2web" => "Web fonts",
            _ => "Additional font collection"
        };
        new_structure.push_str(&format!("- `{}/` - {} (Size: {})\n", dirname, description, size_str));
    }
    new_structure.push('\n');

    // Replace content between "## Directory Structure" and "## Current Collections"
    if let Some(start) = content.find("## Directory Structure") {
        if let Some(end) = content.find("## Current Collections") {
            let mut new_content = content[..start].to_string();
            new_content.push_str(&new_structure);
            new_content.push_str(&content[end..]);
            
            fs::write(readme_path, new_content)?;
        }
    }

    println!("✓ README.md actualizado correctamente");
    Ok(())
}

fn main() {
    println!("🔍 Iniciando escaneo de directorios...");
    
    match get_font_directories() {
        dirs if !dirs.is_empty() => {
            println!("\n📁 Directorios encontrados:");
            for (path, size) in &dirs {
                println!("- {:?} ({} bytes)", path, size);
            }

            println!("\n📝 Actualizando README.md...");
            if let Err(e) = update_readme(&dirs) {
                eprintln!("❌ Error actualizando README.md: {}", e);
            } else {
                println!("✅ Proceso completado exitosamente!");
            }
        },
        _ => println!("❌ No se encontraron directorios de fuentes!")
    }
}
