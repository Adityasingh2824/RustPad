(function(mod) {
    if (typeof exports == "object" && typeof module == "object") // CommonJS
      mod(require("../../lib/codemirror"));
    else if (typeof define == "function" && define.amd) // AMD
      define(["../../lib/codemirror"], mod);
    else // Plain browser env
      mod(CodeMirror);
  })(function(CodeMirror) {
    "use strict";
  
    // Default configuration for closing brackets
    var defaults = {
      pairs: "()[]{}''\"\"``",
      closeBefore: ")]}'\":;>",
      triples: "",
      explode: "[]{}"
    };
  
    // Matches the configuration
    var Pos = CodeMirror.Pos;
  
    CodeMirror.defineOption("autoCloseBrackets", false, function(cm, val, old) {
      if (old && old != CodeMirror.Init) {
        cm.removeKeyMap(keyMap);
        cm.state.closeBrackets = null;
      }
      if (val) {
        ensureBoundKeys(cm);
        cm.state.closeBrackets = val;
        cm.addKeyMap(keyMap);
      }
    });
  
    // Get the configuration for specific pairs of characters
    function getOption(conf, name) {
      return conf && conf[name] != null ? conf[name] : defaults[name];
    }
  
    // Determine whether a given character is an opening bracket
    function closingBracket(chars, closed, ch) {
      return chars.indexOf(ch) > -1 && closed.indexOf(ch) == -1;
    }
  
    // Ensure that key bindings are set up
    function ensureBoundKeys(cm) {
      if (!cm.state.boundAutoClose) {
        cm.state.boundAutoClose = true;
      }
    }
  
    // Map of key bindings for automatic bracket closing
    var keyMap = {
      name: "autoCloseBrackets",
      Backspace: handleBackspace,
      "Ctrl-Backspace": handleBackspace,
      Enter: handleEnter,
      "'": handleChar("'"),
      "\"": handleChar("\""),
      "(": handleChar("("),
      "[": handleChar("["),
      "{": handleChar("{"),
      ")": handleChar(")"),
      "]": handleChar("]"),
      "}": handleChar("}"),
      "`": handleChar("`")
    };
  
    function handleChar(ch) {
      return function(cm) {
        var conf = cm.state.closeBrackets;
        var pairs = getOption(conf, "pairs");
  
        if (!closingBracket(pairs, pairs, ch)) return CodeMirror.Pass;
        var ranges = cm.listSelections();
        var opening = ch + pairs.charAt(pairs.indexOf(ch) + 1);
        cm.operation(function() {
          for (var i = ranges.length - 1; i >= 0; i--) {
            var range = ranges[i], cur = range.head, curChar = cm.getRange(cur, Pos(cur.line, cur.ch + 1));
            if (range.empty() && curChar == ch) cm.replaceRange(opening, cur);
            else cm.replaceRange(opening, range.from(), range.to());
          }
          cm.setSelection(cm.getCursor());
        });
      };
    }
  
    function handleBackspace(cm) {
      var conf = cm.state.closeBrackets;
      var pairs = getOption(conf, "pairs");
      var ranges = cm.listSelections();
      var opened = getOption(conf, "explode");
  
      for (var i = 0; i < ranges.length; i++) {
        var range = ranges[i];
        if (!range.empty()) return CodeMirror.Pass;
  
        var cur = range.head;
        var curLine = cm.getLine(cur.line);
  
        if (cur.ch > 0 && closingBracket(pairs, opened, curLine.charAt(cur.ch - 1))) {
          if (curLine.charAt(cur.ch) == pairs.charAt(pairs.indexOf(curLine.charAt(cur.ch - 1)) + 1)) {
            cm.operation(function() {
              cm.replaceRange("", Pos(cur.line, cur.ch - 1), Pos(cur.line, cur.ch + 1));
            });
          }
        }
      }
    }
  
    function handleEnter(cm) {
      var conf = cm.state.closeBrackets;
      var explode = getOption(conf, "explode");
  
      var ranges = cm.listSelections();
      for (var i = 0; i < ranges.length; i++) {
        var range = ranges[i];
        if (!range.empty()) return CodeMirror.Pass;
  
        var cur = range.head;
        var curLine = cm.getLine(cur.line);
  
        if (cur.ch < curLine.length && closingBracket(explode, explode, curLine.charAt(cur.ch))) {
          cm.operation(function() {
            cm.replaceRange("\n\n", cur);
            cm.indentLine(cur.line + 1, null, true);
          });
          cm.setCursor(Pos(cur.line + 1, cm.getLine(cur.line + 1).length));
        }
      }
    }
  });
  