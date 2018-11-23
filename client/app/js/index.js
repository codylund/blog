const wasm = import("../wasm/pkg");

// Initialize the nav bar of topics
wasm.then(module => {
    module.init_nav();

    module.load_repos().then((succeeded) => {
        console.log("Loaded GitHub repos.");
    }, (failed) => {
        console.log("Failed to load GitHub repos.");
    });
});