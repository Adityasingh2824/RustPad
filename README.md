# RustPad - Collaborative Real-Time Code Editor

RustPad is a real-time collaborative code editor built with Rust and WebAssembly. It enables multiple users to simultaneously edit code in the browser with synchronized cursors, live previews, and a built-in chat system. RustPad enhances remote teamwork and productivity by facilitating real-time collaboration directly in the browser without requiring complex setups or additional installations.

## Problem it Solves

In today's era of remote work, traditional code editors and IDEs lack seamless support for collaborative editing. This makes it difficult for teams to work together in real-time on the same document. RustPad addresses this problem by providing a fluid real-time collaboration experience directly in the browser, perfect for brainstorming sessions, pair programming, code reviews, or teaching environments.

## How it Works

RustPad is powered by WebSockets and WebAssembly, combining the real-time performance of Rust with the accessibility of the web. Key features include:

- **Real-Time Collaboration:** All users see the same document, with updates instantly synchronized across all connected clients.
- **Cursor Syncing:** Each collaborator's cursor is tracked and displayed with a unique color, allowing users to see where their teammates are editing.
- **Built-In Chat:** Contextual communication within the editor, removing the need for external messaging tools.
- **Live Preview:** Real-time preview of HTML, CSS, and JavaScript code, making it ideal for front-end development collaboration.
- **Version History:** Track changes and revert to previous document states if necessary.


## Why You Should Adopt RustPad

RustPad is designed for developers who prioritize collaboration, productivity, and seamless real-time editing. Whether you're pair programming, reviewing code, or working with a remote team, RustPad enables you to collaborate effectively and efficiently without the need for screen sharing or additional software.

### Use Cases:
- **Pair Programming:** Collaborate with your partner in real-time, no matter the location.
- **Code Reviews:** Make adjustments and comments in real-time during code review sessions.
- **Remote Teamwork:** Enable smooth collaboration for distributed teams.
- **Learning and Teaching:** Ideal for tutorials and online mentoring sessions, RustPad allows instructors and students to interact within the same codebase.

## Installation and Setup

### Prerequisites

To build and run RustPad locally, you need:
- **Rust** (Nightly Toolchain)
- **wasm-pack** (For WebAssembly Compilation)
- **Node.js and npm** (For managing frontend dependencies)

### Build and Run

Follow these steps to set up RustPad:

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/yourusername/rustpad.git
   cd rustpad
   cargo build
   cargo run
2. **Build the WebAssembly Project:**
    wasm-pack build --target web
3. **Install Frontend Dependencies and Start the Development Server:**
   npm install
   npm run start
   The frontend will be available at http://localhost:8080

### Usage
Once everything is running, open http://localhost:8080 in your browser to start collaborating. Users can choose a username, and their changes will be synchronized with other collaborators in real-time.

### Features:
**Collaborative Editing:** All changes are synchronized in real-time.
**Cursor Tracking:** Collaborators' cursor positions are shown to prevent conflicts.
**Chat:** In-editor chat allows for real-time discussion while working on the code.

## License
This project is licensed under the MIT License - see the `LICENSE` file for details.
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python 3.x](https://img.shields.io/badge/python-3.x-blue.svg)](https://www.python.org/downloads/)

