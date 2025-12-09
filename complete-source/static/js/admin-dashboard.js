function initAdminDashboard() {
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.get('deploy') === 'success') {
        window.showAlert('Success', 'Deployment triggered successfully!');
        // clear param
        window.history.replaceState({}, document.title, window.location.pathname);
    }

    // Handle Deploy Confirmation
    const deployForm = document.getElementById('deploy-form');
    if (deployForm) {
        deployForm.addEventListener('submit', (e) => {
            e.preventDefault();
            window.showPublish(
                'Publish Website?',
                'This will trigger a GitHub Action to build and deploy your static site. It may take a few minutes.',
                () => deployForm.submit()
            );
        });
    }
}

document.addEventListener('DOMContentLoaded', initAdminDashboard);
document.addEventListener('router:load', initAdminDashboard);
