// === IMPORTS ===
use slint::{ComponentHandle, Timer, TimerMode};
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use rand::Rng;

// Macro que inclui o código Rust gerado a partir dos arquivos .slint
slint::include_modules!();

// === ESTRUTURAS DE DADOS ===
// Estrutura para salvar/carregar dados da aplicação
#[derive(Serialize, Deserialize, Debug)]
struct AppData {
    counter: i32,
    timer_seconds: i32,
    user_text: String,
    random_min: f32,
    random_max: f32,
    dark_theme: bool,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            counter: 0,
            timer_seconds: 0,
            user_text: String::new(),
            random_min: 1.0,
            random_max: 100.0,
            dark_theme: false,
        }
    }
}

// === FUNÇÕES AUXILIARES ===
// Valida se o texto é um email válido
fn validate_email(text: &str) -> (bool, String) {
    if text.is_empty() {
        return (true, "Digite um email...".to_string());
    }
    
    if !text.contains('@') {
        return (false, "Email deve conter @".to_string());
    }
    
    if !text.contains('.') {
        return (false, "Email deve conter um domínio".to_string());
    }
    
    let parts: Vec<&str> = text.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return (false, "Formato de email inválido".to_string());
    }
    
    (true, "✓ Email válido!".to_string())
}

// Salva dados da aplicação em arquivo JSON
fn save_app_data(data: &AppData) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Erro ao serializar dados: {}", e))?;
    
    fs::write("app_data.json", json)
        .map_err(|e| format!("Erro ao salvar arquivo: {}", e))?;
    
    println!("Dados salvos com sucesso!");
    Ok(())
}

// Carrega dados da aplicação do arquivo JSON
fn load_app_data() -> Result<AppData, String> {
    let json = fs::read_to_string("app_data.json")
        .map_err(|e| format!("Erro ao ler arquivo: {}", e))?;
    
    let data: AppData = serde_json::from_str(&json)
        .map_err(|e| format!("Erro ao deserializar dados: {}", e))?;
    
    println!("Dados carregados com sucesso!");
    Ok(data)
}

// === FUNÇÃO PRINCIPAL ===
#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    // Cria instância da janela principal
    let ui = AppWindow::new()?;
    
    // === ESTADO COMPARTILHADO ===
    // Timer compartilhado entre threads
    let timer_running = Arc::new(Mutex::new(false));
    let timer_seconds = Arc::new(Mutex::new(0i32));
    
    // === CONFIGURAÇÃO INICIAL ===
    ui.set_counter(0);
    ui.set_timer_seconds(0);
    ui.set_timer_running(false);
    ui.set_random_min(1.0);
    ui.set_random_max(100.0);
    ui.set_dark_theme(false);
    
    // === CALLBACKS BÁSICOS ===
    // Incrementar contador
    {
        let ui_weak = ui.as_weak();
        ui.on_increment_counter(move || {
            let ui = ui_weak.unwrap();
            let current = ui.get_counter();
            ui.set_counter(current + 1);
            println!("Contador incrementado para: {}", current + 1);
        });
    }
    
    // Reset contador
    {
        let ui_weak = ui.as_weak();
        ui.on_reset_counter(move || {
            let ui = ui_weak.unwrap();
            ui.set_counter(0);
            println!("Contador resetado!");
        });
    }
    
    // === TIMER GLOBAL ===
    // Cria timer global que sempre roda
    let global_timer = Timer::default();
    let ui_weak_timer = ui.as_weak();
    let timer_running_global = timer_running.clone();
    let timer_seconds_global = timer_seconds.clone();
    
    global_timer.start(TimerMode::Repeated, Duration::from_millis(1000), move || {
        let running = timer_running_global.lock().unwrap();
        if *running {
            let mut seconds = timer_seconds_global.lock().unwrap();
            *seconds += 1;
            
            if let Some(ui) = ui_weak_timer.upgrade() {
                ui.set_timer_seconds(*seconds);
                println!("Timer: {} segundos", *seconds);
            }
        }
    });
    
    // === CALLBACKS DO TIMER ===
    // Iniciar timer
    {
        let ui_weak = ui.as_weak();
        let timer_running = timer_running.clone();
        
        ui.on_start_timer(move || {
            let ui = ui_weak.unwrap();
            let mut running = timer_running.lock().unwrap();
            *running = true;
            ui.set_timer_running(true);
            println!("Timer iniciado!");
        });
    }
    
    // Parar timer
    {
        let ui_weak = ui.as_weak();
        let timer_running = timer_running.clone();
        
        ui.on_stop_timer(move || {
            let ui = ui_weak.unwrap();
            let mut running = timer_running.lock().unwrap();
            *running = false;
            ui.set_timer_running(false);
            println!("Timer parado!");
        });
    }
    
    // Reset timer
    {
        let ui_weak = ui.as_weak();
        let timer_running = timer_running.clone();
        let timer_seconds = timer_seconds.clone();
        
        ui.on_reset_timer(move || {
            let ui = ui_weak.unwrap();
            let mut running = timer_running.lock().unwrap();
            let mut seconds = timer_seconds.lock().unwrap();
            
            *running = false;
            *seconds = 0;
            
            ui.set_timer_running(false);
            ui.set_timer_seconds(0);
            println!("Timer resetado!");
        });
    }
    
    // === CALLBACKS DE VALIDAÇÃO ===
    // Validar texto (função que retorna resultado)
    ui.on_validate_text(|text| {
        let (is_valid, message) = validate_email(&text);
        println!("Validando texto '{}': {} - {}", text, is_valid, message);
        message.into()
    });
    
    // Notificação de mudança de texto
    {
        let ui_weak = ui.as_weak();
        ui.on_text_changed(move |text| {
            let ui = ui_weak.unwrap();
            let (is_valid, _) = validate_email(&text);
            ui.set_text_valid(is_valid);
            println!("Texto alterado: '{}' - Válido: {}", text, is_valid);
        });
    }
    
    // === CALLBACKS DE NÚMEROS ALEATÓRIOS ===
    // Gerar número aleatório
    {
        let ui_weak = ui.as_weak();
        ui.on_generate_random(move || {
            let ui = ui_weak.unwrap();
            let min = ui.get_random_min() as i32;
            let max = ui.get_random_max() as i32;
            
            let mut rng = rand::thread_rng();
            let random_num = rng.gen_range(min..=max);
            
            ui.set_random_number(random_num);
            println!("Número aleatório gerado: {} (entre {} e {})", random_num, min, max);
        });
    }
    
    // Definir intervalo de números aleatórios
    {
        let ui_weak = ui.as_weak();
        ui.on_set_random_range(move |min, max| {
            let ui = ui_weak.unwrap();
            println!("Intervalo de números alterado: {} a {}", min as i32, max as i32);
            
            // Garante que min < max
            if min >= max {
                ui.set_random_max(min + 1.0);
            }
        });
    }
    
    // === CALLBACKS DE PERSISTÊNCIA ===
    // Salvar dados
    {
        let ui_weak = ui.as_weak();
        ui.on_save_data(move || {
            let ui = ui_weak.unwrap();
            
            let data = AppData {
                counter: ui.get_counter(),
                timer_seconds: ui.get_timer_seconds(),
                user_text: ui.get_user_text().to_string(),
                random_min: ui.get_random_min(),
                random_max: ui.get_random_max(),
                dark_theme: ui.get_dark_theme(),
            };
            
            match save_app_data(&data) {
                Ok(_) => println!("✓ Dados salvos com sucesso!"),
                Err(e) => println!("✗ Erro ao salvar: {}", e),
            }
        });
    }
    
    // Carregar dados
    {
        let ui_weak = ui.as_weak();
        ui.on_load_data(move || {
            let ui = ui_weak.unwrap();
            
            match load_app_data() {
                Ok(data) => {
                    ui.set_counter(data.counter);
                    ui.set_timer_seconds(data.timer_seconds);
                    ui.set_user_text(data.user_text.into());
                    ui.set_random_min(data.random_min);
                    ui.set_random_max(data.random_max);
                    ui.set_dark_theme(data.dark_theme);
                    
                    println!("✓ Dados carregados com sucesso!");
                },
                Err(e) => {
                    println!("✗ Erro ao carregar: {}", e);
                    println!("Usando valores padrão...");
                }
            }
        });
    }
    
    // === INICIALIZAÇÃO ===
    println!("🚀 Aplicação Rust + Slint iniciada!");
    println!("📋 Funcionalidades disponíveis:");
    println!("   • Contador com incremento/reset");
    println!("   • Timer com controles start/stop/reset");
    println!("   • Validação de email em tempo real");
    println!("   • Gerador de números aleatórios");
    println!("   • Persistência de dados em JSON");
    println!("   • Tema claro/escuro");
    println!("");
    
    // Inicia o loop de eventos da aplicação
    ui.run()
}
