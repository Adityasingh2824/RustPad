# RustPad - Collaborative Real-Time Code Editor

**RustPad** is a real-time collaborative code editor built with Rust and WebAssembly. It enables multiple users to simultaneously edit code in the browser with synchronized cursors, live previews, and collaboration features. RustPad empowers teams to work together on coding projects directly in the browser without needing additional installations or complex setups.

## Problem it Solves

In the era of remote work and distributed teams, the need for effective real-time collaboration has become crucial. Traditional code editors and IDEs do not easily support multiple users editing the same document simultaneously. RustPad addresses this gap by enabling real-time collaborative editing directly in the browser, allowing developers to see their collaborators' changes and positions instantly. Whether it's brainstorming, coding interviews, or working on code reviews together, RustPad creates a fluid experience that enhances productivity and communication.

## How it Works

RustPad is powered by WebSockets for real-time communication and WebAssembly (compiled from Rust) for high-performance backend logic. Here’s how it works:

1. **Real-Time Collaboration**: As multiple users work on a document, RustPad broadcasts their edits in real-time. Each user sees the same document, updated instantly, with no delay.
2. **Cursor Syncing**: Users’ cursors are tracked in real-time. Each collaborator's cursor is displayed with a unique color, so everyone knows where their teammates are working.
3. **Chat Integration**: The built-in chat system enables users to communicate directly within the editor without the need for external messaging tools, keeping conversations contextually tied to the code.
4. **Live Preview**: RustPad provides live previews for HTML, CSS, and JavaScript code, making it easier for front-end developers to see how their changes render in real-time.
5. **Version History**: RustPad supports tracking changes with a version history, so users can revert to previous versions of the document if needed.

### Video Demo

Watch the video demo of **RustPad** in action, showing its core features and the collaborative editing experience: [Demo Link](#)

## Why You Should Adopt RustPad

RustPad is designed for developers who value collaboration and productivity. With its smooth real-time editing features, RustPad is perfect for:
- **Pair Programming**: Work together with your partner, no matter where they are.
- **Code Reviews**: Collaboratively review code and make adjustments in real-time.
- **Remote Teamwork**: Sync up with your team without the need for screen sharing, leveraging real-time collaboration.
- **Learning and Teaching**: Perfect for code tutorials, online teaching, and mentoring, RustPad allows instructors and students to work together on the same codebase seamlessly.

RustPad’s unique combination of Rust and WebAssembly ensures that the editor performs efficiently, even with multiple collaborators, making it an ideal tool for fast-paced development environments.

## Installation and Setup

### Prerequisites

- **Rust** (Nightly Toolchain)
- **wasm-pack** (For WebAssembly Compilation)
- **Node.js** and **npm** (For managing frontend dependencies and running the development server)

### Build and Run

To get started with RustPad on your local machine, follow these steps:

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/rustpad.git
   cd rustpad
Build the WebAssembly Project:

bash
Copy code
wasm-pack build --target web
This command compiles the Rust code into WebAssembly and prepares the necessary bindings.

Install Frontend Dependencies and Start the Development Server:

bash
Copy code
npm install
npm run start
This will start the frontend of RustPad at http://localhost:8080.

Run the WebSocket Server for Real-Time Collaboration:

Navigate to the websocket-server/ directory and run the WebSocket server:

bash
Copy code
cd websocket-server
cargo run
The WebSocket server will be live on ws://localhost:3030.

Building for Production
To build the project for production deployment:

bash
Copy code
npm run build
This command will generate all necessary files in the dist/ folder for production use.

Usage
Once the server is running, open http://localhost:8080 in your browser, and you can start collaborating with your team in real-time. Upon opening the editor, users can choose a username, and their changes will be synchronized with other collaborators immediately.

Collaborative Editing: Every change you make is synchronized with others in real-time.
Cursor Tracking: See your collaborators' cursor positions, allowing you to work together without stepping on each other’s toes.
Chat and Communication: The built-in chat system enables contextual discussions right within the editor.
License
This project is licensed under the MIT License. See the LICENSE file for more details.

