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
            println!("{:?}", pane.pid);
        }
    }
}

struct Pane {
    pid: u32,
}

fn main() {
    let mut window = Window::new(55, "Banana".to_string());
    let pane = Pane { pid: 102 };

    window.push_pane(&pane);
    window.print_panes();
    println!("{:?} {:?}", window.id, window.title);
}
