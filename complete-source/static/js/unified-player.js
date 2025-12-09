/**
 * Unified Audio Player
 * Handles both native HTML5 Audio and YouTube IFrame API
 * to provide a consistent UI for all audio sources.
 */

class UnifiedPlayer {
    constructor(container) {
        this.container = container;
        this.url = container.dataset.url;

        // Controls
        this.playBtn = container.querySelector('.play-pause-btn');
        this.icon = this.playBtn.querySelector('.material-icons');
        this.progressBar = container.querySelector('.progress-bar');
        this.progressContainer = container.querySelector('.progress-container');
        this.timeDisplay = container.querySelector('.time-display');
        this.trackTitle = container.querySelector('.track-title');

        // Volume Controls
        this.muteBtn = container.querySelector('.mute-btn');
        this.muteIcon = this.muteBtn.querySelector('.material-icons');
        this.volumeSliderContainer = container.querySelector('.volume-slider-container');
        this.volumeSlider = container.querySelector('.volume-slider');
        this.volumeLevel = container.querySelector('.volume-level');

        this.backendContainer = container.querySelector('.player-backend');

        this.isPlaying = false;
        this.isMuted = false;
        this.volume = 100;
        this.duration = 0;
        this.currentTime = 0;
        this.type = this.detectType(this.url);

        this.init();
    }

    detectType(url) {
        if (url.includes('youtube.com') || url.includes('youtu.be')) {
            return 'youtube';
        }
        return 'native';
    }

    init() {
        // Clean URL
        this.url = this.url.trim();

        if (this.type === 'youtube') {
            this.initYouTube();
        } else {
            this.initNative();
        }

        this.playBtn.addEventListener('click', () => this.togglePlay());

        // Seek functionality
        this.progressContainer.addEventListener('click', (e) => {
            const rect = this.progressContainer.getBoundingClientRect();
            const pos = (e.clientX - rect.left) / rect.width;
            this.seekTo(pos * this.duration);
        });

        // Volume functionality
        this.muteBtn.addEventListener('click', () => this.toggleMute());

        this.volumeSliderContainer.addEventListener('click', (e) => {
            const rect = this.volumeSlider.getBoundingClientRect();
            const pos = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
            this.setVolume(pos * 100);
        });
    }

    initNative() {
        this.audio = new Audio(this.url);

        // Set Title only if empty
        if (!this.trackTitle.textContent.trim()) {
            const filename = this.url.split('/').pop().split('?')[0];
            this.trackTitle.textContent = decodeURIComponent(filename);
        }

        this.audio.addEventListener('timeupdate', () => {
            this.updateProgress(this.audio.currentTime, this.audio.duration);
        });

        this.audio.addEventListener('loadedmetadata', () => {
            this.duration = this.audio.duration;
            this.updateTimeDisplay(0, this.duration);
        });

        this.audio.addEventListener('ended', () => {
            this.isPlaying = false;
            this.updateIcon();
        });
    }

    initYouTube() {
        // Extract Video ID and List ID
        let videoId = null;
        let listId = null;

        // 1. Try URL object for standard params
        try {
            const urlObj = new URL(this.url);
            videoId = urlObj.searchParams.get('v');
            listId = urlObj.searchParams.get('list');
        } catch (e) {
            // Not a standard URL, fall back to regex
        }

        // 2. Regex fallback for Video ID (handles youtu.be, embed, etc.)
        if (!videoId) {
            const regExp = /^.*((youtu.be\/)|(v\/)|(\/u\/\w\/)|(embed\/)|(watch\?))\??v?=?([^#&?]*).*/;
            const match = this.url.match(regExp);
            if (match && match[7].length == 11) {
                videoId = match[7];
            }
        }

        if (!videoId && !listId) {
            console.error("Could not extract YouTube ID or List ID from:", this.url);
            this.trackTitle.textContent = "Error: Invalid YouTube URL";
            return;
        }

        // Create a unique ID for the player div
        const playerId = 'yt-player-' + Math.random().toString(36).substr(2, 9);
        const playerDiv = document.createElement('div');
        playerDiv.id = playerId;
        this.backendContainer.appendChild(playerDiv);

        // Wait for YT API to be ready
        window.onYouTubeIframeAPIReady = window.onYouTubeIframeAPIReady || (() => { });

        const initPlayer = () => {
            const playerConfig = {
                height: '0',
                width: '0',
                playerVars: {
                    'playsinline': 1,
                    'controls': 0,
                    'disablekb': 1,
                    'origin': window.location.origin
                },
                events: {
                    'onReady': (event) => {
                        // If it's a playlist, we might need to cue it
                        if (!videoId && listId) {
                            event.target.cuePlaylist({ list: listId });
                        }

                        this.duration = event.target.getDuration();
                        this.updateTimeDisplay(0, this.duration);

                        // Get Title
                        const data = event.target.getVideoData();
                        if (data && data.title) {
                            this.trackTitle.textContent = data.title;
                        } else {
                            this.trackTitle.textContent = listId ? "Playlist Loaded" : "YouTube Audio";
                        }
                    },
                    'onStateChange': (event) => {
                        if (event.data === YT.PlayerState.PLAYING) {
                            this.isPlaying = true;
                            this.startYTProgressLoop();

                            // Update title if it changed (e.g. playlist advanced)
                            const data = event.target.getVideoData();
                            if (data && data.title) {
                                this.trackTitle.textContent = data.title;
                            }
                        } else {
                            this.isPlaying = false;
                            this.stopYTProgressLoop();
                        }
                        this.updateIcon();
                    }
                }
            };

            if (videoId) {
                playerConfig.videoId = videoId;
                if (listId) {
                    playerConfig.playerVars.list = listId;
                }
            } else if (listId) {
                playerConfig.playerVars.listType = 'playlist';
                playerConfig.playerVars.list = listId;
            }

            this.ytPlayer = new YT.Player(playerId, playerConfig);
        };

        if (window.YT && window.YT.Player) {
            initPlayer();
        } else {
            // If API is not loaded yet, push to queue or wait
            // For simplicity in this context, we assume API script is loaded in base.html
            // But we can also hook into the global callback if needed
            // A robust implementation would handle multiple players loading simultaneously
            const checkYT = setInterval(() => {
                if (window.YT && window.YT.Player) {
                    clearInterval(checkYT);
                    initPlayer();
                }
            }, 100);
        }
    }

    startYTProgressLoop() {
        this.ytInterval = setInterval(() => {
            if (this.ytPlayer && this.ytPlayer.getCurrentTime) {
                const time = this.ytPlayer.getCurrentTime();
                const dur = this.ytPlayer.getDuration(); // Duration might update
                this.updateProgress(time, dur);
            }
        }, 500);
    }

    stopYTProgressLoop() {
        clearInterval(this.ytInterval);
    }

    togglePlay() {
        if (this.isPlaying) {
            this.pause();
        } else {
            this.play();
        }
    }

    play() {
        // Stop other players
        document.querySelectorAll('.unified-player').forEach(el => {
            if (el !== this.container && el.playerInstance && el.playerInstance.isPlaying) {
                el.playerInstance.pause();
            }
        });

        if (this.type === 'native') {
            this.audio.play();
        } else if (this.ytPlayer) {
            this.ytPlayer.playVideo();
        }
        this.isPlaying = true;
        this.updateIcon();
    }

    pause() {
        if (this.type === 'native') {
            this.audio.pause();
        } else if (this.ytPlayer && this.ytPlayer.pauseVideo) {
            this.ytPlayer.pauseVideo();
        }
        this.isPlaying = false;
        this.updateIcon();
    }

    stop() {
        this.pause();
        if (this.type === 'native') {
            this.audio.currentTime = 0;
        }
    }

    seekTo(time) {
        if (this.type === 'native') {
            this.audio.currentTime = time;
        } else if (this.ytPlayer) {
            this.ytPlayer.seekTo(time, true);
        }
    }

    toggleMute() {
        this.isMuted = !this.isMuted;
        if (this.isMuted) {
            this.setVolume(0, true);
        } else {
            this.setVolume(this.volume || 100);
        }
    }

    setVolume(percent, isMuteAction = false) {
        if (!isMuteAction) {
            this.volume = percent;
            this.isMuted = percent === 0;
        }

        // Update Backend
        if (this.type === 'native') {
            this.audio.volume = percent / 100;
        } else if (this.ytPlayer) {
            this.ytPlayer.setVolume(percent);
        }

        // Update UI
        this.volumeLevel.style.width = `${percent}%`;

        // Update Icon
        if (percent === 0) {
            this.muteIcon.textContent = 'volume_off';
        } else if (percent < 50) {
            this.muteIcon.textContent = 'volume_down';
        } else {
            this.muteIcon.textContent = 'volume_up';
        }
    }

    updateIcon() {
        if (this.isPlaying) {
            this.icon.textContent = 'pause';
        } else {
            this.icon.textContent = 'play_arrow';
        }
    }

    updateProgress(current, total) {
        this.currentTime = current;
        this.duration = total;
        const percent = (current / total) * 100;
        this.progressBar.style.width = `${percent}%`;
        this.updateTimeDisplay(current, total);
    }

    updateTimeDisplay(current, total) {
        const format = (t) => {
            if (!t) return '0:00';
            const m = Math.floor(t / 60);
            const s = Math.floor(t % 60);
            return `${m}:${s.toString().padStart(2, '0')}`;
        };
        this.timeDisplay.textContent = `${format(current)} / ${format(total)}`;
    }
}

// Initialize all players on page load
document.addEventListener('DOMContentLoaded', () => {
    // Load YouTube API if not already loaded
    if (!document.querySelector('script[src*="youtube.com/iframe_api"]')) {
        const tag = document.createElement('script');
        tag.src = "https://www.youtube.com/iframe_api";
        const firstScriptTag = document.getElementsByTagName('script')[0];
        firstScriptTag.parentNode.insertBefore(tag, firstScriptTag);
    }

    const players = document.querySelectorAll('.unified-player');
    players.forEach(container => {
        if (!container.playerInstance) {
            container.playerInstance = new UnifiedPlayer(container);
        }
    });
});

// Re-initialize on HTMX content swap
document.addEventListener('htmx:afterSwap', () => {
    const players = document.querySelectorAll('.unified-player');
    players.forEach(container => {
        if (!container.playerInstance) {
            container.playerInstance = new UnifiedPlayer(container);
        }
    });
});

// Stop all players before HTMX swap
document.addEventListener('htmx:beforeSwap', () => {
    const players = document.querySelectorAll('.unified-player');
    players.forEach(container => {
        if (container.playerInstance) {
            container.playerInstance.stop();
        }
    });
});
