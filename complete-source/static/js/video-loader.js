/**
 * Video Loader
 * Detects video provider from URL and injects appropriate embed code.
 * Supports: YouTube, Vimeo, Cloudflare Stream, Mux
 */

function initVideoLoader() {
    const videoWrappers = document.querySelectorAll('.video-wrapper[data-url]');

    videoWrappers.forEach(wrapper => {
        // Skip if already initialized
        if (wrapper.dataset.initialized) return;
        wrapper.dataset.initialized = 'true';

        const url = wrapper.dataset.url.trim();
        if (!url) return;

        let embedHtml = '';

        if (isYouTube(url)) {
            const videoId = getYouTubeId(url);
            if (videoId) {
                embedHtml = `<iframe src="https://www.youtube.com/embed/${videoId}" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>`;
            }
        } else if (isVimeo(url)) {
            const videoId = getVimeoId(url);
            if (videoId) {
                embedHtml = `<iframe src="https://player.vimeo.com/video/${videoId}" frameborder="0" allow="autoplay; fullscreen; picture-in-picture" allowfullscreen></iframe>`;
            }
        } else if (isCloudflare(url)) {
            const videoId = getCloudflareId(url);
            if (videoId) {
                embedHtml = `<iframe src="https://iframe.videodelivery.net/${videoId}" frameborder="0" allow="accelerometer; gyroscope; autoplay; encrypted-media; picture-in-picture;" allowfullscreen="true"></iframe>`;
            }
        } else if (isMux(url)) {
            const playbackId = getMuxId(url);
            if (playbackId) {
                embedHtml = `<mux-player playback-id="${playbackId}" metadata-video-title="Video" accent-color="#FF0000"></mux-player>`;
            }
        } else {
            // Fallback for direct video files (mp4, etc.)
            embedHtml = `<video controls src="${url}" style="width: 100%; height: 100%;"></video>`;
        }

        if (embedHtml) {
            wrapper.innerHTML = embedHtml;
        }
    });
}

document.addEventListener('DOMContentLoaded', initVideoLoader);
document.addEventListener('htmx:afterSwap', initVideoLoader);

// --- Helpers ---

function isYouTube(url) {
    return url.includes('youtube.com') || url.includes('youtu.be');
}

function getYouTubeId(url) {
    let videoId = null;
    try {
        const urlObj = new URL(url);
        videoId = urlObj.searchParams.get('v');
        if (!videoId && url.includes('youtu.be')) {
            videoId = urlObj.pathname.slice(1);
        }
    } catch (e) { }

    if (!videoId) {
        // Handle Shorts: youtube.com/shorts/{ID}
        if (url.includes('/shorts/')) {
            const parts = url.split('/shorts/');
            if (parts.length > 1) {
                videoId = parts[1].split('?')[0];
            }
        }
    }

    if (!videoId) {
        const regExp = /^.*((youtu.be\/)|(v\/)|(\/u\/\w\/)|(embed\/)|(watch\?))\??v?=?([^#&?]*).*/;
        const match = url.match(regExp);
        if (match && match[7].length == 11) videoId = match[7];
    }
    return videoId;
}

function isVimeo(url) {
    return url.includes('vimeo.com');
}

function getVimeoId(url) {
    const regExp = /vimeo.com\/(\d+)/;
    const match = url.match(regExp);
    return match ? match[1] : null;
}

function isCloudflare(url) {
    return url.includes('cloudflarestream.com') || url.includes('videodelivery.net');
}

function getCloudflareId(url) {
    // Handles https://watch.cloudflarestream.com/{id} and https://iframe.videodelivery.net/{id}
    const parts = url.split('/');
    return parts[parts.length - 1];
}

function isMux(url) {
    return url.includes('stream.mux.com');
}

function getMuxId(url) {
    // Handles https://stream.mux.com/{id}.m3u8
    const parts = url.split('/');
    let id = parts[parts.length - 1];
    if (id.includes('.')) {
        id = id.split('.')[0];
    }
    return id;
}
