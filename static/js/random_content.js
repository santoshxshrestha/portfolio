const contentPairs = [
  {
    title: " echo $HOME > /dev/heart",
    bio: "I don't remember how I built it. But it still feels like home. Neovim,the terminal — these are the places where I’m not the smartest, but I’myself",
  },
  {
    title: " cd ~/dotfiles",
    bio: "Where my system begins and my identity is versioned. Every alias, every keybind — a quiet choice I've made about who I want to be.",
  },
  {
    title: " ~/.config/nixpkgs",
    bio: "My world rebuilds with every flake. Impermanence isn't scary when you can declare everything as code.",
  },
  {
    title: " ~/.cache/sanity",
    bio: "Every line of code I write chips away at the unknown. Somewhere between the bugs and the breakthroughs, I find myself.",
  },
  {
    title: " $EDITOR ~/life.txt",
    bio: "If only real life had syntax highlighting. Still, I try to write clean and meaningful lines where I can.",
  },
  {
    title: " make build",
    bio: "Some days fail with exit code 1. But that's okay — I rerun with more understanding and better flags.",
  },
  {
    title: " systemctl --user start ambition.service",
    bio: "This daemon never sleeps. It logs every crash, every restart — and keeps going.",
  },
  {
    title: " ~/.local/share/time",
    bio: "Time slips through keypresses and terminal tabs. I try to spend it crafting, not just consuming.",
  },
  {
    title: " curl localhost:8080/peace",
    bio: "No route matches ‘peace’ just yet. But I keep sending requests, hoping for a 200 OK one day.",
  },
  {
    title: " while true; do learn; done",
    bio: "This loop defines me. It only breaks when I stop asking questions — which I never will.",
  },
  {
    title: " cd ~/purpose",
    bio: "The directory is empty right now. But every project I start adds a new file.",
  },
  {
    title: " grep -r meaning ~/dev",
    bio: "I search through code and chaos for something more — and sometimes I find it in the comments.",
  },
  {
    title: " echo $PASSION > /dev/heart",
    bio: "The output's silent, but it's always running. Invisible processes keep me alive inside.",
  },
  {
    title: " ssh -t dream@localhost",
    bio: "Connected to something bigger than myself. Every keystroke feels like a step closer.",
  },
  {
    title: " journalctl --boot",
    bio: "The logs are messy, full of warnings and silent successes. But they’re mine — all of them.",
  },
  {
    title: " chmod +x ~/goals",
    bio: "I make my goals executable, not just writable. Permissions matter, in code and in life.",
  },
  {
    title: " zshrc of the soul",
    bio: "Aliased reality, piped dreams. My shell reflects the person I’m trying to become.",
  },
  {
    title: " sleep 8h",
    bio: "One day, maybe. But for now, I’ll just sip code under moonlight and let the compiler hum me to rest.",
  },
  {
    title: " git diff yesterday today",
    bio: "Small changes, but progress. Every line tells a story of effort, failure, and quiet triumph.",
  },
  {
    title: " mount /dev/mind /mnt/focus",
    bio: "I try to stay grounded in systems I barely understand. But that’s the point — to understand them.",
  },
  {
    title: " sudo chown -R santosh ~/destiny",
    bio: "Taking ownership of every path I walk. No more running commands I don’t understand.",
  },
  {
    title: " ~/.config/hypr/hyprland.conf",
    bio: "I tweak pixels like I tweak purpose — minimal, efficient, beautiful to no one but me.",
  },
  {
    title: " read -p 'continue? (y/n)'",
    bio: "Always yes. Even when I don’t feel ready. Especially then.",
  },
  {
    title: " dmesg | tail -n 20",
    bio: "The kernel speaks in errors and warnings. Like me, it struggles sometimes — but it keeps booting.",
  },
  {
    title: 'Santosh | printf "Hello, purpose\\n"',
    bio: "Not everything has to be dynamic. Some truths are best printed clearly and simply.",
  },
  {
    title: " ps aux | grep hope",
    bio: "Still running. Low on memory some days, but never killed — not even by SIGKILL.",
  },
  {
    title: " top",
    bio: "My CPU spikes with ambition, and memory leaks through nostalgia. Still, the system’s alive.",
  },
  {
    title: " g++ -o me main.cpp",
    bio: "Compiled from experience, linked with mistakes. The binary isn’t perfect, but it runs.",
  },
  {
    title: " lsblk",
    bio: "Mounting storage and memories alike. My partitions are scattered but meaningful.",
  },
  {
    title: " df -h",
    bio: "Sometimes I run low on space — mentally, emotionally. That’s when I clean up and begin again.",
  },
  {
    title: " nvim ~/.ideas",
    bio: "Where syntax meets soul. This is where I breathe, edit, and sometimes just stare into the void.",
  },
  {
    title: " uname -a",
    bio: "Just a kernel and a name. But inside — countless threads, endless uptime, and a hunger to learn.",
  },
  {
    title: " make me",
    bio: "Dependencies unresolved. Targets shifting. But the intent — the intent is clear.",
  },
  {
    title: " ping -c 1 thefuture",
    bio: "Still alive. Latency unknown, but hope travels fast.",
  },
  {
    title: " mkdir ~/versions",
    bio: "Every new day gets a new directory. No backups, just lessons.",
  },
  {
    title: " less ~/mistakes.log",
    bio: "I read them often. They don’t scare me anymore — they teach me.",
  },
  {
    title: " tail -f ~/thoughts.log",
    bio: "The stream never ends. I just try to follow it.",
  },
  {
    title: " set +o history",
    bio: "Sometimes I want to try without recording. Quiet experiments. Silent courage.",
  },
  {
    title: " cd /var/lib/intuition",
    bio: "Not everything is documented. Some paths you follow because they feel right.",
  },
  {
    title: " git remote add origin self",
    bio: "My origin isn’t GitHub. It’s me — messy, local, versioned by memory.",
  },
  {
    title: " hyprctl reload",
    bio: "New themes, new configs, same soul. I evolve, but stay me.",
  },
  {
    title: " find / -name passion",
    bio: "It was never lost. Just nested deeper than I expected.",
  },
  {
    title: " rustc life.rs",
    bio: "Strict types. Borrowed time. Undefined behavior when I forget to rest — but powerful when I get it right.",
  },
  {
    title: " crontab -e",
    bio: "I schedule progress like some schedule breaks. Both are necessary.",
  },
  {
    title: " grep '^#' ~/.config",
    bio: "My configs are full of comments I left for future-me. Hope he reads them kindly.",
  },
  {
    title: " nix build .#me",
    bio: "I declare who I want to be. Pure, reproducible, broken only when I stop updating.",
  },
  {
    title: " rsync -av ~/dreams /reality",
    bio: "Syncing is hard. But worth it when dreams reflect something tangible.",
  },
  {
    title: " export PATH=$PATH:~/growth",
    bio: "I keep adding new paths. No need to stick to old binaries forever.",
  },
  {
    title: " tree ~/journey",
    bio: "Deeply nested. Some folders empty, others full. All of them matter.",
  },
  {
    title: " ~/.secrets/env",
    bio: "You don’t need to source everything. Some variables are better left untouched.",
  },
  {
    title: " cargo check",
    bio: "I don’t always build, but I always check. Errors early, peace later.",
  },
  {
    title: " whoami",
    bio: "A student of code. A child of the terminal. A believer in blank screens becoming something meaningful.",
  },
  {
    title: " ~/.vim/autoload/purpose.vim",
    bio: "Sometimes plugins break, and so does motivation. But I debug both — patiently.",
  },
  {
    title: " git stash pop",
    bio: "I pause dreams when I must. But I always come back to them. Always.",
  },
  {
    title: " uptime",
    bio: "System has been running for years. Downtime happens, but restart is always an option.",
  },
  {
    title: " head -n 1 ~/beliefs",
    bio: "The first line is always: keep learning. Everything else follows.",
  },
  {
    title: " ssh into clarity",
    bio: "Each quiet moment is a secure connection to the things I really want.",
  },
  {
    title: " /etc/sudoers",
    bio: "Sometimes I forget how powerful I am. Until I grant myself permission.",
  },
  {
    title: " :wq",
    bio: "Sometimes you don’t need to fix it all. Just write. Just quit. And come back better.",
  },
  {
    title: " cat ~/.bash_history",
    bio: "I look back on past commands, not to regret — but to reflect.",
  },
  {
    title: " clear",
    bio: "A fresh screen. A new start. Not forgetting — just choosing to see forward.",
  },
  {
    title: " man life",
    bio: "Still searching for the right flags. The manual is sparse, but I’m learning the usage patterns.",
  },
  {
    title: " journalctl --since=yesterday",
    bio: "Not all logs are pretty. But every crash report holds a clue to recovery.",
  },
  {
    title: " locate peace",
    bio: "Indexed, but not cached. I find it more often when I stop looking.",
  },
  {
    title: " kill -9 doubt",
    bio: "Force quitting insecurity one process at a time. It respawns sometimes — but weaker each time.",
  },
  {
    title: " ssh santosh@localhost",
    bio: "I’m learning to connect with myself. No password needed — just honesty.",
  },
  {
    title: " grep -i hope /dev/diary",
    bio: "Some days it’s lowercase, others uppercase. But it’s always there if I look close.",
  },
  {
    title: " diff self_old self_now",
    bio: "Changes detected. Improved error handling. Better habits. Fewer segfaults.",
  },
  {
    title: " tar -czf ~/memories.tar.gz ~/nostalgia",
    bio: "Compressed, but never forgotten. Sometimes I unpack them just to remember how far I’ve come.",
  },
  {
    title: " alias santosh='always learning'",
    bio: "A shortcut to who I really am — defined once, echoed always",
  },
  {
    title: " watch -n1 'life --progress'",
    bio: "Nothing flashy. Just quiet updates every second — and that’s enough.",
  },
  {
    title: " echo $BRAIN > /dev/zen",
    bio: "Feeding the machine with mindful thoughts. Between neurons and neural nets, I find calm in computation.",
  },
  {
    title: " tmux attach -t self",
    bio: "I split my mind like panes — focused, parallel, connected. This is how I stay present.",
  },
  {
    title: " nix run .#peace",
    bio: "In a declarative world, I try to make serenity reproducible.",
  },
  {
    title: " rust-analyzer --love",
    bio: "It doesn’t exist as a flag — but maybe it should. Empathy makes every program better.",
  },
  {
    title: " nvim +silent! +normal! gg",
    bio: "Every journey starts at the top. But the real story unfolds as I scroll.",
  },
  {
    title: " zshrc > ~/personality",
    bio: "My shell holds more than commands — it holds habits, humor, and hints of who I am.",
  },
  {
    title: " git log --author='friends'",
    bio: "I trace the commits of kindness and chaos. They all shaped the repo of my heart.",
  },
  {
    title: " nmap -p openworld 0.0.0.0/0",
    bio: "Scanning the world for connection points — hoping something responds with kindness.",
  },
  {
    title: " mv /pain /art",
    bio: "What hurts becomes what teaches. What teaches becomes what shapes.",
  },
  {
    title: " journalctl -u mind",
    bio: "Logs full of overthinking, interrupted by moments of clarity.",
  },
  {
    title: " nix flake show --soul",
    bio: "Everything I am, declared in clean YAML and unspoken hope.",
  },
  {
    title: " while :; do love; done",
    bio: "An infinite loop of kindness, executed without condition.",
  },
  {
    title: " head -n1 ~/truth.txt",
    bio: "The only line that matters: we are here to help each other.",
  },
  {
    title: " rsync -av --progress ~/joy /friends",
    bio: "Sharing happiness like files — recursively, with care.",
  },
  {
    title: " :%!rustfmt",
    bio: "Sometimes life just needs a formatter. Rust taught me to write carefully.",
  },
  {
    title: " alias peace='understanding + patience'",
    bio: "Mapped for daily use. May throw fewer exceptions over time.",
  },
  {
    title: " mkdir ~/clarity && cd ~/clarity",
    bio: "Sometimes I have to create the space before I can enter it.",
  },
  {
    title: " chmod +x ~/voice",
    bio: "Giving myself permission to speak up. Execution begins within.",
  },
  {
    title: " :help life",
    bio: "No manual exists. But asking still helps.",
  },
  {
    title: " scp ~/kindness you:/",
    bio: "Send it often. Encrypt it with trust.",
  },
  {
    title: " echo $FRIENDS | tee /dev/heart",
    bio: "Their names live in memory and overflow into everything I create.",
  },
  {
    title: " tmux new -s dreamspace",
    bio: "In this session, everything’s possible.",
  },
  {
    title: " fd purpose ~",
    bio: "Recursive search for meaning in a deeply nested world.",
  },
  {
    title: " cargo doc --open",
    bio: "Sometimes I need to remind myself how things work — so I build the docs, not just the code.",
  },
  {
    title: " kill -STOP ego",
    bio: "Paused the part of me that always needs to be right.",
  },
  {
    title: " echo 'I believe in people' >> ~/.config/hope",
    bio: "Trusting strangers has given me teachers, friends, and faith.",
  },
  {
    title: " git diff --staged humanity",
    bio: "Changes we haven’t committed yet — but I see the patches.",
  },
  {
    title: " cd ~/questions",
    bio: "Where I always return to — and where most of my growth happens.",
  },
  {
    title: " rustc --edition=2021 mind.rs",
    bio: "Upgraded with empathy and a stricter compiler. Still fearless.",
  },
  {
    title: " sudo pacman -S friendship",
    bio: "Lightweight. Dependency-free. Works on any system.",
  },
  {
    title: " echo $SELF | lolcat",
    bio: "Every version of me is colorful — even the broken ones.",
  },
  {
    title: " nvim --startuptime peace.log",
    bio: "Even in speed, I search for stillness — measuring startup, chasing serenity.",
  },
  {
    title: " tree -L 2 ~/growth",
    bio: "At a glance, you might miss the depth. But it's there.",
  },
  {
    title: " curl -sL universe.sh | bash",
    bio: "Running the universe’s install script — one silent hope at a time.",
  },
  {
    title: " :!make love",
    bio: "No errors. Build successful.",
  },
  {
    title: " htop --sort-key=kindness",
    bio: "Priority threads: those who care. Always.",
  },
  {
    title: " ping -c 3 innerpeace",
    bio: "Responding. TTL looks good.",
  },
  {
    title: " git commit -S -m 'Thank you, friend'",
    bio: "Signed with gratitude. Merged with love.",
  },
];

function getRandomContentPair() {
  return contentPairs[Math.floor(Math.random() * contentPairs.length)];
}

let title = "";
function updateContent() {
  // const titleElement = document.querySelector(".title");
  const bioElement = document.querySelector(".bio");

  const randomPair = getRandomContentPair();
  // titleElement.textContent = randomPair.title;
  title = randomPair.title;
  bioElement.textContent = randomPair.bio;
}

document.addEventListener("DOMContentLoaded", updateContent);
