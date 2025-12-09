console.log("UI Script loaded");

function initUI() {
    // --- Modals Logic ---
    const confirmModal = document.getElementById('confirmModal');
    const confirmBtn = document.getElementById('confirmBtn');
    const cancelBtn = document.getElementById('cancelBtn');
    const confirmTitle = document.getElementById('confirmTitle');
    const confirmMessage = document.getElementById('confirmMessage');
    let confirmCallback = null;

    if (confirmModal) {
        window.showConfirm = function (title, message, callback, btnText = 'Confirm') {
            confirmTitle.textContent = title;
            confirmMessage.textContent = message;
            confirmBtn.textContent = btnText;
            confirmCallback = callback;
            confirmModal.style.display = 'flex';
        };

        // Open modal for forms with class 'confirm-delete'
        // Using event delegation on body to handle dynamically loaded content
        document.body.addEventListener('submit', function (e) {
            if (e.target.classList.contains('confirm-delete')) {
                e.preventDefault();
                window.showConfirm(
                    'Are you sure?',
                    'This action cannot be undone.',
                    () => e.target.submit(),
                    'Delete'
                );
            }
        });

        // Use standard non-delegated listeners for the modal buttons themselves (they inside base.html)
        // Clone and replace to prevent duplicate listeners if re-run
        const newConfirm = confirmBtn.cloneNode(true);
        confirmBtn.parentNode.replaceChild(newConfirm, confirmBtn);

        newConfirm.addEventListener('click', function () {
            if (confirmCallback) {
                confirmCallback();
            }
            confirmModal.style.display = 'none';
        });

        const newCancel = cancelBtn.cloneNode(true);
        cancelBtn.parentNode.replaceChild(newCancel, cancelBtn);
        newCancel.addEventListener('click', function () {
            confirmModal.style.display = 'none';
            confirmCallback = null;
        });

        // Modal Background Click
        confirmModal.onclick = function (e) {
            if (e.target === confirmModal) {
                confirmModal.style.display = 'none';
                confirmCallback = null;
            }
        };
    }

    // --- Alert Modal Logic ---
    const alertModal = document.getElementById('alertModal');
    const alertTitle = document.getElementById('alertTitle');
    const alertMessage = document.getElementById('alertMessage');
    const alertOkBtn = document.getElementById('alertOkBtn');

    if (alertModal) {
        window.showAlert = function (title, message) {
            alertTitle.textContent = title;
            alertMessage.textContent = message;
            alertModal.style.display = 'flex';
        };

        const newOk = alertOkBtn.cloneNode(true);
        alertOkBtn.parentNode.replaceChild(newOk, alertOkBtn);
        newOk.addEventListener('click', function () {
            alertModal.style.display = 'none';
        });

        alertModal.onclick = function (e) {
            if (e.target === alertModal) {
                alertModal.style.display = 'none';
            }
        };
    }

    // --- Publish Modal Logic ---
    const publishModal = document.getElementById('publishModal');
    const publishBtn = document.getElementById('publishConfirmBtn');
    const publishCancelBtn = document.getElementById('publishCancelBtn');
    const publishTitle = document.getElementById('publishTitle');
    const publishMessage = document.getElementById('publishMessage');
    let publishCallback = null;

    if (publishModal) {
        window.showPublish = function (title, message, callback) {
            publishTitle.textContent = title;
            publishMessage.textContent = message;
            publishCallback = callback;
            publishModal.style.display = 'flex';
        };

        const newPublishBtn = publishBtn.cloneNode(true);
        publishBtn.parentNode.replaceChild(newPublishBtn, publishBtn);

        newPublishBtn.addEventListener('click', function () {
            if (publishCallback) {
                publishCallback();
            }
            publishModal.style.display = 'none';
        });

        const newPublishCancel = publishCancelBtn.cloneNode(true);
        publishCancelBtn.parentNode.replaceChild(newPublishCancel, publishCancelBtn);
        newPublishCancel.addEventListener('click', function () {
            publishModal.style.display = 'none';
            publishCallback = null;
        });

        publishModal.onclick = function (e) {
            if (e.target === publishModal) {
                publishModal.style.display = 'none';
                publishCallback = null;
            }
        };
    }

    // --- Menu Closing Logic ---
    const menuCheckbox = document.getElementById('menu__active');
    const menuSection = document.querySelector('.menu');
    const menuLinks = document.querySelectorAll('.menu__listings a');

    if (menuCheckbox && menuSection) {
        // Close menu when a link is clicked
        menuLinks.forEach(link => {
            link.addEventListener('click', () => {
                menuCheckbox.checked = false;
            });
        });

        // Close menu when clicking outside
        // Add listener only once per page load ideally, but simpler to just re-add
        // To avoid duplicates, we can check a flag or just use a named function but 
        // effectively doing it safely:
        document.onclick = function (e) {
            // If menu is open AND click is NOT inside the menu section
            // AND click is not on the label/toggle itself (which handles the check)
            if (menuCheckbox.checked && !menuSection.contains(e.target) && !e.target.closest('label[for="menu__active"]')) {
                menuCheckbox.checked = false;
            }
        };
    }
}

document.addEventListener('DOMContentLoaded', initUI);
document.addEventListener('router:load', initUI);
