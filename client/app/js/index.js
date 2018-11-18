import("../wasm/fetch-github-wasm/pkg").then(module => {
    module.get_repos().then((result) => {
        result.forEach((element) => {
            console.log(element);
                
            var name = document.createElement("h1");
            var html_url = document.createElement("h3");
            var updated_at = document.createElement("h4");
            var created_at = document.createElement("h4");
            var language = document.createElement("h5");
            var description = document.createElement("p");
        
            name.innerText = element.name;
            html_url.innerText = element.html_url;
            updated_at.innerText = element.updated_at;
            created_at.innerText = element.created_at;
            language.innerText = element.language;
            description.innerText = element.description;
            
            var project = document.createElement("div");
            project.appendChild(name);
            project.appendChild(html_url);
            project.appendChild(updated_at);
            project.appendChild(created_at);
            project.appendChild(language);
            project.appendChild(description);

            var optionsContainer = document.getElementById("project-options");
            console.log(optionsContainer);
            optionsContainer.appendChild(project);
        });
    });
});
