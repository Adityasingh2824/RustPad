// wasm.js
async function initWasm() {
    try {
        // Fetch and initialize the WebAssembly module
        const wasm = await fetch('rustpad_bg.wasm'); // Path to the compiled WebAssembly file
        const { instance, module } = await WebAssembly.instantiateStreaming(wasm, {
            env: {
                // Here you can pass custom imports (e.g., memory, table, functions) if needed
                memory: new WebAssembly.Memory({ initial: 256 }), // Example memory allocation
                // Add more functions to interact with the environment if needed
                // Example:
                // console_log: (ptr, len) => {
                //     const message = getStringFromWasm(ptr, len);
                //     console.log(message);
                // }
            }
        });

        // Access the exports of the WebAssembly module
        const { init, update, process_edit, memory } = instance.exports;

        // Call the init function from the Rust side to set up the application
        init();

        console.log("Wasm Module Initialized");

        // Example usage: You can now call the `process_edit` function from Rust
        // You would typically interact with Rust's exported functions here
        function applyEdit(content) {
            const ptr = allocateString(content, memory); // Allocate string in wasm memory
            process_edit(ptr);
        }

        // Example of updating the editor with some content
        applyEdit("Initial content from JavaScript");

    } catch (e) {
        console.error("Failed to initialize WebAssembly module:", e);
    }
}

// Helper function to allocate strings in WebAssembly memory
function allocateString(str, memory) {
    const encoder = new TextEncoder();
    const encodedStr = encoder.encode(str);
    const ptr = instance.exports.alloc(encodedStr.length); // Allocate memory in wasm
    const memoryView = new Uint8Array(memory.buffer, ptr, encodedStr.length);
    memoryView.set(encodedStr);
    return ptr;
}

// Initialize WebAssembly on page load
window.addEventListener('load', () => {
    initWasm();
});
