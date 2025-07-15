const contentPairs = [
    {
        title: "Santosh | ~/.config/nixpkgs",
        bio: "My world rebuilds with every flake. Impermanence isn't scary when you can declare everything as code.",
    },
    {
        title: "Santosh | ~/.cache/sanity",
        bio: "Every line of code I write chips away at the unknown. Somewhere between the bugs and the breakthroughs, I find myself.",
    },
    {
        title: "Santosh | $EDITOR ~/life.txt",
        bio: "If only real life had syntax highlighting. Still, I try to write clean and meaningful lines where I can.",
    },
    {
        title: "Santosh | make build",
        bio: "Some days fail with exit code 1. But that's okay — I rerun with more understanding and better flags.",
    },
    {
        title: "Santosh | systemctl --user start ambition.service",
        bio: "This daemon never sleeps. It logs every crash, every restart — and keeps going.",
    },
    {
        title: "Santosh | ~/.local/share/time",
        bio: "Time slips through keypresses and terminal tabs. I try to spend it crafting, not just consuming.",
    },
    {
        title: "Santosh | curl localhost:8080/peace",
        bio: "No route matches ‘peace’ just yet. But I keep sending requests, hoping for a 200 OK one day.",
    },
    {
        title: "Santosh | while true; do learn; done",
        bio: "This loop defines me. It only breaks when I stop asking questions — which I never will.",
    },
    {
        title: "Santosh | cd ~/purpose",
        bio: "The directory is empty right now. But every project I start adds a new file.",
    },
    {
        title: "Santosh | grep -r meaning ~/dev",
        bio: "I search through code and chaos for something more — and sometimes I find it in the comments.",
    },
    {
        title: "Santosh | echo $PASSION > /dev/heart",
        bio: "The output's silent, but it's always running. Invisible processes keep me alive inside.",
    },
    {
        title: "Santosh | ssh -t dream@localhost",
        bio: "Connected to something bigger than myself. Every keystroke feels like a step closer.",
    },
    {
        title: "Santosh | journalctl --boot",
        bio: "The logs are messy, full of warnings and silent successes. But they’re mine — all of them.",
    },
    {
        title: "Santosh | chmod +x ~/goals",
        bio: "I make my goals executable, not just writable. Permissions matter, in code and in life.",
    },
    {
        title: "Santosh | zshrc of the soul",
        bio: "Aliased reality, piped dreams. My shell reflects the person I’m trying to become.",
    },
    {
        title: "Santosh | sleep 8h",
        bio: "One day, maybe. But for now, I’ll just sip code under moonlight and let the compiler hum me to rest.",
    },
    {
        title: "Santosh | git diff yesterday today",
        bio: "Small changes, but progress. Every line tells a story of effort, failure, and quiet triumph.",
    },
    {
        title: "Santosh | mount /dev/mind /mnt/focus",
        bio: "I try to stay grounded in systems I barely understand. But that’s the point — to understand them.",
    },
    {
        title: "Santosh | sudo chown -R santosh ~/destiny",
        bio: "Taking ownership of every path I walk. No more running commands I don’t understand.",
    },
    {
        title: "Santosh | ~/.config/hypr/hyprland.conf",
        bio: "I tweak pixels like I tweak purpose — minimal, efficient, beautiful to no one but me.",
    },
    {
        title: "Santosh | read -p 'continue? (y/n)'",
        bio: "Always yes. Even when I don’t feel ready. Especially then.",
    },
    {
        title: "Santosh | dmesg | tail -n 20",
        bio: "The kernel speaks in errors and warnings. Like me, it struggles sometimes — but it keeps booting.",
    },
    {
        title: 'Santosh | printf "Hello, purpose\\n"',
        bio: "Not everything has to be dynamic. Some truths are best printed clearly and simply.",
    },
    {
        title: "Santosh | ps aux | grep hope",
        bio: "Still running. Low on memory some days, but never killed — not even by SIGKILL.",
    },
    {
        title: "Santosh | top",
        bio: "My CPU spikes with ambition, and memory leaks through nostalgia. Still, the system’s alive.",
    },
    {
        title: "Santosh | g++ -o me main.cpp",
        bio: "Compiled from experience, linked with mistakes. The binary isn’t perfect, but it runs.",
    },
    {
        title: "Santosh | lsblk",
        bio: "Mounting storage and memories alike. My partitions are scattered but meaningful.",
    },
    {
        title: "Santosh | df -h",
        bio: "Sometimes I run low on space — mentally, emotionally. That’s when I clean up and begin again.",
    },
    {
        title: "Santosh | nvim ~/.ideas",
        bio: "Where syntax meets soul. This is where I breathe, edit, and sometimes just stare into the void.",
    },
    {
        title: "Santosh | uname -a",
        bio: "Just a kernel and a name. But inside — countless threads, endless uptime, and a hunger to learn.",
    },
    {
        title: "Santosh | make me",
        bio: "Dependencies unresolved. Targets shifting. But the intent — the intent is clear.",
    },
    {
        title: "Santosh | ping -c 1 thefuture",
        bio: "Still alive. Latency unknown, but hope travels fast.",
    },
    {
        title: "Santosh | mkdir ~/versions",
        bio: "Every new day gets a new directory. No backups, just lessons.",
    },
    {
        title: "Santosh | less ~/mistakes.log",
        bio: "I read them often. They don’t scare me anymore — they teach me.",
    },
    {
        title: "Santosh | tail -f ~/thoughts.log",
        bio: "The stream never ends. I just try to follow it.",
    },
    {
        title: "Santosh | set +o history",
        bio: "Sometimes I want to try without recording. Quiet experiments. Silent courage.",
    },
    {
        title: "Santosh | cd /var/lib/intuition",
        bio: "Not everything is documented. Some paths you follow because they feel right.",
    },
    {
        title: "Santosh | git remote add origin self",
        bio: "My origin isn’t GitHub. It’s me — messy, local, versioned by memory.",
    },
    {
        title: "Santosh | hyprctl reload",
        bio: "New themes, new configs, same soul. I evolve, but stay me.",
    },
    {
        title: "Santosh | find / -name passion",
        bio: "It was never lost. Just nested deeper than I expected.",
    },
    {
        title: "Santosh | rustc life.rs",
        bio: "Strict types. Borrowed time. Undefined behavior when I forget to rest — but powerful when I get it right.",
    },
    {
        title: "Santosh | crontab -e",
        bio: "I schedule progress like some schedule breaks. Both are necessary.",
    },
    {
        title: "Santosh | grep '^#' ~/.config",
        bio: "My configs are full of comments I left for future-me. Hope he reads them kindly.",
    },
    {
        title: "Santosh | nix build .#me",
        bio: "I declare who I want to be. Pure, reproducible, broken only when I stop updating.",
    },
    {
        title: "Santosh | rsync -av ~/dreams /reality",
        bio: "Syncing is hard. But worth it when dreams reflect something tangible.",
    },
    {
        title: "Santosh | export PATH=$PATH:~/growth",
        bio: "I keep adding new paths. No need to stick to old binaries forever.",
    },
    {
        title: "Santosh | tree ~/journey",
        bio: "Deeply nested. Some folders empty, others full. All of them matter.",
    },
    {
        title: "Santosh | ~/.secrets/env",
        bio: "You don’t need to source everything. Some variables are better left untouched.",
    },
    {
        title: "Santosh | cargo check",
        bio: "I don’t always build, but I always check. Errors early, peace later.",
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
