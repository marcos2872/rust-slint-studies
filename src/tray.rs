// === MÓDULO SYSTEM TRAY ===
use std::sync::{Arc, Mutex};
use tray_icon::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use slint::ComponentHandle;
use crate::AppWindow;
use std::path::Path;

// Estrutura para gerenciar o system tray
pub struct SystemTray {
    _tray_icon: TrayIcon,
    pub is_window_visible: Arc<Mutex<bool>>,
}

impl SystemTray {
    pub fn new(ui: slint::Weak<AppWindow>) -> Result<Self, Box<dyn std::error::Error>> {
        // === DEBUG: Verifica suporte ao system tray ===
        println!("🔍 Inicializando system tray...");
        
        // === ÍCONE DO TRAY ===
        // Tenta carregar PNG dos assets, senão usa ícone gerado
        let icon = if let Ok(icon_from_png) = load_icon_from_assets() {
            println!("✅ Ícone PNG carregado dos assets");
            icon_from_png
        } else {
            println!("⚠️ PNG não encontrado, usando ícone gerado");
            let icon_data = create_tray_icon();
            println!("✅ Ícone gerado: {} bytes", icon_data.len());
            
            tray_icon::Icon::from_rgba(icon_data, 16, 16)
                .map_err(|e| {
                    println!("❌ Erro ao criar ícone: {}", e);
                    e
                })?
        };

        // === CONSTRUÇÃO DO TRAY ===
        println!("🔧 Construindo tray icon...");
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("Slint App - Clique para mostrar/ocultar janela")
            .with_icon(icon)
            .build()
            .map_err(|e| {
                println!("❌ Erro ao criar tray icon: {}", e);
                e
            })?;
            
        println!("✅ System tray criado com sucesso!");

        // === ESTADO ===
        let is_window_visible = Arc::new(Mutex::new(true));

        // === HANDLER DE EVENTOS DO TRAY ===
        let ui_clone = ui.clone();
        let is_visible_clone = is_window_visible.clone();

        // Usa slint::invoke_from_event_loop para operações seguras na UI thread
        std::thread::spawn(move || {
            loop {
                if let Ok(event) = tray_icon::TrayIconEvent::receiver().try_recv() {
                    match event {
                        TrayIconEvent::Click { .. } => {
                            // Clona referências para o closure
                            let ui_ref = ui_clone.clone();
                            let visible_ref = is_visible_clone.clone();
                            
                            // Executa na thread principal do Slint
                            slint::invoke_from_event_loop(move || {
                                if let Some(ui) = ui_ref.upgrade() {
                                    let mut visible = visible_ref.lock().unwrap();
                                    if *visible {
                                        let _ = ui.window().hide();
                                        *visible = false;
                                        println!("🔸 Janela ocultada via clique no tray");
                                    } else {
                                        let _ = ui.window().show();
                                        *visible = true;
                                        println!("🔹 Janela mostrada via clique no tray");
                                    }
                                }
                            }).unwrap_or_else(|_| {
                                println!("⚠️ Erro ao processar evento do tray");
                            });
                        }
                        _ => {}
                    }
                }
                // Reduz polling para melhor performance
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });

        Ok(SystemTray {
            _tray_icon: tray_icon,
            is_window_visible,
        })
    }
}

// Carrega ícone PNG dos assets
fn load_icon_from_assets() -> Result<tray_icon::Icon, Box<dyn std::error::Error>> {
    let icon_paths = [
        "assets/icon-48.png",
        "assets/icon-32.png", 
        "assets/icon.png",
    ];
    
    for path in &icon_paths {
        if Path::new(path).exists() {
            println!("📁 Tentando carregar ícone: {}", path);
            
            // Carrega e decodifica o PNG
            let img_bytes = std::fs::read(path)?;
            let img = image::load_from_memory(&img_bytes)?;
            
            // Converte para RGBA e redimensiona se necessário
            let rgba_img = img.to_rgba8();
            let (width, height) = rgba_img.dimensions();
            
            println!("🖼️ Imagem carregada: {}x{} pixels", width, height);
            
            // Redimensiona para 16x16 se necessário
            let final_img = if width != 16 || height != 16 {
                println!("📏 Redimensionando para 16x16...");
                image::imageops::resize(&rgba_img, 16, 16, image::imageops::FilterType::Lanczos3)
            } else {
                rgba_img
            };
            
            let icon_data = final_img.into_raw();
            println!("✅ Ícone PNG processado: {} bytes", icon_data.len());
            
            return Ok(tray_icon::Icon::from_rgba(icon_data, 16, 16)?);
        }
    }
    
    Err("Nenhum ícone PNG encontrado nos caminhos especificados".into())
}

// Cria um ícone simples para o tray (16x16 pixels)
fn create_tray_icon() -> Vec<u8> {
    let mut icon_data = Vec::with_capacity(16 * 16 * 4); // RGBA

    for y in 0..16 {
        for x in 0..16 {
            // Cria um ícone mais visível - quadrado azul com borda
            let border = 1;
            let is_border = x < border || x >= 16 - border || y < border || y >= 16 - border;
            let is_inner_area = x >= border + 1 && x < 16 - border - 1 && y >= border + 1 && y < 16 - border - 1;
            
            if is_border {
                // Borda branca para contraste
                icon_data.extend_from_slice(&[255, 255, 255, 255]); // RGBA - branco
            } else if is_inner_area {
                // Área interna azul brilhante
                icon_data.extend_from_slice(&[0, 150, 255, 255]); // RGBA - azul brilhante
            } else {
                // Área de transição - azul escuro
                icon_data.extend_from_slice(&[0, 80, 180, 255]); // RGBA - azul escuro
            }
        }
    }

    println!("🎨 Ícone criado: 16x16 pixels, {} bytes total", icon_data.len());
    icon_data
}