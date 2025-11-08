let EasyMDEManager = {
    imported: false,
    init() {
        if (this.imported) {
            return;
        }
        this.imported = true;

        let script = document.createElement('script');
        script.src = 'https://cdn.jsdelivr.net/npm/easymde@2.20.0/dist/easymde.min.js';
        script.integrity = 'sha256-1oTYOxou0ItmTm4UCYTbssJtfmGYplN/yuM1XoXNR7o=';
        script.crossOrigin = 'anonymous';
        document.head.appendChild(script);

        let link = document.createElement('link');
        link.rel = 'stylesheet';
        link.href = 'https://cdn.jsdelivr.net/npm/easymde@2.20.0/dist/easymde.min.css';
        link.integrity = 'sha256-56f+69jkoZPAP4feqfeC1AwOULqouLk8xVL5KqUaYcA=';
        link.crossOrigin = 'anonymous';
        document.head.appendChild(link);
    },
    async start(options = {}) {
        this.init();
        while (window.EasyMDE === undefined) {
            await new Promise(resolve => setTimeout(resolve, 10));
        }
        return new window.EasyMDE(options);
    }
}

export default EasyMDEManager;