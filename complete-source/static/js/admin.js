console.log("Admin script loaded");

// Global initialization function for Block Form
window.initBlockForm = function () {
    console.log("initBlockForm called");
    const form = document.getElementById('block-form');
    if (!form) {
        console.log("Block form not found");
        return;
    }

    // Prevent double initialization on the same element
    if (form.dataset.initialized === 'true') {
        console.log("Block form already initialized");
        return;
    }
    form.dataset.initialized = 'true';

    console.log("Initializing Block Form...");

    // Get data from attributes
    const blockType = form.dataset.blockType;
    const contentInput = document.getElementById('content-input');
    const initialContent = contentInput ? contentInput.value : '';

    // Elements
    const textGroup = document.getElementById('text-editor-group');
    const videoGroup = document.getElementById('video-input-group');
    const fileGroup = document.getElementById('file-uploader-group');
    const fileList = document.getElementById('file-list');
    const dropZone = document.getElementById('drop-zone');
    const fileInput = document.getElementById('file-input');
    const saveBtn = document.getElementById('save-btn');

    // State
    let uploadedFiles = [];
    let isUploading = false;

    // Helper to update save button state
    function updateSaveBtn() {
        if (saveBtn) {
            saveBtn.disabled = isUploading;
            saveBtn.textContent = isUploading ? 'Uploading...' : 'Save Block';
            saveBtn.style.opacity = isUploading ? '0.5' : '1';
        }
    }

    // --- UI Initialization ---
    if (blockType === 'Text') {
        if (textGroup) textGroup.classList.remove('hidden');

        // Clear previous instance if any
        const editorContainer = document.getElementById('quill-editor');
        if (editorContainer) {
            editorContainer.innerHTML = '';

            var quill = new Quill('#quill-editor', {
                theme: 'snow',
                modules: {
                    toolbar: [
                        [{ 'header': [1, 2, 3, false] }],
                        ['bold', 'italic', 'underline', 'strike'],
                        ['link', 'blockquote', 'code-block'],
                        [{ 'list': 'ordered' }, { 'list': 'bullet' }],
                        ['clean']
                    ]
                }
            });

            // Custom Link Handler
            var Link = Quill.import('formats/link');
            class CustomLink extends Link {
                static create(value) {
                    let node = super.create(value);
                    value = this.sanitize(value);
                    node.setAttribute('href', value);
                    node.setAttribute('target', '_blank');
                    return node;
                }
                static sanitize(url) {
                    if (!url.match(/^(https?:\/\/|mailto:|tel:)/)) {
                        if (url.match(/^[\w.-]+\.[\w]{2,}/)) {
                            return 'https://' + url;
                        }
                    }
                    return super.sanitize(url);
                }
            }
            Quill.register(CustomLink, true);

            if (initialContent) {
                quill.clipboard.dangerouslyPasteHTML(initialContent);
            }

            quill.on('text-change', function () {
                if (contentInput) contentInput.value = quill.root.innerHTML;
            });
        }

    } else if (blockType === 'Video') {
        if (videoGroup) videoGroup.classList.remove('hidden');
        const vidInput = document.getElementById('video-url-input');
        if (vidInput) vidInput.value = initialContent;
    } else {
        // Gallery, Audio, File
        if (fileGroup) fileGroup.classList.remove('hidden');
        try {
            if (initialContent) {
                const parsed = JSON.parse(initialContent);
                if (blockType === 'Gallery') {
                    uploadedFiles = parsed.map(url => ({ url, title: '' }));
                } else {
                    uploadedFiles = parsed.map(item => ({ url: item[0], title: item[1] }));
                }
                renderFileList();
            }
        } catch (e) {
            console.error("Error parsing initial content", e);
        }
    }

    // --- File Upload Logic ---
    if (dropZone && fileInput) {
        dropZone.addEventListener('click', () => fileInput.click());
        dropZone.addEventListener('dragover', (e) => {
            e.preventDefault();
            dropZone.classList.add('dragover');
        });
        dropZone.addEventListener('dragleave', () => dropZone.classList.remove('dragover'));
        dropZone.addEventListener('drop', (e) => {
            e.preventDefault();
            dropZone.classList.remove('dragover');
            handleFiles(e.dataTransfer.files);
        });
        fileInput.addEventListener('change', (e) => handleFiles(e.target.files));
    }

    async function handleFiles(files) {
        isUploading = true;
        updateSaveBtn();
        for (let file of files) {
            await uploadFile(file);
        }
        isUploading = false;
        updateSaveBtn();
    }

    async function uploadFile(file) {
        // Image Compression Logic
        if (file.type.startsWith('image/')) {
            try {
                console.log('Compressing image:', file.name);
                const options = {
                    maxSizeMB: 1,
                    maxWidthOrHeight: 1920,
                    useWebWorker: true,
                    fileType: 'image/webp'
                };
                const compressedBlob = await imageCompression(file, options);

                // Create new File from Blob with .webp extension
                const newName = file.name.replace(/\.[^/.]+$/, "") + ".webp";
                file = new File([compressedBlob], newName, {
                    type: 'image/webp',
                    lastModified: Date.now()
                });
                console.log('Compression successful:', file.name, file.size);
            } catch (error) {
                console.error('Compression failed, using original file:', error);
            }
        }

        const formData = new FormData();
        formData.append('file', file);

        const progressDiv = document.createElement('div');
        progressDiv.className = 'file-item';
        progressDiv.innerHTML = `<span>Uploading ${file.name}...</span><div class="progress-bar" style="width: 0%"></div>`;
        const progressContainer = document.getElementById('upload-progress-container');
        if (progressContainer) progressContainer.appendChild(progressDiv);

        try {
            const xhr = new XMLHttpRequest();
            xhr.open('POST', '/admin/api/upload', true);
            const csrfToken = document.getElementById('csrf_token') ? document.getElementById('csrf_token').value : '';
            if (csrfToken) {
                xhr.setRequestHeader('X-CSRF-Token', csrfToken);
            }

            xhr.upload.onprogress = (e) => {
                if (e.lengthComputable) {
                    const percent = (e.loaded / e.total) * 100;
                    progressDiv.querySelector('.progress-bar').style.width = percent + '%';
                }
            };

            const promise = new Promise((resolve, reject) => {
                xhr.onload = () => {
                    if (xhr.status === 200) {
                        const resp = JSON.parse(xhr.responseText);
                        resolve(resp);
                    } else {
                        reject(xhr.responseText);
                    }
                };
                xhr.onerror = () => reject("Network error");
            });

            xhr.send(formData);
            const result = await promise;

            uploadedFiles.push({
                url: result.url,
                title: result.original_name
            });
            renderFileList();

        } catch (error) {
            alert("Upload Failed: " + error);
        } finally {
            progressDiv.remove();
        }
    }

    function renderFileList() {
        if (!fileList) return;
        fileList.innerHTML = '';
        uploadedFiles.forEach((file, index) => {
            const div = document.createElement('div');
            div.className = 'file-item';

            let preview = '';
            if (blockType === 'Gallery') {
                preview = `<img src="${file.url}" alt="preview">`;
            } else {
                preview = `<span class="material-icons file-icon">description</span>`;
            }

            const inputPlaceholder = blockType === 'File' ? 'Description' : (blockType === 'Audio' ? 'Song Title' : 'Caption (Optional)');

            div.innerHTML = `
                    ${preview}
                    <div class="file-info">
                        <div class="file-name">${file.url.split('/').pop()}</div>
                        ${blockType !== 'Gallery' ? `<input type="text" class="form-input" placeholder="${inputPlaceholder}" value="${file.title}" onchange="window.updateFileTitle(${index}, this.value)">` : ''}
                    </div>
                    <div class="remove-btn" onclick="window.removeFile(${index})">
                        <span class="material-icons">close</span>
                    </div>
                `;
            fileList.appendChild(div);
        });
    }

    // Expose helpers globally so onclick handlers work
    window.removeFile = function (index) {
        uploadedFiles.splice(index, 1);
        renderFileList();
    };

    window.updateFileTitle = function (index, value) {
        uploadedFiles[index].title = value;
    };

    // --- Form Submission ---
    form.addEventListener('submit', function (e) {
        // CRITICAL: Prevent default submission so we can update values first
        e.preventDefault();

        console.log('Form submitting, block type:', blockType);

        if (blockType === 'Text') {
            // Explicitly sync Quill content to hidden input before submission
            if (typeof quill !== 'undefined' && quill) {
                const htmlContent = quill.root.innerHTML;
                console.log('Quill content being saved:', htmlContent.substring(0, 100));
                if (contentInput) {
                    contentInput.value = htmlContent;
                    console.log('Hidden input value set to:', contentInput.value.substring(0, 100));
                }
            } else {
                console.warn('Quill instance not found!');
            }
        } else if (blockType === 'Video') {
            const vidInput = document.getElementById('video-url-input');
            if (contentInput && vidInput) {
                contentInput.value = vidInput.value;
                console.log('Video URL saved:', vidInput.value);
            }
        } else {
            // Serialize files
            let data;
            if (blockType === 'Gallery') {
                data = uploadedFiles.map(f => f.url);
            } else {
                data = uploadedFiles.map(f => [f.url, f.title]);
            }
            const jsonData = JSON.stringify(data);
            console.log('File data being saved:', jsonData);
            if (contentInput) contentInput.value = jsonData;
        }

        console.log('Final content value:', contentInput ? contentInput.value.substring(0, 100) : 'NO CONTENT INPUT');

        // Now submit the form with updated values
        form.submit();
    });
};
