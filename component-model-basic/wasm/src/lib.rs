wit_bindgen::generate!({path: "../wit/example.wit"});

struct ExampleGreeter {}

// let's implement the "greeter" interface
impl greeter::Greeter for ExampleGreeter {
  fn greet(name: String) -> String {
      let log_message = format!("Greeting {name}.");
      let greeting = format!("Hello, {name}!");
      
      // we will use the "host" interface to log what we are doing
      host::log(&log_message);
      greeting
  }
}

export_example!(ExampleGreeter);
