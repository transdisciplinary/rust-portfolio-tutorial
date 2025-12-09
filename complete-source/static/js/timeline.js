function initTimeline() {
    const labels = document.querySelectorAll('.cards label');

    labels.forEach(label => {
        label.addEventListener('click', function (e) {
            const radio = document.getElementById(this.getAttribute('for'));

            // If already checked, uncheck it
            if (radio.checked) {
                e.preventDefault();
                radio.checked = false;
            }
        });
    });
}

document.addEventListener('DOMContentLoaded', initTimeline);
document.addEventListener('router:load', initTimeline);
