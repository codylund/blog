function populateTestItems() {
    var archive = document.getElementById("blog");

    var items = [
        {
            "title": "10 Things the Deorderant Industry Doesn't Want You To Know About Cheese",
            "description": "Why you should buy provolone instead of roll-on."
        },
        {
            "title": "Do bears actually shit in the forest?",
            "description": "An educated guess: yes."
        },
        {
            "title": "How one sheep is shaking up the alpaca sweater game",
            "description": "This one is just plain dumb."
        },
        {
            "title": "How many fake titles can I write before I start loosing my creativity?",
            "description": "Hint: we're getting close."
        },
        {
            "title": "How many fake titles can I write before I start loosing my creativity: a Follow Up",
            "description": "Two. The answer is two."
        },
        {
            "title": "10 Things the Deorderant Industry Doesn't Want You To Know About Cheese",
            "description": "Why you should buy provolone instead of roll-on."
        },
        {
            "title": "Do bears actually shit in the forest?",
            "description": "An educated guess: yes."
        },
        {
            "title": "How one sheep is shaking up the alpaca sweater game",
            "description": "This one is just plain dumb."
        },
        {
            "title": "How many fake titles can I write before I start loosing my creativity?",
            "description": "Hint: we're getting close."
        },
        {
            "title": "How many fake titles can I write before I start loosing my creativity: a Follow Up",
            "description": "Two. The answer is two."
        }
    ];

    var sidebarContent = "";
    for (i in items) {
        console.log(items[i]);
        var newDiv = document.createElement("h2"); 
        newDiv.innerText = `${items[i].title}`;
        archive.appendChild(newDiv);
    }
}