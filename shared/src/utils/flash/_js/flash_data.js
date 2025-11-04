({
    show: true, hide() {
        setTimeout(() => {
            this.show = false;
            setTimeout(() => {
                $el.remove();
            }, 1000);
        }, 5000);
    }
})
