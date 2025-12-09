function initContact() {
    const emailLink = document.getElementById('email-link');
    if (emailLink) {
        emailLink.addEventListener('click', function (e) {
            e.preventDefault();
            window.showConfirm(
                'Contact',
                'Open your email client?',
                () => window.location.href = 'mailto:contact@example.com',
                'Open'
            );
        });
    }
}

document.addEventListener('DOMContentLoaded', initContact);
document.addEventListener('router:load', initContact);
