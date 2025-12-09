function initLightbox() {
    const lightbox = document.getElementById('image-lightbox');
    const lightboxImg = document.getElementById('lightbox-img');

    // Guard clause: if lightbox elements aren't found, stop.
    if (!lightbox || !lightboxImg) return;

    // Check if already initialized to prevent duplicate listeners
    if (lightbox.dataset.initialized === 'true') return;
    lightbox.dataset.initialized = 'true';

    const images = document.querySelectorAll('.gallery-item img');

    images.forEach(img => {
        img.style.cursor = 'zoom-in';
        img.addEventListener('click', (e) => {
            e.stopPropagation();
            lightboxImg.src = img.src;
            lightbox.classList.add('active');
            document.body.style.overflow = 'hidden';
        });
    });

    // Close handler
    const closeLightbox = () => {
        lightbox.classList.remove('active');
        document.body.style.overflow = '';
        setTimeout(() => {
            lightboxImg.src = '';
        }, 300);
    };

    // Click outside to close
    lightbox.addEventListener('click', (e) => {
        if (e.target !== lightboxImg) {
            closeLightbox();
        }
    });

    // Escape key to close
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape' && lightbox.classList.contains('active')) {
            closeLightbox();
        }
    });
}

// Initialize on initial load
document.addEventListener('DOMContentLoaded', initLightbox);

// Initialize after Router navigation
document.addEventListener('router:load', initLightbox);

// Fallback
initLightbox();
