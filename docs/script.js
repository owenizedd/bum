function switchTab(tabName) {
    // Hide all contents
    document.querySelectorAll('.code-block-container').forEach(el => {
        el.classList.remove('active');
    });
    
    // Deactivate all buttons
    document.querySelectorAll('.tab-btn').forEach(el => {
        el.classList.remove('active');
    });

    // Show selected
    document.getElementById('tab-' + tabName).classList.add('active');
    
    // Activate button using event delegation or just finding by text/index. 
    // Here strictly assuming the onclick passer knows what they are doing.
    const buttons = document.querySelectorAll('.tab-btn');
    if(tabName === 'npm') buttons[0].classList.add('active');
    if(tabName === 'shell') buttons[1].classList.add('active');
}

function copyCode(elementId) {
    const text = document.getElementById(elementId).innerText;
    navigator.clipboard.writeText(text).then(() => {
        // Find the button that triggered this
        const btn = document.activeElement;
        const originalHTML = btn.innerHTML;
        
        btn.innerHTML = '<i class="fas fa-check"></i> Copied!';
        setTimeout(() => {
            btn.innerHTML = originalHTML;
        }, 2000);
    });
}

function copyToClipboard(text) {
    navigator.clipboard.writeText(text).then(() => {
        const wrapper = document.querySelector('.copy-command-wrapper');
        const originalHTML = wrapper.innerHTML;
        
        wrapper.innerHTML = '<span class="command-text" style="color:white"><i class="fas fa-check"></i> Copied!</span>';
        
        setTimeout(() => {
            wrapper.innerHTML = originalHTML;
        }, 2000);
    });
}
