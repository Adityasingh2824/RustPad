(function(mod) {
    if (typeof exports == "object" && typeof module == "object") // CommonJS
      mod(require("../../lib/codemirror"));
    else if (typeof define == "function" && define.amd) // AMD
      define(["../../lib/codemirror"], mod);
    else // Plain browser env
      mod(CodeMirror);
  })(function(CodeMirror) {
    "use strict";
  
    CodeMirror.defineMode("rust", function(conf) {
      var keywords = {
        "as": "keyword", "break": "keyword", "const": "keyword", "continue": "keyword", 
        "crate": "keyword", "else": "keyword", "enum": "keyword", "extern": "keyword", 
        "false": "keyword", "fn": "keyword", "for": "keyword", "if": "keyword", 
        "impl": "keyword", "in": "keyword", "let": "keyword", "loop": "keyword", 
        "match": "keyword", "mod": "keyword", "move": "keyword", "mut": "keyword", 
        "pub": "keyword", "ref": "keyword", "return": "keyword", "self": "keyword", 
        "Self": "keyword", "static": "keyword", "struct": "keyword", "super": "keyword", 
        "trait": "keyword", "true": "keyword", "type": "keyword", "unsafe": "keyword", 
        "use": "keyword", "where": "keyword", "while": "keyword", "async": "keyword",
        "await": "keyword", "dyn": "keyword", "abstract": "keyword", "become": "keyword",
        "box": "keyword", "do": "keyword", "final": "keyword", "macro": "keyword",
        "override": "keyword", "priv": "keyword", "typeof": "keyword", "unsized": "keyword",
        "virtual": "keyword", "yield": "keyword"
      };
  
      var atoms = {
        "true": "atom", "false": "atom", "None": "atom", "Some": "atom", "Ok": "atom", "Err": "atom"
      };
  
      var operators = /[\+\-\/*%=<>!&|^~]/;
      var delimiters = /[\[\]{}\(\),;\:\.]/;
  
      function tokenBase(stream, state) {
        if (stream.eatSpace()) return null;
  
        var ch = stream.next();
  
        // Handle single-line comments
        if (ch == "/" && stream.eat("/")) {
          stream.skipToEnd();
          return "comment";
        }
  
        // Handle block comments
        if (ch == "/" && stream.eat("*")) {
          state.tokenize = tokenComment;
          return state.tokenize(stream, state);
        }
  
        // Handle strings
        if (ch == '"' || ch == "'") {
          state.tokenize = tokenString(ch);
          return state.tokenize(stream, state);
        }
  
        // Handle numbers
        if (/\d/.test(ch)) {
          stream.eatWhile(/[\w\.]/);
          return "number";
        }
  
        // Handle operators
        if (operators.test(ch)) {
          stream.eatWhile(operators);
          return "operator";
        }
  
        // Handle delimiters
        if (delimiters.test(ch)) {
          return null;
        }
  
        // Handle keywords and atoms
        stream.eatWhile(/\w/);
        var cur = stream.current();
        if (keywords.hasOwnProperty(cur)) {
          return keywords[cur];
        }
        if (atoms.hasOwnProperty(cur)) {
          return atoms[cur];
        }
  
        return "variable";
      }
  
      function tokenComment(stream, state) {
        var maybeEnd = false, ch;
        while ((ch = stream.next()) != null) {
          if (maybeEnd && ch == "/") {
            state.tokenize = tokenBase;
            break;
          }
          maybeEnd = (ch == "*");
        }
        return "comment";
      }
  
      function tokenString(quote) {
        return function(stream, state) {
          var escaped = false, next;
          while ((next = stream.next()) != null) {
            if (next == quote && !escaped) {
              state.tokenize = tokenBase;
              break;
            }
            escaped = !escaped && next == "\\";
          }
          return "string";
        };
      }
  
      return {
        startState: function() {
          return {tokenize: tokenBase};
        },
        token: function(stream, state) {
          return state.tokenize(stream, state);
        },
        blockCommentStart: "/*",
        blockCommentEnd: "*/",
        lineComment: "//"
      };
    });
  
    CodeMirror.defineMIME("text/x-rustsrc", "rust");
  });
  