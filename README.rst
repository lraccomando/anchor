=====================
Cast off with Anchor.
=====================

Goal of this project is to create a simple, fast, and intuitive web framework in Rust. The end result will be something like:

.. code-block:: rust

    struct HelloWorld;

    impl Controller for HelloWorld {
        fn get(&self, request: &mut Request) -> Response {
            Body("Hello, World")
        }
    }

    let mut app = Anchor::new();
    app.register("/", HelloWorld);
    app.run();

We are not there yet, but steady progress is the name of the game.
