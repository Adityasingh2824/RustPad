(function(mod) {
    if (typeof exports == "object" && typeof module == "object") // CommonJS
      mod(require("../../lib/codemirror"));
    else if (typeof define == "function" && define.amd) // AMD
      define(["../../lib/codemirror"], mod);
    else // Plain browser environment
      mod(CodeMirror);
  })(function(CodeMirror) {
    "use strict";
  
    // Define the scrollbars
    function SimpleScrollbars(cm) {
      this.cm = cm;
  
      // Create the vertical scrollbar
      this.vert = document.createElement("div");
      this.vert.className = "CodeMirror-vscrollbar";
      this.vert.innerHTML = "<div class='CodeMirror-scrollbar-inner'></div>";
      
      // Create the horizontal scrollbar
      this.horiz = document.createElement("div");
      this.horiz.className = "CodeMirror-hscrollbar";
      this.horiz.innerHTML = "<div class='CodeMirror-scrollbar-inner'></div>";
  
      cm.getWrapperElement().appendChild(this.vert);
      cm.getWrapperElement().appendChild(this.horiz);
  
      this.vertWidth = null;
      this.horizHeight = null;
  
      this.updateScrollbars();
      this.cm.on("update", () => this.updateScrollbars());
      this.vert.addEventListener("scroll", () => this.onScroll());
      this.horiz.addEventListener("scroll", () => this.onScroll());
    }
  
    // Update the scrollbars on every update of the CodeMirror content
    SimpleScrollbars.prototype.updateScrollbars = function() {
      const cm = this.cm;
      const scrollInfo = cm.getScrollInfo();
      
      // Set dimensions and max scroll positions
      this.vert.firstChild.style.height = scrollInfo.height + "px";
      this.horiz.firstChild.style.width = scrollInfo.width + "px";
  
      // Calculate scrollbar width/height if not already done
      if (this.vertWidth == null) {
        this.vertWidth = this.vert.offsetWidth - this.vert.clientWidth;
        this.horizHeight = this.horiz.offsetHeight - this.horiz.clientHeight;
      }
  
      // Set scrollbar visibility and position
      if (scrollInfo.height > cm.getWrapperElement().clientHeight) {
        this.vert.style.display = "block";
        this.vert.scrollTop = cm.getScrollTop();
      } else {
        this.vert.style.display = "none";
      }
  
      if (scrollInfo.width > cm.getWrapperElement().clientWidth) {
        this.horiz.style.display = "block";
        this.horiz.scrollLeft = cm.getScrollLeft();
      } else {
        this.horiz.style.display = "none";
      }
    };
  
    // Handle the scrolling event
    SimpleScrollbars.prototype.onScroll = function() {
      if (this.vert.scrollTop !== this.cm.getScrollTop()) {
        this.cm.scrollTo(this.cm.getScrollLeft(), this.vert.scrollTop);
      }
      if (this.horiz.scrollLeft !== this.cm.getScrollLeft()) {
        this.cm.scrollTo(this.horiz.scrollLeft, this.cm.getScrollTop());
      }
    };
  
    // Register the add-on with CodeMirror
    CodeMirror.defineOption("scrollbarStyle", "native", function(cm, val, old) {
      if (old && old != CodeMirror.Init) {
        if (cm.state.simpleScrollbars) {
          cm.state.simpleScrollbars.vert.remove();
          cm.state.simpleScrollbars.horiz.remove();
          cm.state.simpleScrollbars = null;
        }
      }
      if (val == "simple") {
        cm.state.simpleScrollbars = new SimpleScrollbars(cm);
      }
    });
  
  });
  