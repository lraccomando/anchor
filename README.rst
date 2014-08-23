=====================
Cast off with Anchor.
=====================

Goal of this project is to create a simple, fast, and intuitive web framework in Rust.

Very much WIP but this is where we are at thus far.

Obligatory Hello World -

.. code-block:: rust

    struct HelloWorld;
    impl Controller for HelloWorld {
        fn get(&self, request: &mut Request) -> Response {
            Body("Hello, World".to_string())
        }
    }

    let mut app = Anchor::new();
    app.register("/", HelloWorld);
    app.run();

Pass parameters to controllers -

.. code-block:: rust

    struct HelloName;
    impl Controller for HelloName {
         fn get(&self, request: &mut Request) -> Response {
         	let name = request.get_param("name");
         	let greeting = "Hello, ".to_string() + name;
            Body(greeting)
        }
    }

    let mut app = Anchor::new();
    // Enter a named route, Sinatra style and the value will be accessible from
    // the request.
    app.register("/hello/:name/", HelloName);
    app.run();
