use crate::usb::send_keyboard_values;


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    rainbow_enabled: bool,
    primary_colour: [u8; 3],
    secondary_colour: [u8; 3],
    brightness_percent: u8,
    speed_percent: u8,

    lighting_mode: LightingMode,
    lighting_direction: LightingDirection,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            rainbow_enabled: false,
            primary_colour: [255, 0, 0],
            secondary_colour: [0, 0, 255],
            brightness_percent: 50,
            speed_percent: 50,
            lighting_mode: LightingMode::Static,
            lighting_direction: LightingDirection::Up,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]
enum LightingDirection {
    Up,
    Down,
    Left,
    Right,
    Clockwise,
    Anticlockwise,
    Inwards,
    Outwards,
}
impl LightingDirection {
    fn backend_value(&self) -> u8 {
        match *self {
            LightingDirection::Left => 0x01,
            LightingDirection::Right => 0x00,
            LightingDirection::Up => 0x02,
            LightingDirection::Down => 0x03,
            LightingDirection::Outwards => 0x04,
            LightingDirection::Inwards => 0x05,
            LightingDirection::Clockwise => 0x06,
            LightingDirection::Anticlockwise => 0x07,
        }
    }
}
impl ToString for LightingDirection {
    fn to_string(&self) -> String {
        match *self {
            LightingDirection::Up => "Up".to_string(),
            LightingDirection::Down => "Down".to_string(),
            LightingDirection::Left => "Left".to_string(),
            LightingDirection::Right => "Right".to_string(),
            LightingDirection::Clockwise => "Clockwise".to_string(),
            LightingDirection::Anticlockwise => "Anticlockwise".to_string(),
            LightingDirection::Inwards => "Inwards".to_string(),
            LightingDirection::Outwards => "Outwards".to_string(),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]
enum LightingMode {
    Static,
    Neon,
    Breathe,
    Wave,
    Stars,
    Spiral,
    AutoRipple,
    Snake,
    Aurora,
    Ripple,
    Reactive,
    Cross,
    Fireworks,
    TypeResponse,
    MusicalRhythm,

}
impl LightingMode {
    fn backend_value(&self) -> u8 {
        match *self {
            LightingMode::Static => 0x00,
            LightingMode::Neon => 0x03,
            LightingMode::Breathe => 0x01,
            LightingMode::Wave => 0x02,
            LightingMode::Stars => 0x09,
            LightingMode::Spiral => 0x04,
            LightingMode::AutoRipple => 0x0e,
            LightingMode::Snake => 0x0f,
            LightingMode::Aurora => 0x07,
            LightingMode::Ripple => 0x08,
            LightingMode::Reactive => 0x06,
            LightingMode::Cross => 0x0b,
            LightingMode::Fireworks => 0x10,
            LightingMode::TypeResponse => 0x0c,
            LightingMode::MusicalRhythm => 0x64,
        }
    }

    fn has_speed(&self) -> bool {
        match *self {
            LightingMode::Static | LightingMode::TypeResponse | LightingMode::MusicalRhythm => false,
            LightingMode::Neon | LightingMode::Breathe | LightingMode::Wave | LightingMode::Stars | LightingMode::Spiral | LightingMode::Ripple | LightingMode::AutoRipple | LightingMode::Snake | LightingMode::Aurora | LightingMode::Reactive | LightingMode::Cross | LightingMode::Fireworks => true,
        }
    }

    fn get_directions(&self) -> &[Option<LightingDirection>; 6] {
        match *self {
            LightingMode::Static | LightingMode::Neon | LightingMode::Breathe | LightingMode::Stars | LightingMode::AutoRipple | LightingMode::Ripple | LightingMode::Reactive | LightingMode::Cross | LightingMode::Fireworks | LightingMode::MusicalRhythm => &[None, None, None, None, None, None],
            LightingMode::Wave => &[Some(LightingDirection::Up), Some(LightingDirection::Down), Some(LightingDirection::Left), Some(LightingDirection::Right), Some(LightingDirection::Inwards), Some(LightingDirection::Outwards)],
            LightingMode::Spiral | LightingMode::Snake => &[Some(LightingDirection::Clockwise), Some(LightingDirection::Anticlockwise), None, None, None, None],
            LightingMode::Aurora => &[Some(LightingDirection::Inwards), Some(LightingDirection::Outwards), None, None, None, None],
            LightingMode::TypeResponse => &[Some(LightingDirection::Up), Some(LightingDirection::Down), None, None, None, None],
        }
    }

    fn has_directions(&self) -> bool {
        for el in *self.get_directions() {
            if let Some(_) = el {
                return true;
            }
        }
        return false;
    }

    
}
impl ToString for LightingMode {
    fn to_string(&self) -> String {
        match *self {
            LightingMode::Static => "Static".to_string(),
            LightingMode::Neon => "Neon".to_string(),
            LightingMode::Breathe => "Breathe".to_string(),
            LightingMode::Wave => "Wave".to_string(),
            LightingMode::Stars => "Stars".to_string(),
            LightingMode::Spiral => "Spiral".to_string(),
            LightingMode::AutoRipple => "Auto Ripple".to_string(),
            LightingMode::Snake => "Snake".to_string(),
            LightingMode::Aurora => "Aurora".to_string(),
            LightingMode::Ripple => "Ripple".to_string(),
            LightingMode::Reactive => "Reactive".to_string(),
            LightingMode::Cross => "Cross".to_string(),
            LightingMode::Fireworks => "Fireworks".to_string(),
            LightingMode::TypeResponse => "Typing Response".to_string(),
            LightingMode::MusicalRhythm => "Musical Response".to_string(),

        }
    }
}


impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let frame_design = egui::containers::Frame {
            inner_margin: egui::epaint::Margin { left: 60., right: 60., top: 60., bottom: 60., },
            fill: egui::Color32::from_rgb(28, 28, 28),
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame_design).show(ctx, |ui| {
            ctx.set_pixels_per_point(1.1);


            ui.heading("FL Esports - Keyboard Colour");
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);


            ui.horizontal(|ui| {
                ui.label("Lighting Mode: ");
                egui::ComboBox::from_label(" ").width(200.0).selected_text(format!("{}", self.lighting_mode.to_string())).show_ui(ui, |ui| {
                    for mode in &[
                        LightingMode::Static, LightingMode::Neon, LightingMode::Breathe, LightingMode::Wave, LightingMode::Stars, LightingMode::Spiral, LightingMode::AutoRipple, LightingMode::Snake, LightingMode::Aurora, LightingMode::Ripple, LightingMode::Reactive, LightingMode::Cross, LightingMode::Fireworks, LightingMode::TypeResponse, LightingMode::MusicalRhythm
                    ] {
                        if ui.selectable_value(&mut self.lighting_mode, *mode, format!("{}", mode.to_string())).clicked() {
                            if !self.lighting_mode.get_directions().contains(&Some(self.lighting_direction)) && self.lighting_mode.has_directions() {
                                self.lighting_direction = self.lighting_mode.get_directions()[0].unwrap();
                            }
                        }
                    }
                });
            });

            ui.add_space(16.0);

            ui.horizontal(|ui| {
                if self.rainbow_enabled {
                    ui.disable();
                }
                ui.label("Primary colour:" );
                ui.color_edit_button_srgb(&mut self.primary_colour);
            });
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.label("Secondary colour:" );
                ui.color_edit_button_srgb(&mut self.secondary_colour);
            });
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.label("Rainbow enabled: ");
                ui.checkbox(&mut self.rainbow_enabled, "");
            });
            
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                ui.label("Backlight brightness: ");
                ui.add(egui::Slider::new(&mut self.brightness_percent, 0..=4).step_by(1.0).show_value(false));
            });
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                if !self.lighting_mode.has_speed() {
                    ui.disable();
                }

                ui.label("Animation speed: ");
                ui.add(egui::Slider::new(&mut self.speed_percent, 0..=4).step_by(1.0).show_value(false));
            });
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                if !self.lighting_mode.has_directions() {
                    ui.disable();

                    ui.label("Animation direction: ");
                    ui.label("No directions available for this mode. ");
                } else {
                    ui.label("Animation direction: ");
                    egui::ComboBox::from_label("").width(150.0).selected_text(format!("{}", self.lighting_direction.to_string())).show_ui(ui, |ui| {
                        for direction in self.lighting_mode.get_directions() {
                            if let Some(dir) = direction {
                                ui.selectable_value(&mut self.lighting_direction, *dir, format!("{}", dir.to_string()));
                            }
                        }
                    });

                }

            });


            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                if ui.add_sized([60.0, 32.0], egui::Button::new("Apply")).clicked() {
                    if send_keyboard_values(
                        self.lighting_mode.backend_value(),
                        self.brightness_percent,
                        self.speed_percent,
                        self.primary_colour[0],
                        self.primary_colour[1],
                        self.primary_colour[2],
                        self.secondary_colour[0],
                        self.secondary_colour[1],
                        self.secondary_colour[2],
                        self.lighting_direction.backend_value(),
                        self.rainbow_enabled
                    ) {
                        println!("usb write executed successfully");
                    } else {
                        println!("error during usb write");
                    }
                }

                if ui.add_sized([44.0, 32.0], egui::Button::new("Exit")).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }


                ui.set_opacity(0.3);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

