// Importa trait necessário para gerenciar componentes Slint
use slint::ComponentHandle;

// Macro que inclui o código Rust gerado a partir dos arquivos .slint
// Cria structs e métodos baseados nos componentes definidos em app.slint
slint::include_modules!();

// Função principal da aplicação
fn main() -> Result<(), slint::PlatformError> {
    // Cria uma nova instância da janela principal (AppWindow)
    // O "?" propaga erros se a criação falhar
    let ui = AppWindow::new()?;
    
    // Define o valor inicial do contador como 0
    // Este método é gerado automaticamente baseado na propriedade counter do .slint
    ui.set_counter(0);
    
    // Inicia o loop de eventos da aplicação
    // A aplicação permanece ativa até ser fechada pelo usuário
    ui.run()
}
