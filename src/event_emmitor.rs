pub struct EventEmitter {
    listeners: Vec<Box<dyn Fn(String)>>,
}

pub impl EventEmitter {
    fn new() -> Self {
        EventEmitter {
            listeners: Vec::new(),
        }
    }

    fn subscribe<F>(&mut self, listener: F)
    where
        F: Fn(String) + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    fn emit(&self, event: String) {
        for listener in &self.listeners {
            listener(event.clone());
        }
    }
}

fn main() {
    let mut emitter: EventEmitter = EventEmitter::new();
    print!("111111111111111111111\n");
    emitter.subscribe(|event| {
        println!("Received event: {}\n", event);
    });
    print!("222222222222222222222222222\n");

    emitter.emit("Event 1".to_string());
    print!("33333333333333333333333333333\n");

    emitter.emit("Event 2".to_string());
    print!("4444444444444444444444444444444444444\n");
}
