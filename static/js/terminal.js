class TerminalTyper {
    constructor() {
        this.commandText = document.getElementById("command-text");
        this.cursor = document.getElementById("cursor");
        this.output = document.getElementById("output");
        this.finalPrompt = document.getElementById("final-prompt");
        this.infoLines = document.querySelectorAll(".info-line");

        this.command = "mefetch";
        setTimeout(() => {
            this.startTyping();
        }, 2000);
    }

    async startTyping() {
        for (let i = 0; i < this.command.length; i++) {
            this.commandText.textContent += this.command[i];
            await this.delay(100 + Math.random() * 100);
        }
        await this.delay(500);
        this.cursor.style.display = "none";
        await this.delay(200);
        this.showOutput();
    }

    async showOutput() {
        this.output.classList.remove("hidden");
        this.output.classList.add("typing");
        // Reveal each info line one by one
        for (let line of this.infoLines) {
            line.classList.remove("hidden-line");
            await this.delay(200);
        }
        // Show final prompt after a pause
        setTimeout(() => {
            this.finalPrompt.classList.remove("hidden");
        }, 500);
    }

    delay(ms) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }
}

window.addEventListener("load", () => {
    new TerminalTyper();
});
