(function(mod) {
    if (typeof exports == "object" && typeof module == "object") // CommonJS
      mod(require("../../lib/codemirror"), require("../css/css"));
    else if (typeof define == "function" && define.amd) // AMD
      define(["../../lib/codemirror", "../css/css"], mod);
    else // Plain browser env
      mod(CodeMirror);
  })(function(CodeMirror) {
    "use strict";
  
    CodeMirror.defineMode("javascript", function(conf, parserConfig) {
      var indentUnit = conf.indentUnit;
      var statementIndent = parserConfig.statementIndent;
      var jsonldMode = parserConfig.jsonld;
      var jsonMode = parserConfig.json || jsonldMode;
      var isTS = parserConfig.typescript;
      var wordRE = parserConfig.wordCharacters || /[\w$\xa1-\uffff]/;
  
      var keywords = function(){
        function kw(type) {return {type: type, style: "keyword"};}
        var A = kw("keyword a"), B = kw("keyword b"), C = kw("keyword c");
        var operator = kw("operator"), atom = {type: "atom", style: "atom"};
        var jsKeywords = {
          "if": A, "while": A, "with": A, "else": B, "do": B, "try": B, "finally": B,
          "return": C, "break": C, "continue": C, "new": C, "delete": C, "throw": C, "debugger": C,
          "var": kw("var"), "const": kw("var"), "let": kw("var"),
          "function": kw("function"), "catch": kw("catch"),
          "for": A, "switch": A, "case": B, "default": B, "in": operator, "typeof": operator, "instanceof": operator,
          "true": atom, "false": atom, "null": atom, "undefined": atom, "NaN": atom, "Infinity": atom,
          "this": kw("this"), "class": kw("class"), "super": kw("atom"),
          "yield": C, "export": C, "import": C, "extends": C
        };
        if (isTS) {
          jsKeywords["namespace"] = kw("module");
          jsKeywords["type"] = kw("type");
          jsKeywords["interface"] = kw("type");
          jsKeywords["public"] = kw("modifier");
          jsKeywords["private"] = kw("modifier");
          jsKeywords["protected"] = kw("modifier");
          jsKeywords["readonly"] = kw("modifier");
          jsKeywords["abstract"] = kw("modifier");
          jsKeywords["implements"] = C;
        }
        return jsKeywords;
      }();
  
      var isOperatorChar = /[+\-*&%=<>!?|~^]/;
      var isBracketChar = /[[\]{}()]/;
      var atomicTypes = {"atom": true, "number": true, "variable": true, "string": true, "regexp": true};
  
      function readToken(stream, state) {
        var style = state.tokenize(stream, state);
        var kwtype = state.type;
        var ctx = state.context;
        if ((kwtype == "var" || kwtype == "let" || kwtype == "const") && state.lexical.info == "stat") {
          state.lexical.info = "vardef";
        }
        if (kwtype == "def") {
          return style;
        }
        if (ctx && ctx.align == null) {
          ctx.align = true;
        }
  
        if ((kwtype == ";" || kwtype == ":" || kwtype == ",") && ctx && ctx.type == "vardef") {
          state.context = ctx.prev;
        }
        return style;
      }
  
      function handleJSX(stream, state) {
        if (stream.match(/^<\/?[\w\.-]+/) || stream.eat(">")) {
          state.jsxTag = true;
          return "tag";
        }
        return null;
      }
  
      function handleESM(stream, state) {
        if (state.esModule && stream.match(/^(from|import|export)\b/)) {
          return "module";
        }
        return null;
      }
  
      return {
        startState: function() {
          return {
            tokenize: jsTokenBase,
            context: new Context((jsonMode ? "}" : null), 0, "block", false),
            lexical: new Lexical((jsonMode ? "}" : null), 0, "block", false),
            indented: 0,
            startOfLine: true,
            lastType: "sof"
          };
        },
  
        token: function(stream, state) {
          if (stream.sol()) {
            if (state.lastType == "operator" || state.lastType == "sof" || state.lastType == "newstatement")
              state.indented = stream.indentation();
            pushContext(state);
            state.startOfLine = true;
          }
          var style = handleESM(stream, state) || handleJSX(stream, state) || readToken(stream, state);
          if (style == "comment" && state.tokenize != jsTokenComment) style = "error";
          state.startOfLine = false;
          return style;
        },
  
        indent: function(state, textAfter) {
          if (state.tokenize != jsTokenBase && state.tokenize != jsTokenBaseAlt) return 0;
          var firstChar = textAfter && textAfter.charAt(0);
          var ctx = state.context;
          if (ctx.type == "vardef") ctx = ctx.prev;
          var closing = firstChar == ctx.type;
          if (ctx.type == "stat" && statementIndent != null) {
            return ctx.indented + (closing ? 0 : statementIndent);
          }
          if (ctx.align) {
            return ctx.col + (closing ? 0 : 1);
          }
          return ctx.indented + (closing ? 0 : indentUnit);
        },
  
        closeBrackets: {pairs: "()[]{}''\"\"``"},
        electricInput: /^\s*(?:case .*?:|default:|\{|\})$/,
        fold: "brace",
        blockCommentStart: "/*",
        blockCommentEnd: "*/",
        lineComment: "//",
        jsonMode: jsonMode,
        jsonldMode: jsonldMode,
        expressionAllowed: expressionAllowed,
        helperType: "javascript",
        jsx: parserConfig.jsx,
        type: isTS ? "typescript" : "javascript"
      };
    });
  
    function jsTokenBase(stream, state) {
      if (stream.sol()) state.indented = stream.indentation();
      if (stream.eatSpace()) return null;
  
      var style = state.tokenize(stream, state);
      if (style != "comment" && state.type != "variable") state.lastType = state.type;
      if (style == "punctuation" && state.startOfLine) {
        state.lastType = "sof";
        state.startOfLine = false;
      }
      return style;
    }
  
    function jsTokenBaseAlt(stream, state) {
      return jsTokenBase(stream, state);
    }
  
    function jsTokenString(quote, type) {
      return function(stream, state) {
        var escaped = false, next;
        if (jsonldMode && stream.match(/^(?:@[\w\-]+)?[:]{0,1}/)) {
          state.type = "jsonld";
          return "keyword";
        }
        while ((next = stream.next()) != null) {
          if (next == quote && !escaped) {
            state.tokenize = jsTokenBase;
            break;
          }
          escaped = !escaped && next == "\\";
        }
        if (jsonldMode) return "string-2";
        state.type = type;
        return "string";
      };
    }
  
    function jsTokenComment(stream, state) {
      var maybeEnd = false, ch;
      while ((ch = stream.next()) != null) {
        if (ch == "/" && maybeEnd) {
          state.tokenize = jsTokenBase;
          break;
        }
        maybeEnd = (ch == "*");
      }
      state.type = "comment";
      return "comment";
    }
  
    function Context(type, col, align, prev) {
      this.type = type;
      this.col = col;
      this.align = align;
      this.prev = prev;
    }
  
    function Lexical(type, col, info, prev) {
      this.type = type;
      this.col = col;
      this.info = info;
      this.prev = prev;
    }
  
    function pushContext(state) {
      var t = state.type;
      if (t == "stat") t = "block";
      state.context = new Context(t, state.indented, false, state.context);
    }
  
    function expressionAllowed(stream, state, backUp) {
      return (state.lastType == "sof" || state.lastType == "operator" ||
              state.lastType == "keyword c" || state.lastType == "keyword b" ||
              state.lastType == "}" || (state.lastType == ")" && !stream.match(/^:/, false)));
    }
  });
  