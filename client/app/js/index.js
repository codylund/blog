import("../wasm/fetch-github-wasm/pkg").then(module => {
    module.get_repos().then((result) => {
        result.forEach((element) => {
            console.log(element);
        });

        // var project = options.createElement("div");
                
        // var name = options.createElement("h1");
        // var html_url = options.createElement("h3");
        // var updated_at = options.createElement("h4");
        // var created_at = options.createElement("h4");
        // var language = options.createElement("h5");
        // var description = options.createElement("p");
    
        // name.innerText = element.name;
        // html_url.innerText = element.html_url;
        // updated_at.innerText = element.updated_at;
        // created_at.innerText = element.created_at;
        // language.innerText = element.language;
        // description.innerText = element.description;
        
        // project.appendChild(name);
        // project.appendChild(html_url);
        // project.appendChild(updated_at);
        // project.appendChild(created_at);
        // project.appendChild(language);
        // project.appendChild(description);

        // optionsContainer.appendChild(project);
    });
});
// var optionsContainer = document.getElementById("project-options");
// if (optionsContainer) {
//     js.then(js => {
//         js.get_repos().then((result) => {
//             result.forEach(element => {
//                 var project = options.createElement("div");
                
//                 var name = options.createElement("h1");
//                 var html_url = options.createElement("h3");
//                 var updated_at = options.createElement("h4");
//                 var created_at = options.createElement("h4");
//                 var language = options.createElement("h5");
//                 var description = options.createElement("p");
            
//                 name.innerText = element.name;
//                 html_url.innerText = element.html_url;
//                 updated_at.innerText = element.updated_at;
//                 created_at.innerText = element.created_at;
//                 language.innerText = element.language;
//                 description.innerText = element.description;
                
//                 project.appendChild(name);
//                 project.appendChild(html_url);
//                 project.appendChild(updated_at);
//                 project.appendChild(created_at);
//                 project.appendChild(language);
//                 project.appendChild(description);

//                 optionsContainer.appendChild(project);
//             });
//         });
//     });
// }