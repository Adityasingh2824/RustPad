


RustPad
RustPad is a real-time, collaborative code editor built with Rust. It enables multiple users to simultaneously work on the same document, with peer-to-peer synchronization ensuring all changes are updated in real time. RustPad is designed to be lightweight, efficient, and secure, offering syntax highlighting and integrated version control to streamline collaboration on coding projects.

Why RustPad?
RustPad stands out for its:

Speed & Efficiency: Rust's memory safety and concurrency guarantees make RustPad perform faster and more reliably than traditional code editors.
Peer-to-Peer Collaboration: RustPad utilizes peer-to-peer synchronization, ensuring low-latency communication without relying on a centralized server.
Syntax Highlighting: With built-in support for syntax highlighting in multiple programming languages, RustPad offers an excellent coding experience.
Version Control: RustPad tracks all code changes with built-in version control, enabling easy rollback and management of code history during collaborative sessions.
Cross-Platform: RustPad is designed to be cross-platform, working seamlessly on web and desktop environments, giving developers flexibility and accessibility.



Installation
To install and build RustPad on your machine, follow these steps:

Prerequisites
Rust: Ensure that you have the latest version of Rust installed. You can install Rust by following the instructions at rust-lang.org.

Cargo: Cargo, the Rust package manager, is required to build the project. It comes bundled with Rust.

Web or Desktop Environment:

If you're running the web version, you'll need to have Node.js and npm installed for building the frontend.
For the desktop version, Tauri needs to be installed (optional, depending on your deployment choice).
Build and Run
To build and run RustPad, follow the steps below:

For Web-based RustPad:
Clone the repository:

bash
Copy code
git clone https://github.com/yourusername/rustpad.git
cd rustpad
Build the project:

bash
Copy code
cargo build --release
Run the WebSocket server:

bash
Copy code
cargo run --bin websocket-server
Run the web UI:

bash
Copy code
cd static
npm install
npm run start
Open in Browser: Open your browser and navigate to http://localhost:3000/ to start using RustPad.

For Desktop-based RustPad:
Clone the repository:

bash
Copy code
git clone https://github.com/yourusername/rustpad.git
cd rustpad
Install Tauri:
Follow Tauri’s installation guide to ensure your system is set up.

Build the desktop app:

bash
Copy code
cargo tauri build
Run the desktop app:

bash
Copy code
cargo tauri dev
How It Works
Real-time Collaboration
RustPad uses peer-to-peer WebSockets for real-time collaboration. When multiple users are editing a document, changes are sent to peers and applied immediately, keeping everyone’s code in sync without relying on a central server.

Syntax Highlighting
RustPad leverages the syntect library to provide syntax highlighting for a wide range of programming languages. This enhances code readability and makes collaboration more efficient.

Version Control
RustPad includes a simple version control system to track and manage changes. Users can undo/redo operations and view a history of edits, making collaboration more organized and secure.
