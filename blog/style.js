const ID_HAMBURGER_BUTTON = '#hamburger';
const ID_NAV_EXIT_BUTTON = '#nav-exit';

var showNav;
var hideNav;
var showHamburgerButton;
var hideHamburgerButton;
var showSidebar;
var hideSidebar;

function initStyle() {
    // Init all the show/hide actions for the page elements
    setUpActions('#nav', 'nav-active', (show, hide) => {
        showNav = show;
        hideNav = hide;
    });
    setUpActions('#nav-exit', 'nav-exit-active', (show, hide) => {
        showNavExit = show;
        hideNavExit = hide;
    });
    setUpActions(ID_HAMBURGER_BUTTON, 'hamburger-active', (show, hide) => {
        showHamburgerButton = show;
        hideHamburgerButton = hide;
    });
    setUpActions('#sidebar', 'sidebar-active', (show, hide) => {
        showSidebar = show;
        hideSidebar = hide;
    });

    var hamburgerButton = $(ID_HAMBURGER_BUTTON);
    hamburgerButton.click(() => {
        console.log("Clicked hamburger button.");
        showNav();
        showNavExit();
        hideHamburgerButton();
        return false;
    });

    var navExitButton = $(ID_NAV_EXIT_BUTTON);
    navExitButton.click(() => {
        hideNav();
        hideNavExit();
        showHamburgerButton();
        return false;
    });

    $('#contentcontainer').click(() => {
        showHamburgerButton();
        hideNav();
        return false;
    });
}

function setUpActions(id, toggleClass, callback) {
    var element = $(id);
    
    var show = () => {
        element.addClass(toggleClass);
    };

    var hide = () => {
        element.removeClass(toggleClass);
    };

    callback(show, hide);
}