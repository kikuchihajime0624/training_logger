const formInput = document.getElementById('form');

formInput.addEventListener('submit', (event) => {
    if (event.submitter.id === 'delete_button') {
        if (window.confirm("この記録を削除します") === false) {
            event.preventDefault();
        }
    }
});