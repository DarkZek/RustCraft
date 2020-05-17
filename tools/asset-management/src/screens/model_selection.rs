use imgui::{Ui, Condition, ImString, ImStr};
use std::fs;

pub struct ModelSelection {
    pub model_selected_index: i32,
    pub model_search_query: ImString,
    pub model_active_query: ImString,
    pub model_search_results: Vec<ImString>,
    pub model_search_objects: Vec<ImString>,
    pub material_selected_index: i32,
    pub material_search_query: ImString,
    pub material_active_query: ImString,
    pub material_search_results: Vec<ImString>,
    pub material_search_objects: Vec<ImString>
}

impl ModelSelection {

    pub fn new() -> ModelSelection {
        let mut selection = ModelSelection {
            model_selected_index: 0,
            model_search_query: ImString::with_capacity(50),
            model_active_query: ImString::with_capacity(50),
            model_search_results: Vec::new(),
            model_search_objects: Vec::new(),
            material_selected_index: 0,
            material_search_query: ImString::with_capacity(50),
            material_active_query: ImString::with_capacity(50),
            material_search_results: Vec::new(),
            material_search_objects: Vec::new()
        };

        selection.get_results();
        selection.model_search_results = selection.model_search_objects.clone();
        selection.material_search_results = selection.material_search_objects.clone();
        selection
    }

    pub fn frame(&mut self, ui: &Ui) {

        let window = imgui::Window::new(im_str!("Model Selection"));
        window
            .size([400.0, 300.0], Condition::FirstUseEver)
            .build(&ui, || {
                if ui.input_text(im_str!("Model Search"), &mut self.model_search_query).auto_select_all(true).build() {
                    self.update_model_search_query();
                }

                let model_search_results = self.model_search_results.iter()
                    .map(|x| x.as_ref())
                    .collect::<Vec<&ImStr>>();

                ui.list_box(im_str!("Models"), &mut self.model_selected_index, model_search_results.as_slice(), 4);

                ui.separator();

                if ui.input_text(im_str!("Material Search"), &mut self.material_search_query).build() {
                    self.update_material_search_query();
                }

                let material_search_results = self.material_search_results.iter()
                    .map(|x| x.as_ref())
                    .collect::<Vec<&ImStr>>();

                ui.list_box(im_str!("Materials"), &mut self.material_selected_index, material_search_results.as_slice(), 4);
            });
    }

    pub fn update_model_search_query(&mut self) {
        if self.model_search_query == self.model_active_query {
            return;
        }

        // Query changed

        self.model_search_results.clear();

        for result in &self.model_search_objects {
            if result.to_str().contains(self.model_search_query.to_str()) {
                self.model_search_results.push(result.clone());
            }
        }

        self.model_active_query = self.model_search_query.clone();
    }

    pub fn update_material_search_query(&mut self) {
        if self.material_search_query == self.material_active_query {
            return;
        }

        // Query changed

        self.material_search_results.clear();

        for result in &self.material_search_objects {
            if result.to_str().contains(self.material_search_query.to_str()) {
                self.material_search_results.push(result.clone());
            }
        }

        self.material_active_query = self.material_search_query.clone();
    }

    pub fn get_results(&mut self) {
        let paths = fs::read_dir("/home/darkzek/Projects/RustCraft/assets/models/").unwrap();
        self.model_search_objects.clear();
        self.material_search_objects.clear();

        for path in paths {

            let name = path.unwrap().file_name();
            let filename = name.to_str().unwrap();

            if filename.ends_with(".mcv3m") {
                self.model_search_objects.push(ImString::new(filename))
            } else if filename.ends_with(".mcv3t") {
                self.material_search_objects.push(ImString::new(filename))
            }

        }
    }
}