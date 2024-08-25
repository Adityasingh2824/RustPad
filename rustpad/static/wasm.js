async function initWasm() {
    try {
        // Fetch and initialize the WebAssembly module
        const response = await fetch('rustpad_bg.wasm'); // Path to the compiled WebAssembly file
        const bytes = await response.arrayBuffer();
        const wasmModule = await WebAssembly.instantiate(bytes, {
            env: {
                memory: new WebAssembly.Memory({ initial: 256 }), // Allocate memory
                // Additional imports can be added here if needed
                // Example for logging from Rust:
                // console_log: (ptr, len) => {
                //     const message = getStringFromWasm(ptr, len);
                //     console.log(message);
                // }
            }
        });

        const { instance } = wasmModule;
        const { init, process_edit, alloc, dealloc, memory } = instance.exports;

        // Initialize the Rust module by calling its init function
        init();
        console.log("Wasm Module Initialized");

        // Example of applying edits from JavaScript to WebAssembly
        function applyEdit(content) {
            const ptr = allocateString(content, memory, alloc); // Allocate string in wasm memory
            process_edit(ptr);
            dealloc(ptr); // Free memory after use
        }

        // Example of interacting with the WebAssembly module:
        applyEdit("Initial content from JavaScript");

    } catch (e) {
        console.error("Failed to initialize WebAssembly module:", e);
    }
}

// Helper function to allocate strings in WebAssembly memory
function allocateString(str, memory, alloc) {
    const encoder = new TextEncoder();
    const encodedStr = encoder.encode(str);
    const ptr = alloc(encodedStr.length); // Allocate memory in wasm
    const memoryView = new Uint8Array(memory.buffer, ptr, encodedStr.length);
    memoryView.set(encodedStr);
    return ptr;
}

// Initialize WebAssembly on page load
window.addEventListener('load', () => {
    initWasm();
});
