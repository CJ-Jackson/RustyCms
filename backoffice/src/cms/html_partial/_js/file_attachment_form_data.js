({
    count: 1,
    add() {
        this.count++;
    },
    remove() {
        if (this.count > 1) {
            this.count--;
        }
    },
    reset() {
        this.count = 1;
    },
    show() {
        return this.count > 1;
    }
})