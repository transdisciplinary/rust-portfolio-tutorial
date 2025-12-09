// Media URL to Embed Converter

class MediaEmbedHandler {
    constructor() {
        this.init();
    }

    init() {
        // Process all video embeds
        document.querySelectorAll('.video-embed').forEach(container => {
            const iframe = container.querySelector('iframe');
            if (iframe && iframe.src) {
                const embedUrl = this.convertVideoUrl(iframe.src);
                if (embedUrl) {
                    iframe.src = embedUrl;
                }
            }
        });

        // Process all audio players
        document.querySelectorAll('.audio-player').forEach(container => {
            const url = container.dataset.url;
            if (url && this.isSpotifyUrl(url)) {
                container.innerHTML = this.createSpotifyEmbed(url);
            }
        });
    }

    convertVideoUrl(url) {
        // YouTube patterns
        const youtubeRegex = /(?:youtube\.com\/watch\?v=|youtu\.be\/)([a-zA-Z0-9_-]+)/;
        const youtubeMatch = url.match(youtubeRegex);
        if (youtubeMatch) {
            return `https://www.youtube.com/embed/${youtubeMatch[1]}`;
        }

        // Vimeo patterns
        const vimeoRegex = /vimeo\.com\/(\d+)/;
        const vimeoMatch = url.match(vimeoRegex);
        if (vimeoMatch) {
            return `https://player.vimeo.com/video/${vimeoMatch[1]}`;
        }

        // If it's already an embed URL or unknown, return as-is
        return url;
    }

    isSpotifyUrl(url) {
        return url.includes('spotify.com');
    }

    createSpotifyEmbed(url) {
        // Convert Spotify URL to embed
        // Track: https://open.spotify.com/track/TRACK_ID
        // Playlist: https://open.spotify.com/playlist/PLAYLIST_ID
        // Album: https://open.spotify.com/album/ALBUM_ID

        let embedUrl = url;

        if (url.includes('open.spotify.com')) {
            embedUrl = url.replace('open.spotify.com', 'open.spotify.com/embed');
        }

        return `
            <iframe 
                src="${embedUrl}" 
                width="100%" 
                height="352" 
                frameborder="0" 
                allowtransparency="true" 
                allow="encrypted-media"
                style="border-radius: 8px;">
            </iframe>
        `;
    }
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => new MediaEmbedHandler());
} else {
    new MediaEmbedHandler();
}
