class ElementFactory {

    constructor() {
        // Elements created by the factory
        this.elements = [];
    }

    // Applys click action to all elements in the factory
    applyActions() {
        this.elements.forEach((element) => {
            $(element.id).click(() => {
                console.log("Clicked element with id ", element.id);

                // Show all these elements
                if (element.showDependents && Array.isArray(element.showDependents)) {
                    element.showDependents.forEach((other) => {
                        if (other.show) {
                            other.show();
                        }
                    });    
                }
                // Hide all these elements
                if (element.hideDependents && Array.isArray(element.hideDependents)) {
                    element.hideDependents.forEach((other) => {
                        if (other.hide) {
                           other.hide();
                        }
                    });
                }
                return false;
            });
        });
    }

    // Creates a new element with the provided id
    newElement(id, active) {
        var element = new Element(id, active);
        this.elements.push(element);
        return element;
    }
}

class Element {
    constructor(id, active) {
        this.id = id;
        this.active = null;
        if (active) {
            this.show = () => { $(id).addClass(active); };
            this.hide = () => { $(id).removeClass(active); };
        } else {
            this.show = () => { };
            this.hide = () => { };
        }
        this.showDependents = [];
        this.hideDependents =[];
    }
}

function initStyle() {
    // Set-up references for all the interactive DOM elements.
    var elementFactory = new ElementFactory();

    // Contains all primary page content
    var CONTENT_CONTAINER = elementFactory.newElement('body');

    // Contained on homepage and opens the sidebar
    var HAMBURGER = elementFactory.newElement('#hamburger', 'hamburger-active');

    // Contains all sidebar elements
    var NAV = elementFactory.newElement('#nav', 'nav-active');

    // Contained in sidebar and closes the sidebar
    var NAV_EXIT = elementFactory.newElement('#nav-exit', 'nav-exit-active');

    // Contained in sidebar and shows/hides the blog options
    var BLOG_OPTIONS_HEADER = elementFactory.newElement('#blog-options-header', 'blog-options-header-active');

    // Contained in header and shown/hidden by the blog options header
    var BLOG_OPTIONS = elementFactory.newElement('#blog-options');

    // Contained in sidebar and shows/hides the project options
    var PROJECT_OPTIONS_HEADER = elementFactory.newElement('#project-options-header', 'project-options-header-active');

    // Contained in sidebar and shown/hidden by the project options header
    var PROJECT_OPTIONS = elementFactory.newElement('#project-options');

    //
    // The references themselves are done, so now setup the 
    // relationships between them.
    //

    NAV_EXIT.hideDependents.push(NAV, NAV_EXIT);

    HAMBURGER.hideDependents.push(HAMBURGER);
    HAMBURGER.showDependents.push(NAV, NAV_EXIT);

    BLOG_OPTIONS_HEADER.showDependents.push(BLOG_OPTIONS);
    BLOG_OPTIONS_HEADER.hideDependents.push(PROJECT_OPTIONS);

    PROJECT_OPTIONS_HEADER.showDependents.push(PROJECT_OPTIONS);
    PROJECT_OPTIONS_HEADER.hideDependents.push(BLOG_OPTIONS);

    CONTENT_CONTAINER.showDependents.push(HAMBURGER);
    CONTENT_CONTAINER.hideDependents.push(NAV, NAV_EXIT);

    addNavOptionsAnimations(PROJECT_OPTIONS, BLOG_OPTIONS);
    initNavOptionsHeights(PROJECT_OPTIONS, BLOG_OPTIONS);

    elementFactory.applyActions();
}

function addNavOptionsAnimations() {
    for (var i = 0; i < arguments.length; i += 1) {
        var options = arguments[i];
        var optionsDOM = $(options.id);
        
        options.show = getShowFunction(optionsDOM, optionsDOM.height());
        options.hide = getHideFunction(optionsDOM);

        optionsDOM.height(0);
    }
}

function getShowFunction(element, height) {
    return () => {
        element
            .delay(150)
            .animate({ height: height }, 50)
            .delay(50)
            .animate({ opacity: 1.0 }, 100);
    };
}

function getHideFunction(element) {
    return () => {
        element
            .animate({ opacity: 0.0 }, 100)
            .delay(100)
            .animate({ height: 0 }, 50);
    };
}

function initNavOptionsHeights() {
    for (var i = 0; i < arguments.length; i += 1) {
        $(arguments[i].id).height(0);
    }
}