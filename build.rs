// Script de build que executa antes da compilação principal
// Necessário para transformar arquivos .slint em código Rust
fn main() {
    // Compila o arquivo de interface Slint em código Rust
    // O arquivo .slint será transformado em structs e métodos utilizáveis
    slint_build::compile("ui/app.slint").unwrap();
}