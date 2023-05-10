/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct IronCoderApp {
    // Example stuff:
    label: String,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for IronCoderApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Iron Coder".to_owned(),
            value: 2.7,
        }
    }
}

impl IronCoderApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // we mutate cc.egui_ctx (the context) to set the overall app style
        setup_fonts_and_style(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        
        // Now return a default IronCoderApp
        Default::default()
    }
}

impl eframe::App for IronCoderApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                // ui.spacing_mut().window_margin.left  = 24.0;
                // ui.spacing_mut().window_margin.right = 24.0;
                // ui.spacing_mut().menu_margin.left  = 24.0;
                // ui.spacing_mut().menu_margin.right = 24.0;
                //   example of how to create and display an image
                //   TODO - figure out how to load from a file
                // let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                //     "my-image",
                //     egui::ColorImage::new([64, 16], egui::Color32::WHITE),
                //     Default::default()
                // );
                // // Show the image:
                // ui.image(texture, texture.size_vec2());

                ui.menu_button("MENU", |ui| {
                    if ui.button("SAVE").clicked() {
                        println!("todo!");
                    }
                    if ui.button("OPEN").clicked() {
                        println!("todo!");
                    }
                    if ui.button("BOARDS").clicked() {
                        println!("todo!");
                    }
                    if ui.button("SETTINGS").clicked() {
                        println!("button clicked!");
                    }
                    if ui.button("ABOUT").clicked() {
                        // egui::Window::new("My Window")
                        //     .open(&mut true)
                        //     .default_size(egui::vec2(512.0, 512.0))
                        //     .resizable(true)
                        //     .show(ctx, |ui| {
                        //         ui.label("Hello World!");
                        // });
                        println!("learn how to open a sub window!");
                    }
                    if ui.button("QUIT").clicked() {
                        _frame.close();
                    }
                });

                // add the logo, centered
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    pretty_header(ui, "IRON CODER");
                });
                // TODO - Understand the layout better and get the alignments right
                // ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // ui.menu_button("MENU", |ui| {
                    //     if ui.button("SETTINGS").clicked() {
                    //         println!("button clicked!");
                    //     }
                    //     if ui.button("QUIT").clicked() {
                    //         _frame.close();
                    //     }
                    // });
                // });

            });
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                // pretty_header(ui, "SPEC VIEWER");
                ui.heading("SPEC VIEWER");
            });

            ui.horizontal(|ui| {
                ui.label("3D model will show here: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    egui::warn_if_debug_build(ui);
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        let central_frame = egui::Frame::default();
        egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("eframe template");
            // Try adding a top panel to the centralpanel
            // TODO - why doesn't it push down the TextEdit?
            // egui::TopBottomPanel::top("editor_control_panel").show(ctx, |ui| {
            //     ui.label("test");
            // });
            let mut s: String = "//test".into();
            ui.add(
                egui::TextEdit::multiline(&mut s)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .frame(false),
                    // .layouter(&mut layouter),
            );
        });

    }
}

fn setup_fonts_and_style(ctx: &egui::Context) {

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "platinum_sign_under".to_owned(),    // serves as the unique font identifier?
        egui::FontData::from_static(include_bytes!(
            "../assets/fonts/platinum-sign/Platinum-Sign-Under.ttf"
        )),
    );
    fonts.font_data.insert(
        "platinum_sign_over".to_owned(),    // serves as the unique font identifier?
        egui::FontData::from_static(include_bytes!(
            "../assets/fonts/platinum-sign/Platinum-Sign-Over.ttf"
        )),
    );
    // create a new font family called "heading_fonts"
    fonts.families.insert(
        egui::FontFamily::Name("HeadingBackground".into()),
        vec!(String::from("platinum_sign_under"))
    );
    fonts.families.insert(
        egui::FontFamily::Name("HeadingForeground".into()),
        vec!(String::from("platinum_sign_over"))
    );
    //   example of how to install font to an existing style 
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Monospace)
    //     .or_default()
    //     .push("platinum_sign_over".to_owned());

    ctx.set_fonts(fonts);

    // setup our custom style
    let mut style = egui::style::Style::default();

    // we could change certain aspects of the global spacing like so:
    // style.spacing.menu_margin.left  = 64.0;
    // style.spacing.menu_margin.right = 64.0;
    // println!("{:?}", style.spacing.menu_margin.left_top());

    // Redefine text_styles
    use egui::FontId;
    use egui::FontFamily;
    use egui::TextStyle::*;
    style.text_styles = [
        (Small, FontId::new(10.0, FontFamily::Monospace)),
        (Body, FontId::new(14.0, FontFamily::Monospace)),
        (Monospace, FontId::new(14.0, FontFamily::Monospace)),
        (Button, FontId::new(12.0, FontFamily::Monospace)),
        (Heading, FontId::new(14.0, FontFamily::Monospace)),
        (Name("HeadingBg".into()), FontId::new(18.0, FontFamily::Name("HeadingBackground".into()))),
        (Name("HeadingFg".into()), FontId::new(18.0, FontFamily::Name("HeadingForeground".into()))),
        // example for creating a custom style
        //(Name("Context".into()), FontId::new(23.0, FontFamily::Proportional)),
    ].into();

    ctx.set_style(style);
}

fn pretty_header(ui: &mut egui::Ui, text: &str) {
    /* Displays a cool looking header in the Ui element,
     *  utilizing our custom fonts */
    use egui::{RichText, Label, Color32};
    // draw the background and get the rectangle we drew to
    let text_bg = RichText::new(text.to_uppercase())
        .text_style(egui::TextStyle::Name("HeadingBg".into()));
    let heading_bg = Label::new(text_bg);
    let rect = ui.add(heading_bg).rect;
    // put the overlay text
    let text_fg = RichText::new(text)
        .color(Color32::WHITE)
        .text_style(egui::TextStyle::Name("HeadingFg".into()));
    let heading_fg = Label::new(text_fg);
    // let location = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::ZERO);
    ui.put(rect, heading_fg);
}