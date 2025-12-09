function initAdminBlocks() {
    // 1. Sortable Logic (for Project Blocks List)
    const list = document.getElementById('blocks-list');
    const saveBtn = document.getElementById('save-order-btn');

    if (list && saveBtn) {
        // Ensure Sortable is loaded (it should be via script tag in template)
        if (typeof Sortable !== 'undefined') {
            const sortable = new Sortable(list, {
                handle: '.drag-handle',
                animation: 150,
                onUpdate: function () {
                    saveBtn.style.display = 'inline-block';
                }
            });

            saveBtn.addEventListener('click', async function () {
                const updates = [];
                const items = list.querySelectorAll('.block-item');

                items.forEach((item, index) => {
                    updates.push({
                        id: item.dataset.id,
                        sort_order: index
                    });
                });

                try {
                    const csrfToken = document.getElementById('csrf_token') ? document.getElementById('csrf_token').value : '';
                    const resp = await fetch('/admin/api/reorder', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                            'X-CSRF-Token': csrfToken
                        },
                        body: JSON.stringify({ updates })
                    });

                    if (resp.ok) {
                        saveBtn.style.display = 'none';
                        window.showAlert('Success', 'Order saved!');
                    } else {
                        window.showAlert('Error', 'Failed to save order.');
                    }
                } catch (e) {
                    console.error(e);
                    window.showAlert('Error', 'Error saving order.');
                }
            });
        }
    }

    // 2. Block Form Initialization (for Block Edit Page)
    if (window.initBlockForm) {
        if (document.getElementById('block-form')) {
            window.initBlockForm();
        }
    }
}

document.addEventListener('DOMContentLoaded', initAdminBlocks);
document.addEventListener('router:load', initAdminBlocks);
