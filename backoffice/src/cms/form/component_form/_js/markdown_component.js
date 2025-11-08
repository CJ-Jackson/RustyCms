({
    value: '',
    async init() {
        let editor = await this.$store.loader.easymde({
            element: this.$el,
        });
        this.value = this.$el.dataset.value;
        editor.value(this.$el.dataset.value);
        editor.codemirror.on('change', () => {
            this.value = editor.value();
        })
        editor.codemirror.on('blur', () => {
            this.value = editor.value();
        });
    },
})