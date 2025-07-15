const contentPairs = [
    {
        title: "Santosh | $HOME",
        bio: "I don't remember how I built it. But it still feels like home. Neovim, the terminal — these are the places where I'm not the smartest, but I'm myself.",
    },
    {
        title: "Santosh | ~/dev",
        bio: "Code is poetry written in a language only machines truly understand. I'm just here translating dreams into syntax, one semicolon at a time.",
    },
    {
        title: "Santosh | /usr/local/bin",
        bio: "They say home is where the heart is. Mine beats in 60fps, dreams in RGB, and finds comfort in the gentle hum of spinning drives.",
    },
    {
        title: "Santosh | vim ~/.vimrc",
        bio: "Between the brackets and the braces, I've found my sanctuary. Each function call is a prayer, each commit a small act of faith.",
    },
    {
        title: "Santosh | git commit -m 'life'",
        bio: "My relationship status: It's complicated with merge conflicts. But when the code compiles clean, everything feels right with the world.",
    },
    {
        title: "Santosh | sudo su -",
        bio: "I speak fluent sarcasm and broken JavaScript. My IDE knows me better than most humans, and my terminal history tells stories I'd rather forget.",
    },
    {
        title: "Santosh | ps aux | grep dreams",
        bio: "Building digital castles in the cloud, one API call at a time. Sometimes they crash, but that's just the universe teaching me humility.",
    },
    {
        title: "Santosh | cat /dev/random",
        bio: "I collect bugs like some people collect stamps. Each one teaches me something new about the beautiful chaos of software development.",
    },
    {
        title: "Santosh | ls -la ~/thoughts",
        bio: "Late nights, early mornings, and the eternal search for that one missing semicolon. This is my life, and I wouldn't trade it for anything.",
    },
];

function getRandomContentPair() {
    return contentPairs[Math.floor(Math.random() * contentPairs.length)];
}

function updateContent() {
    const titleElement = document.querySelector(".title");
    const bioElement = document.querySelector(".bio");

    const randomPair = getRandomContentPair();
    titleElement.textContent = randomPair.title;
    bioElement.textContent = randomPair.bio;
}

document.addEventListener("DOMContentLoaded", updateContent);
