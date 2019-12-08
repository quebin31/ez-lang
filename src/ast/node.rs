use std::io::Write;

pub struct Visitor {
    out: Box<dyn Write>,
}

impl Visitor {
    pub fn write(&mut self, string: &str) {
        write!(self.out, "{}", string).expect("Failed to write!");
    }

    pub fn writeln(&mut self, string: &str) {
        writeln!(self.out, "{}", string).expect("Failed to write!");
    }

    pub fn emit_inst(&mut self, string: &str) {
        self.writeln(&format!("\t{}", string));
    }

    pub fn emit_label(&mut self, label: usize) {
        self.write(&format!("L{}", label));
    }

    pub fn emit_jump(&mut self, test: &str, true_label: usize, false_label: usize) {
        match (true_label, false_label) {
            (0, 0) => {}
            (0, _) => self.emit_inst(&format!("jmpf L{} {}", false_label, test)),
            (_, 0) => self.emit_inst(&format!("jmpt L{} {}", true_label, test)),
            (_, _) => {
                self.emit_inst(&format!("jmpt L{} {}", true_label, test));
                self.emit_inst(&format!("jmp L{}", false_label));
            }
        }
    }
}
