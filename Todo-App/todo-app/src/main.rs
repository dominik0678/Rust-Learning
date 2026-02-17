use eframe::egui;
use csv::Writer;
use std::path::Path;


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ToDo App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    input: String,
    todos: Vec<Todo>,
}
struct Todo {
    text: String,
    done: bool,
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ToDos:");

            // check if csv file exists
            if !Path::new("todos.csv").exists() {
                let mut wtr = csv::Writer::from_path("todos.csv")
                    .expect("Could not create CSV file");
                wtr.write_record(&["Text", "Done"])
                    .expect("Could not write header");
                wtr.flush().unwrap();
            }
            //read csv file
            let mut rdr = csv::Reader::from_path("todos.csv").expect("Could not read CSV file");
            self.todos.clear();
            for result in rdr.records() {
                let record = result.expect("Could not read record");
                self.todos.push(Todo {
                    text: record[0].to_string(),
                    done: record[1].parse::<bool>().expect("Could not parse boolean"),
                });
            }
            
            // Eingabe + Add
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input);
                if ui.button("Add ➕").clicked() && !self.input.trim().is_empty() {
                    self.todos.push(Todo {
                        text: self.input.trim().to_string(),
                        done: false,
                    });
                    self.input.clear();

                    
                }
            });

            // Liste (scrollbar)
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut delete_idx: Option<usize> = None; 

                for (i, todo) in self.todos.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut todo.done, "");
                        ui.label(&todo.text);

                        if ui.button("🗑").clicked() {
                            delete_idx = Some(i);
                        }
                    });
                }

                if let Some(i) = delete_idx {
                    self.todos.remove(i);
                }
            });

            // CSV export
            let mut wtr = Writer::from_path("todos.csv").expect("Could not create CSV file");
            wtr.write_record(&["Text", "Done"]).expect("Could not write header");
            for todo in &self.todos {
                wtr.write_record(&[&todo.text, &todo.done.to_string()]).expect("Could not write record");
            }
            wtr.flush().expect("Could not flush CSV writer");
        });

        
    }
}
