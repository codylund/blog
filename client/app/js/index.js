import("../wasm/fetch-github-wasm/pkg").then(module => {
    console.log(module.init_nav());
});
