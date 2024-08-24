(function(mod) {
    if (typeof exports == "object" && typeof module == "object") // CommonJS
      mod(require("../../lib/codemirror"));
    else if (typeof define == "function" && define.amd) // AMD
      define(["../../lib/codemirror"], mod);
    else // Plain browser environment
      mod(CodeMirror);
  })(function(CodeMirror) {
    "use strict";
  
    // Configuration for the match brackets add-on
    var DEFAULT_OPTIONS = {
      brackets: "[]{}()",
      maxScanLineLength: 10000,
      maxScanLines: 1000,
      showError: true
    };
  
    CodeMirror.defineOption("matchBrackets", false, function(cm, val, old) {
      if (old && old != CodeMirror.Init) cm.off("cursorActivity", doMatchBrackets);
      if (val) cm.on("cursorActivity", doMatchBrackets);
    });
  
    function doMatchBrackets(cm) {
      cm.operation(function() {
        if (cm.state.matchBrackets) {
          var ranges = cm.listSelections();
          for (var i = 0; i < ranges.length; i++) {
            var match = findMatchingBracket(cm, ranges[i].head);
            if (match) highlightMatch(cm, match);
          }
        }
      });
    }
  
    function findMatchingBracket(cm, pos) {
      var maxScanLineLength = DEFAULT_OPTIONS.maxScanLineLength;
      var maxScanLines = DEFAULT_OPTIONS.maxScanLines;
      var brackets = DEFAULT_OPTIONS.brackets;
  
      // Look for a matching bracket
      var line = cm.getLine(pos.line);
      var tokenType = cm.getTokenTypeAt(pos);
      var ch = line.charAt(pos.ch - 1) || line.charAt(pos.ch);
      if (brackets.indexOf(ch) == -1 || (tokenType && /\bcomment\b/.test(tokenType))) return null;
  
      var forward = brackets.indexOf(ch) % 2 === 0, other = brackets.charAt(brackets.indexOf(ch) + (forward ? 1 : -1));
      var stack = [];
      var dir = forward ? 1 : -1;
  
      for (var i = 0; i < maxScanLines; i++) {
        var lineNumber = pos.line + i * dir;
        if (lineNumber < 0 || lineNumber >= cm.lineCount()) break;
  
        var lineText = cm.getLine(lineNumber);
        if (lineText.length > maxScanLineLength) continue;
  
        var offset = dir > 0 ? 0 : lineText.length - 1, end = dir > 0 ? lineText.length : -1;
        for (var j = offset; j != end; j += dir) {
          var nextCh = lineText.charAt(j);
          if (brackets.indexOf(nextCh) !== -1) {
            if (nextCh === ch) stack.push(nextCh);
            else if (nextCh === other && stack.length === 0) return {from: CodeMirror.Pos(lineNumber, j), to: pos};
            else if (nextCh === other) stack.pop();
          }
        }
      }
  
      return null;
    }
  
    function highlightMatch(cm, match) {
      if (!match) return;
  
      cm.addOverlay({
        token: function(stream) {
          if (stream.pos === match.from.ch) return "bracket";
          if (stream.pos === match.to.ch) return "bracket";
          stream.next();
        },
        collapse: true
      });
  
      // Optional: Show an error for unmatched brackets
      if (DEFAULT_OPTIONS.showError) {
        if (!match) {
          cm.addLineClass(match.from.line, "background", "CodeMirror-matchingbracket-error");
        }
      }
    }
  });
  