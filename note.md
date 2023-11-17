## Draft Ideas
- Resolved linux kernel permission. Involved OS configuration to give user `input` `plugdev` permission. This
permission is required in order to block the OS default keyboard listener to avoid conflicts with our keyboard
simulation.
- Research open-source library to listen keyboard commands
- Design solution to record combination keybinding.
- Event-driven application.


## Make it better now
- Keyboard mapper is an event-driven desktop application that map operating system hotkeys to
customized shortcuts.
- Reseached open-source library to listen keyboard events.
- Created an keyboard simulator that alters the event destination to the application logic instead of the OS kernel handler.
- Designed solution to record combination keybinding; the solution involves differientiating between pressing and releasing events.


## GPT-ed
- Developed "Keyboard Mapper" – a Rust-based desktop application for mapping OS hotkeys to custom shortcuts with an event-driven design.
- Researched and integrated an open-source keyboard event library for precise input management.
- Designed a custom keyboard simulator to route events to the application's logic, bypassing OS kernel handlers.
- Engineered a solution to record complex keybindings, differentiating between keypress and release events for accuracy and flexibility.
