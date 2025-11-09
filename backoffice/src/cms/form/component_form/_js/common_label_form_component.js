({
    uri: '',
    label: '',
    init() {
        this.uri = this.$el.dataset.uri;
    },
    labelHandle(element) {
        this.label = element.dataset.value;
        element.addEventListener('blur', () => {
            if (this.label !== element.value) {
                this.label = element.value;
                this.save();
            }
        })
        element.addEventListener('keydown', (event) => {
            if (event.key === 'Enter') {
                event.preventDefault();
                if (this.label !== element.value) {
                    this.label = element.value;
                    this.save();
                }
            }
        })
    },
    async save() {
        let response = await this.$store.csrf.fetch(this.uri, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            },
            body: new URLSearchParams({
                label: this.label,
            })
        });
        let toHtml = await response.text();
        await this.$store.util.morphFooterSplit(this.$root, toHtml);
    }
})