class ClassToggler {

    constructor(id, cssClass) {
        this.element = $(id);
        this.cssClass = cssClass;
    }

    show() {
        console.log('showing ', this.element, " ", this.cssClass);
        this.element.addClass(this.cssClass);
    }

    hide() {
        console.log('hiding ', this.element);
        this.element.removeClass(this.cssClass);
    }

    toggle() {
        if (this.element.hasClass(this.cssClass)) {
            this.hide();
        } else {
            this.show();
        }
    }
}

class NavOptionToggler {

    constructor(id) {        
        this.element = $(id);
        this.height = this.element.height();
        this.element.height(0);
        this.element.css({ opacity: 0 });
    }

    show() {
        this.element
            .delay(150)
            .animate({ height: this.height }, 50)
            .delay(50)
            .animate({ opacity: 1.0 }, 100);
    }
    
    hide() {
        this.element
            .animate({ opacity: 0.0 }, 100)
            .delay(100)
            .animate({ height: 0 }, 50);
    }
    
    toggle() {
        if (this.element.height() > 0) {
            this.hide();
        } else {
            this.show();
        }
    }
}

class ElementInteractionHandler {

    constructor(toggler) {
        this.element = toggler.element;
        this.showElements = [];
        this.hideElements = [];
        this.toggleElements = [];
        this.toggled = false;
    }

    show(...args) {
        args.forEach((arg) => this.showElements.push(arg));
        return this;
    }

    hide(...args) {
        args.forEach((arg) => this.hideElements.push(arg));
        return this;
    }

    toggle(...args) {
        args.forEach((elem) => this.toggleElements.push(elem));
        return this;
    }

    onclick() {
        this.element.click(() => {
            this.showElements.forEach((elem) => elem.show());
            this.hideElements.forEach((elem) => elem.hide());
            this.toggleElements.forEach((elem) => elem.toggle());
        });
    }
}

function initStyle() {

    // Contained on homepage and opens the sidebar
    var HAMBURGER = new ClassToggler('#hamburger', 'hamburger-active');

    // Contains all sidebar elements
    var NAV = new ClassToggler('#nav', 'nav-active');

    // Contained in sidebar and shows/hides the blog options
    var BLOG_OPTIONS_HEADER = new ClassToggler('#blog-options-header', 'blog-options-header-active');

    // Contained in header and shown/hidden by the blog options header
    var BLOG_OPTIONS = new NavOptionToggler('#blog-options');

    // Contained in sidebar and shows/hides the project options
    var PROJECT_OPTIONS_HEADER = new ClassToggler('#project-options-header', 'project-options-header-active');

    // Contained in sidebar and shown/hidden by the project options header
    var PROJECT_OPTIONS = new NavOptionToggler('#project-options');

    //
    // The references themselves are done, so now setup the 
    // relationships between them.
    //

    new ElementInteractionHandler(HAMBURGER)
        .toggle(NAV)
        .onclick();

    new ElementInteractionHandler(BLOG_OPTIONS_HEADER)
        .toggle(BLOG_OPTIONS)
        .hide(PROJECT_OPTIONS)
        .onclick();

    new ElementInteractionHandler(PROJECT_OPTIONS_HEADER)
        .toggle(PROJECT_OPTIONS)
        .hide(BLOG_OPTIONS)
        .onclick();
}

