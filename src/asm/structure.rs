pub struct AsmFile {
    imports: Vec<Label>,
    exports: Vec<Label>,
}

impl AsmFile {
    pub fn new() -> Self {
        AsmFile {
            imports: Vec::new(),
            exports: Vec::new(),
        }
    }

    pub fn add_import(&mut self, name: Vec<u8>) {
        self.imports.push(Label { name });
    }

    pub fn add_export(&mut self, name: Vec<u8>) {
        self.exports.push(Label { name });
    }

    pub fn get_imports(&self) -> &Vec<Label> {
        &self.imports
    }

    pub fn get_exports(&self) -> &Vec<Label> {
        &self.exports
    }
}

pub struct Label {
    name: Vec<u8>,
}

impl Label {
    pub fn new(name: Vec<u8>) -> Self {
        Label { name }
    }

    pub fn get_name(&self) -> &Vec<u8> {
        &self.name
    }
}
