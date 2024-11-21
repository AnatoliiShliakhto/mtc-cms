let obj = JSON.stringify(await dioxus.recv());
let file = new Blob([obj], { type: "application/json" });

if( window.showSaveFilePicker ) {
    let opts = {
        types: [{
            description: 'JSON',
            accept: {'application/json': ['.json']},
        }],
        suggestedName: 'mtc-users',
    };
    let handle = await showSaveFilePicker(opts);
    let writable = await handle.createWritable();
    await writable.write(file);
    writable.close();
} else { alert( "File save error" ); }
