console.log("Router loaded");

class Router {
    constructor() {
        this.cache = new Map();
        this.init();
    }

    init() {
        // Intercept clicks
        document.addEventListener('click', (e) => {
            const link = e.target.closest('a');
            if (link && this.shouldIntercept(link)) {
                e.preventDefault();
                this.navigate(link.href);
            }
        });

        // Handle Back/Forward
        window.addEventListener('popstate', (e) => {
            if (e.state) {
                this.loadPage(window.location.href, false);
            } else {
                // Initial state, full reload usually handles this but if we pushed state...
                window.location.reload();
            }
        });
    }

    shouldIntercept(link) {
        // 1. Must be same origin
        if (link.origin !== window.location.origin) return false;
        // 2. Ignore downloads, target=_blank, or data-no-router
        if (link.hasAttribute('download')) return false;
        if (link.target === '_blank') return false;
        if (link.hasAttribute('data-no-router')) return false;
        // 3. Ignore simple anchors on same page
        if (link.getAttribute('href').startsWith('#')) return false;

        return true;
    }

    async navigate(url) {
        window.history.pushState({}, '', url);
        await this.loadPage(url);
    }

    async loadPage(url, pushState = true) {
        console.log(`Navigating to ${url}`);

        // Show loading indicator if desired
        document.body.classList.add('loading');

        try {
            const response = await fetch(url, {
                headers: {
                    'X-Router': 'true'
                }
            });

            if (!response.ok) throw new Error('Network response was not ok');

            const html = await response.text();
            this.swapContent(html);

            // Dispatch event for other scripts to re-initialize
            const event = new CustomEvent('router:load', { detail: { url } });
            document.dispatchEvent(event);

        } catch (error) {
            console.error('Navigation failed:', error);
            // Fallback to hard reload
            window.location.href = url;
        } finally {
            document.body.classList.remove('loading');
        }
    }

    swapContent(html) {
        const parser = new DOMParser();
        const doc = parser.parseFromString(html, 'text/html');

        // Swap Main
        const newMain = doc.querySelector('main');
        const oldMain = document.querySelector('main');
        if (newMain && oldMain) {
            oldMain.replaceWith(newMain);
        }

        // Update Title
        document.title = doc.title;

        // Scroll to top
        window.scrollTo(0, 0);
    }
}

// Initial Load
document.addEventListener('DOMContentLoaded', () => {
    window.router = new Router();
});
