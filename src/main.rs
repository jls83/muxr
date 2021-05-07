struct Window<'a> {
    id: u32,  // TODO: I bet we can make this better
    title: String,
    panes: Vec<&'a Pane>,
}

impl<'a> Window<'a> {
    fn new(id: u32, title: String) -> Window<'a> {
        Window {
            id,
            title,
            panes: vec![],
        }
    }

    fn push_pane(&mut self, pane: &'a Pane) {
        self.panes.push(pane);
    }

    fn print_panes(&self) {
        for pane in self.panes.iter() {
            pane.print_pane();
        }
    }
}

struct Pane {
    pid: u32,
    // Sizes are in "text cells"
    width: u32,
    height: u32,
}

impl Pane {
    fn new(pid: u32, width: u32, height: u32) -> Pane {
        Pane {
            pid,
            width,
            height,
        }
    }

    fn print_pane(&self) {
        println!("{:?} {:?} {:?}", self.pid, self.width, self.height);
    }
}

fn main() {
    let mut window = Window::new(55, "Banana".to_string());
    let pane = Pane::new(102, 100, 100);

    window.push_pane(&pane);
    window.print_panes();
    println!("{:?} {:?}", window.id, window.title);
}
