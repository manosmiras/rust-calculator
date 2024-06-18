use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operation {
    Add,
    Append,
    Subtract,
    Multiply,
    Divide,
    Square,
    SquareRoot,
    Negate,
    Equal,
    Decimal,
    None,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Calculator {
    #[serde(skip)]
    total: f64,
    #[serde(skip)]
    current: f64,
    #[serde(skip)]
    operation_history: Vec<Operation>,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            total: 0.0,
            current: 0.0,
            operation_history: Vec::new(),
        }
    }
}

impl Calculator {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn operate(&mut self, operation: Operation, rhs: Option<f64>) {
        match operation {
            Operation::Add => {
                self.total = self.total + rhs.unwrap();
                self.current = 0.0;
            }
            Operation::Append => {
                let previous_operation = self.operation_history.clone().pop();
                match previous_operation {
                    None => {
                        let appended_numbers =
                            self.current.to_string() + &*rhs.unwrap().to_string();
                        self.current = appended_numbers.parse().unwrap();
                    }
                    Some(previous_operation) => {
                        if previous_operation == Operation::Decimal && self.current.fract() == 0.0 {
                            let appended_numbers =
                                self.current.to_string() + "." + &*rhs.unwrap().to_string();
                            self.current = appended_numbers.parse().unwrap();
                        } else {
                            let appended_numbers =
                                self.current.to_string() + &*rhs.unwrap().to_string();
                            self.current = appended_numbers.parse().unwrap();
                        }
                    }
                }
            }
            Operation::Subtract => {
                if self.total == 0.0 {
                    self.total = rhs.unwrap();
                } else {
                    self.total = self.total - rhs.unwrap();
                }
                self.current = 0.0;
            }
            Operation::Multiply => {
                if self.total == 0.0 {
                    self.total = rhs.unwrap();
                } else {
                    self.total = self.total * rhs.unwrap();
                }
                self.current = 0.0;
            }
            Operation::Divide => {
                if self.total == 0.0 {
                    self.total = rhs.unwrap();
                } else {
                    self.total = self.total / rhs.unwrap();
                }
                self.current = 0.0;
            }
            Operation::Square => {
                self.total = rhs.unwrap().powf(2.0);
                self.current = 0.0;
            }
            Operation::SquareRoot => {
                self.total = rhs.unwrap().sqrt();
                self.current = 0.0;
            }
            Operation::Negate => {
                self.current = self.current * -1.0;
            }
            Operation::Equal => {
                let previous_operation = self.find_last_operation_excluding(vec![
                    Operation::Equal,
                    Operation::Append,
                    Operation::Decimal,
                ]);
                match previous_operation {
                    None => {}
                    Some(previous_operation) => {
                        self.operate(previous_operation, self.current.into());
                    }
                }
            }
            _ => {}
        }
        self.operation_history.push(operation);
    }

    pub fn find_last_operation_excluding(
        &self,
        excluded_operations: Vec<Operation>,
    ) -> Option<Operation> {
        let cloned_operation_history = self.operation_history.clone();
        for operation in cloned_operation_history.iter().rev() {
            if excluded_operations.contains(operation) {
                continue;
            }
            return Some(operation.clone());
        }
        None
    }
}

impl eframe::App for Calculator {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::CentralPanel::default().show(ctx, |ui| {
            /*ui.horizontal(|ui| {
                let previous_operation = self.operation_history.pop().unwrap();
                ui.label(previous_operation.to_string());
            });*/

            ui.horizontal(|ui| {
                ui.label(self.total.to_string());
            });

            ui.horizontal(|ui| {
                ui.label(self.current.to_string());
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let button_width =
                    calculate_button_length(ui.available_width(), ui.spacing().item_spacing.x, 4);
                let button_height = 0.0;
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("C"))
                    .clicked()
                {
                    self.current = 0.0;
                    self.total = 0.0;
                    //self.previous_operation = Operation::None;
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("sqrt"))
                    .clicked()
                {
                    self.operate(Operation::SquareRoot, self.current.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("pow"))
                    .clicked()
                {
                    self.operate(Operation::Square, self.current.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("/"))
                    .clicked()
                {
                    self.operate(Operation::Divide, self.current.into());
                }
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let button_width =
                    calculate_button_length(ui.available_width(), ui.spacing().item_spacing.x, 4);
                let button_height = 0.0;
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("7"))
                    .clicked()
                {
                    self.operate(Operation::Append, 7.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("8"))
                    .clicked()
                {
                    self.operate(Operation::Append, 8.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("9"))
                    .clicked()
                {
                    self.operate(Operation::Append, 9.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("x"))
                    .clicked()
                {
                    self.operate(Operation::Multiply, self.current.into());
                }
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let button_width =
                    calculate_button_length(ui.available_width(), ui.spacing().item_spacing.x, 4);
                let button_height = 0.0;
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("4"))
                    .clicked()
                {
                    self.operate(Operation::Append, 4.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("5"))
                    .clicked()
                {
                    self.operate(Operation::Append, 5.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("6"))
                    .clicked()
                {
                    self.operate(Operation::Append, 6.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("-"))
                    .clicked()
                {
                    self.operate(Operation::Subtract, self.current.into());
                }
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let button_width =
                    calculate_button_length(ui.available_width(), ui.spacing().item_spacing.x, 4);
                let button_height = 0.0;
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("1"))
                    .clicked()
                {
                    self.operate(Operation::Append, 1.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("2"))
                    .clicked()
                {
                    self.operate(Operation::Append, 2.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("3"))
                    .clicked()
                {
                    self.operate(Operation::Append, 3.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("+"))
                    .clicked()
                {
                    self.operate(Operation::Add, self.current.into());
                }
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let button_width =
                    calculate_button_length(ui.available_width(), ui.spacing().item_spacing.x, 4);
                let button_height = 0.0;
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("+/-"))
                    .clicked()
                {
                    self.operate(Operation::Negate, None);
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("0"))
                    .clicked()
                {
                    self.operate(Operation::Append, 0.0.into());
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("."))
                    .clicked()
                {
                    self.operate(Operation::Decimal, None);
                }
                if ui
                    .add_sized([button_width, button_height], egui::Button::new("="))
                    .clicked()
                {
                    self.operate(Operation::Equal, None);
                }
            });
        });
    }
}

fn calculate_button_length(available_width: f32, padding: f32, num_buttons: u8) -> f32 {
    (available_width - padding * (num_buttons - 1) as f32) / num_buttons as f32
}
