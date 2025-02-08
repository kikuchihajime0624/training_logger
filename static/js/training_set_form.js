const eventNameInput = document.getElementById('event_name');
const eventIdSelect = document.getElementById('event_id');
eventNameInput.addEventListener('input', () => {

    if (eventNameInput.value === '') {
        eventIdSelect.disabled = false;
    } else {
        eventIdSelect.disabled = true;
    }

});
eventIdSelect.addEventListener('change', () => {
    if (eventIdSelect.value === '') {
        eventNameInput.disabled = false;
    } else {
        eventNameInput.disabled = true;
    }
})

const partsNameInput = document.getElementById('parts_name');
const partsIdSelect = document.getElementById('parts_id');
partsNameInput.addEventListener('input', () => {

    if (partsNameInput.value === '') {
        partsIdSelect.disabled = false;
    } else {
        partsIdSelect.disabled = true;
    }

});

partsIdSelect.addEventListener('change', () => {
    if (partsIdSelect.value === '') {
        partsNameInput.disabled = false;
    } else {
        partsNameInput.disabled = true;
    }
})
