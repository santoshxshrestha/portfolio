class TerminalTyper {
    constructor() {
        this.commandText = document.getElementById("command-text");
        this.cursor = document.getElementById("cursor");
        this.output = document.querySelector(".output-hidden");
        this.finalPrompt = document.getElementById("final-prompt");
        this.infoLines = document.querySelectorAll(".info-line");

        this.command = "mefetch";
        this.commandText.textContent = "";
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

        for (let line of this.infoLines) {
            line.classList.remove("hidden");
            await this.delay(100);
        }

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
