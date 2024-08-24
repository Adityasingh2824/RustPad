(function() {
    // Ensure CodeMirror is available globally
    if (typeof window.CodeMirror === "undefined") {
        throw new Error("CodeMirror is not loaded. Please include the CodeMirror library.");
    }

    // Default options for initializing the CodeMirror editor
    const defaultOptions = {
        lineNumbers: true,
        mode: "javascript",  // Default to JavaScript, change as needed
        theme: "default",     // Default theme, can be changed dynamically
        autoCloseBrackets: true,  // Auto-close brackets for better coding experience
        matchBrackets: true,      // Highlight matching brackets
        tabSize: 4,               // Set tab size to 4 spaces
        indentWithTabs: false,    // Use spaces instead of tabs
        scrollbarStyle: "simple", // Simple scrollbar, changeable via add-ons
        lineWrapping: true,       // Enable line wrapping for long lines
    };

    // Function to initialize CodeMirror with specific options
    function initializeCodeMirror(textareaId, customOptions = {}) {
        const textarea = document.getElementById(textareaId);

        if (!textarea) {
            throw new Error(`Textarea with ID ${textareaId} not found.`);
        }

        // Merge custom options with the default options
        const editorOptions = { ...defaultOptions, ...customOptions };

        // Initialize the CodeMirror editor on the specified textarea
        const editor = CodeMirror.fromTextArea(textarea, editorOptions);

        // Return the editor instance for further manipulation if needed
        return editor;
    }

    // Function to dynamically change the mode (language) of the editor
    function changeEditorMode(editor, mode) {
        editor.setOption("mode", mode);
    }

    // Function to dynamically change the theme of the editor
    function changeEditorTheme(editor, theme) {
        editor.setOption("theme", theme);
    }

    // Function to programmatically get the content of the editor
    function getEditorContent(editor) {
        return editor.getValue();
    }

    // Function to programmatically set the content of the editor
    function setEditorContent(editor, content) {
        editor.setValue(content);
    }

    // Function to reset the editor's content and clear history (for new files, etc.)
    function resetEditor(editor) {
        editor.setValue("");
        editor.clearHistory();
    }

    // Expose global interface for RustPad to interact with CodeMirror
    window.RustPadEditor = {
        initialize: initializeCodeMirror,
        changeMode: changeEditorMode,
        changeTheme: changeEditorTheme,
        getContent: getEditorContent,
        setContent: setEditorContent,
        reset: resetEditor,
    };
})();
